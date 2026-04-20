# Design: Rust MCP gateway + API gateway (control plane)

**Status:** Target architecture — informs how Panda’s **inbound** story can evolve from today’s YAML-first MCP host toward a **Microsoft-style control plane** without giving up **Rust performance** or **simple install**.

**Data flow:** The possible **Agent → outer API GW → Panda → corporate API GW → REST** combinations are summarized in **[`panda_data_flow.md`](./panda_data_flow.md)** — keep that doc in mind alongside §4 below. **Feature list:** [`panda_api_gateway_features.md`](./panda_api_gateway_features.md). **Detailed API + MCP gateway design:** [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md).

**Principles:**

1. **One product, two planes** — **Control plane** registers *what* may run (adapters, tools, policies); **data plane** handles *hot* MCP and HTTP traffic at line rate.
2. **Panda’s own API gateway (all-in-one)** — Panda ships a **first-class API gateway** inside the product. It can run **in front of** the MCP gateway (ingress: TLS, routing, auth into MCP/chat) **or behind** the MCP gateway on the path to **corporate** API gateways and REST (egress: outbound HTTP policy toward internal L7). **External** Kong/NGINX/Envoy remains optional **outside** Panda. **`trusted_gateway`** applies when such an **external** hop wraps Panda ([`kong_handshake.md`](./kong_handshake.md)).
3. **Easy path first** — Single static binary, `panda.yaml` **or** minimal bootstrap + control-plane API; Docker one-liner; optional Helm later.
4. **Performance** — Rust **async** I/O, bounded buffers, no mandatory hop through a second language runtime on the data path.
5. **Storage from day one** — One **abstract** control-plane persistence API implemented for **SQLite, PostgreSQL, MySQL** (including **cloud-managed** instances over the same protocols) and **Redis** for cache, pub/sub invalidation, and optional session affinity — not bolted on later as a fork.

---

## 1. Logical architecture

```mermaid
flowchart TB
  subgraph edge["Ingress — Panda API gateway and/or external L7"]
    TLS[TLS / mTLS]
    AUTH[OIDC / API keys]
    ROUTE[Routing to MCP + chat listeners]
  end

  subgraph panda["Panda process — Rust"]
    subgraph cp["Control plane (admin)"]
      API[HTTP: adapters / tools / policies CRUD]
      STORE[(SQL: system of record)]
      REDIS[(Redis: cache · pubsub · sessions)]
      API <--> STORE
      API <--> REDIS
    end

    subgraph dp["Data plane (hot path)"]
      MCPIN[MCP ingress: streamable HTTP / optional stdio bridge]
      ROUTER[Session + tool router]
      AGG[tools/list aggregate · tools/call forward]
      MCPIN --> ROUTER --> AGG
    end

    cp -.->|register routes backends| dp
  end

  subgraph backends["Tool backends"]
    S1[MCP server A]
    S2[MCP server B]
    PROC[Subprocess / remote MCP]
  end

  CLIENT[Agents / IDEs / OpenAI client] --> edge --> dp
  AGG --> S1
  AGG --> S2
  AGG --> PROC

  ADMIN[Platform engineer / GitOps / CI] --> API
```

**Separation of concerns**

| Plane | Responsibility | Latency expectation |
|-------|----------------|----------------------|
| **Control plane** | CRUD for adapters, tool definitions, allowlists, rollout/version pins; optional deploy hooks (K8s Job, compose up) | Milliseconds–seconds OK |
| **Data plane** | MCP JSON-RPC / streamable HTTP, session affinity, tool routing, aggregation, budgets hooks | Sub-millisecond overhead target on top of I/O |

---

## 2. Control plane — resources (Microsoft-inspired, Panda-shaped)

Borrow **concepts** from [microsoft/mcp-gateway](https://github.com/microsoft/mcp-gateway) without copying their stack.

| Resource | Purpose |
|----------|---------|
| **Adapter** | A logical MCP backend: stdio command, remote URL, or K8s service reference + health + version. |
| **Tool registration** | Optional metadata overlay: expose/hide, JSON Schema hints, rate limits, cache policy pointers. |
| **Profile / workspace** | Named bundle of adapters (Docker “profile” analogue) for one-click enablement. |
| **Policy attachment** | Link to `tool_routes`, intent rules, HITL — *declared* in control plane, *enforced* in data plane. |

**API style**

- **REST + OpenAPI** for widest tooling (Bruno, Terraform providers, portals). Optional **gRPC** later for internal high-QPS automation only.
- **Separate listen address** or path prefix, e.g. `https://panda.internal:8443/admin/...` vs public `https://ai.company/mcp` — so edge WAF can **deny** `/admin` from the internet.

**Auth**

- Control plane: **mTLS**, **admin JWT**, or **network policy** (private VPC only). Never reuse “public chat” JWT for adapter delete.
- Align with existing Panda **`identity` / `auth`** where sensible; add **`admin`/`control_plane`** section in config.

---

### 2.1 Unified persistence — SQL + Redis (designed in from the start)

**Two roles (do not conflate them):**

| Role | Technology | Holds |
|------|------------|--------|
| **System of record** | **SQLite**, **PostgreSQL**, or **MySQL** | Adapters, tool registrations, profiles, policy attachments, audit rows, schema version, migration history |
| **Acceleration & coordination** | **Redis** (optional but recommended for multi-replica) | Hot catalog snapshots, **pub/sub** “config changed”, optional **session affinity** leases, rate-limit counters for admin API |

Authoritative **writes** go to **SQL** first; Redis is updated or invalidated **after** commit. If Redis is down, the data plane can fall back to SQL + in-process cache at higher latency — **no silent loss of registry truth**.

**Why support all of these from the beginning**

- **SQLite** — Zero external deps for developers and CI; file in a volume for single-node Docker.
- **PostgreSQL + MySQL** — What enterprises and clouds already run; **managed** offerings use the same wire protocol (e.g. Amazon RDS/Aurora, GCP Cloud SQL, Azure Database for PostgreSQL / for MySQL, AlloyDB-compatible Postgres drivers, etc.).
- **Redis** — The practical way to **fan out** “catalog changed” across N gateway replicas without hammering SQL on every `tools/list`.

**Rust shape (conceptual)**

- A small set of **traits** (names illustrative): `ControlPlaneMetadata` (CRUD + transactions), `CatalogCache`, `ConfigEvents` (pub/sub), `SessionAffinityStore`.
- **One SQL schema** expressed with **portable** DDL where possible; where dialects differ (e.g. `BOOLEAN`, `JSON`, auto-increment), use **versioned migrations** per backend (same tables, different migration files — pattern used by `sqlx` / refinery / SeaORM migrator).
- **Feature flags** on the binary: `sqlite`, `postgres`, `mysql`, `redis` so minimal installs stay small; **full enterprise** builds enable all drivers.

**Example configuration (illustrative)**

```yaml
control_plane:
  enabled: true
  listen: "127.0.0.1:8443"
  store:
    # sqlite | postgres | mysql
    kind: sqlite
    url: "sqlite:panda_control.db?mode=rwc"
    # postgres example: postgresql://user:pass@host:5432/panda_control
    # mysql example: mysql://user:pass@host:3306/panda_control
    # Cloud: same URLs pointing at RDS / Cloud SQL / Azure — TLS via query params or separate tls section
    pool_max: 16
    connect_timeout_ms: 5000
  redis:
    optional: true
    url: "redis://127.0.0.1:6379"
    # Sentinel / cluster URLs as supported by the chosen Redis client
    roles:
      catalog_cache: true
      invalidation_pubsub: true
      session_affinity: false
```

**Data plane read path**

- In-memory **aggregated tool catalog** per process, rebuilt on: startup, SQL poll (fallback), or **Redis pub/sub** “version bumped”.
- `tools/list` stays **off the SQL hot path** when cache is warm; **first request** or **after change** may refresh from SQL once per replica.

---

### 2.2 How developers vs enterprises use the control plane

| Persona | Typical stack | How they interact | What “easy” means |
|---------|----------------|-------------------|-------------------|
| **Solo developer** | SQLite only, no Redis | `panda.yaml` + optional `POST /admin/v1/...` from Bruno; or **import** a small YAML bundle | Single binary, no DB server to install |
| **Small team** | SQLite or Postgres in Docker Compose; Redis optional | Shared compose file; GitOps **export/import** of adapter definitions | Same APIs as enterprise, smaller footprint |
| **Enterprise platform** | **Postgres or MySQL** (often **managed**), **Redis** cluster, many Panda replicas | Private admin listener; **CI/CD** applies manifests; **RBAC** on admin API; audit to SIEM | One operational model: SQL truth + Redis fan-out |

**Enterprise extras (same product, toggled on):**

- **Audit log** table + optional webhook export
- **Read replicas** for SQL (reports / console) while writes go primary
- **Per-tenant** namespaces in metadata (schema: `tenant_id` on rows) when multi-tenant control plane is required

---

### 2.3 Control plane UX — simple default, full features when needed

**Layers (progressive disclosure):**

1. **Declarative file** — Check `adapters.yaml` into git; CI runs `import` or sidecar sync — no manual clicking.
2. **REST + OpenAPI** — Full CRUD, filtering, pagination; same semantics regardless of SQLite vs Aurora.
3. **Export / import** — `GET /admin/v1/export?format=yaml` and `POST /admin/v1/import` for backups and GitOps (validate + dry-run first).
4. **Optional UI** — Developer console tab “MCP registry” calling the same API (no second source of truth).

**“Easy but complete” rule:** The **default** developer path is **SQLite + file import**; the **default** enterprise path is **Postgres/MySQL URL + Redis** — **no separate product**, only config and scale.

---

## 3. Data plane — MCP gateway behavior

**Ingress**

- **Streamable HTTP MCP** (primary for multi-client, K8s-friendly): one URL per deployment or per adapter, e.g. `/mcp` or `/adapters/{id}/mcp`.
- **Stdio** remains valuable for local dev and “sidecar in pod”; can be a **child process** spawned by the gateway or a **relay** from IDE → TCP → gateway.

**Routing**

- **Tool router** — Like Microsoft’s “tool gateway router”: single MCP endpoint that dispatches `tools/call` by tool name → backend adapter.
- **Session affinity** — If backends are stateful, key on `session_id` (header or MCP metadata) → consistent backend. Implementation options: sticky LB, gateway-side consistent hash, or embedded session table (Redis when replicated).

**Performance tactics (Rust)**

- **Bounded** body and frame sizes; cancel on client disconnect.
- **Pool** outbound HTTP connections to remote MCP servers.
- **Aggregate `tools/list`** with incremental cache invalidation on control-plane events.
- **Avoid** per-request allocation hot paths where profiling shows regressions; prefer **`Bytes`**, pre-serialized tool catalogs when unchanged.

**OpenAI integration**

- Keep today’s model: chat ingress on same process optionally **injects** OpenAI function list from aggregated MCP tools (`mcp_openai` shaping). Data plane reads **same** in-memory catalog as native MCP clients.

---

## 4. Panda API gateway — all-in-one, ingress and/or egress

**Product intent:** Panda includes **its own API gateway** as a **core** building block (same binary / same deployment), not an afterthought. It is designed to work:

| Placement | Flow (simplified) | Role |
|-----------|-------------------|------|
| **In front of MCP** | Agent → **Panda API gateway (ingress)** → **Panda MCP** → … | Perimeter into MCP and chat: TLS, routing, auth, coarse limits. |
| **Behind MCP** | … → **Panda MCP** → **Panda API gateway (egress)** → **corporate API gateway** → REST | Outbound HTTP from tools: signing, routing, resilience, identity toward **existing** corporate L7. |
| **Both** | Agent → **Panda API GW (ingress)** → **Panda MCP** → **Panda API GW (egress)** → corporate L7 → REST | **All-in-one** path without requiring a separate product for either hop. |

**External L7 (optional):** Many enterprises still place **Kong / NGINX / Envoy** **outside** Panda (`Agent → Kong → Panda …`). That does **not** replace Panda’s API gateway; it **adds** an outer policy layer. Use **`trusted_gateway`** when that outer hop sets identity headers ([`kong_handshake.md`](./kong_handshake.md)).

**Scope honesty:** Panda’s API gateway targets **AI gateway + MCP** operational needs: **TLS management**, **load balancing**, **rate limiting**, routing, auth, HTTP egress to corporate APIs, and a **simple developer portal** to start (OpenAPI, tool catalog, then API keys). It is **not** a full **enterprise marketplace** or **billing engine** on day one — integrate for those depths when required ([`panda_api_gateway_features.md`](./panda_api_gateway_features.md)).

### 4.1 Diagrams (canonical)

See **[`panda_data_flow.md`](./panda_data_flow.md)** for the diagrams and checklist. The combined enterprise shape is often:

**Agent → [optional external Kong] → Panda API gateway (ingress) → Panda MCP → Panda API gateway (egress) → corporate API gateway → internal REST**

---

## 5. Install, deploy, use

**Goals**

| Persona | Experience |
|---------|------------|
| **Developer** | `cargo run -p panda-server -- panda.yaml` or `docker run …` with one volume for config |
| **Platform** | Helm chart or Compose: 1–3 replicas, Redis optional, secrets from env/vault |
| **Security** | Control plane off public internet; use Panda API gateway **ingress** and/or **egress** per [`panda_data_flow.md`](./panda_data_flow.md); optional external Kong on top |

**Suggested packaging**

1. **Single binary** — `panda-server` (or split `panda-mcpd` only if binary size forces it; default is **one** artifact with feature flags).
2. **Config bootstrap** — Either file-only (GitOps) **or** “empty start + first `POST /adapters`” for lab; production should prefer **declarative export/import** (YAML/JSON bundle) for reviewability.
3. **Health** — `GET /health`, `GET /ready`, `GET /mcp/status` extended with **control-plane readiness** (store connected, schema version).

---

## 6. Evolution from today’s codebase

Today: **`mcp.*` in `panda.yaml`**, `inbound/mcp*`, subprocess stdio — **no** separate control-plane API.

**Suggested phases**

| Phase | Deliverable |
|-------|-------------|
| **0 — Current** | YAML MCP + API gateway doc; Phase 1 scope ([`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md)) |
| **1 — Catalog in-process** | Load adapters from **file/CRD-style YAML**; data plane router by name; still no mutating HTTP API |
| **2 — Control plane v1** | Admin REST + **metadata trait** with **SQLite + Postgres + MySQL** drivers; **Redis** optional for cache/pubsub; streamable HTTP MCP ingress; session affinity MVP |
| **3 — Enterprise** | HA SQL (managed cloud), Redis cluster, audit log, GitOps sync, export/import hardening, optional K8s operator |

This avoids a “big bang” rewrite: each phase keeps the **prior** config path working.

---

## 7. Non-goals (for this design)

- Replacing the customer’s **full** API-management suite (complex marketplaces, full billing products, vast plugin bazaars). **Panda** ships **LB, rate limits, TLS management**, and a **simple developer portal** first; outer Kong/Envoy remains common for org-wide programs that need extra depth ([`panda_api_gateway_features.md`](./panda_api_gateway_features.md)).
- Mandating **Kubernetes** for first usable install — K8s is a **deployment option**, not a prerequisite.
- **Guaranteed** wire compatibility with Microsoft or Docker gateways — **learn** from them; define Panda’s OpenAPI and MCP URLs explicitly.

---

## Related docs

- [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) — MCP + Panda API gateway implementation milestones  
- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — minimal inbound scope today  
- [`mcp_gateway_reference_designs.md`](./mcp_gateway_reference_designs.md) — Docker / Microsoft reference notes  
- [`architecture_two_pillars.md`](./architecture_two_pillars.md) — inbound vs outbound  
- [`protocol_evolution.md`](./protocol_evolution.md) — future protocols  
- [`kong_handshake.md`](./kong_handshake.md) — edge trust contract  

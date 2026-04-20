# Control plane — evolution roadmap

**Purpose:** Order work on the control plane so it stays **useful before it is “enterprise-complete”**: first expand **what operators can configure through CP** to cover **all major shipped features** (parity with static YAML where it makes sense), **then** **full multi-tenant RBAC** + **row namespacing**, **then** **extra SQL backends** (MySQL / Azure SQL) only where customers require them.

**See also:** [`panda_config::ControlPlaneConfig`](../crates/panda-config/src/lib.rs), [`enforce_control_plane_auth_async`](../crates/panda-proxy/src/lib.rs), [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) (**E1**), [`security_review_gate.md`](./security_review_gate.md).

---

## Strategic order (agreed)

| Step | Goal | Rationale |
|------|------|-----------|
| **1 — CP feature coverage** | Operators can use the control plane to **configure and evolve** the same capabilities that today require **editing YAML** / restarts (or only partial hot paths), with **export/import**, **versioning story**, and **clear scope** per resource type. | Without broad **config coverage**, RBAC would only guard a **small** surface; **GitOps + YAML** remains the real source of truth for everything else. |
| **2 — Full multi-tenant RBAC + row namespacing** | **Tenant isolation**, **platform vs tenant admin**, **scoped list/get/import**, optional **Postgres RLS**, **audit**. | Depends on **stable resource model** and APIs from step 1; otherwise policies have nothing consistent to attach to. |
| **3 — Additional SQL backends** | **MySQL** / **Azure SQL** (or similar) for `control_plane.store` | **High** maintenance; only after **Postgres** path and **policy** are mature, unless a customer mandates a specific engine early. |

Steps **2** and **3** are independent in theory, but **RBAC usually comes before** new stores so you do not duplicate **migrations × dialects × security** unnecessarily.

---

## Coverage today: YAML vs control plane

**Rule of thumb:** Static [`panda.yaml`](../panda.yaml) is still the **bootstrap** (listen, TLS, `control_plane` store URL, secrets **references**). The **control plane HTTP API** is for **dynamic** data and **operational** changes without rolling a full file.

| Area | Today (typical) | Control plane today | Direction (step 1) |
|------|-----------------|----------------------|----------------------|
| **API gateway — ingress routes** | Static `api_gateway.ingress.routes` + **dynamic** rows | **Yes:** `GET/POST/DELETE` routes, `export` / `import`, persisted store | **Done** for this resource family; extend with **more fields** / **validation** as catalog grows. |
| **API gateway — top-level chat routes** | `routes[]` in YAML | **No** | **Candidate:** optional dynamic route table or “effective route overlay” — **design** heavy (interaction with `forward_to_upstream`). |
| **MCP** — servers, `tool_routes`, limits | YAML | **Read-only:** `GET …/v1/runtime/summary` (MCP slice + `api_gateway` effective flags) and **`GET …/v1/mcp/config`** (non-secret YAML summary: transports, counts, limits) **without** URLs or commands | **Candidate:** mutable **dynamic** MCP surface (servers, tool allow/deny, **safe** reload of stdio vs HTTP) — **process / lifecycle** constraints. |
| **Egress** — allowlist, profiles, pools | YAML | **Read-only:** `ingress` / `egress` **effective** flags and counts under **`GET …/v1/runtime/summary`** (same shape as `api_gateway` in `/portal/summary.json`; no allowlist URLs) | **Candidate:** **subset** mutation API (e.g. extra `allow_hosts` lines, profile headers) with **threat review** (SSRF). |
| **Identity / JWT / budgets / TPM** | YAML | **Read-only:** **`GET …/v1/runtime/summary`** exposes **`identity`** (`require_jwt`, `jwks_url_configured`), **`tpm`** (enforce flag, per-minute cap, Redis **configured** booleans — **no** URLs), **`budget_hierarchy`** (enabled, claim name, department count, Redis **configured**, **`runtime_active`**) — same spirit as **`/portal/summary.json`** | **Candidate:** policy **fragments** or **mutable** references; richer JWT claim introspection. |
| **Semantic cache / routing / plugins** | YAML | **Read-only:** `semantic_cache` / `agent_sessions` / `model_failover` / **`plugins`** (`wasm_runtime_loaded`) on **`GET …/v1/runtime/summary`** (not full tuning surface) | **Candidate:** tunables (thresholds, enable flags) **if** safe hot-reload exists in-process. |
| **Control plane meta** | YAML (`control_plane.*`) | **Self** | Changing **store URL** / **auth** may always require **restart** or **out-of-band** GitOps — document exceptions. |

The **inventory** above is a **backlog map**, not a commitment that every row becomes CP-managed. Some features will **remain YAML-first** (TLS material, root secrets) by policy.

---

## Step 1 — Expand CP coverage (phased engineering)

**Shipped in-repo:** **`GET {path_prefix}/v1`** returns a **discovery JSON** document (`panda_control_plane_api`, `endpoints` with methods and descriptions, no secrets) — including **`/v1/runtime/summary`** and **`/v1/mcp/config`**. Same auth as other CP routes. Listed under **`capabilities`** as **`discovery`** on **`GET …/v1/status`**.

Suggested **internal** order (adjust per product):

1. **Ingress completeness** — **`validate_ingress_route_row`** / **`ApiGatewayIngressRoute::validate_for_control_plane`** (same rules as static YAML) runs on **POST** upsert and **import** (including **duplicate** `(tenant_id, path_prefix)` detection on **replace** batches); **`GET /v1/status`** exposes **`api_gateway.ingress_enabled`**, **`egress_enabled`**, **`ingress_router_built`**.
2. **Observability & safety** — **Shipped:** **`GET {path_prefix}/v1/runtime/summary`** (`kind`: `panda_control_plane_runtime_summary`) — listener/routing/identity/MCP/plugins/semantic cache/agent sessions/model failover/budget hierarchy/TPM + **`api_gateway`** (parity with **`GET /portal/summary.json`** for non-secret fields); **`GET {path_prefix}/v1/mcp/config`** (`kind`: `panda_control_plane_mcp_config`) — MCP YAML summary only (no secrets). Listed on **`GET …/v1/status`** under **`capabilities`** as **`runtime_summary`** and **`mcp_config_read`**. **Not** full mutation until modeled.
3. **MCP operational surface** — smallest useful **dynamic** slice (e.g. **tool_routes** or **remote_mcp_url** overrides) that **reloads** without full process restart where **safe**.
4. **Egress** — **narrow** mutable allowlist or profile **delta** API with **strict** validation and **audit** logging.

Each slice needs: **JSON schema** (or Rust struct), **store** table or blob column, **migrations** for Postgres, **authz** hook (at least **admin vs read-only** using today’s `ControlPlaneAccess`).

---

## Step 2 — Multi-tenant RBAC + row namespacing (items 8 & 10 — enterprise)

| Item | Essence | Shipped today (baseline) | Still open |
|------|---------|---------------------------|------------|
| **8 — Multi-tenant RBAC + row namespacing** | Isolation + **roles** on **all** CP resources. | **Partial:** `tenant_id` on ingress rows; **tenant-scoped Redis API keys**; **RO** credentials; mutation checks for scoped keys on ingress. | **Full** matrix, **list/filter** by tenant, **import** rules, **platform admin**, **RLS**, **audit**. |
| **10 — OIDC + Redis key UX** | IdP + **scoped** keys without only shared secrets. | **OIDC** RW/RO; **Redis** keys with **`scopes`** / **`tenant_id`**; issue/revoke HTTP. | **Bearer** CP JWT, **richer claims**, **portal UI**, **list/rotate** APIs. |

Detailed behavior matches the **baseline** table in the next section.

---

## What exists today (baseline) — accurate

| Area | Behavior |
|------|----------|
| **Route prefix** | `control_plane.enabled` + `path_prefix` (default logical base `/ops/control/...`). |
| **Persistence** | `store.kind`: **memory** \| **json_file** \| **sqlite** \| **postgres** (`sqlx` migrations under `crates/panda-proxy/migrations/`). |
| **Replication** | `reload_from_store_ms`, Postgres **NOTIFY**, `reload_pubsub` (Redis **PUBLISH**). |
| **Tenant header (ingress runtime)** | `tenant_resolution_header` — used for **ingress** dynamic row matching (`tenant_id` on routes), **not** automatically for all CP authorization (see **scoped API keys**). |
| **Auth — shared secrets** | `observability.admin_secret_env` + `additional_admin_secret_envs` → **read/write** CP access. |
| **Auth — read-only secrets** | `read_only_secret_envs` → **GET-only** CP (mutations **403**). |
| **Auth — OIDC (console session)** | `allow_console_oidc_session` + **`required_console_roles`** / **`mode`** → **RW**; **`oidc_read_only_roles`** / **`mode`** → **RO** when RW not granted. |
| **Auth — Redis API keys** | Header + Redis; JSON **`scopes`** + optional **`tenant_id`**; **ReadWrite** \| **ReadOnly**; tenant-scoped keys **403** on mismatched ingress mutations. |
| **Access model** | `ControlPlaneAccess` + `api_key_tenant_id` for scoped keys. |
| **Read-only effective snapshots** | **`GET …/v1/runtime/summary`** — `panda_control_plane_runtime_summary`: **`listener`**, **`routing`**, **`identity`**, MCP, **`plugins`**, semantic cache, agent sessions, model failover, **`budget_hierarchy`**, **`tpm`**, **`api_gateway`** (no secrets, no Redis URLs). **`GET …/v1/mcp/config`** — `panda_control_plane_mcp_config`: per-server **`transport`**, HITL **`approval_url_configured`** only, tool route / cache **counts**. Same auth as **`GET …/v1`** / **`GET …/v1/status`**. |

---

## Step 3 — Additional SQL backends

**Postgres** already covers most managed PostgreSQL offerings. **MySQL** / **SQL Server** only with **clear** customer demand and **duplicated** migration discipline — see earlier **strategic order**.

---

## External control plane (optional)

GitOps / Terraform / vendor API; Panda **export/import** remains a valid **integration** path for teams that **never** want in-proxy CP mutation.

---

## Quick wins

- **Metrics / audit:** dedicated counters for CP **deny** reasons (`tenant_forbidden`, `read_only_mutate`).
- **Docs:** keep [`panda.example.yaml`](../panda.example.yaml) **control_plane.auth** comments in sync with auth fields.
- **E1 backlog:** [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) — step **1** (coverage) before **2** (RBAC) / **3** (backends).

---

## Backlog ID

**E1** in [`gateway_backlog_progress.md`](./gateway_backlog_progress.md).

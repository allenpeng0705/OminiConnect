# Panda API gateway — feature catalog

**Scope:** What **Panda’s built-in API gateway** is meant to provide for **AI + MCP** workloads. It has two roles: **ingress** (in front of MCP/chat) and **egress** (behind MCP, toward corporate API gateways and REST). See [`panda_data_flow.md`](./panda_data_flow.md).

**Implementation status:** This page is the **product / capability catalog**. A **phase-by-phase and feature-vs-code matrix** lives in [`gateway_design_completion.md`](./gateway_design_completion.md) (updated when gateway work merges). Items below remain the **north star**; **G2** (ingress + top-level **HTTP RPS**, optional **Redis**) and **G4** (egress **TLS min version**, **SIGHUP** / **watch** reload) are **shipped** — see the matrix for detail.

---

## 1. Ingress features (client → Panda)

| Feature | What it does | Typical use |
|---------|----------------|-------------|
| **TLS termination** | HTTPS on Panda’s listener (reuse / extend existing `tls` config). | Encrypt agent and app traffic to Panda. |
| **TLS management** | **Cert/key paths** (today), optional **hot reload** on file change (SIGHUP or periodic watch), **min TLS version**, **cipher suite** policy, optional **SNI** when multiple certs. **ACME / Let’s Encrypt** as a later milestone. | Safe defaults + painless rotation without long restarts. |
| **mTLS (server)** | Require client certificates on ingress (optional). | Service-to-service or zero-trust inner mesh. |
| **Path / host routing** | Map `path_prefix` / `Host` to **MCP**, **OpenAI-shaped chat**, **ops**, **admin**, **developer portal** (simple). | One hostname, multiple handlers. |
| **Load balancing (ingress context)** | Panda runs **horizontally** behind **L4/L7** (K8s Service, cloud LB, Envoy); document **session / MCP affinity** when stateful MCP backends matter. Optional **ingress-side** “upstream pool” only if the same binary fans out to multiple **internal** listeners (advanced). | Scale out; avoid pinning all traffic to one replica when safe. |
| **Rate limiting** | **Shipped (partial):** **HTTP RPS** per matching **`api_gateway.ingress.routes[]`** prefix and per top-level **`routes[]`** (fixed **1s** window); optional **`api_gateway.ingress.rate_limit_redis`** (**`INCR` + short TTL**) for **shared** counters across replicas. **Not yet:** per-identity / concurrency on ingress, **low-cardinality Prometheus** labels per RL row (design target). | Protect Panda and upstreams; fair use between tenants. |
| **JWT / API-key validation** | Reuse or tighten **`identity` / `auth`** on selected routes. | Stop unauthenticated access to `/v1` or `/mcp`. |
| **Trusted downstream identity** | When **external** L7 already authenticated: **`trusted_gateway`** attestation + subject/tenant headers. | Kong → Panda hop (existing pattern). |
| **Request size / header limits** | Bounded bodies and headers on ingress. | Stability and DoS resistance. |
| **Correlation / trace context** | Propagate **`x-request-id`**, W3C **`traceparent`** into MCP and egress. | Ops and compliance joins (partially present today). |
| **CORS / simple transforms** | Optional allowlists for browser or multi-origin clients. | AI web clients hitting Panda directly. |

---

## 2. Egress features (Panda → corporate / REST)

| Feature | What it does | Typical use |
|---------|----------------|-------------|
| **HTTP/HTTPS client** | Pooled **Hyper + rustls** (or equivalent) outbound calls. | Tool calls to internal REST behind corporate gateway. |
| **Load balancing (egress)** | **Multiple upstream URLs** per logical target: **round-robin**, **random**, **least-pending** (or weighted); health-aware **passive** backoff on repeated failures. | Spread tool traffic across replicas of an internal API. |
| **Base URL + path templates** | Resolve tool targets against **`corporate.default_base`** or a **pool**; safe path join. | Corporate API gateway + HA backends. |
| **Timeouts & cancel** | Per-route / global **connect + total** timeouts; cancel on client disconnect. | Avoid hung tools. |
| **Retries (safe methods)** | Configurable retry for **GET**/idempotent ops; careful for POST. | Resilience to transient 5xx. |
| **Rate limiting (egress)** | Optional **per-target** outbound concurrency or RPS caps so tools cannot stampede internal services. | Protect corporate APIs from agent storms. |
| **SSRF protection** | **Host / scheme allowlist** for egress targets (required for GA). | Prevent malicious tool configs from scanning the network. |
| **Header / auth injection** | Service JWT, API keys from env/vault, custom headers per route. | Corporate gateway expects `Authorization` or internal headers. |
| **mTLS (client)** | Client cert toward corporate gateway when required; optional **min TLS version** (**`tls12`** / **`tls13`**-only), **SIGHUP** (Unix) and **PEM mtime watch** to reload trust + identity without process restart. | Mutual TLS to internal L7; cert rotation. |
| **Response size limits** | Cap tool response bodies before returning to MCP/model. | Safety and memory bounds. |
| **Egress metrics** | Counters + latency histograms with **bounded** labels (`route`, `result_class`, `upstream_slot`). | SRE dashboards without high cardinality. |

---

## 3. Developer portal — **one** surface for **existing** Panda features

We want **a single developer entry point** that reflects what the **shipping binary** already offers (OpenAI-shaped routes, ops/status endpoints, MCP tool listing)—**not** a large standalone portal product or Apigee-class marketplace.

| Scope | What we mean |
|-------|----------------|
| **Baseline (in repo)** | **`GET /portal`** (operator HTML + live **`/portal/summary.json`** snapshot), **`/portal/summary.json`** (read-only instance snapshot for managing Panda), **`/portal/openapi.json`**, **`/portal/tools.json`** — same unauthenticated portal rules as today (ops paths still secret-gated separately). |
| **Ongoing** | Refresh those pages/JSON as features change; link to **`/console`**, **`/mcp/status`**, health/metrics, and repo docs so integrators do not hunt across disconnected surfaces. |
| **Explicitly optional** | API key self-service, per-key limits, multi-version doc sites, usage dashboards — **only** if a product line requires them; they are **not** the definition of “portal done” for Panda. |

Reuse **`/console`** patterns where it avoids duplicate UI ([`developer_console.md`](./developer_console.md)).

---

## 4. Cross-cutting (ingress + egress + MCP + portal)

| Feature | Notes |
|---------|--------|
| **Single binary / process** | Ingress, MCP, AI gateway, egress share config and **request context** where applicable. |
| **YAML + future control plane** | Static config first; dynamic routes/targets via control plane later ([`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md)). |
| **Wasm / policy hooks** | Existing **Wasm** plugins may apply to paths that hit the main service (same as today). |
| **Compliance hooks** | Optional audit / JSONL alignment for ingress, egress, and portal admin actions (enterprise track). |

---

## 5. What we deliberately **do not** try to be (at first)

| Capability | Stance |
|------------|--------|
| **Full enterprise API marketplace** | **Simple portal yes**; full **multi-vendor** marketplace and **complex monetization** — integrate externally or phase much later. |
| **Billing engine** | Panda can emit **usage metrics / exports**; **charging** stays external unless a later product decision says otherwise. |
| **Huge Lua-style plugin bazaar** | **Rust + Wasm** with a reviewed surface beats unbounded plugins. |
| **Generic GraphQL / gRPC gateway** | **HTTP-first** for MCP + REST tools; optional modules later if demand is clear. |

---

## 6. Suggested maturity tiers (for roadmap)

| Tier | Ingress | Egress | Portal |
|------|---------|--------|--------|
| **MVP** | TLS + path routing + `identity`/`trusted_gateway` | Pooled HTTPS + **LB pool (round-robin)** + allowlist + timeouts + metrics; **mTLS** + **min TLS** + **reload** (**SIGHUP** / PEM watch) on egress client | **Static** OpenAPI + tool list page |
| **Next** | **Ingress RL metrics** labels; **mTLS** on listener; per-identity / concurrency RL | **Cipher** tuning; **cluster-wide** egress RL (Redis) | **Optional:** API keys + per-key limits (not required for baseline portal) |
| **Stretch** | Advanced LB hints for MCP session affinity | Circuit breaker, active health checks | Usage dashboards, billing **export** only |

---

## Related docs

- [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md) — **detailed design** (pipelines, modules, config)  
- [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) — build phases  
- [`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md) — control-plane architecture  
- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — MCP Phase 1 scope  
- [`developer_console.md`](./developer_console.md) — existing UI surface  

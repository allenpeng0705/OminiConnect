# Panda API gateway + MCP gateway — design completion matrix

**Purpose:** Single place to see **which design artifacts match shipping code** and **what remains** (so [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) and [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md) stay honest).

**Canonical flows:** [`panda_data_flow.md`](./panda_data_flow.md)

---

## 1. Phase map (implementation plan §4)

| Phase | Theme | In code today | Notes |
|-------|--------|----------------|--------|
| **A** | Config + no-op ingress when disabled | **Done** | `panda-config`: `ApiGatewayConfig` (`ingress`, `egress`); defaults off; validation. `api_gateway` module + `ProxyState.api_gateway`. |
| **B** | Egress client + allowlist + metrics | **Done** | `crates/panda-proxy/src/api_gateway/egress.rs`: pooled Hyper client, host/path allowlist, `corporate.default_base`, global + **named `profiles`** default headers, retries on **429 / 502 / 503 / 504**, `panda_egress_*` metrics. |
| **C** | Ingress path routing | **Done** | `api_gateway/ingress.rs`: longest-prefix routes, backends `ai` / `ops` / `deny` / `gone` / `not_found` / `mcp`, optional **HTTP methods** (405 + `Allow`), optional **`upstream` override** per route when `backend: ai`. Wired in `lib.rs` `dispatch`. |
| **D** | MCP ↔ egress + ingress MCP HTTP | **Done** | `http_tool` / `http_tools` → `EgressClient`. Ingress **`backend: mcp`:** JSON-RPC 2.0 over **POST** (`initialize`, `ping`, `tools/list`, `tools/call`, empty `resources/list` / `prompts/list`, notifications without `id`) in `inbound/mcp_http_ingress.rs`. **JSON-RPC arrays** = sequential **batch** (`handle_post_batch`). Tool names match **`mcp_{server}_{tool}`**. **`tools/call`** uses the same **`mcp.tool_routes`**, **`mcp.tool_cache`**, and **`mcp.hitl`** path as chat MCP follow-up (`lib.rs` `mcp_http_ingress_execute_tools_call` + context from `mcp_http_ingress_build_context`). **Streamable HTTP:** `Mcp-Session-Id`, GET SSE listener (keepalive), DELETE session, Origin check, SSE/JSON responses. **Still shallow vs full spec:** **`Last-Event-ID`** on GET listener is accepted but **not** used for replay; sessions are **in-memory** only (no cross-replica / durable resumption). |
| **E** | Control plane / dynamic routes | **E4 partial** | E3 plus optional **`control_plane.reload_pubsub`** (Redis **`PUBLISH`** / **`SUBSCRIBE`** to reload from store) and **`control_plane.additional_admin_secret_envs`** (extra secrets for `observability.admin_auth_header`, union with `admin_secret_env`). Postgres trigger for external SQL writers: [`runbooks/control_plane_postgres_external_writes.md`](./runbooks/control_plane_postgres_external_writes.md). Full **multi-tenant RBAC** still future. MySQL / **Azure SQL** → not implemented. |
| **F** | Hardening (mTLS egress, SLO evidence, formal sec review) | **Partial (F1 + in-repo F2/F3)** | SSRF allowlist shipped (Phase B). **Egress mTLS + corporate CA:** **`api_gateway.egress.tls`** — same as Phase B egress. **F2 (methodology + evidence package):** [`runbooks/ingress_gateway_slo.md`](./runbooks/ingress_gateway_slo.md) + [`grafana/ingress_gateway_slo.json`](./grafana/ingress_gateway_slo.json); numeric SLO targets are **per-environment** (attach to release notes / tickets). **F3 (formal review gate):** [`security_review_gate.md`](./security_review_gate.md); external pen-test sign-off stays outside the repo. **TLS reload / min version** in **G4** (shipped). |
| **G** | Pools, ingress RL, TLS mgmt | **Partial** | **Shipped:** [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) — **G2** ingress + legacy RPS metrics; **G2b** per-route JWT (`auth`); **G3** egress Redis / per-route caps / TLS cipher + `min_protocol_version`; **G1** pool RR; listener TLS reload patterns. **Design gaps:** split public/admin listen, ACME — [`panda_api_gateway_features.md`](./panda_api_gateway_features.md). |
| **H** | Developer portal (**one** surface) | **Partial (baseline shipped)** | **`/portal`** (HTML operator hub + embedded **`/portal/summary.json`**), **`/portal/summary.json`** (safe config/runtime snapshot), **`/portal/openapi.json`**, **`/portal/tools.json`**. **Goal:** help operators **manage** this Panda instance, not a full marketplace. **Optional later:** API keys (**H3**). |

---

## 2. Feature catalog vs code ([`panda_api_gateway_features.md`](./panda_api_gateway_features.md))

| Area | Shipped | Planned / partial |
|------|---------|-------------------|
| **Ingress TLS** | Reuse top-level **`tls`** on `listen` | Split listen, ACME, advanced cipher policy as in catalog |
| **Ingress routing** | Prefix → backend; method filters; AI upstream override; **`/portal/*`** on builtin ops prefix; **per-row `rate_limit.rps`** + optional **Redis**; **per-row Prometheus** (`panda_gateway_ingress_rps_total`, bounded labels); per-row **`auth`** (`inherit` / `required` / `optional`) | Split listen, ACME — catalog |
| **Ingress → MCP HTTP** | **POST** JSON-RPC on `backend: mcp` routes; **minimal streamable HTTP** (SSE `message` when `Accept: text/event-stream`) | Full MCP streamable session / resumption — future |
| **Egress HTTP** | Client, allowlist, base URL join, **`pool_bases` RR**, headers, profiles, retries, size cap, metrics (`pool_slot`), **`rate_limit`** (process-local + optional **Redis** + **`per_route`** / `route_label`), **TLS** (WebPKI + **`extra_ca_pem`**, optional **client cert mTLS**; **`min_protocol_version`**, **`cipher_suites`**, **SIGHUP** + **`watch_reload_ms`** reload), **`panda_egress_rps_total`** | Further tuning or org-specific policy only |
| **MCP gateway** | Stdio, OpenAI tools, rounds, policies, tool cache, `http_tool`, **remote `remote_mcp_url`** (JSON or **SSE** response), ingress MCP POST (+ **SSE wrap**), metrics, `/mcp/status` | Full MCP streamable session semantics |
| **Correlation** | `observability.correlation_header` through chat + egress tools | Full **route_id** label plumbing everywhere — partial |

---

## 3. Design doc maintenance

| Document | Role | When to update |
|----------|------|----------------|
| [`panda_data_flow.md`](./panda_data_flow.md) | Diagrams + checklist | New gateway role or listener split |
| [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md) | Internal pipelines, config *targets* | New module or breaking YAML |
| [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) | Milestones | Phase completion or reprioritization |
| **This file** | Truth table vs `main` | After each gateway-related merge |

---

## 4. Suggested next engineering milestones (close remaining *design* gaps)

1. **MCP streamable HTTP (deeper):** session IDs, resumption, multi-chunk SSE — beyond one JSON-RPC envelope per POST.
2. **Ingress RL observability:** per-row **Prometheus** counters shipped; **per-identity** RPS/concurrency (beyond prefix RL) remains a design gap.
3. **Egress rate caps** — process-local **`max_in_flight` / `max_rps`** shipped; optional **cluster-wide** egress caps still a design item if required.
4. **Portal maintenance:** keep **`/portal`** + JSON exports truthful as the product grows; treat **API keys** / heavy doc portals as **optional** backlog, not a “full Phase H” gate.

Items **E** (full multi-tenant RBAC, extra SQL backends) remain product-sized epics. **G** ingress RL metrics and egress caps/ciphers are **shipped**—operators wire dashboards/alerts. **H** baseline is intentionally **minimal**; **H3**-class work is separate.

---

## Related

- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — onboarding YAML for MCP + `http_tool`  
- [`mcp_gateway_reference_designs.md`](./mcp_gateway_reference_designs.md) — external reference notes

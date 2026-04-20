# Gateway backlog progress

Ordered execution (same as the implementation plan): F2 → F3 → G2 → G3 → …

| ID | Item | Status | Notes |
|----|------|--------|--------|
| F2 | Ingress SLO methodology + evidence | Done | [`docs/runbooks/ingress_gateway_slo.md`](./runbooks/ingress_gateway_slo.md) (PromQL, GA record template); optional Grafana [`docs/grafana/ingress_gateway_slo.json`](./grafana/ingress_gateway_slo.json) |
| F3 | Formal security review gate | Done | [`docs/security_review_gate.md`](./security_review_gate.md) (charter + checklist + sign-off; external pen-test stays out of repo) |
| G2 | Ingress + legacy RPS metrics | Done | `panda_gateway_rps_{allowed,denied}_total{layer="ingress\|legacy"}`; per-row `panda_gateway_ingress_rps_total{tenant_id,path_prefix,result}` when an ingress row rate limit applies; wired in `dispatch` (`inc_gateway_ingress_rps_row`) and `forward_to_upstream` (legacy route RPS) |
| G2b | Ingress per-route JWT (`auth`) | Done | `api_gateway.ingress.routes[].auth` + dynamic `auth_mode` column; `enforce_jwt_effective` for MCP + AI paths; `docs/ingress_tls_acme_catalog.md` (TLS/ACME catalog) |
| G3 | Egress cluster caps / per-target / TLS cipher policy | Done | `api_gateway.egress.rate_limit.redis` + `per_route` (`route_label`); `api_gateway.egress.tls.cipher_suites` + `min_protocol_version`; `panda_egress_rps_total{scope,route,result}` on `/metrics`; `/portal/summary.json` exposes redis + per-route counts + cipher policy (no secrets) |
| H1 | MCP Streamable HTTP — configurable session / SSE replay | Done | `mcp.streamable_http`: `sse_ring_max_events`, `session_ttl_seconds`, `sse_keepalive_interval_seconds`; `McpStreamableSessionStore::from_config` |
| E1 | Control plane — **coverage → RBAC → backends** | Planned | **Order:** (1) CP APIs to configure **all** major features that are YAML-only today (see [`control_plane_evolution.md`](./control_plane_evolution.md)); (2) **full** multi-tenant RBAC + namespacing; (3) **MySQL / Azure SQL** stores if required. Baseline auth: [`ControlPlaneConfig`](../crates/panda-config/src/lib.rs). |

## References

- Example MCP Phase 1: `panda.example.yaml` (`api_gateway` / MCP blocks).
- MCP gateway stub: `docs/mcp_gateway_phase1.md`.
- Control plane evolution: `docs/control_plane_evolution.md`.

# Next work — advanced control plane & streamable MCP depth

**Purpose:** Actionable backlog for **`control_plane.*`** and **MCP Streamable HTTP** beyond what [`gateway_design_completion.md`](./gateway_design_completion.md) marks as shipped.

**Canonical behavior today:** [`panda_data_flow.md`](./panda_data_flow.md), ingress MCP in [`crates/panda-proxy/src/inbound/mcp_http_ingress.rs`](../crates/panda-proxy/src/inbound/mcp_http_ingress.rs), sessions in [`mcp_streamable_http.rs`](../crates/panda-proxy/src/inbound/mcp_streamable_http.rs).

---

## 1. Control plane (advanced)

| Item | Today | Suggested direction |
|------|--------|---------------------|
| **Auth model** | Ops secret, optional `additional_admin_secret_envs`, Redis API keys, optional OIDC session | Narrow **roles** (read-only vs mutate routes), audit log for POST/DELETE/import |
| **Multi-tenant** | `tenant_id` on ingress rows + `tenant_resolution_header` | Policy tests: deny cross-tenant route reads/writes; document required headers |
| **Stores** | memory, json_file, sqlite, postgres (+ NOTIFY / reload loops) | Operational runbooks only unless you need **MySQL** / **Azure SQL** (new store kinds — large effort) |
| **Observability** | JSON status, portal slice | Prometheus counters for control-plane mutations; optional OpenAPI for CP REST |

**Pragmatic first slice:** tighten **docs + tests** for tenant-scoped dynamic routes and add **one** integration test that POSTs a route with `tenant_id` and classifies with the matching header.

---

## 2. Streamable MCP (depth)

| Item | Today | Suggested direction |
|------|--------|---------------------|
| **Session store** | In-process `McpStreamableSessionStore` (UUID sessions, TTL touch, prune) | Optional **Redis** backing for session existence (sticky LB or shared subscribers); or document **session affinity** requirement |
| **GET listener SSE** | Keepalive (`: ping`), `Last-Event-ID` passed but **ignored** in [`mcp_streamable_get_listener_response`](../crates/panda-proxy/src/inbound/mcp_streamable_http.rs) | Emit monotonic **`id:`** fields on server-push events; honor **`Last-Event-ID`** for **replay** of buffered events (requires a small per-session ring buffer) |
| **Long-running tool / streaming results** | Tool execution returns JSON into JSON-RPC envelope | If spec requires streaming tool output over SSE, extend `mcp_ingress_emit_jsonrpc_envelope` path (design first) |
| **Spec gaps** | Read spec [Streamable HTTP](https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http) for deltas | Track upstream MCP spec releases |

**Pragmatic first slice:** implement **`id:`** + optional **ring buffer** (last N SSE events per session) so `Last-Event-ID` can reconnect without full spec compliance on every edge case.

---

## 3. How to pick an epic

- **Ops / multi-replica MCP clients:** prioritize **Redis-backed sessions** or **documented affinity** + **Last-Event-ID replay**.
- **GitOps / platform team:** prioritize **control** plane **tenant** tests + **import** validation.
- **Spec compliance:** prioritize **SSE `id`** + replay buffer.

---

## Related

- [`runbooks/control_plane_postgres_external_writes.md`](./runbooks/control_plane_postgres_external_writes.md)
- [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md)

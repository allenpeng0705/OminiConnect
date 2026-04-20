# Testing the MCP gateway with the API gateway

This doc describes **how to test** Panda when **`api_gateway` ingress/egress** and **`mcp`** are used together, and points at the **framework the repo already ships** (Rust integration tests + optional shell smoke).

**Catalog:** For a **per-test map** (purpose + data flow through `dispatch`), see [`testing_scenarios_and_data_flows.md`](./testing_scenarios_and_data_flows.md).

---

## 1. What you are testing

| Layer | What to verify |
|--------|----------------|
| **Ingress** | Prefix → `backend: mcp` routes accept **POST JSON-RPC** (`initialize`, `tools/list`, `tools/call`); optional **RPS** / **Redis** counters; **portal** paths when ingress is on. |
| **MCP host** | Tool catalog, `tools/call` execution, **`http_tool`** and allowlist, **`mcp.tool_cache`** / **`mcp.tool_routes`** where configured. |
| **Egress** | Allowlisted URL; **`rate_limit`** ( **`max_in_flight`**, **`max_rps`**, optional **Redis** for cluster-wide RPS, **`per_route`** by **`route_label`** ); **`tls`** (mTLS, **`cipher_suites`**, **`min_protocol_version`**); **`panda_egress_*`** on **`/metrics`**; see §2.8 and egress tests. |

End-to-end shape: **HTTP client → Panda (ingress) → MCP → egress → mock corporate HTTP**.

---

## 2. Detailed reference: mock APIs and MCP contracts

Use this section when reading **`gateway_workflow.rs`**, **`ingress_mcp_*`** tests in **`lib.rs`**, or the Python / E2E fixtures. It describes **what each fake backend speaks**, not Panda’s full product behavior.

### 2.1 How clients address tools on ingress MCP

YAML uses **`mcp.servers[].name`** and per-server **`tool_name`** (or remote tool names from the upstream MCP). On the **ingress** JSON-RPC surface, advertised / callable tools are typically named:

**`mcp_<server_name>_<tool_name>`** (with underscores; server and tool names come from config).

Examples from tests:

| Config | Ingress `tools/call` `params.name` |
|--------|-------------------------------------|
| `servers: [{ name: corp, http_tools: [{ tool_name: from_a, ... }] }]` | `mcp_corp_from_a` |
| `servers: [{ name: corpapi, http_tool: { tool_name: fetch, ... } }]` | `mcp_corpapi_fetch` |
| `servers: [{ name: mock, command: ... }]` (stdio tool `ping`) | `mcp_mock_ping` |
| `servers: [{ name: remote1, remote_mcp_url: ... }]` (remote lists tool `alpha`) | `mcp_remote1_alpha` |

Remote MCP over HTTP uses the **remote server’s** tool names (`alpha` in the mock); Panda still prefixes them for the **ingress** catalog the same way.

### 2.2 Ingress HTTP shape the tests use

Most MCP-over-HTTP tests speak **raw HTTP/1.1** to Panda:

1. **`POST /mcp`** with **`Content-Type: application/json`**, **`Accept`** including streamable MCP (see **`mcp_streamable_accept_value()`** in test helpers).
2. **`initialize`** JSON-RPC body (tests often use **`protocolVersion`: `2025-03-26`** in the request; responses echo negotiated protocol).
3. Read **`Mcp-Session-Id`** from the response (helper **`parse_mcp_session_id_from_raw_http`**).
4. Further JSON-RPC (**`tools/list`**, **`tools/call`**) on **`POST /mcp`** or **`POST /mcp/sess`** with header **`Mcp-Session-Id: <sid>`** (see **`ingress_mcp_http_initialize_and_tools_list`** vs streamable tests).

Connections are often **`Connection: close`** end-to-end so each request is easy to correlate in small TCP mocks.

### 2.3 Remote MCP over HTTP (Rust inline mock)

**Location:** `mock_remote_mcp_upstream` in [`crates/panda-proxy/src/tests/gateway_workflow.rs`](../crates/panda-proxy/src/tests/gateway_workflow.rs); the same behavior (with a different `tools/call` text) in [`mcp_http_remote.rs`](../crates/panda-proxy/src/inbound/mcp_http_remote.rs) **`mock_mcp_upstream`** for **`remote_mcp_list_and_call_via_egress`**.

**Transport:** One **HTTP POST** handled per **TCP accept**; response includes **`Connection: close`**. The handler reads the JSON body, branches on **`method`**, echoes **`id`** (numeric or string).

| JSON-RPC `method` | HTTP status | Response body |
|-------------------|------------|---------------|
| **`initialize`** | 200 | `result.protocolVersion` **`2024-11-05`**, `serverInfo.name` **`mock`** |
| **`notifications/initialized`** | 200 | *empty body*, `Content-Length: 0` |
| **`tools/list`** | 200 | One tool: **`name`: `alpha`**, `description`: `d`, minimal `inputSchema` |
| **`tools/call`** | 200 | `result.content[0].text` is **`remote_ok`** in workflow tests, **`ok`** in `mcp_http_remote` unit test |
| anything else | 400 | `{}` |

**Config pattern:** `remote_mcp_url: 'http://127.0.0.1:<port>/mcp'` plus egress **`allowlist`** for that host/port and path prefix **`/`**. The workflow then calls **`mcp_remote1_alpha`** on ingress and asserts the response contains **`remote_ok`**.

### 2.4 REST / “corporate” HTTP mocks (Rust inline)

These are **not** JSON-RPC; they are minimal **HTTP/1.1** servers on **`127.0.0.1:0`**. They usually read one request, assert the request line or path, return **`200`** + **`Content-Type: application/json`** + **`Connection: close`**.

| Test / context | Expected upstream request | Response body (notes) |
|----------------|---------------------------|------------------------|
| **`workflow_full_stack_two_http_tools_two_mock_paths`** | 1st: **`GET /corp/service-a`**, 2nd: **`GET /corp/service-b`** (order matters; two accepts) | `{"service":"A"}` then `{"service":"B"}` |
| **`ingress_mcp_http_tools_call_uses_tool_cache_second_hit`** | **`GET /allowed/toolpath`** (twice unless cache hits) | 1st `{"marker":"first"}`, 2nd `{"marker":"second"}` — second call should **not** hit upstream when cache works |
| **`workflow_stdio_python_and_http_tool_ingress`** | **`GET /api/hi`** | `{"via":"rest"}` |

**Egress wiring:** `corporate.default_base` is `http://127.0.0.1:<mock_port>`; **`allow_hosts`** / **`allow_path_prefixes`** must cover the path prefix used above (`/corp`, `/allowed`, `/api`, etc.).

### 2.5 Stdio MCP (Python)

**Tests — [`crates/panda-proxy/tests/mcp_mock_stdio.py`](../crates/panda-proxy/tests/mcp_mock_stdio.py)**  
Transport: **NDJSON** on stdin/stdout (one JSON object per line).

| `method` | Behavior |
|----------|----------|
| **`initialize`** | `protocolVersion` **`2024-11-05`**, `serverInfo.name` **`mock`** |
| **`notifications/initialized`** | No stdout (notification) |
| **`tools/list`** | Single tool **`ping`** |
| **`tools/call`** | Returns text **`pong:`** + tool name (e.g. **`pong:ping`**) |

**`workflow_stdio_python_and_http_tool_ingress`** wires **`command`**: `python3` or `python`, **`args`**: path to this script; server name **`mock`** → ingress tool **`mcp_mock_ping`**. It skips if Python or the file is missing.

**Manual sample — [`examples/mcp_stdio_minimal/server.py`](../examples/mcp_stdio_minimal/server.py)**  
Same protocol style; tools **`ping`** and **`echo_message`** (not used by the default CI workflow test).

### 2.6 Live E2E mock HTTP API

**[`examples/gateway_mcp_e2e/mock_corp_api.py`](../examples/gateway_mcp_e2e/mock_corp_api.py)** — `HTTPServer` on **`127.0.0.1:<port>`** (port from argv, default **18081**). Prints the list of paths at startup.

| Method + path | Response (JSON) |
|---------------|-----------------|
| **`GET /allowed/toolpath`** | `{"ok": true, "via": "mock_corp_api"}` |
| **`GET /corp/service-a`** | `{"service": "A"}` |
| **`GET /corp/service-b`** | `{"service": "B"}` |
| **`GET /api/hi`** | `{"via": "rest", "message": "hello"}` |
| **`GET /v1/status`** | `{"status": "ok", "component": "inventory"}` |
| Other paths | **404** |

The minimal E2E template ([`panda.e2e.yaml.template`](../examples/gateway_mcp_e2e/panda.e2e.yaml.template)) only wires **`corpapi`** + **`/allowed/toolpath`**. The repo-root **[`panda.yaml`](../panda.yaml)** profile uses **several** `mcp.servers` entries against the **same** `default_base` (see section 2.7).

### 2.7 Multi-server, multi-tool local profile (`panda.yaml`)

Typical production setups have **multiple MCP server blocks** (different owners, transports, or rate limits). For local learning, **`panda.yaml`** enables four REST-backed servers at once (all paths hit **`mock_corp_api.py`** on port **18081**):

| `mcp.servers[].name` | Config | Ingress `tools/call` `params.name` |
|----------------------|--------|-------------------------------------|
| **`corpapi`** | `http_tool` → `/allowed/toolpath` | **`mcp_corpapi_fetch`** |
| **`corp`** | `http_tools` → `/corp/service-a`, `/corp/service-b` | **`mcp_corp_from_a`**, **`mcp_corp_from_b`** |
| **`inventory`** | `http_tool` → `/v1/status` | **`mcp_inventory_health`** |
| **`edge`** | `http_tool` → `/api/hi` | **`mcp_edge_hi`** |

**Egress:** one **`corporate.default_base`**; **`allow_path_prefixes`** must list every path prefix used above (`/allowed`, `/corp`, `/api`, `/v1`). Add **`pool_bases`** or extra hosts when backends differ (see **`panda.example.yaml`**).

**Other transports (commented in `panda.yaml`):** **`command`/`args`** (stdio MCP), **`remote_mcp_url`** (MCP over HTTP via egress). Each server uses **at most one** of: `command`, `http_tool`, `http_tools`, or `remote_mcp_url`.

### 2.8 Egress rate limits, Redis, TLS, and metrics

Use this when validating **G3**-style egress policy: cluster RPS, per-route RPS, and TLS cipher posture — you can rely on **`EgressClient`** unit/integration tests without driving full **`POST /mcp`** flows.

| Topic | Config (YAML) | What to check |
|--------|----------------|---------------|
| **Global + per-route RPS** | `api_gateway.egress.rate_limit`: `max_rps`, `per_route: [{ route_label, max_rps }]`, optional `redis: { url_env, key_prefix }` | **`cargo test -p panda-proxy api_gateway::egress::tests::rate_limit_`** — local 1s windows; with Redis URL set at process start, limits are shared across replicas (same key prefix pattern as ingress RPS). |
| **In-flight cap** | `rate_limit.max_in_flight` | **`rate_limit_max_in_flight_blocks_second_concurrent_call`** — second concurrent **`request()`** fails fast with **`RateLimited`**. |
| **TLS / cipher policy** | `api_gateway.egress.tls.cipher_suites`, `min_protocol_version` | Validated at config load; client built in **`build_egress_http_client`**. Integration-style tests: **`integration_https_mtls_presents_client_cert_to_upstream`**. |
| **Prometheus** | — | **`panda_egress_rps_total{scope,route,result}`** and **`panda_egress_requests_total`** on **`GET /metrics`**. |
| **Portal (no secrets)** | — | **`GET /portal/summary.json`** → **`api_gateway.egress.rate_limit`** (Redis env set, resolved flag, per-route count) and **`tls`** (cipher policy summary). |

Example snippets: [`panda.example.yaml`](../panda.example.yaml) (commented **`api_gateway.egress`** block). Backlog row: [`gateway_backlog_progress.md`](./gateway_backlog_progress.md) (G3).

---

## 3. In-repo “framework”: Rust integration tests (primary)

The strongest coverage is **`cargo test -p panda-proxy`**. Tests spin up a **local `TcpListener`**, build **`ProxyState`** from **`PandaConfig::from_yaml_str`**, and drive **`dispatch`**—no separate binary required.

**Dedicated workflow suite (toggle layers):** `crates/panda-proxy/src/tests/gateway_workflow.rs` (wired from `lib.rs` `mod tests { mod gateway_workflow; }`).

| Test | What it proves |
|------|----------------|
| **`workflow_full_stack_two_http_tools_two_mock_paths`** | **Client → ingress (`/mcp`) → MCP → egress → mock** with two **`http_tools`** on one server (two corporate GETs, `Connection: close` mock — same harness shape as `ingress_mcp_http_tools_call_uses_tool_cache_second_hit`). |
| **`workflow_init_only_http_tools_config_reaches_200`** | Ingress MCP **`initialize`** with **`http_tools`** + egress configured (no backend call). |
| **`workflow_ingress_off_post_mcp_not_handled_by_mcp_ingress`** | **`api_gateway.ingress.enabled: false`** — `POST /mcp` is **not** handled as MCP ingress (proxied / upstream error). |
| **`workflow_mcp_runtime_off_ingress_mcp_returns_unavailable`** | Ingress MCP route but **`state.mcp` unset** — **503** JSON-RPC (miswired / disabled runtime). |
| **`workflow_http_tool_requires_egress_enabled`** | Config validation: **`http_tool`** without **`api_gateway.egress.enabled`** fails. |
| **`workflow_ingress_remote_mcp_tools_call_via_egress`** | **`remote_mcp_url`** — mock upstream speaks JSON-RPC (`initialize` → `notifications/initialized` → `tools/call`); ingress **`tools/call`** for **`mcp_remote1_alpha`** returns **`remote_ok`**. Same mock shape as `mcp_http_remote::tests::remote_mcp_list_and_call_via_egress` (3 TCP rounds, `Connection: close`). |
| **`workflow_stdio_python_and_http_tool_ingress`** | **`tests/mcp_mock_stdio.py`** as **stdio** MCP (`mcp_mock_ping`) **plus** a second server with **`http_tool`** to a tiny REST mock (`mcp_corp_hi` → JSON `via: rest`). Skips quietly if **`python3`/`python`** or the script is missing (same idea as `mcp_followup_stops_at_max_rounds`). |

```bash
cargo test -p panda-proxy tests::gateway_workflow -- --nocapture
```

**Control plane + tenant + streamable replay (scenario matrix):** `crates/panda-proxy/src/tests/control_plane_and_streamable_scenarios.rs` documents **CP-RO-*** (read-only vs write secrets), **TN-*** (global vs `tenant_id`-scoped dynamic ingress with `tenant_resolution_header`), and **SSE-1** (`Last-Event-ID` replays only newer buffered POST events on the GET listener). Run:

```bash
cargo test -p panda-proxy control_plane_and_streamable -- --nocapture
```

**Examples (additional full stack in `lib.rs`):**

- **`ingress_mcp_http_initialize_and_tools_list`** — `initialize` + `tools/list` on **`POST /mcp`** (streamable `Accept` + session id).
- **`ingress_mcp_http_tools_call_uses_tool_cache_second_hit`** — **`http_tool`** calls a **mock upstream** on an ephemeral port; **`tools/call`** over ingress; asserts **tool cache** prevents a second backend hit.
- **`ingress_mcp_initialize_accepts_streamable_sse`** / **`ingress_mcp_streamable_get_listener_and_delete_session`** — streamable HTTP surface.

**Ingress routing (unit-level):** `crates/panda-proxy/src/api_gateway/ingress.rs` (`#[test]` on classify / merge).

**Egress client:** `crates/panda-proxy/src/api_gateway/egress.rs` (`integration_hits_mock_upstream_when_allowed`, **`integration_https_mtls_presents_client_cert_to_upstream`**, pool / retry cases, **`rate_limit_max_rps_*`**, **`rate_limit_per_route_rps_metrics_scope`**, **`rate_limit_max_in_flight_*`** — see §2.8).

**How to run a focused subset:**

```bash
# All tests whose names mention ingress MCP
cargo test -p panda-proxy ingress_mcp -- --nocapture

# Ingress router unit tests
cargo test -p panda-proxy api_gateway::ingress:: -- --nocapture

# Egress integration-style tests (names vary)
cargo test -p panda-proxy api_gateway::egress:: -- --nocapture
```

**Convenience script (same filters):** [`../scripts/gateway_mcp_integration_tests.sh`](../scripts/gateway_mcp_integration_tests.sh)

**Pattern for new tests:** Copy **`ingress_mcp_http_tools_call_uses_tool_cache_second_hit`** in `crates/panda-proxy/src/lib.rs`: spawn a tiny TCP mock that speaks HTTP/1.1, point **`api_gateway.egress.corporate.default_base`** at it, set **`allowlist`**, add **`mcp.servers[]`** with **`http_tool`**, then issue raw HTTP to **`POST /mcp`** with JSON-RPC bodies.

For **exact request/response shapes** of the mocks those tests use, see **section 2** above.

---

## 4. Mock MCP over stdio (host process tests)

Full method/tool matrix: **section 2.5**. In short:

- **`crates/panda-proxy/tests/mcp_mock_stdio.py`** — CI-oriented minimal stdio MCP (**`ping`**).
- **`examples/mcp_stdio_minimal/`** — **`ping`** + **`echo_message`** for manual runs; see its README.

Wire with **`mcp.servers[].command` + `args`** pointing at the script path.

---

## 5. Live binary smoke (optional)

For a **real `panda-server` process** talking to a **mock corporate API**:

1. See **`examples/gateway_mcp_e2e/README.md`** — fixed or scripted ports, **`mock_corp_api.py`**, generated YAML.
2. Run **`scripts/gateway_mcp_e2e_smoke.sh`** — starts mock + Panda, runs **`curl`** JSON-RPC **`initialize` → `tools/call`**, tears down.

Use this when you care about **CLI startup**, **logging**, or **TLS on `listen`**, which the embedded `dispatch` tests do not exercise. Mock API details: **section 2.6**.

---

## 6. Staging / CI against a running deployment

[`scripts/staging_readiness_gate.sh`](../scripts/staging_readiness_gate.sh) runs **`cargo test -p panda-config`** and **`cargo test -p panda-proxy`**, then **`curl`** **`/health`**, **`/ready`**, **`/mcp/status`**. Point **`PANDA_BASE_URL`** at your instance; optional load with **`READINESS_LOAD_PAYLOAD`**.

---

## 7. Suggested workflow for a change

1. **`cargo test -p panda-config`** if you touched YAML structs / validation.  
2. **`./scripts/gateway_mcp_integration_tests.sh`** (or full **`cargo test -p panda-proxy`**).  
3. If you changed **process** behavior: **`./scripts/gateway_mcp_e2e_smoke.sh`**.  
4. Before release: **`./scripts/staging_readiness_gate.sh`** against a staging URL.

---

## Related

- [`panda_data_flow.md`](./panda_data_flow.md) — ingress / MCP / egress positions  
- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — minimal MCP + **`http_tool`** config  
- [`gateway_design_completion.md`](./gateway_design_completion.md) — what is implemented vs backlog  

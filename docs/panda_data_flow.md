# Panda — data flow (keep in mind)

**Purpose:** Canonical **traffic shapes** for Panda’s **all-in-one** design: a **first-class Panda API gateway** that can sit **in front of** the MCP gateway (ingress) **or behind** it (egress toward the corporate API gateway and REST). External L7 (Kong, etc.) remains optional **outside** the whole product.

**See also:** [`panda_scenarios_summary.md`](./panda_scenarios_summary.md) (scenario matrix + top-down diagrams: MCP + API gateway + AI gateway), [`testing_scenarios_and_data_flows.md`](./testing_scenarios_and_data_flows.md) (which tests exercise which dispatch stages), [`openai_api_surface_coverage.md`](./openai_api_surface_coverage.md) (chat vs generic `/v1/*` proxy, Anthropic, Realtime gaps), [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md) (detailed design), [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) (build plan), [`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md) §4, [`architecture_two_pillars.md`](./architecture_two_pillars.md), [`kong_handshake.md`](./kong_handshake.md) (when an **additional** edge gateway wraps Panda). **Per-request order of handlers** (`listen` → `dispatch` → optional ingress → `forward_to_upstream`): section **Request dispatch** below.

---

## All-in-one idea (one binary / one process)

| Panda API gateway role | Where it sits | Job |
|------------------------|---------------|-----|
| **Ingress (in front of MCP)** | Client → **Panda API GW** → **Panda MCP** | TLS, routing, auth, limits on the way **into** MCP + chat. |
| **Egress (behind MCP)** | **Panda MCP** → **Panda API GW** → **corporate API gateway** → REST | Uniform HTTP client path for tools: signing, routing, retries, identity toward **existing** corporate L7. |

Same **Panda API gateway** component, **two configurable positions** — not two different products. You can enable **ingress only**, **egress only**, or **both**.

Optional **external** Kong/NGINX in front of **everything** is still supported for enterprises that already standardize on it.

---

## Request dispatch — from `listen` to response (implementation order)

Every client request hits the HTTP server bound to **`listen`**, then **`dispatch`** in [`crates/panda-proxy/src/lib.rs`](../crates/panda-proxy/src/lib.rs). **Order matters:** an earlier stage may **return** a response; later stages never run.

**Correlation ID:** observability headers are applied at the start of `dispatch` for every request.

### 1. Control plane (optional)

**When:** `control_plane.enabled` **and** the path is under `control_plane.path_prefix` (default logical base `/ops/control/...` if the prefix is empty — see `control_plane_rest_path` in code).

**If the path matches:** authenticate (ops secret, control-plane secrets, API keys, etc.). On failure → **4xx** and stop. On success → handle REST sub-routes (status, API keys, ingress route import/export/CRUD, …) → **JSON** and stop. Unknown sub-path → **404** JSON and stop.

**If the path does not match control plane:** skip this block.

### 2. API gateway ingress (optional)

**When:** `api_gateway.ingress.enabled` **and** the ingress router is built (`ingress_router` is `Some`).

**Classification:** longest-prefix match on **`api_gateway.ingress.routes`** merged with **dynamic** ingress rows and tenant resolution; if **`ingress.routes`** is empty, **built-in defaults** apply (see ingress router in [`ingress.rs`](../crates/panda-proxy/src/api_gateway/ingress.rs)).

| Outcome | HTTP | Reaches `forward_to_upstream`? |
|--------|------|--------------------------------|
| No matching prefix | **404** `ingress: no matching route` | No |
| Method not allowed (per-row `methods`) | **405** + `Allow` | No |
| Ingress RPS exceeded (if configured) | **429** | No |
| `backend: deny` | **403** | No |
| `backend: gone` | **410** | No |
| `backend: not_found` (tombstone) | **404** | No |
| `backend: mcp` | MCP HTTP ingress handler (JSON-RPC / streamable HTTP) | No |
| `backend: ai` | Optional per-row `backend_base` stored as override; **continue** | Yes (unless a later stage returns) |
| `backend: ops` | **Continue** (no return in ingress) | Yes (unless a later stage returns) |

**Where MCP is selected here:** Ingress picks the **handler kind** for the URL prefix. **`backend: mcp`** is chosen when the request path matches an ingress row whose **`backend`** is **`mcp`** (built-in default: prefix **`/mcp`** — JSON-RPC `initialize`, `tools/list`, `tools/call`, etc.). That branch **returns inside** `mcp_http_ingress` and does **not** go to `forward_to_upstream`. It is independent of **top-level** `mcp.enabled` in the sense that routing is by prefix; the MCP runtime must still be configured for tools to actually work.

**If ingress is disabled:** this entire block is skipped. That matters for the **MCP HTTP surface** (`POST /mcp`, session paths under `/mcp/…`): without ingress, those URLs are **not** classified as **`backend: mcp`**, so they **do not** run `mcp_http_ingress` — they fall through to **`forward_to_upstream`** and are proxied like any other path (typically incorrect; integration test `workflow_ingress_off_post_mcp_not_handled_by_mcp_ingress`). So for **JSON-RPC MCP over HTTP on `/mcp`**, you should set **`api_gateway.ingress.enabled: true`** (and keep a row with **`backend: mcp`** on that prefix — the built-in table includes **`/mcp`** when `ingress.routes` is empty). **Chat + MCP tools** on **`POST /v1/chat/completions`** does **not** require ingress: it uses **`mcp.enabled`** inside **`forward_to_upstream`** only.

**Caveat:** If you use a **non-empty** custom `api_gateway.ingress.routes` that **replaces** the built-in table, you must list every prefix you still need (including `/health` → `ops` if you want health under ingress, and **`/mcp` → `mcp`** if you want MCP HTTP). Otherwise **`GET /health`** can fail at ingress **before** the hardcoded `/health` handler later in `dispatch`.

### 3. Developer console (`GET /console/...`)

OAuth login/callback (if OIDC enabled), static UI, WebSocket, metadata — or **404** if disabled / unknown path. Can return without proxying.

### 4. Built-in ops and portal endpoints (still in `dispatch`)

Examples: **`GET /health`**, **`GET /ready`**, **`GET /metrics`**, TPM/MCP/plugins/fleet/compliance status, **`GET /portal/*`**. Often require **ops auth** when configured. These run **after** ingress (if ingress ran) and only if the request **reaches** them.

### 5. JWT (optional)

**When:** `identity.require_jwt` (and related settings). Failure → **401** (or configured response) and stop.

### 6. `forward_to_upstream` — AI gateway / HTTP proxy

**When:** nothing above returned.

**Backend base URL:** `effective_backend_base(path)` from top-level **`default_backend`** + **`routes[]`** (longest `path_prefix` wins), unless an ingress **`ai`** row set **`backend_base`**, or agent profile / semantic routing overrides the base.

**MCP on this path (not the ingress `/mcp` route):** If **`mcp.enabled`** and the runtime is present, **`POST /v1/chat/completions`** can **merge MCP tool definitions** into the JSON body (when `routes` / global `mcp_advertise_tools` and headers allow). When the **upstream** returns tool calls, **`McpRuntime`** runs tools (stdio / remote MCP / **egress** HTTP tools) and may issue **follow-up** chat rounds — still inside **`forward_to_upstream`** / follow-up logic. **`POST /mcp` over ingress** (`backend: mcp`) is a **separate** entry: caller sends JSON-RPC **`tools/call`** directly; no LLM is required on that path.

**Inside this function (summary):** prompt safety, route RPS, route method allowlist, WASM/plugins, TPM, PII, MCP tool injection on chat, semantic cache, semantic routing, Anthropic adapter on `POST /v1/chat/completions`, MCP follow-up loops, model failover, HTTP client to upstream, response transforms.

Typical error mappings include **413** (payload), **403** (policy), **429** (TPM / budgets), **405** (top-level route `methods`), **502/503/504** (upstream). Success → proxied response from the backend.

### 7. Two different `routes` tables

| Config | Role |
|--------|------|
| **`api_gateway.ingress.routes`** (+ dynamics + built-ins when empty) | **Handler class** when ingress is on: `ai` / `mcp` / `ops` / `deny` / `gone` / `not_found`. |
| **Top-level `routes` + `default_backend`** | **Outbound HTTP base** and per-path AI options **inside `forward_to_upstream`** (`effective_backend_base`, TPM, MCP advertise, adapter `type`, etc.). |

### Overview diagram

```mermaid
flowchart TD
  A[Client to listen] --> B[dispatch]
  B --> C{control_plane path and enabled?}
  C -->|yes| CP[Control plane handlers]
  CP --> Z[Return response]
  C -->|no| D{ingress enabled?}
  D -->|no| E[Skip ingress]
  D -->|yes| IG[Ingress classify_merged]
  IG -->|backend mcp| MCP[MCP HTTP ingress — JSON-RPC]
  MCP --> Z
  IG -->|no match / deny / gone / 404 / 405 / 429| Z
  IG -->|ai or ops| E
  E --> F[Console / health / portal / ops GETs]
  F -->|matched| Z
  F -->|no| G{require_jwt?}
  G -->|fail| Z
  G -->|ok| H[forward_to_upstream — AI proxy; optional MCP on chat]
  H --> Z
```

**Legend — two MCP touchpoints:** **`backend: mcp`** (branch to **MCP HTTP ingress**) = MCP **protocol** on **`/mcp`** (or a custom prefix row). **`forward_to_upstream`** = **OpenAI-shaped** proxy; for **`POST /v1/chat/completions`**, **`mcp`** settings may **inject tools** and **`McpRuntime`** may **run tools** after the model returns (separate from the `/mcp` JSON-RPC route).

---

## Positioning: Panda as the REST-to-MCP bridge

This section states Panda’s **technical architecture** and **enterprise role** as implemented in the Rust codebase (not a separate “probabilistic brain” inside the proxy).

### 1. Architectural role (MCP terminology)

In strict **Model Context Protocol** vocabulary, Panda is **not** the **Host** (the end-user application such as an IDE or chat UI). In one process, Panda behaves as a **protocol gateway** with a **dual** role:

| Direction | Role toward peers | What the code does |
|-----------|-------------------|---------------------|
| **Ingress (northbound)** | **MCP server** toward callers | Unified HTTP surface (e.g. **`POST /mcp`**) speaking **JSON-RPC** (`initialize`, `tools/list`, `tools/call`). Callers see one tool catalog and session semantics. |
| **Egress (southbound)** | **MCP client(s)** toward backends | **`McpRuntime`** holds one internal client per configured `mcp.servers` entry: **stdio** subprocesses (**`StdioMcpClient`**), **remote MCP over HTTP** (**`McpHttpRemoteClient`** + **`remote_mcp_url`**), and **REST tools** (**`McpHttpToolClient`** + **`api_gateway.egress`**) for **`http_tool`** / **`http_tools`**. |

So: **JSON-RPC MCP on the front**; **stdio, remote MCP HTTP, or corporate REST** on the back—depending on YAML, not a single transport.

### 2. The “AI facade” (legacy REST without rewriting backends)

Panda’s practical value for migration is an **AI-shaped interface** over existing HTTP services: declarative mapping from **tool names** to **allowed** corporate URLs.

- **Declarative tooling:** **`http_tool`** (one tool per server block) or **`http_tools`** (several tools on one server). Paths join to **`api_gateway.egress.corporate.default_base`** (and **`allowlist`**). Ingress-visible names follow the OpenAI bridge: **`mcp_{server}_{tool}`** (e.g. `mcp_corpapi_fetch`).
- **Context injection:** On **`/v1/chat/completions`**, when MCP tools are advertised, Panda can **merge** tool definitions into the request so the **upstream LLM** sees a single list. **Tool selection in natural language is performed by that model**, not by Panda.
- **Execution loop:** When the upstream returns OpenAI-style **tool calls**, Panda’s **`McpRuntime`** resolves **server + tool**, runs the right client (including **REST via egress**), and appends **tool** messages for follow-up rounds.

Direct **`POST /mcp`** does **not** use an LLM: the **caller** supplies **`tools/call`** explicitly.

### 3. Deterministic security (intent gating)

Panda does **not** use a secondary LLM to “judge” tool calls. **Heuristic intent** (keyword buckets such as **`data_read`**, **`data_write`**, **`filesystem`**, **`general`**) plus **`mcp.intent_tool_policies`** and **`proof_of_intent_mode`** (`audit` / `enforce`) implement **deterministic** checks on the chat tool path. Details and keyword lists: **section 6** below.

### 4. Panda vs. traditional L7 API gateways (scope)

Traditional gateways optimize **stateless** request routing, auth, and rate limits. Panda adds a **first-class MCP and chat tool execution** path: **multi-round** tool loops, **MCP JSON-RPC** lifecycle on **`/mcp`**, and **egress-governed** calls to internal HTTP. Comparing to a specific product (e.g. Kong) is a **scope** discussion: many gateways can be extended with plugins; Panda is **MCP- and tool-loop-native** in one binary.

| Dimension | Typical L7 API gateway | Panda (this repo) |
|-----------|-------------------------|-------------------|
| **Primary unit** | HTTP request / route | MCP **tool** / JSON-RPC method; chat **tool calls** + optional LLM rounds |
| **Protocols** | REST, often gRPC/GraphQL via config | **MCP JSON-RPC** ingress; **OpenAI chat** to upstream; **stdio / remote MCP / REST** to backends |
| **Core loop** | Request in → route → response | **Tool discovery + execution**; chat path adds **upstream LLM** reasoning |
| **Safety (examples)** | WAF, rate limits, ACLs | **Egress allowlists**, **`mcp.tool_routes`**, **heuristic intent + `intent_tool_policies`**, optional **`pii`** on **buffered request bodies**, identity headers |

---

## 1. Ingress — Panda API gateway **in front of** MCP

```mermaid
flowchart LR
  A[Agent / app] --> PAGI[Panda API gateway — ingress]
  PAGI --> MCP[Panda MCP gateway + chat / AI data plane]
  MCP --> TB[MCP tool backends]
```

---

## 2. Egress — Panda API gateway **behind** MCP (toward corporate L7)

```mermaid
flowchart LR
  A[Agent / MCP client] --> MCP[Panda MCP gateway]
  MCP --> PAGE[Panda API gateway — egress]
  PAGE --> CORP[Corporate API gateway]
  CORP --> R[Internal REST]
```

### Egress limits, TLS policy, and observability

Corporate REST calls use **`EgressClient`** ([`egress.rs`](../crates/panda-proxy/src/api_gateway/egress.rs)). Configuration is under **`api_gateway.egress`**:

| Concern | Config | Behavior |
|--------|--------|----------|
| **Concurrency** | **`rate_limit.max_in_flight`** | Process-local semaphore: fail-fast when concurrent **`request()`** calls (including retries) reach the cap. |
| **Global RPS** | **`rate_limit.max_rps`** | Fixed **1 second** window. If **`rate_limit.redis.url_env`** resolves to a non-empty Redis URL at process start, the window is enforced **cluster-wide** via Redis **`INCR`** + short TTL; otherwise a **process-local** counter is used (same pattern as ingress RPS Redis). |
| **Per-target RPS** | **`rate_limit.per_route[]`** | **`route_label`** + **`max_rps`** — caps **`EgressHttpRequest.route_label`** (e.g. MCP **`http_tool`** / **`http_tools`** routes). Redis keys are separate from the global key (`{key_prefix}route:{label}`). |
| **TLS** | **`egress.tls`** | Optional **mTLS** (`client_cert_pem` / `client_key_pem`), **corporate CA** (`extra_ca_pem`), **`min_protocol_version`** (`tls12` / `tls13`), optional **`cipher_suites`** allowlist (rustls cipher names; empty = default provider suites). Client is built in **`build_egress_http_client`** ([`lib.rs`](../crates/panda-proxy/src/lib.rs)). |

**Metrics** (appended on **`GET /metrics`** when egress is initialized): **`panda_egress_requests_total`**, **`panda_egress_rps_total{scope="global"|"per_route",route,result}`** (allowed/denied at the RPS gates), latency histograms, retries. End-to-end **`rate_limited`** outcomes also appear on **`panda_egress_requests_total`** with **`result=rate_limited`**.

**Operator snapshot:** **`GET /portal/summary.json`** → **`api_gateway.egress`** includes **`rate_limit`** (including whether Redis URL env is set and **`redis_resolved_at_process_start`**, **`per_route_caps_count`**) and **`tls`** (**`cipher_suite_policy`** `default` vs `restricted`, cipher list size, **`min_protocol_version`**) — no secrets.

---

## 3. Both roles on one path (typical “full” Panda stack)

```mermaid
flowchart LR
  A[Agent] --> PAGI[Panda API gateway — ingress]
  PAGI --> MCP[Panda MCP gateway]
  MCP --> PAGE[Panda API gateway — egress]
  PAGE --> CORP[Corporate API gateway]
  CORP --> R[Internal REST]
```

Optional prefix: **Agent → external Kong →** … when the org puts Kong outside Panda.

---

## 4. Outbound AI (second pillar)

**OpenAI-shaped** traffic through Panda to **upstream LLMs** (`default_backend`, top-level `routes`, TPM, cache, failover) is the **AI gateway** pillar. It can share listeners and policy context with the paths above; diagram it separately when designing so LLM hops are not confused with corporate REST egress.

---

## 5. Ingress + MCP today

When **`api_gateway.ingress.enabled`** is true, prefix routing runs **before** the existing chat/MCP handlers. **`backend: ai`** and **`ops`** (and deny/tombstone backends) are fully dispatched. **`backend: mcp`** serves **MCP JSON-RPC 2.0 over HTTP POST** (e.g. builtin prefix `/mcp`): `initialize`, `tools/list`, `tools/call`, etc. Tool names match the OpenAI bridge (`mcp_{server}_{tool}`). Ingress **`tools/call`** shares **`mcp.tool_routes`**, **`mcp.tool_cache`**, and **`mcp.hitl`** with the chat MCP follow-up path (see [`tool_cache_mvp.md`](./tool_cache_mvp.md)). **MCP over OpenAI chat** (`/v1/...` with tools) and **stdio MCP servers** are unchanged. **Streamable HTTP** (SSE) for MCP is not implemented yet. See [`gateway_design_completion.md`](./gateway_design_completion.md).

---

## 6. MCP intent gating — **deterministic** (no “security LLM”)

Panda does **not** embed an LLM to approve or reject tool calls. **Tool choice** on chat is still made by the **upstream model** when tools are advertised; Panda applies **policy** using fixed rules.

### 6.1 Two traffic shapes (who decides the tool)

| Path | Who picks the tool | Panda’s role |
|------|--------------------|--------------|
| **`/v1/chat/completions`** with MCP tools injected | **Upstream LLM** returns OpenAI-style tool calls | Inject merged tool definitions, execute **`McpRuntime`** calls, optional follow-up rounds to the model |
| **`POST /mcp`** (ingress MCP JSON-RPC) | **Caller** sends `tools/call` with `params.name` | Protocol gateway: map `mcp_{server}_{tool}` → backend, no model involved |

### 6.2 Intent label (heuristic bucket)

For **OpenAI chat** requests when MCP tools are advertised, Panda derives a single **intent string** from the **last user message** in the JSON body using **`classify_intent_from_chat_request`** in [`crates/panda-proxy/src/inbound/mcp.rs`](../crates/panda-proxy/src/inbound/mcp.rs).

- Input: raw POST body bytes; parses `messages`, takes the **last** message with **`role: user`**, uses **`content`** as a lowercase string (non-string content is not used).
- Method: **keyword counts** — each listed substring match scores **+1** in one of three groups (**write**, **read**, **filesystem**). No ML, no embeddings.

**Write terms (summed → `write` score):**  
`write`, `delete`, `update`, `create`, `insert`, `drop`, `remove`, `modify`

**Read terms (summed → `read` score):**  
`read`, `list`, `show`, `query`, `search`, `find`, `fetch`, `select`

**Filesystem terms (summed → `fs` score):**  
`file`, `directory`, `path`, `folder`, `open`, `save`, `rename`, `filesystem`

**Bucket selection (first match in order):**

1. If `fs >= write` **and** `fs >= read` **and** `fs > 0` → intent **`filesystem`**
2. Else if `write > read` **and** `write > 0` → **`data_write`**
3. Else if `read > 0` → **`data_read`**
4. Else → **`general`**

If the body is not valid JSON or has no usable user string, intent is **`general`**.

### 6.3 Policy mapping (`intent_tool_policies`)

YAML **`mcp.intent_tool_policies`** lists, per **`intent`**, which tools are **allowed** (OpenAI function name `mcp_server_tool` or canonical `server.tool`). Empty policies → no intent filtering.

- **Advertise time:** `filter_tools_for_intent` can **drop** tools from the list injected into chat so the model never sees disallowed names.
- **Call time:** When the upstream returns a tool call, **`tool_allowed_for_intent`** checks the call against the same policies. If **`mcp.proof_of_intent_mode`** is **`enforce`**, a mismatch **blocks** the call (deterministic error path); **`audit`** logs only. Implementation: MCP follow-up loop in [`crates/panda-proxy/src/lib.rs`](../crates/panda-proxy/src/lib.rs) (search `proof_of_intent_mode`).

### 6.4 Why document this

Intent gating is **auditable**: keywords and YAML lists are reviewable. It adds **predictable** latency (string scan + table lookup), not an extra round trip to a model.

---

## 7. Checklist for new work

- [ ] Does this feature belong to **Panda API gateway (ingress)**, **(egress)**, **MCP**, or **AI gateway**?
- [ ] If **external** Kong wraps Panda, is **`trusted_gateway`** documented for identity headers?
- [ ] Egress path: does it assume **Panda API gateway → corporate API gateway**, or direct HTTP (both may need config knobs)?

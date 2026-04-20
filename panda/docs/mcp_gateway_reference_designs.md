# Reference MCP gateways — design notes for Panda

This document captures **design ideas** from two public MCP gateway projects. Panda is a different product (Rust **AI gateway** + **MCP host** integrated with OpenAI-shaped chat); these references inform **direction**, not a feature checklist.

| Project | Repository | Primary focus |
|---------|------------|----------------|
| **Docker MCP Gateway** | [github.com/docker/mcp-gateway](https://github.com/docker/mcp-gateway) | CLI plugin: run many MCP servers (often in **containers**), unify discovery/calls, catalogs & profiles, secrets/OAuth |
| **Microsoft MCP Gateway** | [github.com/microsoft/mcp-gateway](https://github.com/microsoft/mcp-gateway) | **K8s** reverse proxy + **control plane**: deploy adapters, session-aware routing, tool registration & dynamic router |

**Local checkouts (typical layout next to this repo):**

| Upstream | Suggested folder (sibling of `panda/`) | Where to read first |
|----------|----------------------------------------|---------------------|
| Docker | `../mcp-gateway-main` | `docs/message-flow.md`, `docs/profiles.md`, `docs/security.md` |
| Microsoft | `../mcp-gateway-microsoft` | `README.md`, `openapi/mcp-gateway.openapi.json`, `docs/entra-app-roles.md`, `deployment/k8s/local-deployment.yml` |

**Microsoft tree (for code archaeology):** `dotnet/Microsoft.McpGateway.Service` (gateway + session routing), `Microsoft.McpGateway.Management` (adapters/tools CRUD, K8s deploy, stores), `Microsoft.McpGateway.Tools` (tool gateway router), `sample-servers/` for minimal MCP and tool examples.

---

## 1. Docker MCP Gateway — what it optimizes for

**Architecture (conceptual):** `AI Client → MCP Gateway → MCP Servers` — the gateway aggregates **`tools/list`** and forwards **`tools/call`** to the right backend (see their [message flow](https://github.com/docker/mcp-gateway/blob/main/docs/message-flow.md)).

**Ideas worth studying:**

- **Unified entry for clients** — One gateway config so IDEs and agents share the same tool surface ([profiles](https://github.com/docker/mcp-gateway/blob/main/docs/profiles.md), catalog references, `docker mcp gateway run`).
- **Server lifecycle & isolation** — Servers run in **Docker** (or constrained `npx`/`uvx`), reducing “random process on laptop” risk; aligns with “harden the MCP edge” thinking (see also their [security](https://github.com/docker/mcp-gateway/blob/main/docs/security.md) notes).
- **Tool allowlists at the gateway** — CLI flags such as `--tools server1:*,server2:tool2` mirror Panda’s later-stage **`mcp.tool_routes`** / policy mental model: **deny by default** or explicit enable sets.
- **Secrets and OAuth** — First-class handling so keys are not only env vars; relevant for enterprise MCP servers that use OAuth token flows ([oauth-flows](https://github.com/docker/mcp-gateway/blob/main/docs/oauth-flows.md), etc.).
- **Transports** — stdio for single-client ergonomics; **SSE / streaming** for multi-client — same axis Panda touches when exposing MCP alongside HTTP chat.
- **Observability** — Call tracing / telemetry hooks (their `docs/telemetry`); comparable to Panda’s metrics + correlation id story.

**Local checkout:** See table above (`../mcp-gateway-main`).

---

## 2. Microsoft MCP Gateway — what it optimizes for

**Architecture:** Data plane routes MCP traffic to **adapters** (registered MCP servers) and to a **tool gateway router**; control plane manages **adapters** and **tools** via REST ([README overview](https://github.com/microsoft/mcp-gateway)).

**Ideas worth studying:**

- **Control plane vs data plane split** — CRUD for adapters/tools (`/adapters`, `/tools`) separate from **`POST .../mcp`** streamable HTTP; useful if Panda ever exposes a **management API** for dynamic tool registration.
- **Session-aware routing** — `session_id` → sticky routing to the same MCP instance; important for **stateful** servers behind a load balancer. Panda today is more **process-local** stdio + single HTTP service; multi-replica MCP would need a similar concept or external session store.
- **Tool registration + dynamic router** — Central **`POST /mcp`** that dispatches by tool name to registered backends — close to a **capability registry** / router pattern described in [`protocol_evolution.md`](./protocol_evolution.md).
- **Enterprise auth** — Bearer tokens + app roles (e.g. Entra); parallels Panda’s **`identity` / `auth` / `trusted_gateway`** for the HTTP edge, extended to MCP-specific admin APIs if added later.
- **K8s-first ops** — StatefulSets, headless services, manifests — reference for how teams deploy MCP at scale next to Panda in-cluster.

**Local checkout:** See table above (`../mcp-gateway-microsoft`).

---

## 3. Mapping to Panda (inbound Phase 1 and beyond)

| Theme | Docker-ish direction | Microsoft-ish direction | Panda today / note |
|-------|----------------------|-------------------------|-------------------|
| **Client entry** | One gateway, profiles/catalogs | One gateway URL + optional `/adapters/{name}/mcp` | OpenAI **chat** on same listener + **`mcp.*`** in YAML |
| **Tool surface** | Aggregate `tools/list` | Router + registered tools | **`mcp_openai`** shaping + **`McpRuntime`** |
| **Policy / allowlist** | `--tools`, profile tool commands | RBAC on adapter/tool APIs | Phase 1: minimal; advanced: **`tool_routes`**, intent policies |
| **Isolation** | Containers / minimal privileges | K8s pods per server | **stdio** subprocess per server entry |
| **Secrets** | Docker secrets, OAuth helpers | Workload identity / ACR | Env + future secret backends as needed |
| **Session stickiness** | Less central (often local gateway) | Strong (distributed session store) | Mostly single-replica or LB **without** MCP session affinity yet |
| **Outbound AI** | Out of scope | Out of scope | **Core** — TPM, cache, adapters, failover |

---

## 4. Possible Panda evolutions (informed, not committed)

1. **Catalog / profile ergonomics** — Declarative “working sets” of servers (YAML or API) analogous to Docker **profiles**, without requiring Docker Desktop.
2. **Streamable HTTP MCP inbound** — Complement stdio where clients speak MCP over HTTP to Panda (closer to Microsoft’s `.../mcp` paths).
3. **Optional management plane** — Register tools/adapters via API for dynamic backends (Microsoft-style) while keeping **GitOps YAML** as the simple path.
4. **Replica-safe MCP** — If multiple Panda replicas terminate MCP HTTP, add **session affinity** or shared session metadata (Microsoft’s model).

---

## Related Panda docs

- [`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md) — Panda’s own target MCP + control-plane design (Rust)  
- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — minimal MCP + API gateway scope  
- [`architecture_two_pillars.md`](./architecture_two_pillars.md) — inbound vs outbound  
- [`protocol_evolution.md`](./protocol_evolution.md) — protocols and capability registry direction  

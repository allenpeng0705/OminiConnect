# Architecture: two pillars

Panda is organized around **two product targets**:

1. **Inbound — MCP gateway with API gateway (all-in-one):** Panda includes **its own API gateway** as a first-class component. It can run **in front of** the **MCP gateway** (ingress: TLS, routing, auth into MCP/chat) **or behind** the MCP gateway (egress: outbound HTTP toward the **corporate** API gateway and internal REST). **External** Kong/NGINX may still wrap the whole product. **Panda** hosts MCP tool hosting, discovery, execution, and multi-round loops. **Why:** one product for AI ingress, MCP, and governed HTTP egress. Flows: [`panda_data_flow.md`](./panda_data_flow.md), design: [`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md) §4.
2. **Outbound — AI gateway:** Governed traffic to **upstream LLMs and hosted model APIs**—not limited to **OpenAI-compatible** bases alone. **Today:** OpenAI-shaped chat plus **Anthropic** adapter paths; **direction:** **maximum provider coverage** via native request/response adapters (e.g. cloud vendor APIs), passthrough where a provider already speaks OpenAI, and the same **budgets, cache, routing, and failover** across them. **Why:** **govern organizational AI use**—who hits which models, **spend and token budgets**, routing, and observable usage regardless of wire format.

Everything else is **cross-cutting** (identity, ops, policy) or **enterprise** (budget hierarchy, console SSO, model failover chains).

**Data flow (canonical):** **Panda API gateway** (ingress and/or egress) + **Panda MCP** + optional **corporate** API gateway — see **[`panda_data_flow.md`](./panda_data_flow.md)**. **Scenarios (AI + MCP + ingress/egress):** **[`panda_scenarios_summary.md`](./panda_scenarios_summary.md)**.

**New protocols (e.g. A2A alongside MCP):** Panda is a **gateway and governor**, not a single wire format. MCP is the **current** primary tool-facing protocol in `inbound/`; additional protocols should land as **pluggable modules** reusing shared identity, budgets, and policy—see [`protocol_evolution.md`](./protocol_evolution.md).

## 1. Inbound — MCP gateway with API gateway

**Goal:** **Panda** is the **MCP (and chat) gateway** for agents, paired with **Panda’s built-in API gateway**: **ingress** (in front of MCP) and/or **egress** (behind MCP, toward corporate API gateway + REST). See [`panda_data_flow.md`](./panda_data_flow.md). **Panda** hosts **MCP** tool connections, shapes tools for OpenAI-style chat, and runs **multi-round** model ↔ tool loops.

**First step (Phase 1):** Keep inbound **small** — stdio MCP servers, timeouts, rounds, `advertise_tools`, optional `trusted_gateway`. Defer pattern routes, intent policies, tool cache, HITL, and agent session routing until basics work. See **[`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md)** for the exact split and a minimal YAML skeleton.

**Target architecture (control plane + data plane):** **[`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md)** — Rust-native **all-in-one**: **Panda API gateway** (ingress/egress) + **MCP gateway** + Microsoft-style **control plane**, with optional **external** L7 outside Panda.

**Together:** Typical full path: **Panda API gateway (ingress) → Panda MCP → Panda API gateway (egress) → corporate API gateway → REST**; optional **Kong** prefix. If an **external** gateway sits **in front of** Panda, use [`kong_handshake.md`](./kong_handshake.md) for attested hop headers.

| Area | `panda-proxy` layout | Primary `panda.yaml` blocks |
|------|----------------------|----------------------------|
| Tool runtime (Phase 1) | `inbound::{mcp, mcp_stdio, mcp_openai}` | `mcp` (servers, limits, `advertise_tools`) |
| Edge trust | `shared::gateway` | `trusted_gateway`, `identity` / `auth` as needed |
| Advanced MCP (later) | same + `lib.rs` orchestration | `mcp.tool_routes`, `intent_tool_policies`, `hitl`, `tool_cache` |
| Agent fleet (later) | `shared::{gateway,tpm}` (+ `lib.rs`) | `agent_sessions` |

**Shared helpers on MCP paths:** `shared::brain` (optional HITL, summarization, rate-limit fallback on chat) — see pillar 3.

**Ops:** `GET /mcp/status`; Prometheus for stream probe, tool routes, tool cache, agent rounds (advanced paths emit more series).

## 2. Outbound — AI gateway

**Goal:** A **stable client surface** (OpenAI-compatible **`/v1/chat/completions`** and related paths) with **upstream diversity**: route to **many providers**—OpenAI-compatible servers, **Anthropic**-native APIs today, and **additional native adapters** over time—using the same **token budgets**, **semantic cache**, **adapters**, **semantic upstream routing**, and **model failover**. See **`docs/ai_routing_strategy.md` §1.1** (Multi-provider coverage).

| Area | `panda-proxy` layout | Primary `panda.yaml` blocks |
|------|----------------------|----------------------------|
| HTTP client / path join | `outbound::{upstream,sse}` | `upstream`, `routes` |
| Provider shapes | `outbound::{adapter,adapter_stream}` | `adapter`, per-route `type` |
| Cost / latency | `outbound::semantic_cache`, `shared::tpm` | `semantic_cache`, `tpm` |
| Upstream selection | `outbound::{semantic_routing,model_failover}` | `routing`, `model_failover` |

**Shared helpers on chat paths:** `shared::brain` — `rate_limit_fallback`, `context_management`.

**Ops:** `GET /tpm/status`, `GET /ready` (failover / streaming notes), semantic-cache counters on `/metrics` and `/ops/fleet/status`.

## 3. Cross-cutting & enterprise

Source lives under `crates/panda-proxy/src/shared/`:

- **Identity & edge trust:** `gateway`, `jwks`, plus YAML `identity`, `auth`, `trusted_gateway` (attested **API gateway** → Panda hop)
- **Policy & extensions:** `route_rps`, `plugins` (Wasm), `prompt_safety`, `pii` (declared in `lib.rs` and config)
- **Brain (both pillars):** `brain`
- **Observability:** `compliance_export`, `console_oidc`, correlation id, `/metrics`
- **Enterprise:** `budget_hierarchy`, `console_oidc`, `model_failover` (declared outbound, enterprise config), `routing` (semantic)

## Code layout

The HTTP service is still assembled in `crates/panda-proxy/src/lib.rs` (single hyper stack). Implementation files are grouped physically:

- **`inbound/`** — MCP gateway implementation in Panda (`mcp`, `mcp_openai`, `mcp_stdio`); pair with your **API gateway** at the edge for inbound HTTP.
- **`outbound/`** — AI gateway (`adapter`, `upstream`, `sse`, semantic cache/routing, failover)
- **`shared/`** — identity hop, TPM, brain, compliance, console OIDC, JWKS, TLS, route RPS, budget hierarchy

## Related docs

- [`panda_data_flow.md`](./panda_data_flow.md) — canonical inbound data-flow patterns (keep in mind)
- [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md) — detailed design: API gateway + MCP gateway internals  
- [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md) — phased implementation (MCP + Panda API gateway)  
- [`gateway_design_completion.md`](./gateway_design_completion.md) — design vs `main` matrix (ingress / egress / MCP phases)
- [`design_mcp_control_plane_rust.md`](./design_mcp_control_plane_rust.md) — MCP + API gateway: control plane / data plane (Rust target design)
- [`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md) — Phase 1 MCP + API gateway scope (minimal vs advanced)
- [`mcp_gateway_reference_designs.md`](./mcp_gateway_reference_designs.md) — learnings from Docker / Microsoft MCP gateway repos
- [`protocol_evolution.md`](./protocol_evolution.md) — MCP vs A2A-style fragmentation; adapter-style extensibility
- [`deployment.md`](./deployment.md) — binary, config, Redis, Prometheus, edge placement
- [`kong_handshake.md`](./kong_handshake.md) — trusted hop contract when an L7 gateway sits in front (any vendor)
- [`enterprise_track.md`](./enterprise_track.md) — enterprise options
- [`integration_and_evolution.md`](./integration_and_evolution.md) — coexistence with other systems
- [`ai_routing_strategy.md`](./ai_routing_strategy.md) — semantic routing & agent sessions

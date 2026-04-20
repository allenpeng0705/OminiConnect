# Protocol evolution: MCP, A2A, and what comes next

**Audience:** Teams worried that **MCP is young** and the agent stack is **splitting across protocols** (e.g. **A2A** for agent-to-agent delegation vs **MCP** for agent-to-tool access).

**Intent:** Panda is **not** a single wire protocol. It is a **gateway and governor**: one place for **identity, budgets, policy, observability**, and **translation** between clients, models, and backends—so new protocols can land as **modules** without rewriting the core.

---

## 1. Two product jobs (unchanged)

1. **Inbound — MCP gateway (with API gateway):** Make it **easier for AI to use your services**—register tools, enforce allowlists and rounds, and help organizations **move from ad-hoc APIs toward MCP-shaped tool surfaces** without losing edge controls.
2. **Outbound — AI gateway:** **Control how the organization uses AI**—routes, models, **token budgets**, cache, failover, and audit-friendly signals.

Those jobs are **stable** even when the wire format on either side changes.

---

## 2. Why protocol fragmentation happens

Rough split (simplified; specs and vendors evolve):

| Layer | Typical role | Examples |
|-------|----------------|----------|
| **Agent ↔ tool** | Discovery, invocation, structured results | **MCP** (stdio, HTTP/SSE transports) |
| **Agent ↔ agent** | Delegate a sub-task to another agent / runtime | **A2A** and similar “agent mesh” proposals |
| **Client ↔ model** | Chat, streaming, embeddings | OpenAI-compatible HTTP, provider-native APIs |

Panda already sits in the **middle** of several of these hops. The challenge is **not** picking one winner forever—it is **not hard-coding** one protocol into an unmaintainable core.

---

## 3. Design fit: how Panda adapts

### 3.1 Pluggable protocol surfaces (inbound)

Today, MCP-oriented code lives under `crates/panda-proxy/src/inbound/` (`mcp`, `mcp_openai`, `mcp_stdio`). **Future agent protocols** (e.g. A2A-style endpoints) should follow the same idea: **dedicated modules** that parse/emit the wire format and call into **shared** identity, policy, and orchestration—rather than scattering special cases through `lib.rs`.

The **HTTP service** stays one stack; **protocol handlers** stay **replaceable and testable**.

### 3.2 Adapters as translation (outbound today; pattern for tomorrow)

`outbound::adapter` / `adapter_stream` today translate **OpenAI-shaped** requests and streams **to and from** provider-specific shapes (Anthropic, etc.). That is the same **adapter** idea in a different lane: **wire format in → normalized behavior → wire format out**.

New protocols on the **model or agent side** can add **new adapters** (or sibling modules) without changing how **TPM, semantic cache, or routing** reason about traffic.

### 3.3 Protocol-agnostic governance

**Who** is calling, **what** they may spend, **which** tools or routes are allowed, and **how** you audit should depend on **stable request context** (`RequestContext`, budgets, compliance hooks)—not on whether the caller spoke MCP, A2A, or REST. Semantic routing and safety layers should treat **intent and cost**, not a single framing of JSON.

### 3.4 Multi-hop “bridge” pattern (directional)

A common future shape:

- A client speaks **OpenAI-compatible HTTP** to Panda (outbound path).
- Panda decides the task needs **another agent** and uses an **agent-to-agent** protocol **between trusted hops** (future inbound/outbound module).
- That agent uses **MCP through Panda** to reach internal tools (inbound path).

Panda is the **single enforcement point** for budgets and policy across those hops—not three separate ad-hoc proxies.

### 3.5 Capability registry (roadmap)

The operational pain is often **discovery**: “What can this agent do, and under which policy?” A **central registry** in or behind Panda—**register once**, expose as MCP tools, future A2A endpoints, or plain HTTP—is a **product direction**, not a commitment to a particular schema yet. It should reuse the same **authn/z and audit** model as the rest of the gateway.

---

## 4. Suggested implementation phases (engineering, not dates)

| Phase | Focus |
|-------|--------|
| **A — MCP inbound solid** | Follow **[`mcp_gateway_phase1.md`](./mcp_gateway_phase1.md)** for the minimal MCP + API gateway step; keep module boundaries clear for new protocols later. |
| **B — Protocol spike** | Prototype an additional inbound or inter-service protocol (e.g. A2A) behind feature flags; reuse `shared` for identity and budgets. |
| **C — Unified tokens** | Align **OAuth 2.x / OIDC** and **token exchange** across protocols where enterprise IdPs require one story. |

Exact ordering depends on customer demand and spec stability.

---

## Related docs

- [`architecture_two_pillars.md`](./architecture_two_pillars.md) — inbound vs outbound layout
- [`ai_routing_strategy.md`](./ai_routing_strategy.md) — semantic / tool / agent routing tiers
- [`integration_and_evolution.md`](./integration_and_evolution.md) — edge placement and adoption

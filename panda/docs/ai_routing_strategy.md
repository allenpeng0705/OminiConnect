# AI routing strategy: beyond generic API routing

**Audience:** Product and engineering alignment on **AI-native routing**—semantic, model, tool, agent, and MCP dimensions—with **pluggable** behavior and **performance** as first-class constraints.

**Related:** [Panda vs Kong positioning](./panda_vs_kong_positioning.md), [Integration & evolution](./integration_and_evolution.md), [Enterprise track](./enterprise_track.md), [Kong AI Gateway](https://developer.konghq.com/ai-gateway/) (external reference).

---

## 1. Goal

**Panda should offer a Kong-class *surface area* for AI gateway concerns**—each capability **optional**, **declarative**, and **cheap to disable**—while **outperforming Kong** on the dimensions that matter for LLM and MCP traffic:

- **Native** streaming and long-lived connections (no “AI tax” from generic buffering).
- **Unified** routing decisions that understand **messages, tools, MCP servers, and models** in one place.
- **Tiered intelligence**: use **deterministic rules and small models** by default; call **larger LLMs** only when policy or traffic warrants it.

Kong’s AI Gateway excels at **plugin breadth** and **control-plane UX**. Panda’s bet is **depth on the AI data plane**: the same *categories* of features (routing, governance, cache, observability) with **finer-grained AI semantics** and **better hot-path behavior**.

### 1.1 Multi-provider coverage (product direction)

Panda should support **maximum upstream provider coverage**, not only **OpenAI-compatible** HTTP bases (Ollama, vLLM, many clouds’ “OpenAI API” modes). That means:

| Tier | Meaning | Role in Panda |
|------|---------|----------------|
| **Passthrough** | Upstream already implements OpenAI-style `/v1/chat/completions` (and related) | Default path: `adapter.provider: openai`, minimal transformation |
| **Native adapters** | Vendor uses a different URL, headers, or JSON shape (Anthropic Messages, AWS/GCP/Azure hosted models, etc.) | **Per-provider** modules in `outbound::adapter` (and streaming peers): map **ingress** OpenAI-shaped requests to **upstream** wire format and map responses back |
| **Proxy escape hatch** | Rare APIs or previews | Optional raw **`/v1/*` proxy** patterns or dedicated routes—still behind the same auth, TPM, and observability |

**Today:** `adapter.provider` validates **`openai`** and **`anthropic`** for chat; semantic routing’s optional “router” model assumes an OpenAI-compatible **`/v1/chat/completions`** unless extended.

**Direction:** Add validated providers and adapters incrementally (config + tests + docs), reusing one **request context** so TPM, MCP, cache, and policy stay unified. Prefer **native** adapters when passthrough would lose capabilities (auth, streaming quirks, tool formats); prefer **passthrough** when the vendor is already OpenAI-compatible.

This is the same **“Phase 4: universal adapter target provider”** thread already called out on [`AdapterConfig`](../crates/panda-config/src/lib.rs) in `panda-config`.

**Practical guides:** [Provider adapters — Portkey-style patterns](provider_adapters.md) (passthrough labels vs native Rust adapters, checklist). **Gemini / Vertex / Bedrock:** [API keys and base URLs](provider_gemini_bedrock_vertex.md).

---

## 2. Pluggable feature model (Kong-like, Panda-shaped)

Think in **layers**, all **off by default** unless configured:

| Layer | Role | Typical cost | Enablement |
|-------|------|--------------|------------|
| **L0 — Static** | Path/host/header → upstream (today’s `routes`, adapters). | Negligible | YAML `routes` + `upstream` / `type`. |
| **L1 — Policy** | Allow/deny tools, MCP servers, models; TPM; trusted gateway headers. | Low | Existing limits, `mcp_servers`, Wasm, JWT. |
| **L2 — Heuristic / cache** | Semantic cache hits; rule-based model failover; circuit breaker. | Low–medium | `semantic_cache`, `model_failover`, Redis. |
| **L3 — Embedding / classifier** | Route by embedding similarity, intent class, or “cheap model” triage. | Medium | Optional sidecar or managed embed API; **batch + cache** embeddings. |
| **L4 — LLM-as-router** | Full natural-language routing or complex agent delegation. | High | **Explicit** routes or budgets; never the default for every request. |

**Principle:** *Pluggable* means each layer is **independently toggled** in config; missing dependencies (e.g. no embed endpoint) **fail closed or degrade** according to a declared `routing.fallback` policy (e.g. “use L0 only,” “shadow log L3 decision”).

**Kong analogy:** Kong enables plugins per route/service. Panda maps to **per-route (or global) routing profiles** that list **which sub-engines run** and in **what order**—closer to a **composed pipeline** than a flat Lua chain, but still **GitOps-friendly** (one YAML, no mandatory DB on every replica).

---

## 3. Routing dimensions (what “AI routing” means here)

These are **orthogonal axes**; a single request may pass through several.

### 3.1 Model routing

**Definition:** Choose **which upstream model / provider** serves the completion (including failover, parity maps, A/B).

**Today in Panda:** Per-route `upstream`, adapter `type`, enterprise **model parity** and **ordered failover** (see `panda.example.yaml`).

**Better than Kong (directional):** **Stream-safe** failover defaults (e.g. no silent mid-stream switch without policy); **capability-aware** chains (tool-capable vs non-tool models); **explicit** latency vs cost vs quality **profiles** in one config object.

### 3.2 Semantic routing

**Definition:** Route to a **different model or policy** based on **meaning** of the prompt (or session), not only the path.

**Kong:** Semantic routing / semantic load balancing as **plugins** over normalized LLM APIs.

**Panda approach:**

1. **Fast path:** Embedding of a **truncated prefix** + **cached** route table (ANN or bucketed centroids)—**async** where possible so the gateway does not block the client on embed latency unless configured to wait.
2. **Cheap model path:** A **small** classifier or routing model (“is this coding? legal? PII-heavy?”) with **hard timeouts** and fallback to L0.
3. **Never** run a full frontier LLM on every request unless `routing.semantic.mode: llm_judge` (or similar) is explicitly enabled with **budgets**.

**Performance:** Dedupe embeds per **session or rolling window**; **quantized** local embedder optional for air-gapped; **shadow mode** to compare decisions without changing upstream.

### 3.3 Tool routing

**Definition:** Given an **OpenAI-style tools** or **function-call** surface, decide **which tools are visible**, **which backend executes them**, and **ordering / parallelization** hints.

**Today in Panda:** MCP integration, `mcp.intent_tool_policies`, per-route `mcp_servers`, and **`mcp.tool_routes`**: ordered rules with `pattern` (`*` wildcard), `action` (`allow` \| `deny`), optional `servers` allowlist on `allow`, plus `unmatched: allow|deny`. Matches are evaluated against the OpenAI function name (`mcp_server_tool`) and `server.tool`. Blocked tools are omitted from advertisement; if the model still calls one, the gateway returns a tool message and increments **`panda_mcp_tool_route_events_total`**.

**Better than Kong (directional):** **Per-tenant** tool surfaces, **HTTP executors** beside MCP, and **proof-of-intent** alignment (policy + optional small model) on top of pattern rules.

### 3.4 MCP routing

**Definition:** Which **MCP servers** attach to which **routes / clients / sessions**; transport (stdio vs SSE); **multi-server** fan-out and **merge** policies.

**Today in Panda:** MCP host, `mcp_servers` per route, stream probes.

**Better than Kong (directional):** Kong positions “MCP traffic gateway” as part of AI Gateway; Panda already hosts MCP **in-process**. Next steps are **explicit MCP routing rules**: e.g. “`finance/*` tools → server A only,” **sticky** server affinity per session, **health-aware** server selection, and **observability** that attributes **token + tool + MCP** cost in one trace span.

### 3.5 Agent routing

**Definition:** Where **multi-step agent** traffic should go: **single model**, **orchestrator**, **sub-agent** endpoints, or **human-in-the-loop** gates.

**This is orchestration-adjacent**; the gateway should **not** replace full agent frameworks but should **enforce**:

- **Entry routes** per agent profile (which models/tools/MCP are allowed).
- **Delegation budgets** (max sub-calls, max TPM per “agent session” id).
- **Optional** routing to a **dedicated** “planner” upstream when headers or claims mark `agent_profile: research` vs `agent_profile: codegen`.

**Better than Kong:** **Session-scoped** limits and **correlation** (trace + `X-Panda-Agent-Session`) natively on the data plane, with **same** YAML vocabulary as model and MCP routing.

**Implemented in Panda:** optional `agent_sessions` — `enabled`, session `header` (default `x-panda-agent-session`), `profile_header` (default `x-panda-agent-profile`), `tpm_isolated_buckets` (default true), optional `jwt_session_claim` / `jwt_profile_claim` (Bearer JWT, validated like other gateway JWTs), `mcp_max_tool_rounds_with_session` (tightens MCP tool-followup rounds when a session id is present), and `profile_upstream_rules[]` (`profile`, `upstream`, `path_prefix`, optional per-rule `mcp_max_tool_rounds`). TPM prompt-budget keys can include a short hash suffix per session id. Session and profile values are taken from JWT first when configured, then **overridden** by the respective headers when present; both are echoed on responses and included in request logs. Profile rules pick the **longest** matching `path_prefix` (same spirit as `routes`). Trust the edge to set or strip headers for untrusted clients.

---

## 4. Unified decision pipeline (conceptual)

For each AI request, Panda can evaluate a **fixed pipeline** (order configurable per route):

1. **Identity & trust** — JWT, Kong handshake, tenant id.
2. **Static match** — `routes` prefix → default upstream + adapter.
3. **Semantic / model class** — Optional L2/L3/L4 as configured.
4. **Tool & MCP allow** — Strip or inject tool lists; attach MCP servers.
5. **Budget** — TPM / hierarchical TPM (enterprise).
6. **Execute** — Stream to chosen upstream; on failure, **model failover** per policy.
7. **Observe** — One span: `routing.decision`, `model.id`, `mcp.servers`, `tools.filtered`, cache hit/miss.

This gives **one place** to explain “why this request went there,” which is harder when AI behavior is split across many unrelated Kong plugins.

---

## 5. Performance checklist (non-negotiables)

- **No unbounded LLM calls** on the hot path; **defaults** stay L0–L2.
- **Embed cache** with TTL and **normalized** text keys (strip volatile system noise where safe).
- **Timeouts** on every L3/L4 call; **fallback** always defined.
- **Streaming**: routing decision completes **before** first byte upstream when possible; if semantic routing must wait, document **trade-off** and offer **preflight** mode for clients that can tolerate one extra round trip.
- **Wasm** for **synchronous** policy; **async** sidecars for **heavy** ML.

---

## 6. Roadmap shape (documentation-only; tracks engineering planning)

**Config (implemented in `panda-config`):** top-level `routing` with `enabled`, `shadow_mode`, `fallback` (`static` \| `deny`), and `semantic` (`enabled`, `mode`, embed or router URLs, `embed_model`, `router_model`, API key env vars, `similarity_threshold`, `targets[]` with `name` / `routing_text` / `upstream`, timeouts, cache TTL, prompt truncation). Per-route `routes[].routing` can override `enabled`, `shadow_mode`, and `semantic_enabled`. Helpers: `effective_routing_*_for_path`.

**`panda-proxy` semantic modes (for `POST /v1/chat/completions` JSON):**

- **`embed`** — Warms embeddings for each target’s `routing_text` at startup; embeds the prompt (cached); picks the best cosine match vs `similarity_threshold`; `shadow_mode` logs the would-be upstream without switching.
- **`classifier`** / **`llm_judge`** — Calls OpenAI-style `POST {router_upstream}/chat/completions` with `router_model`, using `router_api_key_env`. The model must return JSON: `{"target":"<route id>" | null, "confidence": <0.0–1.0>}` (`confidence` optional, defaults to `1.0`). `similarity_threshold` is the minimum confidence; `routing_text` per target is an optional hint in the system prompt (`llm_judge` uses a stricter instruction). In-memory cache keyed by prompt hash. `fallback: deny` returns 503 on router errors (`semantic_routing_failed`). Set **`routing.semantic.router_response_json: true`** to send `response_format: json_object` (OpenAI-compatible APIs only; leave false for broader compatibility).

Semantic cache keys include the effective upstream base (embed path). **Semantic cache hits** still emit the same semantic-route headers when routing ran before the cache lookup.

**Observability**

- **Response headers (on threshold match):** `x-panda-semantic-route` (applied), `x-panda-semantic-route-shadow` (shadow match), `x-panda-semantic-route-score` (embed: cosine; classifier / judge: model confidence).
- **`panda_semantic_routing_events_total{event,target}`** — `applied`, `shadow`, `below_threshold`, `no_prompt`, `embed_failed_static`, `embed_failed_deny`, `router_failed_static`, `router_failed_deny`.
- **`panda_semantic_routing_resolve_latency_ms_*`** — Cumulative histogram (`_bucket` with `le` in ms), `_sum`, and `_count` for time spent inside semantic `resolve` (embed + match or router chat + parse), per request where the stage runs.
- **`panda_mcp_tool_route_events_total{event,rule}`** — `advertise_blocked` (tool stripped before upstream) and `call_blocked` (invocation denied); `rule` is the matching pattern or `unmatched`.

**Production hardening:** Use `routing.semantic.timeout_ms` for every embed/router call; integration tests in `panda-proxy` use a local TCP mock for `/v1/embeddings` and `/v1/chat/completions` to validate parsing and upstream selection without external APIs.

Suggested evolution (not committed schedule):

1. **Proxy wiring** — Done for embed + classifier + `llm_judge` MVP; `router_response_json` optional; custom prompts and non–OpenAI-compatible routers can follow.
2. **Tool/MCP routing tables** — **Implemented:** `mcp.tool_routes` with patterns, allow/deny, server-scoped allows, and metrics; HTTP-side tool executors remain future work.
3. **Agent session budgets** — **Implemented:** `agent_sessions` (header and/or JWT claims for session + profile), TPM bucket isolation, profile-based static upstream override before semantic routing, per-session and per-profile MCP `max_tool_rounds` caps, response echo and logs.
4. **Console / API** — Read-only “why routed” for ops (parity with Kong’s observability story, without requiring Konnect).

Implementation details belong in [`implementation_plan.md`](./implementation_plan.md) as milestones are picked up.

---

## 7. Summary

| Theme | Kong strength | Panda direction |
|-------|----------------|-----------------|
| Feature breadth | Many **plugins** and vendor integrations | **Same categories**, **pluggable stages**, **native** LLM/MCP/stream |
| Semantic routing | First-class in product | **Tiered** (embed → small model → optional LLM), **cache-heavy**, **stream-aware** |
| MCP | MCP as part of AI Gateway | **In-process host** + **explicit MCP routing** and observability |
| Ops model | Konnect, decK, enterprise UI | **GitOps YAML** + optional console; **shadow** and **export** for existing stacks |

**Bottom line:** Panda should be **easier to turn features on/off** than ad-hoc plugin sprawl, **faster** on streaming AI paths, and **smarter** where it counts—**without** paying an LLM tax on every request.

---

*This document is strategic; align code and `panda.example.yaml` with it as features land.*

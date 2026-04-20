# Panda vs Kong: positioning (one page)

**Kong** (and similar L7 gateways) excel at **enterprise edge concerns**: authentication integration, rate limiting, routing, observability plugins, and a huge ecosystem. They are the natural **front door** for HTTP APIs.

**Panda** is an **AI-native data-plane gateway**: it is shaped around **long streams**, **unstructured bodies**, **MCP tool orchestration**, **Wasm policy plugins**, and **token / TPM budgeting** that tracks LLM usage—not just request counts. Panda’s default story is a **small, stateless Rust binary** with GitOps YAML, optional Redis for shared counters, and an explicit **trusted hop** from the edge when Kong (or another gateway) already performed OIDC or mTLS.

## How they fit together

| Concern | Typical owner |
|--------|----------------|
| Public TLS, WAF, first-line auth, global rate limits | Kong / edge |
| OpenAI-compatible routing, Anthropic adapters, semantic cache | Panda |
| MCP stdio/SSE tool hosts, tool timeouts, fail-open semantics | Panda |
| Per-user/token TPM, degraded mode when Redis is unhealthy | Panda |
| Wasm request/body policy (PII, custom allow/deny) | Panda |
| Compliance-oriented audit export (local stub → object store) | Panda (+ storage pipeline) |

The **unified vision** is **layered**: Kong remains the **control-plane-friendly edge**; Panda is the **AI edge** immediately upstream of model providers or internal inference clusters. Use the [Kong handshake](./kong_handshake.md) so Panda receives **attested** identity headers instead of trusting clients. Where traffic is entirely AI-shaped, a **[domain split](./integration_and_evolution.md)** (APIs on Kong, LLM paths on Panda) keeps each tier doing what it does best.

## When to choose what

- **Kong alone** — Fine for simple “HTTP in, HTTP out” LLM proxies if you do not need MCP hosting, stream-native TPM, Wasm plugins, or the Panda roadmap (intent routing, richer audit).
- **Panda behind Kong** — Preferred for production: reuse Kong’s auth and fleet patterns; add Panda for model traffic only.
- **Panda standalone** — Acceptable for dev, PoCs, or small deployments that accept Panda owning TLS and optional JWT/JWKS (see `panda.example.yaml` and `docs/standalone_deployment.md`).

Panda does not aim to replicate Kong’s full plugin marketplace; it aims to be the **specialized second hop** that makes LLM + MCP traffic **safe, observable, and operable** in the same vocabulary your platform team already uses (Prometheus, structured logs, GitOps).

For a concrete plan to match **Kong-class AI gateway surface area** with **pluggable** stages (static → cache → embeddings → optional LLM routing) and to go **deeper** on **semantic, model, tool, MCP, and agent routing**, see **[AI routing strategy](./ai_routing_strategy.md)**.

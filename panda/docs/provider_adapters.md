# LLM provider adapters — learning from Portkey-style gateways

**Audience:** Engineers adding **upstream diversity** while keeping Panda’s **one client surface** (`POST /v1/chat/completions`, etc.).

**References:** [Portkey AI Gateway](https://github.com/Portkey-AI/gateway) (TypeScript, many providers), Panda [`outbound::adapter`](../crates/panda-proxy/src/outbound/adapter.rs), [`ai_routing_strategy.md`](./ai_routing_strategy.md) §1.1.

---

## 1. What Portkey does well (patterns to borrow)

| Pattern | In Portkey | Panda equivalent |
|--------|------------|------------------|
| **One ingress API** | OpenAI-style routes; clients send familiar JSON | Panda **`/v1/chat/completions`** + optional Anthropic adapter |
| **Provider modules** | `src/providers/<vendor>/` with `chatComplete.ts`, `headers`, `getBaseURL`, transforms | **`outbound/adapter.rs`** + **`adapter_stream.rs`** per **native** protocol (Anthropic today) |
| **Passthrough** | Many vendors expose **OpenAI-compatible** base URLs | Same HTTP shape → **`adapter.provider`** = any [**OpenAI-shaped label**](../crates/panda-config/src/lib.rs) + correct `routes[].backend_base` |
| **Retries / fallbacks** | `retryHandler`, conditional router | Panda **`model_failover`**, **`rate_limit_fallback`** (YAML) |
| **Guardrails** | Large `plugins/` tree | Panda **Wasm**, prompt safety, PII, JWT |

Portkey’s scale comes from **many small transform modules** + **central orchestration** (`handlerUtils`, `ProviderAPIConfig`). Panda’s Rust stack favors **fewer, well-tested adapters** and **explicit** config over header-driven `config` blobs.

---

## 2. Two tiers in Panda

### A. OpenAI-shaped passthrough (no extra Rust)

If the upstream documents an **OpenAI-compatible** chat API (same paths and JSON as OpenAI), set:

- **`routes[].backend_base`** (or `default_backend`) to that vendor’s base URL.
- **`adapter.provider`** to **`openai`** or a **specific label** from [`OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS`](../crates/panda-config/src/lib.rs) (e.g. `groq`, `together`, `ollama`, `openrouter`). Behavior is **identical** to `openai`; the label helps **metrics, logs, and GitOps** show which vendor you target.

**Auth:** Panda **forwards** the client’s `Authorization` (and other) headers to the upstream. For **Gemini, Vertex, and Bedrock** base URLs and keys, see **[`provider_gemini_bedrock_vertex.md`](provider_gemini_bedrock_vertex.md)**. **`model_failover`** / **`rate_limit_fallback`** backends can use **`api_key_env`** where documented in `panda.example.yaml`.

### B. Native adapter (Rust in `panda-proxy`)

If the upstream uses a **different** path, headers, or body shape (e.g. **Anthropic Messages**), add:

1. Request mapping: OpenAI chat JSON → provider JSON (`openai_chat_to_anthropic` style).
2. Response mapping: provider JSON or SSE → OpenAI chat / SSE (`adapter_stream.rs`).
3. Config: **`adapter.provider: anthropic`** (or future provider names once validated).
4. Tests: unit + integration under `crates/panda-proxy`.

**Future examples:** cloud APIs that are *not* drop-in OpenAI-compatible (some Bedrock/Vertex modes, bespoke hosts) — each gets a **named** `adapter.provider` and code in `outbound/`.

---

## 3. Choosing a label vs `openai`

| You want… | Use |
|-----------|-----|
| Minimal config | `adapter.provider: openai` |
| Clear ops / audits (“this route is Groq”) | `groq`, `together`, `mistral`, … (see list in `panda-config`) |
| Anthropic’s API | `anthropic` (triggers native adapter) |
| A vendor not in the list but OpenAI-compatible | `openai_compatible` or ask to add the label in `OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS` |

Unknown names (e.g. `gemini`) **fail config validation** until we add either a passthrough label or a **native** adapter.

---

## 4. Checklist: add a **native** provider

1. Confirm the API is **not** satisfied by OpenAI-shaped passthrough.
2. Add provider string to `ensure_adapter_provider_allowed` / docs (or extend validation rules).
3. Implement `outbound/adapter.rs` (and streaming) transforms.
4. Wire `effective_adapter_provider` / `is_anthropic_provider`-style checks (pattern: `== "anthropic"` → generalize to a small match or registry if many providers).
5. Extend `panda.example.yaml` and this doc.
6. Add `panda-config` and `panda-proxy` tests.

---

## 5. Summary

- **Learn from Portkey:** modular providers + passthrough where the wire format matches; central place for routing and reliability.
- **Panda today:** strong **Anthropic** mapping + **many OpenAI-compatible** upstreams via **labels** and **`backend_base`**.
- **Grow:** add **native** Rust adapters only where the wire format differs; keep **one** ingress story for applications.

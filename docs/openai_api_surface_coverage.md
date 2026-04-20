# OpenAI-style API surface: what Panda implements vs proxies

**Question:** Is Panda “chat only,” or can it match Portkey-style breadth (chat, completions, embeddings, images, audio, files, batches, responses, Anthropic `/v1/messages`, realtime WebSocket)?

**Short answer:** Panda is **not** limited to chat at the **transport** layer: most **`/v1/...`** requests are **reverse-proxied** to `default_backend` / `routes[].backend_base` with the same path and method. **Deep product features** (MCP, semantic cache, semantic routing, Anthropic body mapping, chat TPM heuristics, rate-limit fallback snapshots) are **centered on** **`POST /v1/chat/completions`**. **OpenAI Realtime** (browser/SDK WebSocket to the model API) is **not** implemented for upstream. **Anthropic** is supported by mapping **from** OpenAI chat **`/v1/chat/completions`**, not by advertising a separate first-class **`POST /v1/messages`** entry (that would pass through unchanged).

See also: [`panda_data_flow.md`](./panda_data_flow.md) (`forward_to_upstream`), [`provider_adapters.md`](./provider_adapters.md), [`architecture_two_pillars.md`](./architecture_two_pillars.md).

---

## 1. Layered model

| Layer | Meaning |
|-------|--------|
| **A — HTTP proxy** | Join client path + query to configured upstream base; forward headers/body; stream responses. Applies to **any** path that reaches `forward_to_upstream` (subject to JWT, route `methods`, RPS, body limits, etc.). |
| **B — Chat-centric intelligence** | MCP tool merge, semantic cache (chat), semantic routing (chat), Anthropic adapter (chat), non-streaming MCP tool loops, rate-limit fallback snapshot (chat), context management (chat). **Gated on** `POST /v1/chat/completions` (and related checks) in `lib.rs`. |
| **C — Enterprise model failover** | Optional parity/failover chains for **chat**, **embeddings**, **responses**, **images**, **audio** — see `model_failover` path prefixes in `panda.example.yaml` and `outbound/model_failover.rs`. |

So: **many** OpenAI API routes work as **layer A**. **Layer B** is intentionally **chat-first**.

---

## 2. Rough matrix (today)

| Surface | Typical path | Proxy (A) | Chat-style features (B) | Failover (C) |
|--------|----------------|-----------|---------------------------|----------------|
| Chat completions | `POST /v1/chat/completions` | Yes | **Yes** (MCP, cache, routing, Anthropic map, …) | Yes (if enabled) |
| Legacy completions | `POST /v1/completions` | Yes | No (no MCP/cache/routing stack on this path) | If `path_prefix` matches failover chat prefix only — tune YAML |
| Embeddings | `POST /v1/embeddings` | Yes | No | Yes — `model_failover.embeddings_path_prefix` |
| Images | `POST /v1/images/...` | Yes | No | Yes — `images_path_prefix` |
| Audio | `POST /v1/audio/...` | Yes | No | Yes — `audio_path_prefix` |
| Files / batches | `/v1/files`, `/v1/batches`, … | **Usually** yes (opaque HTTP) | No | Not separately classified in failover helper — treat as proxy unless you extend config |
| Responses API | `POST /v1/responses` | Yes | No | Yes — `responses_path_prefix` |
| **Anthropic Messages** | `POST /v1/messages` | Passthrough to upstream | **No** automatic OpenAI→Anthropic mapping on this path | Anthropic protocol on failover backends is supported in **failover** chains, not as a global “ingress path” |
| **OpenAI Realtime** | `GET` + `Upgrade: websocket` … | **No** end-to-end proxy to upstream Realtime today | N/A | N/A |

**Anthropic usage in Panda:** set `adapter.provider: anthropic` and call Panda at **`POST /v1/chat/completions`**; the proxy maps body + SSE to Anthropic **`/v1/messages`**. Clients that speak **native** `POST /v1/messages` **directly** to Panda get a **plain proxy** (no OpenAI-shaped mapping).

**WebSockets in the repo:** used for the **developer console** (`/console/ws`), not for **OpenAI Realtime** to `api.openai.com` or similar.

---

## 3. Can we support “most of” the OpenAI API + Anthropic + Realtime like Portkey?

**Yes, in principle — with different effort:**

1. **Already largely true for HTTP JSON + many multipart flows** — rely on **layer A** and good `routes` / `backend_base`; validate with integration tests per content type.
2. **Parity with Portkey’s “dedicated handler per route”** — incremental: per-route metrics, transforms, provider quirks (e.g. Azure `api-key` header) — extend `forward_to_upstream` and config **without** forcing every path through the chat pipeline.
3. **Anthropic native ingress** — optional future: detect `POST /v1/messages` + `anthropic-version` and either passthrough only or dual-stack mapping.
4. **Realtime (WebSocket)** — new work: HTTP upgrade on the listen socket, bidirectional copy to upstream WebSocket, auth/header policy, timeouts — comparable to adding a **new subsystem** (Portkey’s `realtimeHandler` on workerd is separate from chat).

---

## 4. Practical guidance

- **Need embeddings + chat with one gateway?** Configure **`routes`** so both `/v1/chat` and `/v1/embeddings` prefixes point at the right bases; enable **`model_failover`** for each surface you care about.
- **Need Portkey-level route surface overnight?** Prefer **layer A** proxy first, then add **first-class** features only where product requires (cost, policy, MCP).
- **Need Realtime?** Plan a **dedicated** feature (or terminate WebSocket at another edge and forward); not a YAML toggle today.

---

*This document is descriptive; align `panda.example.yaml` and tests as features expand.*

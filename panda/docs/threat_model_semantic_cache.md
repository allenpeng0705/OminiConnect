# Threat model notes: semantic cache (Phase 4)

Semantic cache returns a **stored completion** without calling the upstream model when a similarity key matches. This trades cost and latency for **new trust assumptions**. Use this doc for DPIA-style discussions and hardening priorities—not as a formal pen-test sign-off.

## Assets

- **Cached payloads** — Past model outputs (may contain PII or secrets if the model echoed them).
- **Key material** — Embedding inputs (prompt text, optional tool metadata, upstream base) that determine hits.
- **Redis / backend** — Vector or key-value store holding entries; compromise can read or poison cache.

## Threats and mitigations (directional)

| Concern | Risk | Mitigations in Panda direction |
|---------|------|----------------------------------|
| **Cache poisoning** | Attacker crafts prompts that map to the same key as a victim’s topic and receives another tenant’s completion. | Strong **tenant/subject dimensions** in routing and identity; **similarity threshold** tuning; **TTL** and eviction; optional **per-tenant cache namespaces** in deployment (separate Redis DB, key prefix from JWT subject). |
| **Cross-user leakage** | Shared global cache without tenant isolation. | Do **not** share one semantic-cache namespace across untrusted tenants; align cache keys with **effective upstream + model + tools fingerprint** (see routing docs). |
| **Stale or wrong-provider answers** | Failover or model change returns incompatible cached body. | Keys include **logical model / upstream** where configured; review after **model_failover** changes. |
| **Observability leakage** | Cache hit/miss headers or logs expose usage. | Treat logs as sensitive; redact policies for compliance export. |

## Operational controls

- **`semantic_cache.enabled`** and per-route overrides — disable on paths that are too sensitive for reuse.
- **Threshold** — higher threshold reduces false “hits” (and poisoning impact) at the cost of fewer savings.
- **TTL** — bound how long a poisoned or outdated entry survives.
- **`semantic_cache.similarity_fallback`** — default **`false`**: in-memory backend only returns **exact** key matches. Set **`true`** only if you accept Jaccard-based near matches on the chat cache key (Redis backend remains exact-key only).
- **`semantic_cache.scope_keys_with_tpm_bucket`** — when **`true`**, the cache key includes the same TPM bucket string as prompt budgeting (subject / tenant / optional agent-session hash), reducing cross-principal reuse.

## Embedding lookup (optional, memory backend)

When **`semantic_cache.embedding_lookup_enabled=true`** (requires **`backend: memory`**), Panda may call an **OpenAI-compatible embeddings HTTP API** after an exact-key and optional Jaccard miss, then match stored **L2-normalized** vectors by **cosine ≥ `similarity_threshold`**.

| Topic | Notes |
|-------|--------|
| **New egress** | Prompt-derived text (from the canonical cache key’s `messages`) is sent to **`semantic_cache.embedding_url`** using the API key from **`semantic_cache.embedding_api_key_env`**. Same DPIA considerations as any third-party model API: content may include user data; log retention at the provider applies. |
| **Index location** | Embeddings for cache entries live **in-process** with the memory cache (not shared across replicas). Each replica builds its own near-match index from entries it has stored. |
| **Compatibility gate** | Near-match only considers entries with the same **model** and **tools signature** as encoded in the cache key (same contract as Jaccard fallback). |
| **Response labeling** | Successful embedding near-misses may return header **`x-panda-semantic-cache: hit-embedding`**; Prometheus hit counter still increments. |

**When not to enable:** Untrusted multi-tenant traffic without **`scope_keys_with_tpm_bucket`** / **`agent_sessions`**-style isolation, unless you accept higher cross-principal semantic collision risk; environments where **any** extra egress of prompt text to an embedding vendor is prohibited.

Example YAML snippets: [`panda.example.yaml`](../panda.example.yaml) (`semantic_cache.embedding_*` comments).

## References

- [`ai_routing_strategy.md`](./ai_routing_strategy.md) — routing + cache behavior  
- [`compliance_export.md`](./compliance_export.md) — audit fields  
- [`implementation_plan.md`](./implementation_plan.md) — Phase 4 / enterprise north-star  

# Agent fleet gaps — phased roadmap

This document turns the “still missing / still open / still thin / still a gap” items into **ordered work** you can schedule. It complements [`openclaw_fleet_profile.md`](./openclaw_fleet_profile.md) and [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md).

---

## Do we plan first, then “finish everything”?

**Plan first — yes.** **Finish everything in one release — no.**

- The gaps span **metrics**, **compliance**, **caching**, **streaming resilience**, **Wasm**, and **external** systems (Prometheus, log stores, Redis). Trying to land all of that at once increases coupling and rollback risk.
- Use this roadmap to pick **one vertical slice** per iteration (e.g. “on-call can find a hot principal using documented joins” *before* “cluster-wide top-tenant API”).
- Mark items **Panda repo** vs **operator environment** so ownership is clear.

---

## Principles

1. **Low-cardinality Prometheus** — do not add `tenant` / `session` labels to high-volume counters; use `/tpm/status`, logs, compliance JSONL, or **external** top-N aggregation.
2. **Replica-local vs fleet** — `/ops/fleet/status` and in-process counters are **per process**; fleet-wide views need scrape + PromQL or another aggregator.
3. **Explicit non-goals per phase** — say what you are *not* building this quarter to avoid scope creep.

---

## Track A — Fleet / SRE (“top token-burner” in minutes)

| Priority | Work | Owner | Notes |
|----------|------|--------|--------|
| **P0** | Tighten **runbook** with a concrete &lt;5 min playbook: PromQL (existing low-cardinality series) → **which log fields** / compliance rows → **`/tpm/status`** with the same headers as the client. | Docs + SRE | Extends [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md). |
| **P1** | **Recording rules** / alert templates (starter YAML or doc snippets) for TPM reject spikes, MCP rounds exceeded, tool-cache bypass bursts. | Docs or `docs/grafana/` | No Panda code required initially. |
| **P2** | Optional **Panda feature**: bounded **top-K TPM buckets** from Redis (admin-gated, rate-limited, K small) — **design spike** before coding. | Engineering | Only if `/tpm/status` + logs are insufficient at your scale. |

**Non-goal (near term):** High-cardinality per-tenant Prometheus labels on all requests.

---

## Track B — Semantic cache “product” depth

| Priority | Work | Notes |
|----------|------|--------|
| **P0** | Keep **threat model** aligned with **embedding lookup** ([`threat_model_semantic_cache.md`](./threat_model_semantic_cache.md)): egress to embed API, key contract, when to enable. | Mostly docs |
| **P1** | **External vector index** (Redis vector / pgvector / Milvus): adapter behind semantic cache — **larger** epic; depends on keying + TTL + multi-replica invalidation design. | Future major |
| **P2** | **Provider-native prompt cache** headers (passthrough or policy): product decision + per-provider matrix. | OpenClaw-class parity |
| **P2** | **Stable prompt blocks** / prefix-cache semantics at the gateway: may overlap with provider features; avoid duplicating OpenAI/Anthropic unless Panda owns the contract. | Design first |

**Already shipped (baseline):** memory/Redis exact, optional Jaccard (memory), optional embedding cosine (memory) — see [`implementation_plan.md`](./implementation_plan.md) Phase 4.

---

## Track C — Tool cache MVP audit hardening

| Priority | Work | Notes |
|----------|------|--------|
| **P0** | **Operator matrix** in [`tool_cache_mvp.md`](./tool_cache_mvp.md): which events emit **`panda.compliance.tool_cache.v1`**, when **`compliance_log_misses`** matters, retention expectations. | Docs |
| **P1** | **Code audit**: assert every **bypass** path calls `record_tool_cache` when compliance is on; add tests if any gap. | Small code + tests |
| **P2** | **Sampled or rate-limited miss** logging for high-volume tools (configurable sample rate). | Feature |

**Non-goal (near term):** A second parallel schema unless compliance consumers require it; prefer one stream with clear `event` / phase fields.

---

## Track D — Resilience on the agent hot path

| Priority | Work | Notes |
|----------|------|--------|
| **P0** | **Docs truth**: buffered mid-stream SSE failover is **opt-in**; MCP streaming follow-up and Anthropic adapter streaming **excluded** — keep [`enterprise_track.md`](./enterprise_track.md) and `/ready` text aligned. | Ongoing |
| **P1** | **MCP + failover interaction**: design options (e.g. probe completion before buffering, or explicit “no mid-stream for tool loops”). | RFC in docs |
| **P2** | **Seamless** chunk-level splice across backends — **hard** for OpenAI SSE shape; treat as research / non-goal unless product demands. | |
| **P2** | **Upstream health** beyond circuit breaker (passive health, weighted ordering). | Enterprise |

---

## Track E — Wasm / agent policies

| Priority | Work | Notes |
|----------|------|--------|
| **P1** | **RFC doc**: isolation model (memory/time), streaming chunk hook surface, failure modes vs fail-open/fail-closed. | No code |
| **P2** | Implementation spikes against existing plugin hooks. | After RFC |

---

## Track F — Scope boundary (unchanged)

Panda remains the **AI traffic and governance edge** — not OpenClaw’s **scheduler**, **multi-agent RPC**, or **cluster orchestration**. Revisit only if product scope explicitly expands.

---

## Suggested rolling plan (example)

| Horizon | Focus |
|---------|--------|
| **Now → 2 weeks** | Track **A** runbook + recording-rule snippets; Track **C** audit matrix + any missing `record_tool_cache` calls. |
| **2–6 weeks** | Track **B** threat-model refresh; Track **D** MCP×failover design note; Track **E** Wasm RFC. |
| **6+ weeks** | Pick **one** of: Redis vector semantic cache (**B**), sampled miss logging (**C**), upstream health (**D**), Wasm prototype (**E**). |

Adjust to your team size; **do not** parallelize all tracks without owners.

---

## See also

- [`openclaw_agent_quickstart.md`](./openclaw_agent_quickstart.md) — operator surface area  
- [`openclaw_fleet_profile.md`](./openclaw_fleet_profile.md) — persona and success themes  
- [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md) — PromQL + under-5-minute playbook  
- [`grafana/recording_rules.agent_fleet.yaml`](./grafana/recording_rules.agent_fleet.yaml) — Prometheus recording rules / alert snippets  
- [`mcp_failover_streaming.md`](./mcp_failover_streaming.md) — MCP follow-up vs buffered mid-stream failover  
- [`wasm_agent_policy_rfc.md`](./wasm_agent_policy_rfc.md) — Wasm isolation and streaming hooks (RFC)  
- [`kong_replacement_program.md`](./kong_replacement_program.md) — program-level status  

# OpenClaw-Class Agent Quickstart with Panda Unified Gateway

This guide shows how a small agent runtime (OpenClaw-style loop with tools + MCP) can use Panda's unified gateway for:

- token visibility and control across all traffic types,
- intent-aware tool governance with the MCP gateway,
- API gateway capabilities for traditional REST services,
- lower token spend through cache and routing features,
- safer defaults on the AI traffic path,
- unified control plane for all gateway functions.

Use this as the "small agent profile" baseline before enterprise controls.

---

## Why Panda Unified Gateway for OpenClaw-like agents

OpenClaw-style systems usually have:

- long-running sessions,
- many tool calls,
- growing conversation/tool-result context,
- high sensitivity to token cost and safety mistakes,
- integration with traditional REST services.

Panda's unified gateway helps by providing a single entry point for all agent traffic:

- centralizes budget/rate controls across AI, MCP, and API traffic,
- enforces policy once for all agents and services,
- provides unified status/metrics endpoints for operations,
- enables semantic routing/cache and MCP controls,
- simplifies architecture with a single gateway for all traffic types,
- provides API gateway capabilities for traditional services.

Reference materials reviewed:
- [OpenClaw distributed runtime RFC](https://github.com/openclaw/openclaw/issues/42026)
- [OpenClaw AGENTS.md](https://github.com/openclaw/openclaw/blob/main/AGENTS.md)
- [OpenClaw MCP orchestration overview](https://dev.to/ollieb89/how-openclaw-implements-mcp-for-multi-agent-orchestration-36hk)
- [OpenClaw prompt caching](https://open-claw.bot/docs/cli/reference/prompt-caching/)
- [OpenClaw token usage docs](https://openclaws.io/docs/reference/token-use)

---

## Minimal architecture

- **OpenClaw runtime / agent process** -> calls Panda's unified gateway endpoints:
  - OpenAI-compatible AI endpoints for model calls
  - MCP endpoints for tool orchestration
  - API endpoints for traditional REST services
- **Panda Unified Gateway** -> applies auth/policy/cache/routing across all traffic types:
  - AI Gateway: token budgets, semantic cache, model adapters
  - MCP Gateway: tool orchestration, tool result cache
  - API Gateway: ingress/egress routing, auth, rate limits
- **Model providers** -> OpenAI-compatible and/or adapter-backed upstreams.
- **Traditional services** -> accessed through the API gateway.
- Optional: **Kong edge** in front during coexistence migration.

---

## 10-minute starter profile

1. Start with `panda.example.yaml`.
2. Keep one upstream first.
3. Turn on basic controls:
   - JWT or trusted gateway identity,
   - TPM enforcement,
   - MCP enabled with allowlisted servers,
   - API gateway enabled for ingress/egress,
   - semantic cache enabled only for low-risk agent routes.

Example baseline:

```yaml
listen: "127.0.0.1:8081"
default_backend: "https://api.openai.com"

api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /v1
        backend: ai
        methods: [POST, GET]
      - path_prefix: /mcp
        backend: mcp
        methods: [POST]
      - path_prefix: /api
        backend: egress
        methods: [GET, POST, PUT, DELETE]
  egress:
    enabled: true
    corporate:
      default_base: "http://internal-api:8080"
    allowlist:
      allow_hosts: ["internal-api:8080"]
      allow_path_prefixes: ["/api"]

identity:
  require_jwt: true

tpm:
  enforce_budget: true
  budget_tokens_per_minute: 120000

mcp:
  enabled: true
  max_tool_rounds: 6
  # Use intent policies so agents only see the right tools per task class.
  # intent_tool_policies: [...]
```

Then point your OpenClaw/OpenAI client base URL to Panda. For traditional REST services, use the `/api` path prefix.

---

## Operations endpoints for agent teams

- `GET /tpm/status`: live token window and budget status across all gateway functions (call with the **same identity headers** as your agents to see that principal’s bucket on this replica).
- `GET /mcp/status`: tool rounds, intent-policy summary, tool-result cache summary, and semantic-cache hints.
- `GET /ops/fleet/status`: **single JSON snapshot** — process (draining, connections), TPM reject counters, agent-session flags, semantic-cache config + in-process hit/miss/store totals, MCP tool-cache totals, API gateway routing stats, and governance counters since process start.
- `GET /ready`: readiness, failover notes, drain state.
- `GET /metrics`: Prometheus text (when ops auth allows) with metrics for all gateway functions.

`GET /mcp/status` includes **`agent_governance`**: per-intent policy summary (counts only), in-process counters since startup (`max_rounds_exceeded_by_bucket_class`, intent filter/deny totals), and a pointer to Prometheus. It also includes a compact **`semantic_cache`** block (`effective_bucket_scoping` is `true` when either `semantic_cache.scope_keys_with_tpm_bucket` or **`agent_sessions.enabled`** is on — TPM bucket is then part of the semantic-cache key).

`GET /metrics` exports agent-oriented counters for all gateway functions, for example:

**MCP Gateway metrics:**
- `panda_mcp_agent_max_rounds_exceeded_total{bucket_class=...}`
- `panda_mcp_agent_intent_tools_filtered_total`
- `panda_mcp_agent_intent_call_enforce_denied_total`
- `panda_mcp_agent_intent_audit_mismatch_total`
- `panda_mcp_tool_cache_{hit,miss,store,bypass}_total` (labels on hit/miss/store by server/tool; bypass includes `reason`)

**AI Gateway metrics:**
- `panda_semantic_cache_hit_total`, `panda_semantic_cache_miss_total`, `panda_semantic_cache_store_total`  
  (embedding near-misses count as hits in these series; successful responses may carry `x-panda-semantic-cache: hit-embedding`.)
- `panda_model_failover_midstream_retry_total` — extra upstream attempts after the winning hop drops mid-body (see `/ready` → `model_failover`).

**API Gateway metrics:**
- `panda_api_gateway_ingress_requests_total{path_prefix=...,method=...}`
- `panda_api_gateway_egress_requests_total{host=...,path=...}`
- `panda_api_gateway_rate_limit_exceeded_total`
- `panda_api_gateway_auth_failures_total{reason=...}`

For small teams, this is often enough to replace custom ad-hoc scripts. For on-call PromQL patterns, the **under-5-minute playbook**, and cardinality cautions, see [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md). Optional Prometheus **recording rules** live in [`grafana/recording_rules.agent_fleet.yaml`](./grafana/recording_rules.agent_fleet.yaml).

---

## Agent token-reduction playbook

Use these in order:

1. **Intent-scoped tool advertising**  
   Reduce prompt bloat by only advertising tools relevant to the current task intent.

2. **Semantic cache for stable tasks**  
   Enable only on routes where reuse is safe; include model/tool metadata in cache key policy.

3. **Tool-result caching (MVP)**  
   Allowlist safe tools, TTL, identity-scoped keys, metrics, and optional compliance JSONL for cache **hit / store / bypass** (see [`tool_cache_mvp.md`](./tool_cache_mvp.md) and [`compliance_export.md`](./compliance_export.md)).

4. **API gateway optimizations**  
   - Route traditional services through the API gateway to reduce agent token usage
   - Use caching for static or slowly changing API responses
   - Implement rate limiting to prevent overloading backend services

5. **Profile-based upstream selection**  
   Route low-risk tasks to lower-cost models by agent profile/path.

6. **Session-aware budget controls**  
   Keep runaway loops bounded via per-session/profile limits and max tool rounds.

---

## Security baseline for OpenClaw-class agents

- Require identity (`identity.require_jwt=true`) for non-local deployments across all gateway functions.
- Keep ops endpoints behind shared-secret guard.
- Enable prompt/tool intent policies before opening broad tool catalogs.
- Configure API gateway allowlists for egress traffic to prevent unauthorized access.
- Implement rate limiting on API gateway routes to prevent abuse.
- Export compliance logs for all gateway traffic with correlation IDs.
- Fail closed on critical controls (auth/policy) in production.

---

## Recommended next steps

1. Configure **`mcp.tool_cache`** for one low-risk, deterministic tool family — see [`tool_cache_mvp.md`](./tool_cache_mvp.md) (enable **`observability.compliance_export`** if you need signed JSONL for cache decisions).
2. Configure **`api_gateway`** routes for your traditional services, including proper allowlists and rate limits.
3. Turn on **`agent_sessions`** (headers + optional TPM-isolated buckets) if you run many agents and need per-session budgets and safer semantic-cache keying.
4. Optionally enable **`semantic_cache.embedding_lookup_enabled`** with **`semantic_cache.backend: memory`** and an OpenAI-compatible **`embedding_url`** for cosine near-miss reuse (see [`panda.example.yaml`](../panda.example.yaml) comments).
5. If you use **`model_failover`** on streaming chat, evaluate **`allow_failover_after_first_byte`** (buffered SSE; higher TTFT; excludes MCP streaming follow-up and Anthropic adapter streaming).
6. Import the starter dashboard [`grafana/panda_agent_fleet.json`](./grafana/panda_agent_fleet.json) and tune alerts using [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md).
7. Explore the **unified control plane** capabilities by managing all gateway functions through a single `panda.yaml` configuration.

For **prioritized backlog** across fleet SRE joins, semantic-cache depth, tool-cache audit, resilience, and Wasm, see [`agent_fleet_gap_roadmap.md`](./agent_fleet_gap_roadmap.md).

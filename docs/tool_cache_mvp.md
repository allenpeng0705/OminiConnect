# Tool Cache MVP for Agent Cost Reduction

Goal: reduce agent token usage by caching safe MCP tool results, while preserving security and correctness.

Current implementation status: MVP **memory backend** is implemented in `panda-proxy`; Redis backend remains follow-on.

This is an MVP spec for OpenClaw-class agent loops and similar tool-heavy agents.

---

## Scope

Cache only **deterministic, read-only** tool calls, for example:

- local search/index lookups,
- read-only DB/file metadata queries,
- static knowledge fetches.

Do **not** cache:

- write/mutation tools,
- tools returning secrets or volatile privileged data,
- tools with side effects,
- tools requiring per-request freshness stronger than TTL.

---

## Safety model (must-have)

- **Allowlist only**: cache disabled by default; enable per `server.tool`.
- **Identity-scoped**: include tenant/subject/session scope in key.
- **Argument canonicalization**: normalize JSON args before hashing.
- **TTL required**: no infinite cache entries.
- **Max value bytes**: avoid oversized payload abuse.
- **Audit fields**: mark `hit/miss/store/bypass` in Prometheus; when `observability.compliance_export` is enabled, append **`panda.compliance.tool_cache.v1`** JSONL rows for **hit**, **store**, and **bypass**; optional **`miss`** via `mcp.tool_cache.compliance_log_misses: true` (high volume). See [`compliance_export.md`](./compliance_export.md).
- **Bypass switch**: request header for controlled cache bypass (ops/admin only).

### Operator matrix: metrics vs compliance JSONL

| Path | Prometheus | `panda.compliance.tool_cache.v1` (`decision` / `bypass_reason`) | When JSONL is written |
|------|------------|-------------------------------------------------------------------|------------------------|
| Tool not on allowlist | `panda_mcp_tool_cache_bypass_total{reason="not_allowlisted"}` | `decision=bypass`, `bypass_reason=not_allowlisted` | `observability.compliance_export.enabled=true` |
| Cache hit | `panda_mcp_tool_cache_hit_total` | `decision=hit` | compliance on |
| Cache miss | `panda_mcp_tool_cache_miss_total` | `decision=miss` **only if** `mcp.tool_cache.compliance_log_misses: true` | compliance on (miss is high volume — default off) |
| Stored after tool call | `panda_mcp_tool_cache_store_total` | `decision=store` | compliance on |
| Result not cacheable (error, oversized, etc.) | `panda_mcp_tool_cache_bypass_total{reason="not_cacheable"}` | `decision=bypass`, `bypass_reason=not_cacheable` | compliance on |

The same counters and compliance rows apply whether the tool runs in the **chat MCP follow-up loop** or via **ingress MCP HTTP** (`POST` JSON-RPC `tools/call` on API gateway routes with `backend: mcp`). Scope for the cache key uses [`RequestContext`](../crates/panda-proxy/src/shared/gateway.rs) built per request (subject / tenant / agent session from trusted gateway headers, JWT when `identity.require_jwt`, and agent session config).

Implementation reference:

- [`crates/panda-proxy/src/lib.rs`](../crates/panda-proxy/src/lib.rs) — chat follow-up tool cache branch; [`mcp_http_ingress_execute_tools_call`](../crates/panda-proxy/src/lib.rs) for ingress `tools/call`.
- [`crates/panda-proxy/src/inbound/mcp_http_ingress.rs`](../crates/panda-proxy/src/inbound/mcp_http_ingress.rs) — ingress JSON-RPC handler (delegates `tools/call` to the shared execution path above).
- [`crates/panda-proxy/src/shared/compliance_export.rs`](../crates/panda-proxy/src/shared/compliance_export.rs) (`record_tool_cache`, schema `panda.compliance.tool_cache.v1`).

**If compliance export is disabled:** counters above still increment; **no** `panda.compliance.tool_cache.v1` lines are appended.

---

## Proposed config (MVP)

```yaml
mcp:
  tool_cache:
    enabled: true
    backend: memory
    default_ttl_seconds: 120
    max_value_bytes: 65536
    allow:
      - server: rag_lite
        tool: rag_search
        ttl_seconds: 300
      - server: docs
        tool: read_page
        ttl_seconds: 120
```

---

## Cache key format (MVP)

Use stable hash over:

- `server_name`
- `tool_name`
- canonicalized `arguments_json`
- `subject_or_tenant_scope` (and optionally `agent_session`)
- optional `policy_version`

Example key prefix:

`panda:mcp:toolcache:v1:{scope}:{server}:{tool}:{sha256(args+policy)}`

---

## Execution flow

1. Tool call arrives from the **model MCP follow-up loop** (chat completions) **or** from **ingress MCP HTTP** (`tools/call` on a route with `backend: mcp`).
2. Check tool allowlist and scope policy.
3. If cacheable:
   - compute key,
   - try cache read.
4. On hit:
   - return cached tool result,
   - emit `tool_cache_hit` metric/event.
5. On miss:
   - execute real tool call,
   - enforce size/type guardrails,
   - store with TTL,
   - emit `tool_cache_store`.
6. On bypass/unsafe:
   - execute tool directly,
   - emit `tool_cache_bypass` with reason.

---

## Metrics and status (MVP)

Add counters:

- `panda_mcp_tool_cache_hit_total{server,tool}`
- `panda_mcp_tool_cache_miss_total{server,tool}`
- `panda_mcp_tool_cache_store_total{server,tool}`
- `panda_mcp_tool_cache_bypass_total{server,tool,reason}`

Expose summary fields in `/mcp/status`:

- `tool_cache.enabled`
- `tool_cache.backend`
- `tool_cache.allow_count`
- `tool_cache.default_ttl_seconds`

---

## Rollout plan

1. Enable for one low-risk tool (`rag_search`-like).
2. Validate:
   - hit ratio,
   - token savings,
   - no stale/safety incidents.
3. Expand allowlist gradually.
4. Add per-tool freshness tests for critical tools.

---

## Success criteria

- >=20% token reduction on target agent workflows.
- No policy bypass incidents.
- No cache-related correctness regressions in canary.


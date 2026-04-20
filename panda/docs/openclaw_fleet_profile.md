# OpenClaw Fleet Profile with Panda Unified Gateway (Discussion Baseline)

This is the canonical persona for planning agent-first Panda work with the unified gateway architecture.

---

## Persona: "100-agent cloud team"

- 100 OpenClaw-class agent runtimes in Kubernetes.
- 10 tenants (teams), each with 5-20 active agents.
- Mixed workloads:
  - 50% retrieval/research/tool-heavy,
  - 30% structured automation tasks,
  - 20% complex planning tasks.
- Provider mix: one primary model provider + one failover provider.
- Shared Redis for budgets and cache.

---

## Main pain points

1. Token spend spikes from repeated tool and prompt context.
2. Unsafe tool exposure (too many tools advertised to every prompt).
3. Hard-to-debug agent loops and retries.
4. No single fleet view for budget, policy, and tool behavior.
5. Complex architecture with multiple gateways for different traffic types.
6. Inconsistent security policies across AI, MCP, and API traffic.

---

## What Panda must provide first (ordered)

## 1) Unified control plane + intent-scoped tools + loop bounds + great status/metrics

Minimum deliverables:

- Unified configuration for all gateway functions (AI, MCP, API).
- Intent-scoped tool advertising on by policy.
- Per-session/profile `max_tool_rounds` enforcement.
- Agent-focused status:
  - `/mcp/status`: effective rounds, intent policy summary.
  - `/tpm/status`: per-session/profile budget state.
  - `/ops/fleet/status`: unified view across all gateway functions.
- Metrics:
  - tool rounds used,
  - policy deny counts,
  - budget rejection counts,
  - API gateway routing and auth stats.

Success criteria:

- 30%+ reduction in average tools advertised per request.
- 0 runaway loops beyond configured round caps.
- Single configuration file for all gateway functions.
- Unified metrics across all traffic types.

## 2) MCP Tool-result cache MVP (allowlisted, TTL, identity-scoped)

Minimum deliverables:

- Per-tool allowlist within the MCP gateway.
- TTL per tool with global defaults.
- Identity/session scoped keying.
- Bypass and audit events.
- Integration with the unified metrics system.

Success criteria:

- >=20% cache hit rate on designated deterministic tools.
- No cross-tenant cache leakage incidents.
- Cache metrics included in the unified fleet view.

## 3) AI Gateway Semantic cache defaults tuned for agents (safe-by-default)

Minimum deliverables:

- Conservative default threshold/profile for agent routes within the AI gateway.
- Include model/tool/system fingerprint in cache key policy.
- Safe fallback behavior and visibility on bypass.
- Integration with the unified metrics system.

Success criteria:

- >=15% token reduction on selected agent flows.
- No correctness regressions in canary flows.
- Cache metrics included in the unified fleet view.

## 4) API Gateway for traditional services

Minimum deliverables:

- Ingress routing for traditional REST services.
- Egress governance with allowlists and rate limits.
- Integration with the unified identity and policy system.
- Unified metrics for API gateway traffic.

Success criteria:

- Agents can access traditional services through the same gateway.
- Consistent security policies across all traffic types.
- API gateway metrics included in the unified fleet view.

## 5) Richer per-agent fleet views (JSON first, Grafana second)

**Phased gap plan:** [`agent_fleet_gap_roadmap.md`](./agent_fleet_gap_roadmap.md) (what is still missing, priorities, Panda vs external ownership).

Operational runbook: [`runbooks/agent_fleet_oncall.md`](./runbooks/agent_fleet_oncall.md).

Minimum deliverables:

- JSON endpoints remain primary source.
- Grafana starter dashboard over exported metrics:
  - TPM rejects and MCP/tool-cache/semantic-cache aggregates (low-cardinality labels),
  - tool rounds and cache hit/miss,
  - API gateway routing and auth stats,
  - deny/reject and retry/failover trends (plus `panda_model_failover_midstream_retry_total` when buffered failover is enabled).

Success criteria:

- On-call can identify the **top token-burning principal** in &lt;5 minutes using the **documented join** (low-cardinality metrics + logs/compliance + `/tpm/status` with that principal’s headers) — not by adding high-cardinality tenant labels to every Prometheus series.
- Weekly optimization loop runs from dashboard + status data.

---

## Suggested 6-week execution sequence

Use [`agent_fleet_gap_roadmap.md`](./agent_fleet_gap_roadmap.md) for the **cross-cutting** backlog (fleet joins, audit hardening, semantic-cache depth, resilience, Wasm). The **original** sequencing below remains a reasonable default **within** the OpenClaw fleet profile:

- Week 1-2: Item 1 hardening and agent metrics completeness (unified control plane).
- Week 2-3: Item 2 implementation (MCP tool cache) + Item 4 API gateway basics.
- Week 3-4: Item 3 AI gateway semantic cache safe defaults + canary.
- Week 5-6: Item 5 dashboard pack + runbook (includes all gateway functions).


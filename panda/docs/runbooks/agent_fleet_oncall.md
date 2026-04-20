# Runbook: OpenClaw-class agent fleet (Panda Unified Gateway)

Use this when operating Panda as the **unified gateway** (API + MCP + AI) in front of many agent runtimes (OpenClaw-style loops, MCP tools, long sessions, traditional services).

---

## 1. Where to look first

| Signal | Endpoint / source | Notes |
|--------|-------------------|--------|
| One-shot snapshot | `GET /ops/fleet/status` | Same optional ops auth as `/metrics`. Process, TPM rejects, MCP + semantic-cache + API gateway counters since **process start** (not cluster-wide). |
| Per-caller TPM | `GET /tpm/status` | Must send the **same JWT / trusted-gateway / agent-session headers** as the client to see that bucket on **this replica**. |
| MCP + tools | `GET /mcp/status` | Effective `max_tool_rounds`, intent policies, tool-cache summary, semantic-cache hints. |
| API Gateway status | `GET /ops/control/v1/api_gateway/ingress/routes` | Dynamic ingress routes (when control plane enabled). |
| Time series | `GET /metrics` | Requires scrape auth when `observability.admin_secret_env` is set. Includes metrics for all gateway functions. |

---

## 2. Prometheus queries (starter)

Assume metric names as exported today; adjust `job`, `instance`, or recording rules to match your Prometheus setup. For copy-paste **recording rules** (and commented alert examples), see [`../grafana/recording_rules.agent_fleet.yaml`](../grafana/recording_rules.agent_fleet.yaml).

**MCP tool rounds cap pressure (aggregate):**

```promql
sum(increase(panda_mcp_agent_max_rounds_exceeded_total[1h]))
```

**Intent / proof-of-intent friction:**

```promql
sum(increase(panda_mcp_agent_intent_tools_filtered_total[1h]))
```

```promql
sum(increase(panda_mcp_agent_intent_call_enforce_denied_total[1h]))
```

**Tool-result cache (bounded by allowlist):**

```promql
topk(10, sum by (server, tool) (rate(panda_mcp_tool_cache_hit_total[5m])))
```

```promql
topk(10, sum by (server, tool) (rate(panda_mcp_tool_cache_miss_total[5m])))
```

**Per-server/tool bypass reasons:**

```promql
topk(10, sum by (server, tool, reason) (rate(panda_mcp_tool_cache_bypass_total[5m])))
```

**Semantic cache (AI gateway):**

```promql
sum(increase(panda_semantic_cache_hit_total[1h]))
```

```promql
sum(increase(panda_semantic_cache_miss_total[1h]))
```

**API Gateway metrics:**

```promql
sum by (path_prefix) (rate(panda_api_gateway_ingress_requests_total[5m]))
```

```promql
sum by (host) (rate(panda_api_gateway_egress_requests_total[5m]))
```

```promql
sum by (reason) (rate(panda_api_gateway_auth_failures_total[5m]))
```

```promql
sum(rate(panda_api_gateway_rate_limit_exceeded_total[5m]))
```

**TPM throttling:**

```promql
sum by (bucket_class) (rate(panda_tpm_budget_rejected_total[5m]))
```

---

## 3. Cardinality and “top tenant” reality

Panda intentionally keeps **low-cardinality** labels on most counters (`bucket_class`, `server`, `tool`, semantic-routing `event`/`target`). It does **not** attach arbitrary **tenant id** or **session id** to every Prometheus series — that would explode cardinality at scale.

**To find a hot tenant or agent:**

1. Use **`/tpm/status`** (or your access logs) with that principal’s headers.
2. Use **compliance JSONL** (`observability.compliance_export`) or log pipelines keyed by `request_id` / correlation id.
3. Optionally aggregate in **logging/metrics** outside Panda with sampling or top-N rules.

---

## 3.5 Under-5-minute playbook: spike to principal

Use this when you need to **explain a token or policy spike** without high-cardinality Prometheus labels. Recording rules that wrap the queries below live in [`../grafana/recording_rules.agent_fleet.yaml`](../grafana/recording_rules.agent_fleet.yaml).

1. **Spike detection (Prometheus)** — Open your dashboard or run ad hoc queries:
   - TPM: `sum by (bucket_class) (rate(panda_tpm_budget_rejected_total[5m]))` or use recording rule `panda:tpm_rejects:rate5m`.
   - MCP rounds cap: `sum(increase(panda_mcp_agent_max_rounds_exceeded_total[1h]))`.
   - Tool-cache friction: `sum(rate(panda_mcp_tool_cache_bypass_total[5m]))` by `server`, `tool`, `reason` if needed (bounded by allowlist).
   - Semantic cache: `sum(increase(panda_semantic_cache_hit_total[1h]))` / `miss` / `store` as context.
   - API Gateway auth failures: `sum by (reason) (rate(panda_api_gateway_auth_failures_total[5m]))`.
   - API Gateway rate limits: `sum(rate(panda_api_gateway_rate_limit_exceeded_total[5m]))`.
   - If **`model_failover.allow_failover_after_first_byte`** is on: `increase(panda_model_failover_midstream_retry_total[1h])` for buffered SSE replay pressure.
2. **Narrow the replica** — From Prometheus, note **`instance`** / pod / node. Counters are **per Panda process**; aggregate across replicas with `sum` in PromQL, then drill into one hot instance.
3. **Identify the principal (not from raw Prom labels)** — Panda does **not** label every series with tenant or session id. Use one or more of:
   - **Compliance JSONL** (`observability.compliance_export`): `panda.compliance.ingress.v1` / `egress.v1` lines include **`request_id`** (correlation id); join or grep around the spike time window. See [`../compliance_export.md`](../compliance_export.md) for schema fields.
   - **Access / ingress logs** in front of Panda: correlate timestamp + path + status with the same **`request_id`** / trace id if your edge adds it.
4. **Confirm TPM for that principal** — Call **`GET /tpm/status`** on the same replica (or via your LB with sticky note) using the **same** JWT, trusted-gateway identity headers, and **`agent_sessions`** header (if used) as the client you suspect. Compare to `budget_tokens_per_minute` / window snapshot.
5. **Check API Gateway routes** — If the spike is from API gateway traffic, check dynamic routes with **`GET /ops/control/v1/api_gateway/ingress/routes`** (requires control plane enabled).
6. **What this is not** — You cannot rank "top tenant by token burn" from Prometheus alone without an **external** join (logs, compliance, billing, or a sampled pipeline). The playbook above is the supported path within Panda's cardinality model.

---

## 4. Grafana

Import [`../grafana/panda_agent_fleet.json`](../grafana/panda_agent_fleet.json) and point panels at your Prometheus datasource. The dashboard includes:

- **Unified Gateway Overview**: Request rates across all gateways, ops auth denial ratio, TPM rejects
- **AI Gateway**: Semantic cache hit rate and operations
- **MCP Gateway**: Tool cache hit rate, operations, and agent governance metrics
- **API Gateway**: Ingress/egress requests, auth failures, and rate limits

Optional: load recording rules from [`../grafana/recording_rules.agent_fleet.yaml`](../grafana/recording_rules.agent_fleet.yaml) into Prometheus for shorter panel queries and starter alerts (tune thresholds per environment).

See [`../grafana/README.md`](../grafana/README.md) for installation and customization instructions.

---

## 5. Related docs

- [`openclaw_agent_quickstart.md`](../openclaw_agent_quickstart.md) — baseline profile.
- [`openclaw_fleet_profile.md`](../openclaw_fleet_profile.md) — persona and success criteria.
- [`agent_fleet_gap_roadmap.md`](../agent_fleet_gap_roadmap.md) — phased backlog (tracks A–F).
- [`tool_cache_mvp.md`](../tool_cache_mvp.md) — tool-result cache configuration.
- [`compliance_export.md`](../compliance_export.md) — audit JSONL schemas.
- [`mcp_failover_streaming.md`](../mcp_failover_streaming.md) — MCP vs buffered mid-stream failover.
- [`wasm_agent_policy_rfc.md`](../wasm_agent_policy_rfc.md) — Wasm agent-policy RFC.
- [`production_slo.md`](./production_slo.md) — broader SLO notes.

# Ingress gateway — SLO methodology (F2)

**Purpose:** Repeatable way to measure **Panda API gateway ingress** overhead and health—not only generic chat/SSE (see [`production_slo.md`](./production_slo.md)).

**Scope:** Requests that hit **`api_gateway.ingress`** classification (`dispatch` → ingress router) before `forward_to_upstream`.

**F2 status:** This runbook + metrics in `panda-proxy` + optional Grafana import ([`../grafana/ingress_gateway_slo.json`](../grafana/ingress_gateway_slo.json)) are the **in-repo** SLO package. **Numeric targets** (p99 ms, error budget) remain **environment-specific**—record them per release in [§6 Evidence record](#6-slo-evidence-record-audits--ga).

---

## 1. What to measure

| Signal | Question |
|--------|----------|
| **429 from ingress RPS** | Are clients hitting `api_gateway.ingress.routes[].rate_limit` (or Redis-backed shared counters)? |
| **Latency add-on** | Time from TCP accept → first byte of response for paths classified at ingress (`ai`, `mcp`, `ops`, …). |
| **Error mix** | 404 `ingress: no matching route` vs 405 method vs 403 deny vs upstream errors **after** ingress allows. |
| **Per-route RL churn** | Which `tenant_id` / `path_prefix` rows see `allowed` vs `denied` (bounded Prometheus labels). |

---

## 2. Metrics (Prometheus)

Emitted on Panda’s **`/metrics`** (same scrape as other gateway counters).

| Metric | Meaning |
|--------|---------|
| **`panda_gateway_rps_allowed_total{layer="ingress"}`** | Ingress **rate limit** check passed (only when a limit applies to the matched row). |
| **`panda_gateway_rps_denied_total{layer="ingress"}`** | Rejected with **429** at ingress RL. |
| **`panda_gateway_rps_allowed_total{layer="legacy"}`** / **`denied`** | Same for **top-level** `routes[].rate_limit` in `forward_to_upstream`. |
| **`panda_gateway_ingress_rps_total{tenant_id,path_prefix,result}`** | Per matched ingress row: `result` is **`allowed`** or **`denied`**. Empty tenant is exported as **`tenant_id="-"`**. **Cardinality is bounded** by configured routes—not arbitrary paths. |

**Implementation:** `crates/panda-proxy/src/lib.rs` (`render_prometheus` / ingress dispatch).

**Alerts (examples):**

- Spike in **`denied`** without a traffic campaign → tune `rps` or investigate abuse.
- **`allowed` ≈ 0** and traffic expected → misconfigured routes or RL set to 0.

---

## 3. PromQL (copy/paste)

Use in Grafana Explore or recording rules (adjust scrape interval / `[5m]` window).

```promql
# 429 rate at ingress RL (cluster-wide)
sum(rate(panda_gateway_rps_denied_total{layer="ingress"}[5m]))

# Denied fraction of ingress RL checks (when counter > 0)
sum(rate(panda_gateway_rps_denied_total{layer="ingress"}[5m]))
/
(sum(rate(panda_gateway_rps_allowed_total{layer="ingress"}[5m])) + sum(rate(panda_gateway_rps_denied_total{layer="ingress"}[5m])))

# Top ingress rows by denied rate (table panel)
topk(10, sum by (tenant_id, path_prefix) (rate(panda_gateway_ingress_rps_total{result="denied"}[5m])))
```

---

## 4. Grafana

- **Import:** [`docs/grafana/ingress_gateway_slo.json`](../grafana/ingress_gateway_slo.json) — set the Prometheus datasource variable to your TSDB.
- **Agent fleet / semantic cache:** [`docs/grafana/panda_agent_fleet.json`](../grafana/panda_agent_fleet.json) (different scope).

---

## 5. Load / soak (repeatable)

1. **Baseline:** Single replica, `ingress.enabled: true`, one row with `rate_limit.rps` set; drive **synthetic** HTTP against matching prefix (GET/POST as allowed by `methods`).
2. **Compare:** Same workload with **`ingress.enabled: false`** (or route removed) to isolate ingress classification + RL cost vs plain `forward_to_upstream`.
3. **Redis:** When **`api_gateway.ingress.rate_limit_redis`** is set, repeat (2) to verify **shared** counters across replicas (429s appear when **global** budget exceeded).

Use existing scripts where applicable:

- [`scripts/staging_readiness_gate.sh`](../../scripts/staging_readiness_gate.sh)
- Chat-focused: [`scripts/load_profile_chat.sh`](../../scripts/load_profile_chat.sh) — supplement with **non-chat** paths if your ingress routes cover them.

---

## 6. SLO evidence record (audits / GA)

Attach the following **per named release** (internal ticket, change record, or release notes). This is the **F2 “evidence”** artifact; numbers stay outside the repo if policy requires.

| Field | Example |
|-------|---------|
| **Environment** | `staging` / `prod` / customer |
| **Panda version / image digest** | git tag + OCI digest |
| **Ingress config** | `ingress.enabled`, count of static + dynamic routes, Redis RL on/off |
| **Load profile** | RPS, duration, path mix (AI vs MCP vs ops) |
| **SLO targets** | e.g. “p99 add-on ≤ X ms”, “429 rate < Y% of checks” |
| **Measured** | p50/p95/p99 add-on ms; 429 rate; link to **Grafana** dashboard + time range |
| **Metrics sanity** | Paste or link to `/metrics` excerpt showing non-zero `panda_gateway_*` when RL applies |
| **Trace sample** (optional) | `OTEL` span filter for ingress path + `http.route` |

Update [`gateway_backlog_progress.md`](../gateway_backlog_progress.md) if your process tracks F2 evidence there.

---

## 7. Traces (optional)

With **`OTEL_EXPORTER_OTLP_ENDPOINT`**, spans should include route and status; filter **`stage=ingress`** or path prefix in your backend to compare latency percentiles **with vs without** ingress enabled.

---

## Related

- [`production_slo.md`](./production_slo.md) — broader readiness and Phase 5 checks.
- [`security_review_gate.md`](../security_review_gate.md) — F3 formal review gate (complementary).

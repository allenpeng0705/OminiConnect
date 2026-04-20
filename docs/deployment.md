# Deployment

This guide covers how to run Panda in production: the binary, configuration, optional integrations, and how it fits **with or without** an API gateway at the edge.

For **organizational** migration stories (coexistence vs AI-first edge vs consolidation), see [evolution phases](./evolution_phases.md). For **Kong-specific** header contracts, see [Kong handshake](./kong_handshake.md).

---

## What you run

- **Binary:** `panda` (from `panda-server`), configured via **`panda.yaml`** (start from `panda.example.yaml`).
- **Defaults:** Single static binary; GitOps-friendly YAML; no mandatory external database for routing.

---

## Mandatory vs optional third-party tools

**None** of Redis, Prometheus, an OTLP collector, Kong, Grafana, or Datadog is **required** for Panda to run.

| Category | Required? | Notes |
|----------|-----------|--------|
| **Core gateway** | **Yes** | `panda` binary + **`panda.yaml`** + a reachable **`upstream`** (the model API or HTTP backend you proxy to). |
| **TLS** | **Only if** you terminate TLS **on Panda** | Otherwise TLS can live on a load balancer or another hop; see [TLS](#tls). |
| **Redis** | **No** | Optional: shared **TPM** across replicas and/or **semantic cache** on Redis. Without it, TPM is per-process and semantic cache defaults to **memory** (single-replica friendly). |
| **Prometheus** | **No** | Panda only **exposes** `/metrics`; run Prometheus only if you want that scraping pipeline. |
| **OTLP / tracing** | **No** | Only if you set **`OTEL_EXPORTER_OTLP_ENDPOINT`** and want exported traces. |
| **Kong** (or similar) | **No** | Only if you want a **separate** API gateway in front of Panda. |
| **Grafana / Datadog** | **No** | Optional **consumers** of metrics/traces; Panda does not call them. |

Everything in the second group is an **operational or scaling choice**, not an install prerequisite.

---

## Integrations: Redis, Prometheus, OTLP, Kong, Grafana, Datadog

Panda does **not** embed Redis, Prometheus, Grafana, or Datadog. Run them (or a compatible SaaS) **beside** the gateway.

| System | Role |
|--------|------|
| **Redis** | Shared **TPM** counters across replicas; optional **semantic cache** backend. |
| **Prometheus** | Scrapes **`GET /metrics`** (Prometheus text exposition). |
| **OTLP** | **Traces** via HTTP to any OTLP-compatible collector or APM. |
| **Kong** (or similar) | Optional **edge** in front of Panda; **`trusted_gateway`** for identity. |
| **Grafana** / **Datadog** | Dashboards and backends for metrics/traces — **consumers**, not in-process plugins. |

Per-route **HTTP RPS** in `panda.yaml` is **process-local** today; cluster-wide RPS would need a shared store (e.g. Redis) as a future extension.

### Redis

**Why:** Without Redis, TPM totals are **per process**. With Redis, replicas share rolling-minute style budgets. Semantic cache can stay **in-memory** or use **Redis** when `semantic_cache.backend: redis`.

**URL precedence**

- **TPM:** `tpm.redis_url` in `panda.yaml`, then env **`PANDA_REDIS_URL`**.
- **Semantic cache:** `semantic_cache.redis_url` in YAML, then **`PANDA_SEMANTIC_CACHE_REDIS_URL`**, then **`PANDA_REDIS_URL`**.

**Steps**

1. Run **Redis** (or a **Redis-compatible** server such as **Dragonfly** where allowed).
2. Set URLs via YAML and/or env as above.
3. For semantic cache on Redis: `semantic_cache.enabled: true`, `semantic_cache.backend: redis`, plus a non-empty Redis URL (YAML or env).
4. Optional: **`PANDA_SEMANTIC_CACHE_TIMEOUT_MS`** — lookup timeout budget (see repository README).

**Example (env for TPM):**

```bash
export PANDA_REDIS_URL='redis://127.0.0.1:6379'
```

---

### Prometheus

**Why:** Panda exposes metrics in **Prometheus text format** on **`GET /metrics`**. You run the **Prometheus** server (or another scraper) separately.

**Steps**

1. Scrape `http://<panda-host>:<listen-port>/metrics`.
2. **Ops auth:** If **`observability.admin_secret_env`** is set in `panda.yaml`, requests must include the header named by **`observability.admin_auth_header`** (default `x-panda-admin-secret`) whose **full value** equals the secret read from that env var (not a `Bearer` token—raw header match). Example:

   ```bash
   curl -fsS -H "x-panda-admin-secret: $PANDA_OPS_SECRET" "http://127.0.0.1:8080/metrics"
   ```

3. Configure your Prometheus (or **Grafana Agent**, **vmagent**, etc.) to send that header on scrapes. Core Prometheus only gained flexible custom HTTP headers in **newer** versions; if yours cannot, use an agent that can, or a small **reverse proxy** in front of `/metrics` that injects the header.
4. If **`admin_secret_env` is unset**, `/metrics` may be **unauthenticated**—avoid exposing it publicly.

Metric names are prefixed (e.g. `panda_ops_auth_*`, `panda_tpm_*`, `panda_mcp_stream_probe_*`).
Semantic-cache observability now includes `panda_semantic_cache_hit_total`, `panda_semantic_cache_miss_total`, and `panda_semantic_cache_store_total`.
If `agent_sessions.enabled: true`, semantic-cache keys are bucket-scoped by default (same TPM identity bucket) even when `semantic_cache.scope_keys_with_tpm_bucket` is false.
Optional strict audit: `mcp.tool_cache.compliance_log_misses: true` adds high-volume **`miss`** lines to `panda.compliance.tool_cache.v1` when compliance export is enabled (see `docs/compliance_export.md`).

**JSON ops (same admin header when configured):** `GET /tpm/status`, `GET /mcp/status`, and **`GET /ops/fleet/status`** (single snapshot of process, TPM, agent-session flags, semantic-cache config, MCP tool-cache counters, and governance counters since process start). A starter Grafana dashboard lives at [`docs/grafana/panda_agent_fleet.json`](./grafana/panda_agent_fleet.json).

---

### OpenTelemetry (OTLP)

**Why:** **Distributed traces** from `panda-server` when an OTLP **HTTP** endpoint is set.

**Steps**

1. Set **`OTEL_EXPORTER_OTLP_ENDPOINT`** to your collector’s OTLP **HTTP** traces URL, for example:

   `http://127.0.0.1:4318/v1/traces`

2. Optional:
   - **`PANDA_OTEL_SERVICE_NAME`** — sets `service.name` (default `panda-gateway`).
   - **`PANDA_OTEL_TRACE_SAMPLING_RATIO`** — `0.0`–`1.0` (default `1.0`).

3. If OTLP initialization **fails**, Panda logs an error and continues with **JSON logs only** (no trace export).

4. Point your **collector** at Jaeger, **Grafana Tempo**, a cloud APM, etc. Implementation detail: `panda-server` uses the OpenTelemetry Rust SDK with **HTTP** export (`crates/panda-server/src/main.rs`).

---

### Kong (or another API gateway)

**Why:** Keep Kong as the **public edge** (TLS, coarse limits, legacy APIs) and forward **AI** routes to Panda.

**Steps**

1. In Kong, add Panda’s listen address as an **upstream** / **service** target.
2. Route the paths you want (e.g. chat completions) to that upstream.
3. In **`panda.yaml`**, configure **`trusted_gateway`** (attestation header + identity header names) so Panda can **trust** edge-injected identity—see **[Kong handshake](./kong_handshake.md)**.
4. Set **`PANDA_TRUSTED_GATEWAY_SECRET`** in Panda’s environment to match the attestation value Kong sends.

Kong runs **outside** Panda; Panda is only an HTTP upstream target.

---

### Grafana

**Why:** Dashboards and alerts on the metrics and traces you already store.

**Typical wiring**

1. **Metrics:** Run **Prometheus** scraping Panda’s `/metrics` (see [Prometheus](#prometheus)). In Grafana, add a **Prometheus** data source pointing at that Prometheus server; build panels from `panda_*` metrics.
2. **Traces:** Send **OTLP** from Panda to an **OpenTelemetry Collector**, then to **Grafana Tempo** (or compatible backend). Add a **Tempo** data source in Grafana and link traces to logs/metrics where supported.
3. Grafana only **queries** your stores; it does not receive a special protocol from Panda beyond normal HTTP client requests (e.g. Prometheus pulling from Panda).

---

### Datadog

**Why:** Same observability story using Datadog’s stack.

**Typical wiring**

1. **Metrics:** Run the **Datadog Agent** with a **Prometheus** or **OpenMetrics** integration scraping `http://<panda>:<port>/metrics`. If ops auth is enabled, configure the check to send the **`observability.admin_auth_header`** value (same rules as [Prometheus](#prometheus)).
2. **Traces:** Route **OTLP** from Panda through an **OpenTelemetry Collector** with a **Datadog exporter**, or use Datadog’s **OTLP intake** if available for your account—follow current Datadog OpenTelemetry documentation.

Datadog components are **not** embedded in Panda; all configuration is on the agent/collector side.

---

## Standalone (no Kong)

Small teams and single-region deployments can run Panda **without** Kong or another gateway.

### When to use standalone

- You want **one** service in front of your LLM upstream (OpenAI-compatible or adapted).
- TLS terminates at a **cloud load balancer** or **on Panda** (see [TLS](#tls)).
- You do **not** need a large external route/plugin surface.

### Minimal checklist

1. **Config**
   - Copy `panda.example.yaml` → `panda.yaml`.
   - Set `listen` and `upstream` (or use a `server:` block for `listen` / `address`+`port` / `tls`).
2. **Health**
   - `GET /health` — process up.
   - `GET /ready` — config/runtime checks (MCP, enrichment file, etc.).
3. **Identity (optional)**
   - `identity.require_jwt: true` with HS256 secret in env (`identity.jwt_hs256_secret_env`).
   - Leave `trusted_gateway` **unset** or empty if there is **no** Kong-style edge.
4. **TPM / cost guard (optional)**
   - `tpm.enforce_budget` + `tpm.budget_tokens_per_minute` (and optional Redis for multi-replica totals).
5. **Observability**
   - Structured logs: `RUST_LOG=info`.
   - Optional OTLP: `OTEL_EXPORTER_OTLP_ENDPOINT`, `PANDA_OTEL_SERVICE_NAME`, `PANDA_OTEL_TRACE_SAMPLING_RATIO`.

### `trusted_gateway` (standalone)

- **Omit** `trusted_gateway` or leave headers unset when you have **no** trusted edge.
- Without attestation, Panda **strips** configured identity headers if present (anti-spoofing).

---

## With Kong or another edge

- Register Panda as an **upstream** from the edge gateway for AI routes only, or use a **domain split** (`ai.*` → Panda, `api.*` → Kong) — see [integration and evolution](./integration_and_evolution.md).
- Use **`trusted_gateway`** + shared secret when the edge attests identity — [Kong handshake](./kong_handshake.md).

---

## TLS

**Option A — TLS at the load balancer** (common on AWS/GCP/Azure)

- Terminate TLS on the LB; Panda listens HTTP on a private port or security group.

**Option B — TLS on Panda**

- Use `tls:` in `panda.yaml` (or `server.tls`) with `cert_pem` and `key_pem` paths — see `panda.example.yaml`.

---

## Security defaults (recommended)

- **Do not** expose the Developer Console publicly: keep `PANDA_DEV_CONSOLE_ENABLED` off in production unless debugging.
- **Protect ops endpoints** when exposing `/metrics` or the Developer Console:
  - Set `observability.admin_secret_env` and pass the secret via the header name in `observability.admin_auth_header`.
  - Staging script tip: `READINESS_AUTH_HEADER='x-panda-admin-secret: <secret>' ./scripts/staging_readiness_gate.sh`
- **Secrets only from env** (JWT secret, upstream keys, Redis, ops secret)—not committed to git.

---

## Kubernetes

Starter manifests live under **`k8s/`** (`deployment.yaml`, `configmap.yaml`, `service.yaml`, `pdb.yaml`, `hpa.yaml`, `secret.example.yaml`) with probes wired to `/health` and `/ready`. See the repository **README** for `kubectl` examples.

---

## See also

- [Developer Console](./developer_console.md) — optional debug UI.
- [Production SLO runbook](./runbooks/production_slo.md) — readiness fields, scripts, rollout notes.
- [Standalone deployment](./standalone_deployment.md) — short link to this document (legacy filename).
- [High-level design](./high_level_design.md) — architecture overview.
- [Implementation plan](./implementation_plan.md) — engineering roadmap.

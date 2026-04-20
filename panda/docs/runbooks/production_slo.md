# Production SLOs and verification (Phase 5)

Panda does not ship a single numeric SLO contract in YAML; operators define targets per environment. This runbook ties **implementation_plan.md** Phase 5 review items to **repeatable checks** already in the repo.

## Baseline targets (tune per hardware)

| Signal | Direction | Notes |
|--------|-----------|--------|
| Concurrent SSE streams / instance | Capacity | Document baseline machine; watch RSS and CPU at tail latency. |
| Gateway TTFT add-on | Lower is better | Lab target in `implementation_plan.md` (~2 ms) is illustrative—measure on your stack. |
| Error rate under load | Lower | Separate upstream 5xx from Panda 429/503 policy responses. |

## Scripts and endpoints

| Check | How |
|-------|-----|
| Core tests + readiness | `scripts/staging_readiness_gate.sh` |
| Chat load snapshot | `scripts/load_profile_chat.sh` |
| SSE soak / drift | `scripts/soak_guard_sse.sh` |
| Process health | `GET /health` (liveness) |
| Config + MCP + enrichment + drain | `GET /ready` — includes `shutdown_drain_seconds`, `active_connections`, `model_failover.*` |
| TPM / agent / hierarchy hints | `GET /tpm/status` (ops auth when configured) |
| MCP + intent policy hints | `GET /mcp/status` |
| Wasm plugins + **wasm_runtime_by_reason** | `GET /plugins/status` |

## Rollout hygiene

- Set **`PANDA_SHUTDOWN_DRAIN_SECONDS`** for Kubernetes rollouts; `/ready` reports **`draining: true`** after SIGTERM so the Service stops sending new traffic while in-flight streams unwind.
- For **model failover**, use **`/ready` → `model_failover.streaming_failover`**: pre-response failover on 5xx/429 is on for matched routes; when **`allow_failover_after_first_byte`** is true, OpenAI-shaped streaming chat SSE may be **fully buffered** (see `midstream_sse_max_buffer_bytes`) so a **later** backend can run if the winner fails mid-body—watch **`panda_model_failover_midstream_retry_total`** on `/metrics`.
- With **`OTEL_EXPORTER_OTLP_ENDPOINT`**, request completion spans include **`http.response.status_code`**, **`http.request.duration_ms`**, and **`http.route`** for latency / error SLOs in your trace backend.

## See also

- [`../implementation_plan.md`](../implementation_plan.md) — Phase 5 plan and refine loop  
- [`../deployment.md`](../deployment.md) — deploy patterns  
- [`ingress_gateway_slo.md`](./ingress_gateway_slo.md) — **API gateway ingress** RPS metrics, PromQL, Grafana import, and **F2** evidence record (complements this runbook’s chat/SSE focus)

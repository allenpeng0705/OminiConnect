# Panda: Implementation Plan

This document is the **step-by-step execution plan** for building Panda as a self-contained, high-performance AI gateway. Work follows a **Plan → Implement → Review → Refine** cycle within each phase.

**Why Rust:** The shipping artifact is a single **copy-and-run** static binary—no Python runtimes, no embedded Lua VM, and minimal operational surface area compared with traditional API gateways.

The [high-level design](./high_level_design.md) describes *what* Panda is; this document describes *how* we build it in order.

---

## Alignment with the “enterprise AI gateway” bar

This plan matches the **Kong-style lesson** you care about: a **thin, native data plane** on the hot path (fast intercept, stream-safe forward, hooks for policy), with a **separate, long-term control plane story** implied by GitOps config—not a fat database inside every replica.

**Already explicit:** OpenAI-style ingress, real-time SSE, Wasm extensibility (vs Lua), JWT + TPM, PII scrubbing path, intent routing + MCP, OTel + K8s hardening.

**Cargo from discussions, tracked in phases below:** **universal adapter** (one client contract → many provider schemas), **semantic cache** (embedding + vector lookup—not URL key cache), **circuit breaking / failover** between upstreams, optional **context-enrichment** middleware (gateway-side RAG), and **proof-of-intent** gates on dangerous tool calls (see Phase 4).

**High concurrency:** Throughput scales with **horizontal replicas** behind an L4/L7 load balancer; each instance must keep **per-connection memory flat** on long SSE. Phase 1/5 numbers below are **quality gates**, not product ceilings—document baseline hardware when reporting.

---

## Phase 1: Foundations — the core proxy (“Panda Heart”)

**Goal:** A high-performance HTTP/1.1 + **HTTP/2**-ready streaming proxy that can sit **behind Kong** (upstream from the edge gateway) or run **standalone** for LLM chat streaming. Initial MVP targets **OpenAI-compatible** Chat Completions; adapter-style transforms expand in Phase 4.

**Milestone structure:**

| Step | Name | Intent |
|------|------|--------|
| **1.1** | Workspace | Modular crate layout: **proxy engine** decoupled from **config loader** so each is unit-testable without the other. |
| **1.2** | Streaming hot-path | Tokio + Hyper service loop proving **low overhead** on proxied SSE (TTFT target in Review). |
| **1.3** | Kong handshake | Middleware for **trusted gateway headers** so when Kong (or another edge) already authenticated the caller, Panda extracts **subject / tenant** and **scopes** for immediate **TPM accounting keys** and audit context (full JWT validation remains Phase 3). |

### Plan

- Initialize the Rust **workspace** with **Tokio** (async) and **Hyper** (HTTP); keep **config types + file loading** in a dedicated module or crate consumed by the server binary.
- Define a minimal configuration structure (YAML or JSON) for upstream LLM endpoints (URL, timeouts, optional **priority / failover group** for later circuit breaking).
- Specify **request passthrough** for Chat Completions: preserve semantics the backend expects.
- Specify **trusted ingress** configuration: optional **mTLS** or **shared secret** (e.g. network policy) for the Kong → Panda hop; **header allowlist** mapping to internal identity (see Implement).

### Implement

**1.1 — Workspace** *(implemented in repo)*

- Crate boundaries: **`panda-config`** (parse + validate), **`panda-proxy`** (Tokio/Hyper listener + future forwarder), **`panda-server`** binary crate producing the **`panda`** executable.
- Tests: invalid config in `panda-config`; optional proxy tests with mock upstream **without** binding a real port where possible.
- Run locally: `cp panda.example.yaml panda.yaml` then `cargo run -p panda-server` (or `cargo build -p panda-server` → `target/debug/panda`).

**1.2 — Streaming hot-path** *(core reverse proxy implemented)*

- Listener + **Hyper HTTP/1 client** (`hyper-util` legacy client + **`hyper-rustls`** with **ring** + Mozilla roots): forward **any** path (except `/health` and `/ready`) to `upstream` + client path/query.
- Request/response bodies use **`UnsyncBoxBody`** so bytes **stream through** (SSE-friendly; no full-response buffer in Panda).
- Hop-by-hop header filtering; strip upstream **`content-length`** on the way back so Hyper can re-frame chunked streams safely.
- **Still to do for full Phase 1 review:** explicit **max body / stream** limits, **retry / circuit breaker** groups from config, and measured TTFT benchmarks.

### Kong handshake (milestone 1.3) *(complete for Phase 1)*

- **`trusted_gateway` in YAML** (optional): `attestation_header`, `subject_header`, `tenant_header`, `scopes_header`.
- **Env `PANDA_TRUSTED_GATEWAY_SECRET`**: attestation header value is compared with **HMAC-SHA256(secret, ·) digests** + **`constant_time_eq`** (same-length digest compare; avoids naive string timing leaks). Identity headers stripped when not trusted; attestation never forwarded upstream.
- **`observability.correlation_header`** (default `x-request-id`): reuse incoming id, else **W3C `traceparent`**, else UUID; echoed on the **response**; included in stderr logs.
- **Ops endpoint protection (starter):** optional shared-secret header guard for `/metrics`, `/plugins/status`, and `/tpm/status` using `observability.admin_auth_header` + `observability.admin_secret_env`.
- **Ops audit logging (starter):** structured stderr access logs for ops endpoints with allow/deny outcome, endpoint, correlation id, and TPM bucket context (when applicable).
- **Ops auth telemetry (starter):** `/metrics` exports `panda_ops_auth_allowed_total{endpoint="..."}`, `panda_ops_auth_denied_total{endpoint="..."}`, and `panda_ops_auth_deny_ratio{endpoint="..."}` for ops-endpoint anomaly detection.
- **TPM rejection telemetry (starter):** `/metrics` exports `panda_tpm_budget_rejected_total{bucket_class="..."}` with low-cardinality bucket classes (`anonymous`, `subject`, `tenant`, `subject_tenant`).
- **Ops alerting examples (starter):**
  - Denied spike per endpoint (5m): `sum by (endpoint) (increase(panda_ops_auth_denied_total[5m])) > 20`
  - High deny ratio with minimum traffic guard (10m): `(sum by (endpoint) (increase(panda_ops_auth_denied_total[10m])) / clamp_min(sum by (endpoint) (increase(panda_ops_auth_allowed_total[10m]) + increase(panda_ops_auth_denied_total[10m])), 1)) > 0.30 and sum by (endpoint) (increase(panda_ops_auth_allowed_total[10m]) + increase(panda_ops_auth_denied_total[10m])) > 50`
  - Endpoint fully blocked/misconfigured (15m): `sum by (endpoint) (increase(panda_ops_auth_allowed_total[15m])) == 0 and sum by (endpoint) (increase(panda_ops_auth_denied_total[15m])) > 0`
- **TPM alerting examples (starter):**
  - Budget rejection spike by bucket class (5m): `sum by (bucket_class) (increase(panda_tpm_budget_rejected_total[5m])) > 100`
  - Sustained throttling pressure (15m): `sum(increase(panda_tpm_budget_rejected_total[15m])) > 500`
  - Anonymous-heavy rejection share (10m): `sum(increase(panda_tpm_budget_rejected_total{bucket_class="anonymous"}[10m])) / clamp_min(sum(increase(panda_tpm_budget_rejected_total[10m])), 1) > 0.70`
- **TPM**: in-memory prompt (`Content-Length / 4`) and **completion** (SSE: **tiktoken** `cl100k_base` on `delta.content`, or `usage.completion_tokens` when present); optional **Redis** via `tpm.redis_url` or **`PANDA_REDIS_URL`** (`INCRBY` on `panda:tpm:v1:prompt:*` / `completion:*`). Starter per-minute prompt budget enforcement is available via `tpm.enforce_budget` + `tpm.budget_tokens_per_minute` (returns HTTP 429), with response visibility headers (`x-panda-budget-*`), configurable/derived `Retry-After`, and a `/tpm/status` JSON endpoint for current bucket budget inspection.
- **TLS / mTLS**: optional **`tls`** block (`cert_pem`, `key_pem`, optional **`client_ca_pem`**) — Panda listens with **HTTPS** on the same `listen` address; plaintext HTTP path disabled when TLS is configured.
- **Still to do later:** richer audit export, distributed/global budget enforcement, adaptive/sliding-window TPM policies.
- Kong pattern: terminate OIDC at Kong, set attestation + identity headers, forward to Panda (see [integration & evolution](./integration_and_evolution.md)).

### Review

- Does the proxy sustain **~1,000 concurrent** streams per instance **on a defined baseline machine** without degradation (tail latency, error rate)?
- Is **time to first token (TTFT)** overhead from the gateway **under ~2 ms** in a controlled lab setup? (Tune baselines per hardware; treat this as a target, not a guarantee.)
- **Kong handshake:** With trust configured, does a request **without** a valid edge hop **not** inherit privileged identity? With trust + injected headers, do TPM counters attribute to the **correct** subject?

### Refine

- Profile allocations; prefer **`Bytes`** / shared buffers for bodies and SSE chunks where it measurably helps.

---

## Phase 2: Plugin engine — the Wasm sandbox

**Goal:** Extensibility via **Rust / Go (TinyGo)** plugins **without** recompiling the main binary.

### Plan

- Integrate **Wasmtime** (or a similarly mature embedded Wasm runtime) as the plugin VM.
- Define the **host ↔ plugin ABI:** what the plugin receives (headers, body chunks, request context) and what it may return (mutations, early reject, metrics tags).

### Implement

- **Workspace crate `panda-wasm`:** `PANDA_WASM_ABI_VERSION`, **Wasmtime** loader, `plugins.directory` in config; loads `*.wasm` at startup and runs an optional `add` smoke export per module.
- **`panda-proxy`:** loads plugins when `plugins.directory` is set; keeps `Arc<PluginRuntime>` for upcoming per-request hooks.
- **Sample Rust guest (`wasm-plugin-sample`):** minimal `panda_abi_version` + `add` for smoke tests; build with `wasm32-unknown-unknown`.
- **ABI spec doc:** `docs/wasm_abi.md` now defines v0 exports/imports, return codes, limits, and compatibility.
- **TinyGo sample skeleton:** `examples/tinygo-plugin/` added for cross-toolchain parity and rollout validation.
- **Next:** trap isolation hardening per request, TinyGo build matrix validation, optional hot reload.

### Review

- Misbehaving or trapping plugins **must not** crash the gateway process; isolate failures per invocation and surface structured errors.
- **Status semantics (implemented):**
  - plugin policy reject (`panda_on_request*` non-zero return code) -> **HTTP 403** when `plugins.fail_closed=true`
  - runtime/trap/timeout/join failures -> **HTTP 502** when `plugins.fail_closed=true`
  - fail-open mode (`plugins.fail_closed=false`) logs and continues
- **Return-code contract (v0):** `0=allow`, `1=policy denied`, `2=malformed request`, other non-zero values are plugin-specific rejects.
- **Hot reload (implemented):** optional `plugins.hot_reload` with interval `plugins.reload_interval_ms`; runtime swaps atomically on directory fingerprint changes and keeps serving with last-good runtime when reload fails.
- **Observability counters (implemented):** in-process per-plugin/per-hook counters for allow/reject/runtime/timeout outcomes.

### Refine

- Add metric export surface (Prometheus / OTel) for plugin counters and reason codes.
- **`GET /plugins/status`** includes **`wasm_runtime_by_reason`**, aggregating plugin hook **`runtime`** outcomes (e.g. `Trap`) from the same counters exported as `panda_plugin_events_total` on `/metrics`.

---

## Phase 3: Identity & security — the bouncer

**Goal:** Answer **who is calling** and reduce **enterprise data** leakage risk on the hot path.

### Plan

- JWT validation middleware (**OIDC-compatible** issuers: Okta, Azure AD, etc.).
- **Token exchange:** map a **user token → scoped agent token** with tight tool/route claims.

### Implement

- **JWT middleware (Phase 3 entry):** optional bearer auth with HS256 secret env and claim checks (`iss`/`aud`/required `scope`) from config.
- **Token exchange (starter):** mint scoped agent token (`x-panda-agent-token`) from validated user JWT using dedicated signing secret and TTL.
- **Prompt safety (starter):** configurable deny-pattern scan on request path/query/body to block obvious jailbreak/injection strings with policy reject.
- **PII scrubber (starter):** regex-based request-body redaction with configurable replacement and patterns.
- **Token-based rate limiting** (TPM / token budgets): start with **in-memory** per-instance counters; design boundaries so **Redis** (or similar) can back **global** counts later.

### Review

- Can we block or flag prompts that match **jailbreak / injection** heuristics (starter rules before full ML)?

### Refine

- Multi-pattern matching at scale: **Aho–Corasick** (e.g. via the `aho-corasick` crate) for many fixed-string / pattern scans.

---

## Phase 4: Intelligence & MCP — the agentic brain

**Goal:** Move beyond dumb proxying to a **tool-aware** gateway with **cost-aware** reuse and **multi-vendor** egress.

**Program priority note:** Agent usability is now a top differentiator (OpenClaw-class and other small agents). Phase 4 work should prefer outcomes that make Panda easy to adopt for agents while reducing token burn safely (intent-aware caching, selective tool advertising, and clear per-session budget controls).

**Panda API gateway + MCP:** Detailed **pipelines, modules, and config** — [`design_api_gateway_and_mcp_gateway.md`](./design_api_gateway_and_mcp_gateway.md). Phased milestones — [`implementation_plan_mcp_api_gateway.md`](./implementation_plan_mcp_api_gateway.md). **What is implemented vs planned** — [`gateway_design_completion.md`](./gateway_design_completion.md).

### Plan

- Embed an **MCP host client** in the binary.
- Add a **semantic router** using a **small ONNX** model (e.g. sentence embedding + k-NN routes, or a tiny classifier—`all-MiniLM-L6-v2` class of model is a reasonable starting point).
- **Semantic cache:** normalize prompt (+ optional fingerprint of system/tools), embed with a **fast local** model, query **Redis / Milvus / other** vector index for similarity ≥ configured threshold; on hit, return cached completion (skip upstream). Keeps **store pluggable**—the gateway owns the policy and keying, not necessarily the database binary. **Shipped today:** memory/Redis exact keys, optional **Jaccard** on memory, optional **OpenAI embedding cosine** on memory (`semantic_cache.embedding_*`); a dedicated external vector index remains the scale-out direction.
- **Universal adapter:** map **one** stable inbound API (OpenAI-compatible) to **backend-specific** JSON for Anthropic, Gemini, local llama.cpp servers, etc., so clients swap models without code changes.

### Implement

- Connect to at least one MCP server locally (e.g. SQLite or filesystem demo server).
- Surface discovered tools to the LLM through the **`/v1/chat/completions` tools** field (or equivalent for your chosen API compatibility layer).
- Optional **context enrichment** middleware: gateway performs retrieval (vector search against corp KB), **injects** snippets into the prompt or messages, then forwards—keeps application clients thin (same pattern as “RAG at the edge”).
- **Agent sessions (orchestration-adjacent):** optional **`agent_sessions`** — session and planner **profile** from JWT claims and/or headers (headers override JWT), TPM bucket suffix per session, **profile → upstream** rules applied before semantic routing (longest `path_prefix` wins), per-session and per-profile **MCP `max_tool_rounds`** caps, response echo. **`/tpm/status`** includes resolved `agent_sessions.session` / `agent_sessions.profile` when the feature is enabled. **`/mcp/status`** adds **`max_tool_rounds_effective`** (reference path `/v1/chat/completions`, example effective caps, config summary, resolution note). See [`ai_routing_strategy.md`](./ai_routing_strategy.md) §3.5.
- **Compliance + hierarchy:** when **`budget_hierarchy.enabled`**, optional **`budget_hierarchy_nodes`** (`org`, `dept:<name>`) on **`panda.compliance.ingress.v1`** / **`egress.v1`** JSONL rows when the request resolves to an org cap and/or a configured department limit (see [`compliance_export.md`](./compliance_export.md)).

### Review

- End-to-end: **LLM invokes an MCP tool** through Panda and the result flows back correctly.
- **Proof of intent (MVP):** block or audit tool calls that are **inconsistent** with routed intent / user prompt class (starter rules before full ML)—aligns with [HLD proof-of-intent](./high_level_design.md#42-proof-of-intent-agentic-security-model).

### Refine

- **Tool discovery:** send only **intent-relevant** tools to the model to save context window and reduce misuse surface; **`GET /mcp/status`** reports `intent_tool_policies_configured` and `intent_scoped_tool_advertising` when `mcp.intent_tool_policies` is used. Semantic-cache abuse notes: [`threat_model_semantic_cache.md`](./threat_model_semantic_cache.md). gRPC is out of scope for the HTTP hot path today: [`grpc_ingress_roadmap.md`](./grpc_ingress_roadmap.md).
- **Streaming MCP tuning:** tune `mcp.stream_probe_bytes` (how many first-round bytes Panda inspects before passthrough) together with `mcp.probe_window_seconds` (rolling status window) using `/mcp/status` (`probe_window_*`) and `/metrics` (`panda_mcp_stream_probe_*`) to balance TTFT vs early tool-call detection.
- **Context enrichment runtime health:** verify `/mcp/status` enrichment fields (`enrichment_enabled`, `enrichment_rules_count`, `enrichment_last_mtime_ms`) during rollouts to ensure rules are resident in memory and reloaded on file updates.

---

## Phase 5: Production & scale — enterprise finish

**Goal:** Deployment is **boring**; failure modes are **bounded** and observable.

### Plan

- **OpenTelemetry:** OTLP HTTP trace export when `OTEL_EXPORTER_OTLP_ENDPOINT` is set; optional `PANDA_OTEL_SERVICE_NAME` for resource `service.name`. Emitting the same dimensions as rich OTel **metrics** (TTFT, stream duration, TPM, etc.) remains follow-on work.
- **Packaging:** Docker image wrapping the **single static binary**; reproducible builds.
- **Status:** JSON structured logging baseline is in place (`RUST_LOG`) with per-request completion events (`method`, `path`, `status`, `correlation_id`, `elapsed_ms`), independent of OTLP.

### Implement

- Kubernetes **Deployment** + **ConfigMap** (and Secrets) templates; document upgrade and rollback; **PodDisruptionBudget** / **HPA** hooks as needed for **many replicas** at egress.
- Starter manifests are in `k8s/` (`deployment.yaml`, `configmap.yaml`, `service.yaml`, `pdb.yaml`, `hpa.yaml`, `secret.example.yaml`) with probes wired to `/health` and `/ready`.
- Reproducible release packaging helper is available via `scripts/release_repro_build.sh` (`--locked`, stable `SOURCE_DATE_EPOCH`, SHA256 artifact output in `artifacts/release/`).
- **Health endpoints:** `/health` (process up) and `/ready` with explicit checks:
  - upstream config URI is valid,
  - MCP runtime is connected when `mcp.enabled=true` and `mcp.fail_open=false`,
  - context enrichment source exists when `PANDA_CONTEXT_ENRICHMENT_FILE` is configured.
  - JSON also includes **`shutdown_drain_seconds`** (from `PANDA_SHUTDOWN_DRAIN_SECONDS`), **`active_connections`**, and **`model_failover`** summary (`enabled`, `allow_failover_after_first_byte`, `midstream_sse_max_buffer_bytes`, `groups_configured`, streaming failover notes). See [`runbooks/production_slo.md`](./runbooks/production_slo.md).

### Review

- Load test at scale (e.g. **~5,000** concurrent streams **aggregate** or per shard, per environment—state the topology); watch CPU, RSS, and tail latency; validate no systematic memory growth on long-lived SSE.
- **Soak test:** run **≥ 24 hours** of steady **SSE** traffic (mix of short and long streams) and confirm **no creep** in RSS, open sockets, or in-flight task counts; catch slow leaks that burst tests miss.
- Revisit **Phase 1** SLOs under load: TTFT overhead, p99 stream stall time, upstream failover correctness.
- Use `scripts/staging_readiness_gate.sh` as a repeatable pre-Phase-5 gate for core tests, readiness/status checks, and optional short load probing.
- Use `scripts/load_profile_chat.sh` for repeatable load snapshots and `scripts/soak_guard_sse.sh` for SSE soak/resource drift checks.

### Refine

- Allocator tuning (**mimalloc** / **jemalloc**) if fragmentation shows up under long-lived AI streams; validate with benchmarks before committing.
- Graceful shutdown for rollouts: on SIGTERM, fail `/ready` immediately, stop accepting new connections, and drain active streams up to `PANDA_SHUTDOWN_DRAIN_SECONDS`.
- Optional allocator feature is wired in `panda-server` (`--features mimalloc`) for A/B comparisons under load.

---

## Deployment complexity (illustrative)

| Aspect | Phase 1–2 (MVP) | Phase 5 (enterprise) |
|--------|------------------|-------------------------|
| **Binary size** | ~15 MB order of magnitude | ~45 MB order of magnitude (Wasm + ONNX / models bundled or mounted) |
| **Runtime dependencies** | None required at OS level beyond libc | Optional **Redis** (or equivalent) for **global** TPM / shared limits |
| **Deployment** | Copy binary, run `./panda` | `kubectl apply -f` on maintained manifests (e.g. `k8s/` or Helm) |

Sizes are **targets** until measured on the release pipeline; track them in CI.

---

## Mapping to the conceptual layers

| Implementation phase | Primary HLD layers |
|----------------------|--------------------|
| Phase 1 | Ingress + Egress (minimal) |
| Phase 2 | Cross-cutting (extensibility) |
| Phase 3 | Shield (+ Ingress hooks) |
| Phase 4 | Brain + MCP aggregation |
| Phase 5 | Operations, observability, packaging |

This plan can be revised as benchmarks and security reviews land; keep the **Plan → Implement → Review → Refine** loop explicit in PRs and milestones.

---

## Enterprise final targets (north star)

The phases above deliver a **credible MVP through production-ready core**. For **final** enterprise positioning—competing with mature API gateways on operability and trust—track the items below (not all need to ship on day one; prioritize by tenant and industry).

For **Core vs Enterprise** positioning (fast default path for small teams; optional **SSO**, **hierarchical USD budgets**, **model parity / failover**), see [`enterprise_track.md`](./enterprise_track.md).
For the **live Kong-replacement execution tracker** (statuses, scorecard, and progress log), see [`kong_replacement_program.md`](./kong_replacement_program.md).

### Reliability and performance

- Published **SLOs** (e.g. availability, p99 TTFT add-on, p99 time-to-last-token under load) and **error budgets**; dashboards that match those SLOs.
- **Graceful shutdown:** stop accepting new streams, **drain** in-flight SSE within a bounded window, then exit—required for Kubernetes rollouts without client resets.
- **Explicit limits:** max concurrent streams per instance, per-tenant, and per-key; **fair queuing** or backoff when saturated (avoid retry storms).
- **Capacity planning:** document scaling curves (streams vs CPU vs RAM); optional **chaos** or fault-injection drills for upstream blackout and slowloris-style clients.

### Security and compliance

- **Threat model** doc (see HLD document map): injection, tool abuse, **cache poisoning** (semantic cache), scope elevation, plugin escape hardening.
- **Secrets:** upstream API keys and OIDC client secrets from env / Vault / K8s Secrets only; never echo in logs or traces without redaction policy.
- **mTLS** (optional) gateway ↔ sensitive upstreams; **plugin supply chain:** signed Wasm, allowlisted digests, ABI version pinning.
- **Audit trail:** structured, **intent- and tool-centric** events (who, which agent, which tool, allow/deny/synthetic cache hit); retention and **SIEM** export; policy for **PII in audit logs**.
- **Compliance hooks:** tenant **data residency** flags, semantic-cache **TTL/eviction** and “forget” semantics where regulations require it; DPIA-friendly description of what leaves the network.

### Multi-tenancy and governance

- **Strong tenant isolation** in config and quotas (routes, backends, TPM, tool allowlists); optional separate upstream credentials per tenant.
- **Config validation** at load: invalid config **fails closed** or blocks `/ready`; schema versioning for GitOps.
- **Admin vs data plane:** any future **management API** must be authenticated, authorized, **rate-limited**, and separate from customer-facing ingress where possible.

### Protocol and integration completeness

- **gRPC** ingress/egress (per HLD) when REST-only is no longer enough for internal services.
- **Webhooks / eventing** (optional): async notify on policy violations or spend thresholds for finance/SOC workflows.

### Operational excellence

- **Runbooks:** upstream down, cache poisoned, JWT clock skew, OOM during soak, Wasm trap storms.
- **Release safety:** canary or progressive rollout story; **versioned** config migration notes.
- **SBOM** and dependency audit in CI; **signed** container images (cosign or equivalent).

Treat this section as the **backlog umbrella** for “enterprise finish” after Phase 5; fold items into phased work as customers demand them.

# Panda: High-Level Design

**Project codename:** Panda  
**Core language:** Rust  
**Extensibility:** WebAssembly (Wasm)  
**Primary goal:** A high-performance, stateless, **intent-aware** gateway that unifies large language models (LLMs), legacy HTTP APIs, and Model Context Protocol (MCP) tools under one secure, enterprise-ready surface.

This document captures the conceptual model, major subsystems, security posture, deployment pattern, and phased roadmap. It is intentionally high level: APIs, wire formats, and implementation choices belong in subsequent specs.

---

## 1. Design philosophy: the “Panda” way

Traditional API gateways optimize for short, structured requests and responses. AI traffic behaves differently. Panda is shaped around that traffic:

| Property | Implication |
|----------|---------------|
| **Long-lived streams** | Token flows over Server-Sent Events (SSE) and similar streaming patterns must be efficient; the system should favor paths that reduce unnecessary copies and buffering where safe. |
| **Unstructured payloads** | Value is in natural language, not only in headers and paths. Policy and routing may need to reason about **meaning**, not just metadata. |
| **Agentic identity** | The unit of accountability is often an **agent** acting on behalf of a **human**, with **tools** in the middle. Observability and policy must track goals and tool use, not only URLs. |

**Default posture:** Stateless at the data plane so instances can scale horizontally without a custom cluster-wide session protocol. Shared state (when introduced) should be explicit, optional, and pluggable (for example external cache or vector store), not baked into the core binary’s assumptions.

---

## 2. Product shape

- **Single static binary** for the default deployment story: minimal moving parts at the edge, suitable for containers and sidecars.
- **GitOps-friendly configuration** (YAML or similar): one artifact describes routes, backends, guards, and plugins; rolling out thousands of replicas is “update config, restart or reload.”

### 2.1 Repository layout (Rust workspace)

The implementation is a **Cargo workspace** so configuration parsing and the HTTP engine stay **independently testable**—matching [Phase 1.1](./implementation_plan.md) (“Panda Heart” workspace milestone).

| Crate | Responsibility |
|-------|------------------|
| **`panda-config`** | Types, validation, and YAML load for listener address, upstream base URL, and future routes/plugins—**no** socket I/O. |
| **`panda-proxy`** | Async HTTP server (**Tokio** + **Hyper**); **streaming reverse proxy**; **Kong handshake** (optional attestation header + env secret, strip spoofed identity); in-memory **TPM hint** (`Content-Length`/4 per subject bucket). |
| **`panda-server`** | Thin binary crate; wires config path (CLI) → `panda-config` → `panda-proxy::run`. The built artifact is named **`panda`** (`cargo run -p panda-server` / `target/debug/panda`). |
| **`panda-wasm`** | Phase 2: Wasmtime-based plugin host and guest ABI (workspace placeholder until integrated). |

A sample file lives at `panda.example.yaml` in the repo root (copy to `panda.yaml` for local runs).

---

## 2.2 Panda identity — design pillars

These pillars summarize **what Panda is** for engineering and GTM; they align with the [integration & evolution](./integration_and_evolution.md) story (Kong coexistence, domain split) and the [implementation plan](./implementation_plan.md).

| Pillar | Definition | Implementation direction |
|--------|------------|----------------------------|
| **Stateless core** | No local database required in the default data plane. | Crate **`panda-config`**: GitOps **YAML**; **optional Redis** (or similar) for global TPM, semantic cache index, and shared counters—not a mandatory embedded DB per instance. |
| **Stream-native** | Built for the LLM “typewriter” effect (SSE and kin). | Crate **`panda-proxy`**: **Tokio** + **Hyper**; **`bytes`** / minimal copying on the streaming hot path; timeouts and heartbeats suited to long streams. |
| **Wasm extensibility** | Plugin-first customization without Lua lock-in. | **Wasmtime** (or equivalent) sandbox; **Rust / TinyGo** as primary plugin sources. |
| **Intent-aware** | Security and routing that reason about **meaning**, not only paths. | **Embedded ONNX** (or similar) for prompt/tool classification and routing; **proof-of-intent** for dangerous tool use (see §4.2). |
| **The layered play** | Designed to sit **behind** or **beside** Kong-class gateways. | **Trusted gateway headers** for identity when Kong already authenticated; **OpenTelemetry** parity with existing dashboards; **easy upstream** registration in Kong; **[domain split](./integration_and_evolution.md#domain-split-pattern-preferred-for-panda-at-the-ai-edge)** when Panda is the AI edge. |

---

## 3. Logical architecture

Panda is organized as four conceptual layers. Each layer can be implemented incrementally; early phases may stub or simplify “Brain” and “Shield” features while hardening the proxy core.

### 3.1 Ingress layer — “The Mouth”

**Role:** Accept high concurrency connections and normalize inbound protocols into an internal **Panda standard** representation (request metadata + body/stream handles).

- **Runtime:** Async I/O with **Tokio**; HTTP with **Hyper** (or equivalent) as the foundation for thousands of concurrent streams.
- **Protocol support (targets):**
  - OpenAI-compatible REST (including streaming).
  - gRPC for services that prefer it.
  - MCP-over-SSE where clients or tools speak MCP over HTTP/SSE.

The ingress layer is responsible for connection lifecycle, backpressure, timeouts, and early request validation — not for deep semantic policy (that belongs in Shield / Brain).

### 3.2 Shield layer — “The Bamboo”

**Role:** Security and hygiene **before** expensive or risky downstream work.

| Capability | Purpose |
|------------|---------|
| **Prompt-injection firewall** | Inspect inbound text for attempts to override system policy, exfiltrate secrets, or hijack tool behavior. Operates on streaming text where required. |
| **PII masking** | Asynchronous NER-style detection and redaction of emails, keys, identifiers, and other sensitive spans in near real time on the hot path. |
| **Identity exchange** | Validate enterprise OIDC/JWTs (e.g. Okta, Azure AD), map identities to scopes, and issue **scoped agent tokens** that bound what automated callers may do downstream. |

Shield outputs a **sanitized, attributed request context**: who the human is, which application/agent acts, and what scopes apply — feeding the Brain and Egress.

### 3.3 Intelligence layer — “The Brain”

**Role:** Intent, routing, reuse, and tool surfacing.

| Capability | Purpose |
|------------|---------|
| **Intent router** | Classify prompts (e.g. coding vs. HR vs. support) using an **embedded ONNX** model (or similar) and route to the best-fit model or policy profile. |
| **Semantic cache** | Vector similarity over prompts (and optionally context fingerprints) to skip redundant LLM calls when a cached answer is “good enough,” reducing cost and latency. |
| **MCP aggregator** | Maintain connections to multiple MCP servers and expose a **unified tool surface** to the LLM, with consistent naming, auth, and capability advertisement. |

The Brain may call external services (vector DB, model hosts); those integrations should remain **optional modules** so a minimal Panda stays a pure proxy.

### 3.4 Egress layer — “The Hands”

**Role:** Adapt the internal representation to each backend’s reality.

- **Universal adapters:** Translate Panda’s canonical request into vendor-specific formats (OpenAI, Anthropic, internal REST, legacy SOAP, etc.).
- **Stream emulator:** When a backend returns only a full response body, buffer and **re-emit as a stream** (typewriter-style chunks) so clients always see a modern streaming UX where configured.

Egress also enforces **per-backend quotas**, transforms errors into a consistent client-facing shape, and attaches correlation IDs for tracing.

---

## 4. Cross-cutting concerns

### 4.1 Strategic differentiation (vs. typical API gateways)

| Target | Panda emphasis | Typical gap (e.g. Kong-style gateways) |
|--------|----------------|----------------------------------------|
| **Simplicity** | Single binary, no mandatory control plane database | Heavier dependency stacks or embedded script runtimes |
| **Cost control** | Token-based limits (TPM) + semantic cache | Often RPS-oriented metering only |
| **Extensibility** | Rust + **Wasm** plugins (including paths for **Go**-compiled Wasm where applicable) | Historic Lua-centric extension models |
| **Observability** | Intent- and tool-centric audit logs | URL- and status-centric access logs |

### 4.2 Proof of intent (“agentic” security model)

Every processed request should satisfy a **three-way check** before dangerous or irreversible actions (especially tool calls):

1. **Who is the human?** — Strong identity via OIDC/JWT validation.
2. **Which agent is acting?** — Application or agent identity and credential binding.
3. **Is the tool call valid for this prompt?** — **Intent monitor:** compare declared tool use against classified user intent; block incoherent or abusive patterns (e.g. “summarize this file” paired with a “delete database” tool).

This is complementary to API keys: valid keys are insufficient if intent and scope do not align.

### 4.3 Statelessness and horizontal scale

- Default: **no sticky session** requirement; each instance can serve any request given config + optional external stores.
- **Semantic cache** and **vector indices** are treated as **external or optional** components unless a deliberate embedded mode is chosen later.
- Configuration is the **source of truth** for routing and policy version; GitOps and Kubernetes ConfigMaps match this model.

---

## 5. Deployment models

| Mode | Description |
|------|-------------|
| **Standalone** | One or more Panda instances behind a load balancer for an organization or environment. |
| **Sidecar** | A small Panda next to each service pod for **local AI adjacency** (policy, redaction, and egress control near the workload). |
| **Kubernetes** | Stateless replicas; config via ConfigMap/Secret; scale with HPA on connections, CPU, or custom metrics (e.g. TPM). |

Operational requirements should favor **clear observability** (structured logs, metrics, traces) aligned with intent and tool events, not only HTTP access lines.

Readiness should be stricter than process liveness: use `/health` for process up, and `/ready` for dependency/config checks (upstream URI validity, MCP runtime availability when fail-closed, and context-enrichment source availability when configured).

### Operator playbook: streaming MCP probe tuning

For `stream=true` chat traffic with MCP enabled, tune probe settings in small steps:

1. **Baseline**
   - Keep defaults: `mcp.stream_probe_bytes=16384`, `mcp.probe_window_seconds=60`.
   - Capture baseline TTFT and tool-call success rates for representative workloads.
2. **Observe**
   - Check `/mcp/status` windowed fields (`probe_window_seconds`, `probe_window_decisions`, `probe_window_bytes_total`).
   - Check enrichment cache fields in `/mcp/status` (`enrichment_enabled`, `enrichment_rules_count`, `enrichment_last_mtime_ms`) to confirm context rules are loaded and refresh when the source file changes.
   - Check `/metrics` counters (`panda_mcp_stream_probe_decision_total`, `panda_mcp_stream_probe_bytes_total`, `panda_mcp_stream_probe_bytes_bucket_total`).
3. **Tune**
   - If `passthrough` dominates and TTFT is high, reduce `stream_probe_bytes`.
   - If missed/late tool detection appears in streaming flows, increase `stream_probe_bytes`.
   - Use `probe_window_seconds` to match your analysis cadence (short windows for incident response, longer for steady-state tuning).
4. **Validate**
   - Re-run load/soak slices and compare TTFT, tool-call completion, and error-rate deltas before adopting new defaults.

---

## 6. Extensibility: Wasm plugins

Wasm is the primary extension mechanism so operators can add policy hooks, transforms, and integrations without forking the core:

- **Lifecycle hooks:** Request/response, stream chunks, tool-call interception.
- **Language sources:** Rust as first-class; Go (or other) where the toolchain targets **Wasm** cleanly.
- **Sandbox:** Strict resource limits (memory, fuel/instructions, host call allowlists) to match enterprise security expectations.

Detailed ABI, manifest format, and signing are out of scope for this document and belong in a Wasm plugin specification.

---

## 7. Phased roadmap

Phases are ordered to **prove the proxy and streaming story first**, then extensibility, **enterprise identity and guardrails**, intelligence (routing, cache, MCP), and production hardening. Detailed Plan / Implement / Review steps live in the [implementation plan](./implementation_plan.md); the summary below stays aligned with that document.

| Phase | Codename | Focus |
|-------|----------|--------|
| **1** | Heart | Rust proxy core — HTTP/1.1 + HTTP/2-ready streaming (SSE), backpressure, upstream failover hooks, config. |
| **2** | Skin | Wasm plugin system — load, isolate, host ABI, hot reload. |
| **3** | Bouncer | JWT / OIDC, scoped agent tokens, PII scrubbing path, TPM, injection heuristics. |
| **4** | Brain | Intent routing (ONNX), semantic cache, universal adapters, MCP host, proof-of-intent on tools, optional context enrichment. |
| **5** | Enterprise finish | OpenTelemetry, packaging, Kubernetes, health probes, load + soak tests, allocator tuning. |

Later refinements (multi-tenant isolation, formal compliance programs, advanced threat models) build on these phases without changing the core layering.

---

## 8. Non-goals (at this stage)

- Mandating a specific vector database or identity provider.
- Replacing full-featured service meshes; Panda complements mesh policy with **AI-specific** semantics.
- Guaranteeing ONNX or MCP versions in this document — those are implementation choices per phase.

---

## 9. Document map (suggested follow-ups)

| Document | Content |
|----------|---------|
| [Implementation plan](./implementation_plan.md) | Phased execution (Plan → Implement → Review → Refine), benchmarks, packaging. |
| [Integration & evolution (Kong coexistence)](./integration_and_evolution.md) | Neighbor strategy, migration steps, shadow mode, familiar service/route/plugin model. |
| [Evolution phases (1–3)](./evolution_phases.md) | Coexistence, AI-first edge, optional consolidation—org deployment story vs Kong. |
| [Kong / edge handshake](./kong_handshake.md) | Header contract: attestation, identity, correlation; Kong recipe outline. |
| [Deployment](./deployment.md) | Binary, YAML, optional Redis / Prometheus / OTLP / Kong; standalone and TLS. |
| Request / stream canonical model | Internal representation, SSE chunk semantics, error envelope. |
| Configuration schema | Routes, backends, Shield rules, plugin manifests. |
| Wasm plugin ABI | Host functions, versioning, security boundaries. |
| Threat model | Injection, tool abuse, cache poisoning, token scope elevation. |
| [Enterprise final targets](./implementation_plan.md#enterprise-final-targets-north-star) | SLOs, safety, compliance-hint checklist beyond MVP. |

---

*This is a living design. Revise as the Rust implementation and operational feedback narrow choices.*

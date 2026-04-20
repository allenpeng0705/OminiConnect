# RFC: Wasm plugins for agent-class policies (no implementation commitment)

Panda already loads **guest Wasm** modules for request/response hooks ([`panda-wasm`](../crates/panda-wasm), `plugins` config). This RFC scopes **what would be needed** if OpenClaw-class tenants want **richer, per-tenant agent policies** (tool/prompt/stream rules) **without** recompiling Panda.

## Current surface (baseline)

- **Request body** hooks run on **buffered** bodies within `max_request_body_bytes` and `execution_timeout_ms` ([`panda.example.yaml`](../panda.example.yaml) `plugins` block).
- **SSE path:** [`WasmChunkHookBody`](../crates/panda-proxy/src/sse.rs) wraps streaming responses and invokes Wasm on **chunks** of the upstream SSE stream (subject to the same size/timeout limits as configured for plugins). Fail behavior follows **`plugins.fail_closed`** (fail open logs and continues on error when `fail_closed=false`).

## Gaps vs “agent policy platform”

| Area | Today | Desired direction |
|------|--------|-------------------|
| **Isolation** | Wasmtime with configured limits; shared process with the proxy | Stronger **memory + fuel** guarantees per tenant; optional **per-tenant plugin allowlist**; document **blast radius** (guest cannot escape sandbox, but can burn CPU within limits). |
| **Streaming policy** | Chunk-oriented hook on SSE bytes | Optional **structured** hook (parsed `data:` JSON lines) for safer tool/prompt rules; **rate limits** on hook invocations per request. |
| **Policy source** | Directory of `.wasm` on disk | Optional **signed** artifacts, version pinning, hot-reload semantics (already partially configurable). |
| **Identity context** | Hooks see what the gateway passes into the Wasm ABI | Explicit **tenant / session / intent** context in the guest API for agent policies (design-only). |

## Failure modes

- **`fail_closed=false` (fail-open):** Wasm errors or timeouts **do not** block traffic; operators must monitor plugin metrics and logs.
- **`fail_closed=true`:** Policy rejects map to **403** / **502** per existing semantics — suitable only when false positives are acceptable.

## Non-goals (this RFC)

- Replacing **MCP** or **proof-of-intent** logic entirely with Wasm.
- **Multi-tenant arbitrary code upload** without a separate control plane and signing story.

## Next steps (when prioritized)

1. Document **current** Wasm ABI and limits in one place (link from [`CONTRIBUTING.md`](../CONTRIBUTING.md) or plugin sample README).
2. Prototype **structured SSE** hook behind a feature flag (after API review).
3. Security review: **resource limits**, **deterministic** builds, supply chain for `.wasm` binaries.

## References

- [`crates/panda-proxy/src/sse.rs`](../crates/panda-proxy/src/sse.rs) — `WasmChunkHookBody`, idle timeouts, TPM counting path.
- [`docs/agent_fleet_gap_roadmap.md`](./agent_fleet_gap_roadmap.md) — Track E sequencing.

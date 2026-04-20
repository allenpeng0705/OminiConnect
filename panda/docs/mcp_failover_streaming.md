# MCP tool-followup and model failover streaming (design note)

This note explains why **buffered mid-stream SSE failover** (`model_failover.allow_failover_after_first_byte`) does **not** apply to requests that use the **MCP streaming follow-up** path, and records options for future work.

## Current behavior

- **Model failover** retries on retryable HTTP status codes **before** returning response headers to the client.
- When **`allow_failover_after_first_byte=true`**, successful **OpenAI-shaped streaming chat** responses under failover may be **fully buffered** (up to **`midstream_sse_max_buffer_bytes`**) so a **later** backend can be retried if the winning upstream fails **mid-body**—**TTFT** is higher until the buffer completes. See **`GET /ready`** → `model_failover.streaming_failover` (**`midstream_body_failover_note`** + **`midstream_body_failover_detail`**) and [`enterprise_track.md`](./enterprise_track.md) §3.
- The gateway **skips** this buffered path when **`maybe_mcp_followup`** is true (MCP tools advertised and the request participates in the streaming follow-up / probe flow). Rationale: the handler must keep a **live** upstream body stream for the **first-round MCP probe** (`probe_mcp_streaming_first_round`); consuming it into a single buffer would break that control flow.

**Also excluded from buffered failover today:** Anthropic **adapter** streaming responses (same overall “streaming translation” path constraints).

## Non-goals

- **Seamless** concatenation of partial SSE from provider A with partial SSE from provider B is **not** safe for OpenAI-style chat chunks; Panda does not attempt chunk-level splice across backends.

## Future options (RFC-level)

| Option | Idea | Tradeoffs |
|--------|------|-----------|
| **Probe-then-buffer** | Complete the MCP first-round probe (or cap bytes/time), **then** if still streaming and failover is on, switch to buffered collection + mid-stream retry. | More code paths; TTFT and probe limits need clear product defaults. |
| **Feature matrix** | Document operator choice: enable **either** MCP streaming probe **or** buffered mid-stream failover for the same route class. | Simple; may force product tradeoff per route. |
| **Dual connection** | Rarely: tee upstream bytes to probe consumer and buffer (memory). | Complexity and memory cost. |

No implementation commitment here; pick one direction when agent + failover SLOs require both behaviors on the **same** request class.

## References

- [`crates/panda-proxy/src/lib.rs`](../crates/panda-proxy/src/lib.rs) — `maybe_mcp_followup` guard around buffered failover.
- [`crates/panda-proxy/src/model_failover.rs`](../crates/panda-proxy/src/model_failover.rs) — `collect_openai_sse_with_midstream_failover`.
- [`docs/runbooks/production_slo.md`](./runbooks/production_slo.md) — SLO / readiness notes.

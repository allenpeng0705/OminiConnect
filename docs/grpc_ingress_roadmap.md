# gRPC ingress / egress roadmap (not implemented)

**Status:** Panda’s hot path today is **HTTP/1.1** (and HTTP/2-friendly streaming) with **OpenAI-compatible** JSON for chat. **gRPC** is listed under long-term enterprise completeness in [`implementation_plan.md`](./implementation_plan.md). The main gateway API remains HTTP; you can optionally build **`panda-server` with `--features grpc`** and set **`PANDA_GRPC_HEALTH_LISTEN`** to expose **`grpc.health.v1`** for load-balancers that probe gRPC (no chat/RPC surface yet). The health server uses tonic **`serve_with_shutdown`**: when the HTTP gateway’s **`run`** future completes (normal exit after drain or error), a shutdown signal stops the gRPC listener in the same process—no orphan listener after the main server stops.

## Why it might matter later

- Internal services already speak **gRPC** for embeddings, routing, or sidecars.
- Some model hosts expose **gRPC** APIs with different framing than REST+SSE.

## Suggested phases (when prioritized)

1. **Egress-only client** — gRPC client from Panda to selected upstreams (Unary + server-streaming), behind config flags; keep ingress HTTP.
2. **Ingress gRPC** — `tonic` (or equivalent) service for a narrow API surface (e.g. `ChatStream`), with the same policy pipeline as HTTP (JWT, TPM, MCP hooks TBD).
3. **Observability parity** — Trace propagation (W3C + gRPC metadata), metric labels, and readiness rules for gRPC servers.

## Dependencies

- Clear **API contract** (protobuf) versioned beside `panda-config`.
- **Backpressure** and **max message size** limits aligned with existing body limits.

## See also

- [`high_level_design.md`](./high_level_design.md) — protocol completeness theme  
- [`enterprise_track.md`](./enterprise_track.md) — positioning  

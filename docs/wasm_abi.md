# Panda Wasm ABI v0

This document is the canonical host/guest ABI contract for Panda's Wasm plugins.

## Versioning

- Host ABI version constant: `PANDA_WASM_ABI_VERSION = 0`
- Guest **must** export:
  - `panda_abi_version() -> i32`
- If the exported ABI version does not match host version, plugin load fails.

## Guest Exports

- **Required**
  - `panda_abi_version() -> i32`
- **Optional (request headers hook)**
  - `panda_on_request() -> i32`
- **Optional (request body hook)**
  - `panda_on_request_body(ptr: i32, len: i32) -> i32`

If a hook export does not exist, host treats that hook as a no-op for that plugin.

## Host Imports

- `panda_set_header(name_ptr: i32, name_len: i32, value_ptr: i32, value_len: i32)`
  - Guest requests appending one request header.
  - Host validates header token/value before forwarding.
- `panda_set_body(ptr: i32, len: i32)`
  - Guest requests replacing buffered request body.
  - Host enforces output-size limit configured by `plugins.max_request_body_bytes`.

Pointers and lengths reference guest linear memory (`memory` export).

## Return Codes (Policy Semantics)

For `panda_on_request*` exports:

- `0`: allow
- `1`: reject (policy denied)
- `2`: reject (malformed request)
- any other non-zero: reject (plugin-specific code)

Host behavior:

- `plugins.fail_closed = true`
  - policy reject -> HTTP `403`
  - runtime/trap/timeout/join failure -> HTTP `502`
- `plugins.fail_closed = false`
  - host logs and continues request path (fail-open)

Runtime failure reasons are classified into structured buckets for logs/metrics:

- `Trap`
- `MemoryViolation`
- `InvalidInput`
- `HostCallFailure`
- `Internal`

## Safety Limits

- `plugins.execution_timeout_ms`: best-effort timeout per hook call.
- `plugins.max_request_body_bytes`:
  - body hook only runs when request `Content-Length` is present and <= this value
  - body replacement from plugins must also be <= this value
- `plugins.hot_reload`: if enabled, Panda polls plugin directory and atomically swaps in a newly loaded runtime when files change.
- `plugins.reload_interval_ms`: poll interval used by hot-reload scans.

## Memory Rules

- Guest must export `memory`.
- Host writes request body bytes at offset `0` before calling `panda_on_request_body`.
- Guest must only pass in-bounds pointers/lengths to host imports.
- Host treats out-of-bounds/invalid UTF-8 inputs as runtime failures.

## Compatibility Rules

- v0 is additive by optional exports/imports where possible.
- Any signature change requires ABI version bump.
- New return codes may be added; existing code meanings must remain stable within v0.

# Panda plugin ABI — Go (TinyGo)

Rust guests use the `panda_pdk` crate; a minimal sample lives beside this tree at [`../rust/pii_mini`](../rust/pii_mini). For Go, compile with **[TinyGo](https://tinygo.org/)** to **WebAssembly** so exports match the same host imports as `panda-wasm`.

## ABI v1

| Export | Signature | Notes |
|--------|-----------|--------|
| `panda_abi_version` | `() -> i32` | Must return `1` |
| `panda_on_request` | `() -> i32` | Optional |
| `panda_on_request_body` | `(i32 ptr, i32 len) -> i32` | Optional |
| `panda_on_response_chunk` | `(i32 ptr, i32 len) -> i32` | Optional |

Imports (module `env`): `panda_set_header`, `panda_set_body`, `panda_set_response_chunk` (same `(ptr,len)` i32 layout as Rust).

Return codes: `0` allow, `1` policy denied, `2` malformed.

## Build

```bash
cd crates/panda-pdk/go/examples/pii_mask
tinygo build -o pii_mask.wasm -target=wasm -scheduler=none -gc=leaking .
```

Point Panda `plugins` config at `pii_mask.wasm`. If the host rejects the module, ensure TinyGo emits the `env` imports with the exact names above (see `panda/panda.go`).

## Package layout

- `panda/panda.go` — thin `//go:wasmimport` wrappers + helpers
- `examples/pii_mask/` — minimal body redaction sample

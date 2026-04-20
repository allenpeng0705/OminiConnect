# TinyGo Plugin Sample (Panda ABI v1)

This sample mirrors the Rust sample plugin but uses TinyGo.

## What it does

- Exports `panda_abi_version` (returns `1`)
- Exports `panda_on_request` (adds `x-panda-plugin: tinygo-sample`)
- Exports `panda_on_request_body` (optional; currently no-op allow)
- Exports `panda_on_response_chunk` (optional; currently no-op allow)
- Imports `panda_set_header`, `panda_set_body`, and `panda_set_response_chunk` from host

## Build

Requirements:

- TinyGo installed
- A TinyGo target that produces a core Wasm module with exported memory

Example command (adjust for your local TinyGo target support):

`tinygo build -o tinygo_plugin.wasm -target=wasi ./`

## Load in Panda

1. Copy generated `tinygo_plugin.wasm` into your plugin directory.
2. Configure:

```yaml
plugins:
  directory: "./plugins"
```

3. Start Panda and verify log shows plugin loaded.

## Notes

- Depending on TinyGo target/toolchain version, you may need to tune build flags to avoid unsupported imports.
- This sample is a skeleton for ABI parity and rollout testing.
- A second TinyGo plugin with a **real policy** (digit-run / PAN guard) lives in `examples/tinygo-plugin-pci-guard/`.

## Validation Script

Use the repository script for a reproducible build + validation pass:

`./scripts/validate_tinygo_plugin.sh`

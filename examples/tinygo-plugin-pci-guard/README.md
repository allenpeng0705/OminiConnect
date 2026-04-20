# TinyGo plugin: PCI-style digit-run guard

Blocks request bodies that contain **13+ consecutive ASCII digits**, a cheap guardrail against pasting primary account numbers (PANs) into chat / tool payloads before they reach an LLM or backend.

- **Hook**: `panda_on_request_body`
- **Policy**: return `1` (policy denied) when a run is found; sets `x-panda-pci-digit-block: 1`
- **`panda_on_request`**: sets `x-panda-wasm-plugin: pci-digit-guard` for observability

This is intentionally simple (no Luhn, no ISO country rules) to stay tiny in Wasm. Expect false positives on large integers in JSON.

## Build

```bash
tinygo build -o pci_guard.wasm -target=wasi ./
```

Copy `pci_guard.wasm` into the directory configured as `plugins.directory` in `panda.yaml`.

## CI

`bash ../../scripts/validate_tinygo_pci_plugin.sh` (from repo root).

# Examples

| Directory | Description |
|-----------|-------------|
| [`mcp_stdio_minimal/`](./mcp_stdio_minimal/) | Minimal Python MCP server over stdio (`ping`, `echo_message`) for local Panda MCP testing |
| [`tinygo-plugin/`](./tinygo-plugin/) | TinyGo Wasm plugin skeleton (ABI parity with `crates/wasm-plugin-sample`) |
| [`tinygo-plugin-pci-guard/`](./tinygo-plugin-pci-guard/) | TinyGo plugin: block request bodies with long digit runs (PAN-style guardrail) |

Validation scripts (from repo root): `./scripts/validate_tinygo_plugin.sh`, `./scripts/validate_tinygo_pci_plugin.sh`.

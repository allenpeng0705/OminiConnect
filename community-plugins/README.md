# Panda Community Plugin Hub

Third-party and example **Wasm** plugins built with the [`panda-pdk`](../crates/panda-pdk) (see [`docs/wasm_abi.md`](../docs/wasm_abi.md)).

## Important limitation

Guests run in a **sandboxed** Wasm module: they can use the PDK to append request headers, replace buffered bodies, and rewrite streaming response chunks. They **cannot** open TCP connections, so a plugin cannot call Slack (or any HTTP API) directly. Patterns:

1. **Flag + observability** — Set headers such as `x-panda-slack-notify`; your log shipper (Vector, Fluent Bit, Datadog, etc.) or a tiny sidecar forwards to Slack.
2. **Edge / control plane** — An upstream API gateway reads forwarded headers and triggers workflows.

## Plugins in this folder

| Directory | Purpose |
|-----------|---------|
| [`slack-notifier-wasm`](slack-notifier-wasm/) | Marks large / high-`max_tokens` chat requests with headers for downstream alerting (optional Python stdin relay for Slack webhooks). |
| [`pii-masking-i18n`](pii-masking-i18n/) | Body hook: masks emails, China mobile-style digit runs, and Spanish DNI/NIE-style patterns in UTF-8 bodies. |

## Build (any plugin)

From the repository root (Rust `wasm32` target required once: `rustup target add wasm32-unknown-unknown`):

```bash
cargo build --manifest-path community-plugins/<plugin>/Cargo.toml \
  --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/<crate_name>.wasm /path/to/panda/plugins/
```

Point `panda.yaml` at the directory:

```yaml
plugins:
  directory: "/path/to/panda/plugins"
```

Use `plugins.fail_closed: true` only after you trust a plugin in staging.

## Contributing

Open a PR that adds a subdirectory with `README.md`, `Cargo.toml`, `src/lib.rs`, and a one-line entry in this file. Plugins should be **MIT OR Apache-2.0** (or compatible) to match the main repo.

For heavier integrations (databases, OAuth), prefer documenting them in README and keeping the Wasm surface small and testable.

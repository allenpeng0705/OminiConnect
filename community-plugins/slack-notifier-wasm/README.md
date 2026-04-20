# Slack notifier (Wasm flag + optional relay)

## What it does

- **`panda_on_request`** — Sets `x-panda-plugin: community-slack-notify`.
- **`panda_on_request_body`** — For buffered chat/completions-style JSON bodies:
  - Flags when the body is **≥ 96 KiB**, or
  - When a naive scan finds **`max_tokens` ≥ 16384**.

Headers set when flagged:

| Header | Example |
|--------|---------|
| `x-panda-slack-notify` | `high-cost` |
| `x-panda-slack-reason` | `large_body` or `high_max_tokens` |
| `x-panda-slack-body-bytes` | decimal size |

Downstream systems (proxy access logs, Vector, Loki, etc.) can match these and call Slack.

## Why not Slack inside Wasm?

The Panda Wasm ABI does not expose network imports; guests cannot POST to `hooks.slack.com`. Use a relay or log pipeline.

## Optional: stdin relay (demo)

`relay/slack_stdin_relay.py` reads **stdin** (e.g. piped `docker compose logs -f panda`) and posts to a Slack Incoming Webhook when a line mentions the notify headers.

```bash
export SLACK_WEBHOOK_URL='https://hooks.slack.com/services/...'
docker compose -f deploy/mcp-starters/docker-compose.yml logs -f panda 2>&1 \
  | python3 community-plugins/slack-notifier-wasm/relay/slack_stdin_relay.py
```

Tune thresholds by editing the constants in `src/lib.rs` and rebuilding.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --manifest-path community-plugins/slack-notifier-wasm/Cargo.toml \
  --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/wasm_plugin_slack_notify.wasm "$PANDA_PLUGINS_DIR/"
```

Register `plugins.directory` in `panda.yaml` to that folder.

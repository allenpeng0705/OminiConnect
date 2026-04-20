# PII masking (i18n) — Wasm body hook

Replaces common **ASCII-shaped** sensitive tokens in buffered request bodies:

| Pattern | Replacement | Notes |
|---------|-------------|--------|
| Email `local@domain.tld` | `[REDACTED_EMAIL]` | Pragmatic ASCII email heuristic. |
| China mobile (11 digits, `1[3-9]…`) | `[REDACTED_CN_MOBILE]` | Digits must appear as ASCII in JSON/text. |
| Spanish DNI / NIE | `[REDACTED_ES_ID]` | `12345678A` or `X1234567L` style; can false-positive on other data. |

## Limits

- **Not** a compliance guarantee: add your own DLP, classification, and legal review.
- **Unicode** emails (IDN) and non-ASCII local parts are **not** handled.
- Spanish ID detection can match unrelated `NNNNNNNNL` tokens in code or hashes.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --manifest-path community-plugins/pii-masking-i18n/Cargo.toml \
  --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/wasm_plugin_pii_masking_i18n.wasm "$PANDA_PLUGINS_DIR/"
```

Enable `plugins.directory` in `panda.yaml`. For production, consider combining with Panda’s built-in `pii` YAML scrubber for regex control.

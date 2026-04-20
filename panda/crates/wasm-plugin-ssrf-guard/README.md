# `wasm-plugin-ssrf-guard` (Rust)

Wasm guest for Panda that **rejects** request bodies containing URL patterns commonly used in **SSRF** or **metadata exfiltration** (private IPs, localhost, `file://`, GCP metadata host name, etc.).

Matching is **ASCII case-insensitive** on substrings (good enough for `http://` / `https://` and hostnames).

## Build

```bash
cargo build -p wasm-plugin-ssrf-guard --target wasm32-unknown-unknown --release
```

Install the [wasm32 target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html) if needed:

```bash
rustup target add wasm32-unknown-unknown
```

Copy `target/wasm32-unknown-unknown/release/wasm_plugin_ssrf_guard.wasm` into your Panda `plugins.directory`.

## Headers

- On every request (headers hook): `x-panda-wasm-plugin: ssrf-guard`
- On block (body hook): `x-panda-ssrf-block: 1` (then policy denied)

## Limits

This is a **substring** guard, not a full URL parser. Tune `BLOCKED` for your threat model.

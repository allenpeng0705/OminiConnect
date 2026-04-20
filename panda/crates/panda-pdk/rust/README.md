# Panda plugin ABI — Rust

The [`panda_pdk`](../../src/lib.rs) crate (same directory as this `rust/` folder’s parent) is the Rust PDK. This folder holds **example guests** that compile to `wasm32-unknown-unknown` as `cdylib`s.

## Example: `pii_mini/`

Small request-body redactor (Stripe-style prefixes + `password`). Same idea as [`../go/examples/pii_mask`](../go/examples/pii_mask/main.go).

From the **repository root**:

```bash
cargo build -p wasm-plugin-pii-mini --target wasm32-unknown-unknown --release
```

Artifact: `target/wasm32-unknown-unknown/release/wasm_plugin_pii_mini.wasm` (exact filename may vary by platform; look under `target/.../release/*.wasm`).

Point Panda’s plugin config at that `.wasm` file.

## Other Rust samples

- [`wasm-plugin-sample`](../../wasm-plugin-sample/) — headers, body, and response-chunk hooks (workspace member).
- [`wasm-plugin-ssrf-guard`](../../wasm-plugin-ssrf-guard/) — URL guard example.

Those live next to `panda-pdk` in `crates/` for historical workspace layout; **`rust/pii_mini` is the canonical “PDK mini” Rust guest** colocated with the Go example under `panda-pdk/`.

# Contributing to Panda

Thank you for helping improve Panda. This document covers a **Rust development setup** and how to **build and test Wasm plugins**. For architecture and deployment, see `README.md` and `docs/`.

## Rust development environment

### Prerequisites

- **Rust toolchain** via [rustup](https://rustup.rs/) (stable channel is enough; the workspace uses Rust 2021).
- **Git** and a normal Unix-style shell for scripts (`bash`).

Optional but useful:

- **`wasm-tools`** — validates Wasm modules after build (`cargo install wasm-tools`). Used by the TinyGo validation scripts.
- **Redis** — only if you run tests or features that use TPM or semantic cache against a real backend (many unit tests use in-memory stubs).

### Clone and build

```bash
git clone <your-fork-or-upstream-url>
cd panda
cargo build --workspace
```

### Run the gateway locally

```bash
cp panda.example.yaml panda.yaml
# Edit upstream / listen in panda.yaml, then:
cargo run -p panda-server -- panda.yaml
```

Use `cargo run -p panda-server -- --help` for CLI flags and common environment variables.

### Run tests

```bash
# Full workspace (compiles all members, including Wasm-related crates)
cargo test --workspace
```

To focus on the core proxy and config:

```bash
cargo test -p panda-proxy -p panda-config -p panda-wasm -p panda-server
```

Some integration tests spawn subprocesses or use the filesystem; run them from the **repository root** so relative paths resolve correctly.

### Formatting and lints (before a PR)

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
```

Fix new warnings your change introduces; matching the existing style of nearby code is enough.

---

## Wasm plugins

Panda loads **guest** `.wasm` modules from a directory configured as `plugins.directory` in `panda.yaml` (see `panda.example.yaml`). Guests implement ABI **v1** exports such as `panda_abi_version`, `panda_on_request`, `panda_on_request_body`, and optionally `panda_on_response_chunk`. The host runtime lives in `crates/panda-wasm`; plugin authors should build against **`crates/panda-pdk`**.

### 1. Host-side tests (no `.wasm` file required)

The `panda-wasm` crate includes tests that compile small Wasm snippets (WAT) and exercise loading, hooks, and error paths:

```bash
cargo test -p panda-wasm
```

Run this when you change the linker, ABI, or plugin runtime.

### 2. Rust guest: build the sample plugin

Add the WebAssembly target once:

```bash
rustup target add wasm32-unknown-unknown
```

Build the minimal sample (produces a `cdylib`):

```bash
cargo build -p wasm-plugin-sample --target wasm32-unknown-unknown --release
```

The artifact path is:

`target/wasm32-unknown-unknown/release/wasm_plugin_sample.wasm`

Optional sanity check:

```bash
wasm-tools validate target/wasm32-unknown-unknown/release/wasm_plugin_sample.wasm
```

Copy that file into your Panda plugin directory (e.g. `./plugins/`) and enable `plugins.directory` in `panda.yaml`. Restart the gateway and confirm logs show the module loaded. For policy behavior and return codes (`0` allow, `1`/`2` reject, etc.), see comments in `panda.example.yaml` under the `plugins:` block.

A second Rust example with a real policy is **`crates/wasm-plugin-ssrf-guard`** — see its `README.md` for the same `wasm32-unknown-unknown` build command and output name.

### 3. TinyGo guests (optional)

If you use TinyGo instead of Rust, use the checked-in examples and scripts:

```bash
./scripts/validate_tinygo_plugin.sh
./scripts/validate_tinygo_pci_plugin.sh
```

Those build into `target/tinygo/` and run `wasm-tools validate`. Sources live under `examples/tinygo-plugin/` and `examples/tinygo-plugin-pci-guard/`.

### 4. End-to-end check with a running gateway

1. Build a guest (Rust or TinyGo) as above.
2. Point `plugins.directory` at a folder containing your `.wasm` file(s).
3. Start `panda-server` with a valid `panda.yaml`.
4. Inspect **`GET /plugins/status`** (and metrics) if you have ops auth configured; otherwise watch startup logs for load errors.

For hook timeouts, pool size, and fail-open vs fail-closed behavior, see `plugins:` in `panda.example.yaml` and `README.md` (Wasm Plugin Runtime Notes).

---

## Reliability benchmarks, chaos smoke, and dependency audit

- **Proxy latency (hyperfine):** `benchmarks/run_proxy_overhead_hyperfine.sh` — compares direct mock upstream vs through Panda on loopback (see `benchmarks/README.md`).
- **Chaos smoke (Redis + MCP during SSE):** `scripts/chaos_monkey_streaming.sh` — asserts the gateway process stays up while Redis and MCP stdio peers are disrupted (`CHAOS_MCP_FAIL_OPEN` toggles MCP fail-open).
- **RustSec / `cargo audit`:** `docs/dependency_audit.md` and `scripts/run_cargo_audit.sh`.

---

## Questions

Open a discussion or issue with a short reproducer (config snippet, command line, and expected vs actual behavior). For security-sensitive reports, use the contact path your maintainers publish for the repository.

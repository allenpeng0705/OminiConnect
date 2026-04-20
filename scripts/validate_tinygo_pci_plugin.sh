#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SAMPLE_DIR="$ROOT_DIR/examples/tinygo-plugin-pci-guard"
OUT_DIR="$ROOT_DIR/target/tinygo"
OUT_WASM="$OUT_DIR/pci_guard.wasm"

if ! command -v tinygo >/dev/null 2>&1; then
  echo "tinygo is required (https://tinygo.org/getting-started/install/)"
  exit 1
fi

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "wasm-tools is required (cargo install wasm-tools) for wasm validate"
  exit 1
fi

mkdir -p "$OUT_DIR"
echo "Building TinyGo PCI guard plugin..."
(cd "$SAMPLE_DIR" && tinygo build -o "$OUT_WASM" -target=wasi ./)
wasm-tools validate "$OUT_WASM"
echo "OK: TinyGo PCI guard built and validated -> $OUT_WASM"

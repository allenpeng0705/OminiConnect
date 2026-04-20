#!/usr/bin/env bash
# Run RustSec advisory check. Install: cargo install cargo-audit
# CI-friendly: exits 1 if vulnerabilities are reported (adjust with cargo audit --allow).
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

if ! command -v cargo-audit >/dev/null 2>&1; then
  echo "cargo-audit not found. Install with: cargo install cargo-audit"
  exit 1
fi

echo "Running cargo audit in ${ROOT_DIR}..."
cargo audit "$@"

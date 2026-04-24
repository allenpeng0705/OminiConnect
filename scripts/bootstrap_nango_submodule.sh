#!/usr/bin/env bash
# Ensure the Nango submodule is present (OminiConnect bundles Nango as a submodule + optional local patch).
# Prefer: git clone --recurse-submodules <repo-url>
# Or from repo root: ./scripts/ensure_nango.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "${ROOT}/scripts/ensure_nango.sh"

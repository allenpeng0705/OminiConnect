#!/usr/bin/env bash
# Save OminiConnect-local Nango edits into scripts/nango_omini_connect_local.patch so you can
# commit them in the parent repo (instead of leaving "modified content" inside the submodule).
#
# Usage (repo root):
#   ./scripts/snapshot_nango_patch.sh           # only write the patch file
#   ./scripts/snapshot_nango_patch.sh --reset # write patch, then reset submodule to upstream
#                                             # tree and re-apply patch via ensure_nango.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO="${ROOT}/third_party/nango"
PATCH="${ROOT}/scripts/nango_omini_connect_local.patch"

if ! git -C "${NANGO}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "Not a git checkout: ${NANGO}. Run ./scripts/ensure_nango.sh first."
  exit 1
fi

mkdir -p "$(dirname "${PATCH}")"
git -C "${NANGO}" diff >"${PATCH}"
echo "Wrote ${PATCH}"

if [[ "${1:-}" == "--reset" ]]; then
  echo "Resetting ${NANGO} to submodule HEAD (discarding uncommitted changes there)..."
  git -C "${NANGO}" checkout -- .
  echo "Re-applying patch with ensure_nango.sh ..."
  NANGO_FORCE_BUILD=1 "${ROOT}/scripts/ensure_nango.sh"
  echo "Done. The submodule should show no local diff; commit ${PATCH} in OminiConnect."
fi

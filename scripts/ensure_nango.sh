#!/usr/bin/env bash
# One entry point for Nango inside OminiConnect: submodule checkout, local patches, npm install, TypeScript build.
# Nango stays a submodule (not copied into Git as a giant tree); this script makes the workflow feel "combined".
#
# Usage (repo root):
#   ./scripts/ensure_nango.sh
#
# Environment:
#   NANGO_SKIP_PATCH=1     — do not apply scripts/nango_omini_connect_local.patch
#   NANGO_SKIP_INSTALL=1   — skip npm ci (use when you only want submodule + patch)
#   NANGO_SKIP_BUILD=1     — skip npm run ts-build
#   NANGO_FORCE_BUILD=1  — always run ts-build even if dist already exists

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO="${ROOT}/third_party/nango"
PATCH="${ROOT}/scripts/nango_omini_connect_local.patch"
SERVER_DIST="${NANGO}/packages/server/dist/server.js"

cd "${ROOT}"

if [[ ! -f "${ROOT}/.gitmodules" ]] || ! grep -q 'third_party/nango' "${ROOT}/.gitmodules" 2>/dev/null; then
  echo "Missing submodule config for third_party/nango (.gitmodules)."
  exit 1
fi

need_submodule=0
if [[ ! -f "${NANGO}/package.json" ]]; then
  need_submodule=1
elif ! git -C "${NANGO}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  need_submodule=1
fi

if [[ "${need_submodule}" -eq 1 ]]; then
  echo "Fetching Nango submodule (third_party/nango)..."
  git submodule update --init --recursive third_party/nango
fi

if [[ ! -f "${NANGO}/package.json" ]]; then
  echo "Submodule checkout failed: ${NANGO}/package.json still missing."
  exit 1
fi

if [[ "${NANGO_SKIP_PATCH:-0}" != "1" ]] && [[ -f "${PATCH}" ]]; then
  if git -C "${NANGO}" apply --check "${PATCH}" >/dev/null 2>&1; then
    echo "Applying OminiConnect Nango patch (${PATCH})..."
    git -C "${NANGO}" apply "${PATCH}"
  else
    echo "Patch already applied or does not apply cleanly; leaving Nango sources as-is (${PATCH})."
  fi
elif [[ "${NANGO_SKIP_PATCH:-0}" != "1" ]]; then
  echo "No patch file at ${PATCH} (optional)."
fi

if [[ "${NANGO_SKIP_INSTALL:-0}" != "1" ]]; then
  if [[ ! -d "${NANGO}/node_modules" ]]; then
    echo "Running npm ci in Nango (first install can take several minutes)..."
    (cd "${NANGO}" && npm ci)
  else
    echo "Using existing ${NANGO}/node_modules"
  fi
else
  echo "Skipping npm ci (NANGO_SKIP_INSTALL=1)."
fi

if [[ "${NANGO_SKIP_BUILD:-0}" != "1" ]]; then
  if [[ "${NANGO_FORCE_BUILD:-0}" == "1" ]] || [[ ! -f "${SERVER_DIST}" ]]; then
    echo "Running npm run ts-build in Nango..."
    (cd "${NANGO}" && npm run ts-build)
  else
    echo "Skipping ts-build (dist already present). Set NANGO_FORCE_BUILD=1 to rebuild."
  fi
else
  echo "Skipping ts-build (NANGO_SKIP_BUILD=1)."
fi

echo "Nango is ready at ${NANGO}"

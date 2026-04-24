#!/usr/bin/env bash
# Re-clone third_party/nango as a proper Git submodule (e.g. after deleting third_party/nango/.git),
# then re-apply OminiConnect-local patches and restore third_party/nango/.env if you backed it up.
#
# Usage (repo root): ./scripts/repair_nango_submodule.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO="${ROOT}/third_party/nango"
PATCH="${ROOT}/scripts/nango_omini_connect_local.patch"
ENV_BAK="${ROOT}/scripts/.nango_omini_repair_backup/.env"

if [[ ! -f "${PATCH}" ]]; then
  echo "Warning: missing ${PATCH} — continuing without OminiConnect patch (set NANGO_SKIP_PATCH=1 for ensure_nango)."
  export NANGO_SKIP_PATCH=1
fi

if [[ -f "${NANGO}/.env" ]]; then
  mkdir -p "$(dirname "${ENV_BAK}")"
  cp "${NANGO}/.env" "${ENV_BAK}"
  echo "Backed up ${NANGO}/.env -> ${ENV_BAK}"
fi

echo "Removing ${NANGO} ..."
rm -rf "${NANGO}"

cd "${ROOT}"
echo "Cloning submodule third_party/nango ..."
git submodule update --init --recursive third_party/nango

if [[ -f "${ENV_BAK}" ]]; then
  cp "${ENV_BAK}" "${NANGO}/.env"
  echo "Restored ${NANGO}/.env from backup"
fi

echo "Install, patch, and compile via ensure_nango.sh ..."
NANGO_FORCE_BUILD=1 "${ROOT}/scripts/ensure_nango.sh"

echo "Done. Submodule status:"
git submodule status third_party/nango

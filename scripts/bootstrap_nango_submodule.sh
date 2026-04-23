#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO_DIR="${ROOT}/third_party/nango"
NANGO_URL="${NANGO_URL:-https://github.com/NangoHQ/nango.git}"

if [[ -d "${NANGO_DIR}/.git" ]]; then
  echo "Nango already present at ${NANGO_DIR}"
  exit 0
fi

mkdir -p "${ROOT}/third_party"

echo "Adding Nango as git submodule: ${NANGO_DIR}"
cd "${ROOT}"

# Prefer HTTP/1.1 for flaky HTTP2 environments.
GIT_HTTP_VERSION=HTTP/1.1 git submodule add "${NANGO_URL}" third_party/nango

echo "Initializing submodules..."
GIT_HTTP_VERSION=HTTP/1.1 git submodule update --init --recursive

echo "Done."

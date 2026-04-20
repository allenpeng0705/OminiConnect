#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/artifacts/release"
TARGET_TRIPLE="${PANDA_RELEASE_TARGET:-$(rustc -vV | awk '/host:/ {print $2}')}"
TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"
RELEASE_FEATURES="${PANDA_RELEASE_FEATURES:-mimalloc}"

if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${ROOT_DIR}" log -1 --format=%ct)"
  export SOURCE_DATE_EPOCH
fi

mkdir -p "${OUT_DIR}"

echo "Building panda-server (target=${TARGET_TRIPLE}, features=${RELEASE_FEATURES}) with --locked"
(
  cd "${ROOT_DIR}"
  cargo build --locked --release -p panda-server --target "${TARGET_TRIPLE}" --features "${RELEASE_FEATURES}"
)

BIN="${TARGET_DIR}/${TARGET_TRIPLE}/release/panda"
if [[ ! -f "${BIN}" ]]; then
  BIN="${TARGET_DIR}/release/panda"
fi
OUT_BIN="${OUT_DIR}/panda-${TARGET_TRIPLE}"
cp "${BIN}" "${OUT_BIN}"

if command -v shasum >/dev/null 2>&1; then
  shasum -a 256 "${OUT_BIN}" | tee "${OUT_BIN}.sha256"
else
  sha256sum "${OUT_BIN}" | tee "${OUT_BIN}.sha256"
fi

echo "Wrote ${OUT_BIN} and ${OUT_BIN}.sha256"

#!/usr/bin/env bash
# Single local-dev entrypoint: prepare vendored Nango + run Nango and OminiConnect portal.
#
# Usage:
#   ./scripts/dev.sh
#   make dev

set -euo pipefail
# Put each `&` background job in its own process group so one `kill -TERM -- -$pid` tears down
# `npm` → `concurrently` → many `node` children when you press Ctrl+C (otherwise orphans keep ports).
set -m

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO_DIR="${ROOT}/third_party/nango"
PATCH="${ROOT}/scripts/nango_omini_connect_local.patch"
SERVER_DIST="${NANGO_DIR}/packages/server/dist/server.js"

NANGO_APPS_PID=""
PORTAL_PID=""

kill_tree() {
  local root="${1:?}"
  [[ "${root}" =~ ^[0-9]+$ ]] || return 0
  kill -0 "${root}" 2>/dev/null || return 0
  local children c
  children=$(pgrep -P "${root}" 2>/dev/null || true)
  for c in ${children}; do
    kill_tree "${c}"
  done
  kill -TERM "${root}" 2>/dev/null || true
}

kill_tree_force() {
  local root="${1:?}"
  [[ "${root}" =~ ^[0-9]+$ ]] || return 0
  kill -0 "${root}" 2>/dev/null || return 0
  local children c
  children=$(pgrep -P "${root}" 2>/dev/null || true)
  for c in ${children}; do
    kill_tree_force "${c}"
  done
  kill -KILL "${root}" 2>/dev/null || true
}

kill_process_group() {
  local pid="${1:?}"
  kill -0 "${pid}" 2>/dev/null || return 0
  # Negative PID = process group of the job leader (requires `set -m` above).
  if kill -TERM -- "-${pid}" 2>/dev/null; then
    return 0
  fi
  kill_tree "${pid}"
}

cleanup() {
  if [[ -n "${PORTAL_PID}" ]]; then
    kill_process_group "${PORTAL_PID}"
  fi
  if [[ -n "${NANGO_APPS_PID}" ]]; then
    kill_process_group "${NANGO_APPS_PID}"
  fi
  sleep 1
  if [[ -n "${PORTAL_PID}" ]] && kill -0 "${PORTAL_PID}" 2>/dev/null; then
    kill_tree_force "${PORTAL_PID}"
  fi
  if [[ -n "${NANGO_APPS_PID}" ]] && kill -0 "${NANGO_APPS_PID}" 2>/dev/null; then
    kill_tree_force "${NANGO_APPS_PID}"
  fi
}

on_signal() {
  cleanup
  exit 130
}

trap cleanup EXIT
trap on_signal INT TERM

require_cmd() {
  local name="${1:?}"
  if ! command -v "${name}" >/dev/null 2>&1; then
    echo "Missing required command: ${name}"
    exit 1
  fi
}

require_cmd node
require_cmd npm
require_cmd cargo

if [[ ! -f "${ROOT}/.env" ]]; then
  echo "Missing ${ROOT}/.env. Create it from .env.example first."
  exit 1
fi

if [[ ! -f "${NANGO_DIR}/package.json" ]]; then
  echo "Vendored Nango source missing at ${NANGO_DIR}."
  exit 1
fi

prepare_nango() {
  if [[ -f "${PATCH}" ]] && git -C "${NANGO_DIR}" apply --check "${PATCH}" >/dev/null 2>&1; then
    echo "Applying local Nango patch..."
    git -C "${NANGO_DIR}" apply "${PATCH}"
  fi

  if [[ ! -d "${NANGO_DIR}/node_modules" ]]; then
    echo "Installing Nango dependencies (npm ci)..."
    (cd "${NANGO_DIR}" && npm ci)
  fi

  if [[ ! -f "${SERVER_DIST}" ]]; then
    echo "Building Nango TypeScript (npm run ts-build)..."
    (cd "${NANGO_DIR}" && npm run ts-build)
  fi
}

link_nango_env() {
  local env_file="${NANGO_DIR}/.env"
  if [[ -f "${env_file}" && ! -L "${env_file}" ]]; then
    local backup="${env_file}.bak.$(date +%s)"
    echo "Backing up ${env_file} -> ${backup}"
    mv "${env_file}" "${backup}"
  fi
  ln -sfn "../../.env" "${env_file}"
}

prepare_nango
link_nango_env

cd "${NANGO_DIR}"
echo "Starting Nango dev processes..."
npm run dev:watch:web &
NANGO_APPS_PID=$!

cd "${ROOT}"
echo "Waiting for Nango http://localhost:3003/health ..."
for _ in $(seq 1 120); do
  if curl -sf "http://localhost:3003/health" >/dev/null 2>&1; then
    echo "Nango is up."
    break
  fi
  sleep 2
done
if ! curl -sf "http://localhost:3003/health" >/dev/null 2>&1; then
  echo "Nango did not become healthy in time."
  exit 1
fi

set -a
# shellcheck disable=SC1090
source "${ROOT}/.env"
set +a

PORTAL_LISTEN_PORT="${PORTAL_LISTEN_PORT:-9000}"
export PORTAL_LISTEN_PORT

if command -v lsof >/dev/null 2>&1; then
  if lsof -nP -iTCP:"${PORTAL_LISTEN_PORT}" -sTCP:LISTEN >/dev/null 2>&1; then
    echo "Port ${PORTAL_LISTEN_PORT} is already in use (another portal or process)."
    echo "  Show:  lsof -nP -iTCP:${PORTAL_LISTEN_PORT} -sTCP:LISTEN"
    echo "  Stop:  kill \$(lsof -t -iTCP:${PORTAL_LISTEN_PORT} -sTCP:LISTEN)"
    echo "Or pick another port:  PORTAL_LISTEN_PORT=9001 make dev"
    exit 1
  fi
fi

echo "Starting OminiConnect portal on :${PORTAL_LISTEN_PORT} ..."
cargo run -p omini-connect-portal &
PORTAL_PID=$!
# Ctrl+C: INT trap runs cleanup; `wait` may return non-zero — do not trip `set -e` before EXIT trap.
wait "${PORTAL_PID}" || true

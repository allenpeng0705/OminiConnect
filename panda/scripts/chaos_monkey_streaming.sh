#!/usr/bin/env bash
# Chaos-style smoke test: while SSE streaming chat runs, randomly disrupt Redis (TPM) and/or
# MCP stdio child processes. Asserts the Panda process never exits (stays "alive").
#
# Expected behavior (document with your panda.yaml):
#   - TPM + Redis: when Redis dies, Panda falls back to in-memory TPM accounting (see tpm.rs).
#   - MCP: with fail_open=true, tool/MCP errors are logged and requests may still complete;
#          with fail_open=false, MCP failures can surface as upstream errors — Panda must still run.
#
# Requirements: docker, python3, curl, cargo (or prebuilt panda on PATH — see CHAOS_PANDA_BIN).
#
# Usage (from repo root):
#   ./scripts/chaos_monkey_streaming.sh
# Optional:
#   CHAOS_DURATION_SECONDS=45 CHAOS_REDIS_PORT=16379 CHAOS_MCP_FAIL_OPEN=true ./scripts/chaos_monkey_streaming.sh
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REDIS_CONTAINER="${CHAOS_REDIS_CONTAINER:-panda-chaos-redis}"
REDIS_PORT="${CHAOS_REDIS_PORT:-16379}"
PANDA_PORT="${CHAOS_PANDA_PORT:-19280}"
UPSTREAM_PORT="${CHAOS_UPSTREAM_PORT:-19281}"
DURATION="${CHAOS_DURATION_SECONDS:-40}"
STREAM_CHUNKS="${CHAOS_STREAM_CHUNKS:-80}"
MCP_FAIL_OPEN="${CHAOS_MCP_FAIL_OPEN:-true}"
OUT_DIR="${CHAOS_OUTPUT_DIR:-artifacts/chaos}"
MCP_SCRIPT="${ROOT_DIR}/examples/mcp_stdio_minimal/server.py"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/panda-chaos.XXXXXX")"
PAYLOAD="${WORK_DIR}/payload.json"

PANDA_BIN="${CHAOS_PANDA_BIN:-}"

cleanup() {
  if [[ -n "${STREAM_PID:-}" ]]; then kill "${STREAM_PID}" 2>/dev/null || true; wait "${STREAM_PID}" 2>/dev/null || true; fi
  if [[ -n "${PANDA_PID:-}" ]]; then kill "${PANDA_PID}" 2>/dev/null || true; wait "${PANDA_PID}" 2>/dev/null || true; fi
  if [[ -n "${UPSTREAM_PID:-}" ]]; then kill "${UPSTREAM_PID}" 2>/dev/null || true; wait "${UPSTREAM_PID}" 2>/dev/null || true; fi
  docker rm -f "${REDIS_CONTAINER}" >/dev/null 2>&1 || true
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is required for Redis chaos"
  exit 1
fi
if [[ ! -f "${MCP_SCRIPT}" ]]; then
  echo "missing MCP sample at ${MCP_SCRIPT}"
  exit 1
fi

mkdir -p "${OUT_DIR}"
LOG="${OUT_DIR}/chaos_monkey_$(date +%Y%m%d_%H%M%S).log"
exec > >(tee -a "${LOG}") 2>&1

echo "chaos log -> ${LOG}"

echo '{"model":"m","messages":[{"role":"user","content":"stream chaos"}],"stream":true}' > "${PAYLOAD}"

cat > "${WORK_DIR}/upstream_sse.py" <<PY
import json
import os
import time
from http.server import BaseHTTPRequestHandler, HTTPServer

CHUNKS = int(os.environ.get("STREAM_CHUNKS", "80"))

class H(BaseHTTPRequestHandler):
    def do_POST(self):
        n = int(self.headers.get("Content-Length", "0"))
        if n:
            self.rfile.read(n)
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-cache")
        self.end_headers()
        for i in range(CHUNKS):
            chunk = {
                "id": "c",
                "object": "chat.completion.chunk",
                "created": 1,
                "model": "m",
                "choices": [{"index": 0, "delta": {"content": "x"}, "finish_reason": None}],
            }
            line = "data: " + json.dumps(chunk) + "\n\n"
            self.wfile.write(line.encode("utf-8"))
            self.wfile.flush()
            time.sleep(0.03)
        self.wfile.write(b"data: [DONE]\n\n")
        self.wfile.flush()

    def log_message(self, *_a):
        return

HTTPServer(("127.0.0.1", int(os.environ["UPSTREAM_PORT"])), H).serve_forever()
PY

cat > "${WORK_DIR}/panda.yaml" <<EOF
listen: "127.0.0.1:${PANDA_PORT}"
upstream: "http://127.0.0.1:${UPSTREAM_PORT}"
tpm:
  redis_url: "redis://127.0.0.1:${REDIS_PORT}"
  enforce_budget: false
  budget_tokens_per_minute: 1000000
mcp:
  enabled: true
  fail_open: ${MCP_FAIL_OPEN}
  advertise_tools: false
  servers:
    - name: sample
      enabled: true
      command: "python3"
      args: ["${MCP_SCRIPT}"]
EOF

echo "starting redis ${REDIS_CONTAINER} on host port ${REDIS_PORT}"
docker rm -f "${REDIS_CONTAINER}" >/dev/null 2>&1 || true
docker run -d --rm --name "${REDIS_CONTAINER}" -p "${REDIS_PORT}:6379" redis:7-alpine >/dev/null

export UPSTREAM_PORT STREAM_CHUNKS
python3 "${WORK_DIR}/upstream_sse.py" >/dev/null 2>&1 &
UPSTREAM_PID=$!

for _ in $(seq 1 50); do
  if curl -fsS -o /dev/null -X POST "http://127.0.0.1:${UPSTREAM_PORT}/v1/chat/completions" \
    -H "Content-Type: application/json" --data-binary "@${PAYLOAD}" 2>/dev/null; then
    break
  fi
  sleep 0.1
done

echo "starting panda on :${PANDA_PORT}"
(
  cd "${ROOT_DIR}"
  if [[ -z "${PANDA_BIN}" ]]; then
    cargo run -q -p panda-server -- "${WORK_DIR}/panda.yaml"
  else
    "${PANDA_BIN}" "${WORK_DIR}/panda.yaml"
  fi
) >/dev/null 2>&1 &
PANDA_PID=$!

for _ in $(seq 1 100); do
  if curl -fsS "http://127.0.0.1:${PANDA_PORT}/ready" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done

if ! kill -0 "${PANDA_PID}" 2>/dev/null; then
  echo "panda exited before ready"
  exit 1
fi

stream_loop() {
  while kill -0 "${PANDA_PID}" 2>/dev/null; do
    curl -sS -m 60 -N -o /dev/null \
      -X POST "http://127.0.0.1:${PANDA_PORT}/v1/chat/completions" \
      -H "Content-Type: application/json" \
      --data-binary "@${PAYLOAD}" || true
    sleep 0.2
  done
}
stream_loop &
STREAM_PID=$!

chaos_loop() {
  local end=$(( $(date +%s) + DURATION ))
  local flip=0
  while [[ "$(date +%s)" -lt "${end}" ]]; do
    sleep $((2 + RANDOM % 4))
    case $((RANDOM % 3)) in
      0)
        echo "chaos: stopping redis container"
        docker stop "${REDIS_CONTAINER}" >/dev/null 2>&1 || true
        flip=1
        ;;
      1)
        echo "chaos: killing MCP python children (panda respawns on next use, or connection may fail until restart)"
        pkill -f "examples/mcp_stdio_minimal/server.py" 2>/dev/null || true
        ;;
      2)
        if [[ "${flip}" -eq 1 ]]; then
          echo "chaos: restarting redis"
          docker rm -f "${REDIS_CONTAINER}" >/dev/null 2>&1 || true
          docker run -d --rm --name "${REDIS_CONTAINER}" -p "${REDIS_PORT}:6379" redis:7-alpine >/dev/null
          flip=0
        fi
        ;;
    esac
    if ! kill -0 "${PANDA_PID}" 2>/dev/null; then
      echo "FATAL: panda process died during chaos"
      exit 2
    fi
    code="$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:${PANDA_PORT}/health" || echo "000")"
    echo "tick health=${code}"
  done
}

echo "chaos running for ${DURATION}s (panda pid=${PANDA_PID})"
chaos_loop

if ! kill -0 "${PANDA_PID}" 2>/dev/null; then
  echo "FAIL: panda not running after chaos window"
  exit 1
fi

echo "OK: panda still alive after chaos; see ${LOG}"
echo "Review log for transient 5xx or stream errors — acceptable under disruption if fail_open matches config."

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REDIS_CONTAINER="${TPM_SOAK_REDIS_CONTAINER:-panda-tpm-soak-redis}"
PANDA_PORT="${TPM_SOAK_PANDA_PORT:-18082}"
UPSTREAM_PORT="${TPM_SOAK_UPSTREAM_PORT:-18083}"
DURATION_SECONDS="${TPM_SOAK_DURATION_SECONDS:-60}"
OUT_DIR="${TPM_SOAK_OUTPUT_DIR:-artifacts/tpm-soak}"
OUT_FILE="${OUT_DIR}/tpm_redis_failover_$(date +%Y%m%d_%H%M%S).csv"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/panda-tpm-soak.XXXXXX")"

cleanup() {
  if [[ -n "${PANDA_PID:-}" ]]; then kill "${PANDA_PID}" 2>/dev/null || true; wait "${PANDA_PID}" 2>/dev/null || true; fi
  if [[ -n "${UPSTREAM_PID:-}" ]]; then kill "${UPSTREAM_PID}" 2>/dev/null || true; wait "${UPSTREAM_PID}" 2>/dev/null || true; fi
  docker rm -f "${REDIS_CONTAINER}" >/dev/null 2>&1 || true
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is required for redis failover soak"
  exit 1
fi

mkdir -p "${OUT_DIR}"
echo "epoch_s,ready_status,chat_status,note" > "${OUT_FILE}"

cat > "${WORK_DIR}/upstream.py" <<'PY'
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        _ = self.rfile.read(int(self.headers.get("Content-Length", "0")))
        body = {"id":"x","object":"chat.completion","created":1,"model":"m","choices":[{"index":0,"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}]}
        raw = json.dumps(body).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)
    def log_message(self, *_args): return
HTTPServer(("127.0.0.1", int(__import__("os").environ["UPSTREAM_PORT"])), Handler).serve_forever()
PY

cat > "${WORK_DIR}/panda.yaml" <<EOF
listen: "127.0.0.1:${PANDA_PORT}"
upstream: "http://127.0.0.1:${UPSTREAM_PORT}"
tpm:
  enforce_budget: true
  budget_tokens_per_minute: 100000
  redis_url: "redis://127.0.0.1:6379"
EOF

echo "Starting redis container ${REDIS_CONTAINER}"
docker run -d --rm --name "${REDIS_CONTAINER}" -p 6379:6379 redis:7-alpine >/dev/null

UPSTREAM_PORT="${UPSTREAM_PORT}" python3 "${WORK_DIR}/upstream.py" >/dev/null 2>&1 &
UPSTREAM_PID=$!

(
  cd "${ROOT_DIR}"
  cargo run -q -p panda-server -- "${WORK_DIR}/panda.yaml"
) >/dev/null 2>&1 &
PANDA_PID=$!

for _ in $(seq 1 40); do
  if curl -fsS "http://127.0.0.1:${PANDA_PORT}/ready" >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done

end_ts=$(( $(date +%s) + DURATION_SECONDS ))
down_injected=0
up_restarted=0

while [[ "$(date +%s)" -lt "${end_ts}" ]]; do
  now="$(date +%s)"
  ready_status="$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:${PANDA_PORT}/ready" || true)"
  chat_status="$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:${PANDA_PORT}/v1/chat/completions" -H "Content-Type: application/json" -d '{"model":"m","messages":[{"role":"user","content":"ping"}]}' || true)"
  note="steady"
  if [[ "${down_injected}" -eq 0 && $((end_ts - now)) -lt $((DURATION_SECONDS - 10)) ]]; then
    docker stop "${REDIS_CONTAINER}" >/dev/null || true
    down_injected=1
    note="redis_down"
  elif [[ "${down_injected}" -eq 1 && "${up_restarted}" -eq 0 && $((end_ts - now)) -lt $((DURATION_SECONDS - 30)) ]]; then
    docker run -d --rm --name "${REDIS_CONTAINER}" -p 6379:6379 redis:7-alpine >/dev/null
    up_restarted=1
    note="redis_up"
  fi
  echo "${now},${ready_status},${chat_status},${note}" >> "${OUT_FILE}"
  sleep 1
done

python3 - <<PY
import csv, pathlib
rows = list(csv.DictReader(pathlib.Path("${OUT_FILE}").open()))
ready_bad = sum(1 for r in rows if r["ready_status"] != "200")
chat_bad = sum(1 for r in rows if not r["chat_status"].startswith("2"))
print(f"samples={len(rows)} ready_non_200={ready_bad} chat_non_2xx={chat_bad} output=${OUT_FILE}")
if ready_bad > 0 or chat_bad > 0:
    raise SystemExit(1)
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/panda-otlp-smoke.XXXXXX")"
OTLP_PORT="${OTLP_SMOKE_OTLP_PORT:-14318}"
UPSTREAM_PORT="${OTLP_SMOKE_UPSTREAM_PORT:-18080}"
PANDA_PORT="${OTLP_SMOKE_PANDA_PORT:-18081}"
COUNT_FILE="${WORK_DIR}/otlp_count.txt"

cleanup() {
  if [[ -n "${PANDA_PID:-}" ]]; then kill "${PANDA_PID}" 2>/dev/null || true; wait "${PANDA_PID}" 2>/dev/null || true; fi
  if [[ -n "${UPSTREAM_PID:-}" ]]; then kill "${UPSTREAM_PID}" 2>/dev/null || true; wait "${UPSTREAM_PID}" 2>/dev/null || true; fi
  if [[ -n "${OTLP_PID:-}" ]]; then kill "${OTLP_PID}" 2>/dev/null || true; wait "${OTLP_PID}" 2>/dev/null || true; fi
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

cat > "${WORK_DIR}/collector.py" <<'PY'
import pathlib
from http.server import BaseHTTPRequestHandler, HTTPServer

count_file = pathlib.Path(__import__("os").environ["COUNT_FILE"])
count_file.write_text("0\n", encoding="utf-8")

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        _ = self.rfile.read(int(self.headers.get("Content-Length", "0")))
        if self.path == "/v1/traces":
            current = int(count_file.read_text(encoding="utf-8").strip() or "0")
            count_file.write_text(f"{current + 1}\n", encoding="utf-8")
        self.send_response(200)
        self.end_headers()

    def log_message(self, *_args):
        return

HTTPServer(("127.0.0.1", int(__import__("os").environ["OTLP_PORT"])), Handler).serve_forever()
PY

cat > "${WORK_DIR}/upstream.py" <<'PY'
from http.server import BaseHTTPRequestHandler, HTTPServer
import json

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        _ = self.rfile.read(int(self.headers.get("Content-Length", "0")))
        body = {
            "id": "chatcmpl-otlp-smoke",
            "object": "chat.completion",
            "created": 1,
            "model": "smoke-model",
            "choices": [{"index": 0, "message": {"role": "assistant", "content": "ok"}, "finish_reason": "stop"}],
            "usage": {"prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2},
        }
        raw = json.dumps(body).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def log_message(self, *_args):
        return

HTTPServer(("127.0.0.1", int(__import__("os").environ["UPSTREAM_PORT"])), Handler).serve_forever()
PY

cat > "${WORK_DIR}/panda.yaml" <<EOF
listen: "127.0.0.1:${PANDA_PORT}"
upstream: "http://127.0.0.1:${UPSTREAM_PORT}"
EOF

COUNT_FILE="${COUNT_FILE}" OTLP_PORT="${OTLP_PORT}" python3 "${WORK_DIR}/collector.py" >/dev/null 2>&1 &
OTLP_PID=$!
UPSTREAM_PORT="${UPSTREAM_PORT}" python3 "${WORK_DIR}/upstream.py" >/dev/null 2>&1 &
UPSTREAM_PID=$!

OTEL_EXPORTER_OTLP_ENDPOINT="http://127.0.0.1:${OTLP_PORT}/v1/traces" \
PANDA_OTEL_SERVICE_NAME="${PANDA_OTEL_SERVICE_NAME:-panda-smoke}" \
cargo run -q -p panda-server -- "${WORK_DIR}/panda.yaml" >/dev/null 2>&1 &
PANDA_PID=$!

for _ in $(seq 1 40); do
  if curl -fsS "http://127.0.0.1:${PANDA_PORT}/health" >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done

curl -fsS "http://127.0.0.1:${PANDA_PORT}/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d '{"model":"smoke-model","messages":[{"role":"user","content":"ping"}]}' >/dev/null

# Trigger graceful shutdown so tracer provider flushes on process exit.
kill -TERM "${PANDA_PID}"
wait "${PANDA_PID}" || true
unset PANDA_PID

# Batch exporter may flush asynchronously; allow a short grace period.
sleep 2
count="$(tr -d '[:space:]' < "${COUNT_FILE}")"
if [[ "${count}" =~ ^[0-9]+$ ]] && (( count > 0 )); then
  echo "OTLP smoke OK: exported ${count} trace request(s)"
else
  echo "OTLP smoke FAILED: no /v1/traces POST observed"
  exit 1
fi

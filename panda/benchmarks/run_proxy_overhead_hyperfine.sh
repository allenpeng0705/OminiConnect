#!/usr/bin/env bash
# Compare latency: direct to mock upstream vs same path through Panda (loopback).
# Requires: hyperfine, python3, curl, cargo-built panda-server binary.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
UPSTREAM_PORT="${BENCHMARK_UPSTREAM_PORT:-19223}"
PANDA_PORT="${BENCHMARK_PANDA_PORT:-19222}"
RUNS="${HYPERFINE_RUNS:-30}"
WARMUP="${HYPERFINE_WARMUP:-5}"
PAYLOAD="${ROOT_DIR}/benchmarks/payload_chat_min.json"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/panda-bench.XXXXXX")"

if ! command -v hyperfine >/dev/null 2>&1; then
  echo "hyperfine is required: https://github.com/sharkdp/hyperfine"
  exit 1
fi

cleanup() {
  if [[ -n "${PANDA_PID:-}" ]]; then kill "${PANDA_PID}" 2>/dev/null || true; wait "${PANDA_PID}" 2>/dev/null || true; fi
  if [[ -n "${UPSTREAM_PID:-}" ]]; then kill "${UPSTREAM_PID}" 2>/dev/null || true; wait "${UPSTREAM_PID}" 2>/dev/null || true; fi
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

cat > "${WORK_DIR}/upstream.py" <<'PY'
from http.server import BaseHTTPRequestHandler, HTTPServer
import json

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        n = int(self.headers.get("Content-Length", "0"))
        _ = self.rfile.read(n) if n else b""
        body = {
            "id": "bench",
            "object": "chat.completion",
            "created": 1,
            "model": "m",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": "ok"},
                "finish_reason": "stop",
            }],
        }
        raw = json.dumps(body).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def log_message(self, *_args):
        return

port = int(__import__("os").environ["UPSTREAM_PORT"])
HTTPServer(("127.0.0.1", port), Handler).serve_forever()
PY

cat > "${WORK_DIR}/panda.yaml" <<EOF
listen: "127.0.0.1:${PANDA_PORT}"
upstream: "http://127.0.0.1:${UPSTREAM_PORT}"
EOF

UPSTREAM_PORT="${UPSTREAM_PORT}" python3 "${WORK_DIR}/upstream.py" >/dev/null 2>&1 &
UPSTREAM_PID=$!

for _ in $(seq 1 40); do
  if curl -fsS -o /dev/null -X POST "http://127.0.0.1:${UPSTREAM_PORT}/v1/chat/completions" \
    -H "Content-Type: application/json" --data-binary "@${PAYLOAD}" 2>/dev/null; then
    break
  fi
  sleep 0.1
done

(
  cd "${ROOT_DIR}"
  cargo run -q -p panda-server -- "${WORK_DIR}/panda.yaml"
) >/dev/null 2>&1 &
PANDA_PID=$!

for _ in $(seq 1 80); do
  if curl -fsS "http://127.0.0.1:${PANDA_PORT}/ready" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done

DIRECT="curl -sS -o /dev/null -X POST http://127.0.0.1:${UPSTREAM_PORT}/v1/chat/completions -H 'Content-Type: application/json' --data-binary @${PAYLOAD}"
VIA="curl -sS -o /dev/null -X POST http://127.0.0.1:${PANDA_PORT}/v1/chat/completions -H 'Content-Type: application/json' --data-binary @${PAYLOAD}"

echo "Benchmark: direct upstream :${UPSTREAM_PORT} vs Panda :${PANDA_PORT} (runs=${RUNS}, warmup=${WARMUP})"
# shellcheck disable=SC2086
hyperfine -N --warmup "${WARMUP}" --runs "${RUNS}" \
  -n "direct_upstream" "${DIRECT}" \
  -n "via_panda" "${VIA}"

echo ""
echo "Interpretation: median(via_panda) - median(direct_upstream) is the incremental proxy cost on loopback."
echo "Public OpenAI calls add similar WAN latency to both paths; the delta remains the proxy hop."

#!/usr/bin/env bash
set -euo pipefail

# Staging readiness gate helper.
# Usage:
#   PANDA_BASE_URL=http://127.0.0.1:8080 ./scripts/staging_readiness_gate.sh
#   READINESS_AUTH_HEADER='x-panda-admin-secret: <secret>' PANDA_BASE_URL=http://127.0.0.1:8080 ./scripts/staging_readiness_gate.sh
# Optional load check:
#   READINESS_LOAD_PAYLOAD=./payload.json READINESS_LOAD_REQUESTS=100 READINESS_LOAD_CONCURRENCY=10 ./scripts/staging_readiness_gate.sh

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BASE_URL="${PANDA_BASE_URL:-http://127.0.0.1:8080}"
REQS="${READINESS_LOAD_REQUESTS:-0}"
CONC="${READINESS_LOAD_CONCURRENCY:-10}"
PAYLOAD="${READINESS_LOAD_PAYLOAD:-}"
AUTH_HEADER="${READINESS_AUTH_HEADER:-}"

echo "== Staging Readiness Gate =="
echo "root: ${ROOT_DIR}"
echo "base_url: ${BASE_URL}"

echo
echo "[1/4] Running core test suites"
(
  cd "${ROOT_DIR}"
  cargo test -p panda-config
  cargo test -p panda-proxy
)

echo
echo "[2/4] Checking liveness/readiness"
if [[ -n "${AUTH_HEADER}" ]]; then
  health_code="$(curl -sS -o /dev/null -w "%{http_code}" -H "${AUTH_HEADER}" "${BASE_URL}/health" || true)"
  ready_code="$(curl -sS -o /dev/null -w "%{http_code}" -H "${AUTH_HEADER}" "${BASE_URL}/ready" || true)"
  mcp_code="$(curl -sS -o /dev/null -w "%{http_code}" -H "${AUTH_HEADER}" "${BASE_URL}/mcp/status" || true)"
else
  health_code="$(curl -sS -o /dev/null -w "%{http_code}" "${BASE_URL}/health" || true)"
  ready_code="$(curl -sS -o /dev/null -w "%{http_code}" "${BASE_URL}/ready" || true)"
  mcp_code="$(curl -sS -o /dev/null -w "%{http_code}" "${BASE_URL}/mcp/status" || true)"
fi
echo "health: ${health_code}"
echo "ready:  ${ready_code}"
echo "mcp:    ${mcp_code}"
if [[ "${health_code}" != "200" || "${ready_code}" != "200" || "${mcp_code}" != "200" ]]; then
  echo "readiness gate failed: health/readiness/status endpoint check failed"
  exit 1
fi

echo
echo "[3/4] Capturing status snapshot"
if [[ -n "${AUTH_HEADER}" ]]; then
  curl -sS -H "${AUTH_HEADER}" "${BASE_URL}/ready"
  echo
  curl -sS -H "${AUTH_HEADER}" "${BASE_URL}/mcp/status"
  echo
else
  curl -sS "${BASE_URL}/ready"
  echo
  curl -sS "${BASE_URL}/mcp/status"
  echo
fi

echo
echo "[4/4] Optional short load probe"
if [[ -n "${PAYLOAD}" && "${REQS}" -gt 0 ]]; then
  if [[ ! -f "${PAYLOAD}" ]]; then
    echo "load probe failed: payload file not found: ${PAYLOAD}"
    exit 1
  fi
  tmp_file="$(mktemp)"
  trap 'rm -f "${tmp_file}"' EXIT
  seq "${REQS}" | xargs -I{} -P "${CONC}" bash -c '
    start="$(python3 - <<'"'"'PY'"'"'
import time
print(time.time())
PY
)"
    if [[ -n "'"${AUTH_HEADER}"'" ]]; then
      code="$(curl -sS -o /dev/null -w "%{http_code}" -H "Content-Type: application/json" -H "'"${AUTH_HEADER}"'" --data-binary @"'"${PAYLOAD}"'" "'"${BASE_URL}"'/v1/chat/completions" || true)"
    else
      code="$(curl -sS -o /dev/null -w "%{http_code}" -H "Content-Type: application/json" --data-binary @"'"${PAYLOAD}"'" "'"${BASE_URL}"'/v1/chat/completions" || true)"
    fi
    end="$(python3 - <<'"'"'PY'"'"'
import time
print(time.time())
PY
)"
    python3 - <<PY
start=float("${start}")
end=float("${end}")
code="${code}"
print(f"{code},{(end-start)*1000:.3f}")
PY
  ' >> "${tmp_file}"

  python3 - <<PY
rows = [line.strip().split(",") for line in open("${tmp_file}", "r", encoding="utf-8") if line.strip()]
codes = [r[0] for r in rows]
ms = sorted(float(r[1]) for r in rows)
ok = sum(1 for c in codes if c.startswith("2"))
fail = len(rows) - ok
p50 = ms[len(ms)//2] if ms else 0.0
p95 = ms[max(int(len(ms)*0.95)-1, 0)] if ms else 0.0
print(f"load_probe_total={len(rows)} ok={ok} fail={fail} p50_ms={p50:.2f} p95_ms={p95:.2f}")
if fail > 0:
    raise SystemExit(1)
PY
else
  echo "load probe skipped (set READINESS_LOAD_PAYLOAD and READINESS_LOAD_REQUESTS>0 to enable)"
fi

echo
echo "staging readiness gate: PASS"

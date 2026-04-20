#!/usr/bin/env bash
set -euo pipefail

# Lightweight load profile for chat completions.
# Example:
#   PANDA_BASE_URL=http://127.0.0.1:8080 \
#   LOAD_PAYLOAD=./payload.json \
#   LOAD_REQUESTS=500 \
#   LOAD_CONCURRENCY=50 \
#   ./scripts/load_profile_chat.sh

BASE_URL="${PANDA_BASE_URL:-http://127.0.0.1:8080}"
PAYLOAD="${LOAD_PAYLOAD:-}"
REQS="${LOAD_REQUESTS:-200}"
CONC="${LOAD_CONCURRENCY:-20}"
AUTH_HEADER="${LOAD_AUTH_HEADER:-}"
OUT_DIR="${LOAD_OUTPUT_DIR:-artifacts/load}"
OUT_FILE="${OUT_DIR}/load_profile_$(date +%Y%m%d_%H%M%S).csv"

if [[ -z "${PAYLOAD}" || ! -f "${PAYLOAD}" ]]; then
  echo "LOAD_PAYLOAD must point to an existing JSON file"
  exit 1
fi

mkdir -p "${OUT_DIR}"
echo "status,elapsed_ms" > "${OUT_FILE}"

echo "base_url=${BASE_URL} requests=${REQS} concurrency=${CONC} output=${OUT_FILE}"

seq "${REQS}" | xargs -I{} -P "${CONC}" bash -c '
  start_ms="$(python3 - <<'"'"'PY'"'"'
import time
print(int(time.time() * 1000))
PY
)"
  if [[ -n "'"${AUTH_HEADER}"'" ]]; then
    code="$(curl -sS -o /dev/null -w "%{http_code}" -H "Content-Type: application/json" -H "'"${AUTH_HEADER}"'" --data-binary @"'"${PAYLOAD}"'" "'"${BASE_URL}"'/v1/chat/completions" || true)"
  else
    code="$(curl -sS -o /dev/null -w "%{http_code}" -H "Content-Type: application/json" --data-binary @"'"${PAYLOAD}"'" "'"${BASE_URL}"'/v1/chat/completions" || true)"
  fi
  end_ms="$(python3 - <<'"'"'PY'"'"'
import time
print(int(time.time() * 1000))
PY
)"
  elapsed="$((end_ms - start_ms))"
  echo "${code},${elapsed}"
' >> "${OUT_FILE}"

python3 - <<PY
import csv
from pathlib import Path
p = Path("${OUT_FILE}")
rows = list(csv.DictReader(p.open()))
lat = sorted(int(r["elapsed_ms"]) for r in rows if r["elapsed_ms"].isdigit())
ok = sum(1 for r in rows if r["status"].startswith("2"))
total = len(rows)
fail = total - ok
def pct(v, q):
    if not v:
        return 0
    i = max(int(len(v)*q)-1, 0)
    return v[i]
print(f"total={total} ok={ok} fail={fail} p50_ms={pct(lat,0.50)} p95_ms={pct(lat,0.95)} p99_ms={pct(lat,0.99)}")
if fail > 0:
    raise SystemExit(1)
PY

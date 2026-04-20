#!/usr/bin/env bash
# Live E2E: mock corp API + panda-server + curl MCP JSON-RPC (ingress /mcp + egress http_tool).
# Usage from repo root: ./scripts/gateway_mcp_e2e_smoke.sh
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
MOCK_PORT="${GATEWAY_MCP_MOCK_PORT:-18081}"
PANDA_PORT="${GATEWAY_MCP_PANDA_PORT:-18080}"
TEMPLATE="${ROOT_DIR}/examples/gateway_mcp_e2e/panda.e2e.yaml.template"
CFG="$(mktemp)"
trap 'rm -f "${CFG}"; kill $(jobs -p) 2>/dev/null || true' EXIT

sed -e "s/@MOCK_PORT@/${MOCK_PORT}/g" -e "s/@PANDA_PORT@/${PANDA_PORT}/g" "${TEMPLATE}" >"${CFG}"

echo "== gateway_mcp_e2e_smoke: mock_port=${MOCK_PORT} panda_port=${PANDA_PORT} =="

python3 "${ROOT_DIR}/examples/gateway_mcp_e2e/mock_corp_api.py" "${MOCK_PORT}" &
sleep 0.3

cargo run -p panda-server -- "${CFG}" &
# First run may compile; increase if curl fails with connection refused.
sleep 5

P="http://127.0.0.1:${PANDA_PORT}"
A='Accept: application/json, text/event-stream'

curl -sS -f -D "${CFG}.hdr" -o "${CFG}.body" -H "${A}" -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"e2e","version":"0"}}}' \
  "${P}/mcp"

SID="$(awk 'BEGIN{IGNORECASE=1} /^mcp-session-id:/{sub(/^[^:]+:[ \t]*/,"");gsub(/\r/,"");print;exit}' "${CFG}.hdr")"
if [[ -z "${SID}" ]]; then
  echo "smoke failed: no Mcp-Session-Id in initialize response"
  cat "${CFG}.hdr" "${CFG}.body" || true
  exit 1
fi

OUT="$(curl -sS -f -H "${A}" -H "Mcp-Session-Id: ${SID}" -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp_corpapi_fetch","arguments":{}}}' \
  "${P}/mcp")"

rm -f "${CFG}.hdr" "${CFG}.body"

if ! grep -q 'mock_corp_api' <<<"${OUT}"; then
  echo "smoke failed: tools/call response missing mock_corp_api marker"
  echo "${OUT}"
  exit 1
fi

echo "smoke ok: tools/call reached mock corporate API"
echo "${OUT}" | head -c 400
echo

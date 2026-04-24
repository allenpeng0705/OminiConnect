#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NANGO="${ROOT}/third_party/nango"
SRC="${ROOT}/scripts/omini_connect_sync_repo_env.ts"
TMP="${NANGO}/scripts/.omini_connect_sync_repo_env.run.ts"
cp "${SRC}" "${TMP}"
cd "${NANGO}"
rc=0
# Prefer local tsx so `npm warn` from npx does not pollute stdout (breaks --print for kubectl).
if [[ -x "${NANGO}/node_modules/.bin/tsx" ]]; then
  "${NANGO}/node_modules/.bin/tsx" scripts/.omini_connect_sync_repo_env.run.ts "$@" || rc=$?
else
  npx tsx scripts/.omini_connect_sync_repo_env.run.ts "$@" || rc=$?
fi
rm -f "${TMP}"
exit "${rc}"

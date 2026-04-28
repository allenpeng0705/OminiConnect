#!/usr/bin/env bash
# Run LLM tool selection tests with LiteLLM startup/teardown
#
# Usage:
#   cd tests/llm_tool_selection
#   ./scripts/run_tests.sh
#   ./scripts/run_tests.sh --scale medium --rounds 3

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
ROOT_DIR="$(cd "${TEST_DIR}/../.." && pwd)"

# Default values
SCALE="small"
ROUNDS="1"
MODEL="gpt-4o-mini"
DELAY_MS="500"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --scale)
      SCALE="$2"
      shift 2
      ;;
    --rounds)
      ROUNDS="$2"
      shift 2
      ;;
    --model)
      MODEL="$2"
      shift 2
      ;;
    --delay-ms)
      DELAY_MS="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Kill any existing LiteLLM on our port
kill_litellm() {
  if lsof -nP -iTCP:4000 -sTCP:LISTEN >/dev/null 2>&1; then
    echo "Stopping existing LiteLLM on port 4000..."
    lsof -nP -iTCP:4000 -sTCP:LISTEN -t | xargs kill 2>/dev/null || true
    sleep 2
  fi
}

cleanup() {
  echo "Cleaning up..."
  if [[ -n "${LITELLM_PID:-}" ]]; then
    kill "${LITELLM_PID}" 2>/dev/null || true
  fi
  kill_litellm
}

trap cleanup EXIT

# Start LiteLLM
kill_litellm

echo "Starting LiteLLM..."
cd "${ROOT_DIR}"
/Users/shileipeng/Library/Python/3.9/bin/litellm --port 4000 --config litellm_config.yaml &
LITELLM_PID=$!

# Wait for LiteLLM to be healthy
echo "Waiting for LiteLLM to be ready..."
LITELLM_READY=false
for i in $(seq 1 30); do
  if curl -sf http://localhost:4000/health >/dev/null 2>&1; then
    HEALTH=$(curl -s http://localhost:4000/health)
    if echo "$HEALTH" | grep -q '"healthy_count":1'; then
      echo "LiteLLM is ready!"
      LITELLM_READY=true
      break
    fi
  fi
  echo "  Waiting... ($i/30)"
  sleep 2
done

if [[ "$LITELLM_READY" != "true" ]]; then
  echo "ERROR: LiteLLM failed to become healthy"
  exit 1
fi

# Check API key
if [[ -z "${MINIMAX_API_KEY:-}" ]]; then
  echo "ERROR: MINIMAX_API_KEY environment variable not set"
  exit 1
fi

# Run tests
echo ""
echo "=========================================="
echo "Running LLM Tool Selection Tests"
echo "=========================================="
echo "Scale: $SCALE"
echo "Rounds: $ROUNDS"
echo "Model: $MODEL"
echo "Delay between requests: ${DELAY_MS}ms"
echo ""

cd "${TEST_DIR}"

RUST_LOG=info cargo run -- \
  --scale "${SCALE}" \
  --rounds "${ROUNDS}" \
  --model "${MODEL}" \
  --registry tools/registry \
  --queries queries \
  --llm-url http://localhost:4000 \
  --llm-api-key "${MINIMAX_API_KEY}"

echo ""
echo "Tests completed!"

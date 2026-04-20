#!/usr/bin/env bash
# Run focused panda-proxy tests for API gateway (ingress/egress) + MCP ingress path.
# Usage: from repo root: ./scripts/gateway_mcp_integration_tests.sh
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "${ROOT_DIR}"

echo "== gateway_mcp_integration_tests: panda-proxy (ingress MCP + api_gateway modules) =="

cargo test -p panda-proxy tests::gateway_workflow -- --nocapture
cargo test -p panda-proxy ingress_mcp -- --nocapture
cargo test -p panda-proxy portal_openapi_and_tools_json_with_ingress -- --nocapture
cargo test -p panda-proxy api_gateway::ingress:: -- --nocapture
cargo test -p panda-proxy api_gateway::egress:: -- --nocapture

echo "== gateway_mcp_integration_tests: done =="

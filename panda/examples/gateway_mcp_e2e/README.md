# Gateway + MCP end-to-end smoke (manual)

**Purpose:** Run a **real** `panda-server` with **ingress** (`backend: mcp` on `/mcp`), **egress**, and an **`http_tool`** that hits a tiny local mock API.

**Automated path:** from the repo root run **`./scripts/gateway_mcp_e2e_smoke.sh`** (starts mock + Panda, **`curl`** JSON-RPC, stops processes).

**Ports (override with env):**

| Variable | Default |
|----------|---------|
| `GATEWAY_MCP_MOCK_PORT` | `18081` |
| `GATEWAY_MCP_PANDA_PORT` | `18080` |

**Manual steps:**

1. Terminal A — mock corporate API:

   ```bash
   python3 examples/gateway_mcp_e2e/mock_corp_api.py 18081
   ```

2. Terminal B — generate config and start Panda (example uses the same ports as the script):

   ```bash
   MOCK_PORT=18081 PANDA_PORT=18080
   sed -e "s/@MOCK_PORT@/${MOCK_PORT}/g" -e "s/@PANDA_PORT@/${PANDA_PORT}/g" \
     examples/gateway_mcp_e2e/panda.e2e.yaml.template > /tmp/panda.e2e.yaml
   cargo run -p panda-server -- /tmp/panda.e2e.yaml
   ```

3. Terminal C — MCP over HTTP (streamable-style `Accept` header):

   ```bash
   P=http://127.0.0.1:18080
   A='Accept: application/json, text/event-stream'

   # initialize → capture Mcp-Session-Id from response headers (curl -D -)
   curl -sS -D /tmp/mcp.hdr -o /tmp/mcp.body -H "$A" -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"e2e","version":"0"}}}' \
     "$P/mcp"
   SID=$(awk 'BEGIN{IGNORECASE=1} /^mcp-session-id:/{sub(/^[^:]+:[ \t]*/,"");gsub(/\r/,"");print;exit}' /tmp/mcp.hdr)

   curl -sS -H "$A" -H "Mcp-Session-Id: $SID" -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp_corpapi_fetch","arguments":{}}}' \
     "$P/mcp"
   ```

   You should see JSON containing **`mock_corp_api`** in the tool result text.

**Multi-path mock:** [`mock_corp_api.py`](mock_corp_api.py) also serves **`/corp/service-a`**, **`/corp/service-b`**, **`/api/hi`**, **`/v1/status`** (see script output at startup). Use repo-root [`panda.yaml`](../../panda.yaml) for a **multi-server** example (`mcp_corp_from_a`, `mcp_inventory_health`, etc.); the E2E template stays minimal so the smoke script stays stable.

**Requirements:** `curl`, `python3`, Rust toolchain. **`upstream`** in the template points at **`http://127.0.0.1:1`** so the LLM is not contacted for this flow (only `/mcp` is exercised).

See also: [`docs/testing_mcp_api_gateway.md`](../../docs/testing_mcp_api_gateway.md) (sections 2.6–2.7).

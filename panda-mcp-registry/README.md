# Panda MCP Registry

**One-click** stdio MCP tool servers for Panda’s MCP host. Each folder is self-contained Python (3.10+), same NDJSON protocol as [`examples/mcp_stdio_minimal`](../examples/mcp_stdio_minimal/server.py).

| Directory | Use case |
|-----------|----------|
| [`postgres/`](postgres/) | Read-only SQL + schema introspection (`DATABASE_URL`) |
| [`slack-discord/`](slack-discord/) | Slack / Discord incoming webhooks |
| [`local-rag-lite/`](local-rag-lite/) | Index local `.txt` (or custom glob) + keyword search |
| [`google-workspace/`](google-workspace/) | Calendar + Gmail search via `GOOGLE_ACCESS_TOKEN` |
| [`github-issues/`](github-issues/) | List/get/create issues via `GITHUB_TOKEN` |

Shared runtime helpers: [`common/mcp_stdio.py`](common/mcp_stdio.py).

## Quick try

```bash
cd postgres
pip install -r requirements.txt
export DATABASE_URL='postgresql://...'
python3 server.py
```

Point Panda at the absolute path of `server.py` under `mcp.servers[].args` (see each README).

## Plugin “App Store”

Gateway **Wasm** plugins use [`crates/panda-pdk`](../crates/panda-pdk) (Rust + [TinyGo bindings](../crates/panda-pdk/go/README.md)). MCP tools here complement that: drop-in `command` + `args` servers without compiling guests.

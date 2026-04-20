# Minimal stdio MCP sample

This directory is a **copy-paste-friendly** MCP server in Python so you can exercise Panda’s MCP host without installing Node or extra packages (only Python 3).

## Tools

| Name | Arguments | Behavior |
|------|-----------|----------|
| `ping` | none | Returns `pong` |
| `echo_message` | `message` (optional string) | Echoes the string |

## Wire Panda to it

1. Copy `panda.example.yaml` to `panda.yaml` and enable `mcp` with a server entry:

```yaml
mcp:
  enabled: true
  fail_open: true
  advertise_tools: true
  servers:
    - name: sample
      enabled: true
      command: "python3"
      args: ["examples/mcp_stdio_minimal/server.py"]
```

2. Point a route at your LLM upstream and list this server in `mcp_servers` (see comments in `panda.example.yaml`).

3. Run the gateway from the repo root so the path `examples/mcp_stdio_minimal/server.py` resolves:

```bash
cargo run -p panda-server -- panda.yaml
```

4. Check tools are visible:

```bash
curl -s -H "x-panda-admin-secret: $PANDA_OPS_SECRET" http://127.0.0.1:8080/mcp/status
```

(Adjust host, port, and ops secret to match your config.)

The integration test harness uses a smaller mock at `crates/panda-proxy/tests/mcp_mock_stdio.py` (single `ping` tool only).

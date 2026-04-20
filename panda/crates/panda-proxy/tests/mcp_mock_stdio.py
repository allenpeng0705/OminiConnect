#!/usr/bin/env python3
"""Minimal MCP stdio server for tests (JSON-RPC 2.0, one JSON object per line)."""
import json
import sys

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    msg = json.loads(line)
    mid = msg.get("id")
    method = msg.get("method")
    if method == "initialize":
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "serverInfo": {"name": "mock", "version": "1"},
            },
        }
    elif method == "notifications/initialized":
        continue
    elif method == "tools/list":
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "result": {
                "tools": [
                    {
                        "name": "ping",
                        "description": "test ping",
                        "inputSchema": {"type": "object", "properties": {}},
                    }
                ]
            },
        }
    elif method == "tools/call":
        name = msg["params"]["name"]
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "result": {
                "content": [{"type": "text", "text": "pong:" + name}],
                "isError": False,
            },
        }
    else:
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "error": {"code": -32601, "message": "unknown method"},
        }
    print(json.dumps(out), flush=True)

#!/usr/bin/env python3
"""
Minimal MCP server over stdio for trying Panda's MCP integration locally.

Protocol: JSON-RPC 2.0, one JSON object per line (NDJSON), matching Panda's stdio client.

Tools:
  - ping          — no arguments; returns "pong"
  - echo_message  — optional `message` string; echoes it back
"""
import json
import sys

TOOLS = [
    {
        "name": "ping",
        "description": "Health check; responds with pong.",
        "inputSchema": {"type": "object", "properties": {}},
    },
    {
        "name": "echo_message",
        "description": "Echo back a short message (sample tool for OpenAI function calling).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Text to echo",
                }
            },
        },
    },
]


def tool_result_text(text: str, is_error: bool = False) -> dict:
    return {
        "content": [{"type": "text", "text": text}],
        "isError": is_error,
    }


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
                "serverInfo": {"name": "panda-mcp-sample", "version": "1"},
            },
        }
    elif method == "notifications/initialized":
        continue
    elif method == "tools/list":
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "result": {"tools": TOOLS},
        }
    elif method == "tools/call":
        params = msg.get("params") or {}
        name = params.get("name")
        args = params.get("arguments") or {}
        if name == "ping":
            body = tool_result_text("pong")
        elif name == "echo_message":
            m = args.get("message")
            if m is None:
                m = "(no message)"
            elif not isinstance(m, str):
                m = str(m)
            body = tool_result_text(m)
        else:
            body = tool_result_text(f"unknown tool: {name}", is_error=True)
        out = {"jsonrpc": "2.0", "id": mid, "result": body}
    else:
        out = {
            "jsonrpc": "2.0",
            "id": mid,
            "error": {"code": -32601, "message": "unknown method"},
        }
    print(json.dumps(out), flush=True)

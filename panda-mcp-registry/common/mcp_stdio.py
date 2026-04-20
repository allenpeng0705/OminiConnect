"""
Shared stdio MCP (JSON-RPC NDJSON) helpers for registry servers.
Protocol aligned with Panda's stdio MCP client and examples/mcp_stdio_minimal.
"""
from __future__ import annotations

import json
import sys
from typing import Any, Callable, Dict, List


def tool_text(text: str, is_error: bool = False) -> dict:
    return {"content": [{"type": "text", "text": text}], "isError": is_error}


def run_stdio(
    tools: List[dict],
    call: Callable[[str, dict], dict],
    server_name: str,
    version: str = "1",
) -> None:
    """Read stdin line-by-line; dispatch tools/call to `call(tool_name, arguments)`."""

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        mid = msg.get("id")
        method = msg.get("method")
        if method == "initialize":
            out = {
                "jsonrpc": "2.0",
                "id": mid,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": {"name": server_name, "version": version},
                },
            }
        elif method == "notifications/initialized":
            continue
        elif method == "tools/list":
            out = {"jsonrpc": "2.0", "id": mid, "result": {"tools": tools}}
        elif method == "tools/call":
            params = msg.get("params") or {}
            name = params.get("name")
            args = params.get("arguments") or {}
            if not isinstance(name, str):
                body = tool_text("missing tool name", is_error=True)
            else:
                try:
                    body = call(name, args if isinstance(args, dict) else {})
                except Exception as e:  # noqa: BLE001 — tool boundary
                    body = tool_text(f"error: {e}", is_error=True)
            out = {"jsonrpc": "2.0", "id": mid, "result": body}
        else:
            out = {
                "jsonrpc": "2.0",
                "id": mid,
                "error": {"code": -32601, "message": f"unknown method: {method}"},
            }
        print(json.dumps(out), flush=True)

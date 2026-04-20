#!/usr/bin/env python3
"""MCP: post alerts to Slack incoming webhooks or Discord webhook URLs (stdlib only)."""
from __future__ import annotations

import json
import sys
import urllib.error
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "common"))
from mcp_stdio import run_stdio, tool_text  # noqa: E402

TOOLS = [
    {
        "name": "slack_post",
        "description": "POST JSON to a Slack incoming webhook URL.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "webhook_url": {"type": "string"},
                "text": {"type": "string", "description": "Message body"},
            },
            "required": ["webhook_url", "text"],
        },
    },
    {
        "name": "discord_post",
        "description": "POST JSON to a Discord channel webhook URL.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "webhook_url": {"type": "string"},
                "content": {"type": "string", "description": "Message (<=2000 chars recommended)"},
            },
            "required": ["webhook_url", "content"],
        },
    },
]


def http_post_json(url: str, payload: dict, timeout: float = 30.0) -> tuple[int, str]:
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            body = resp.read().decode("utf-8", errors="replace")
            return resp.status, body or "ok"
    except urllib.error.HTTPError as e:
        err = e.read().decode("utf-8", errors="replace")
        return e.code, err


def dispatch(name: str, args: dict) -> dict:
    if name == "slack_post":
        url = args["webhook_url"].strip()
        text = args["text"]
        code, resp = http_post_json(url, {"text": text})
        if code >= 400:
            return tool_text(f"slack webhook HTTP {code}: {resp}", is_error=True)
        return tool_text(f"slack ok ({code}): {resp[:500]}")
    if name == "discord_post":
        url = args["webhook_url"].strip()
        content = args["content"]
        code, resp = http_post_json(url, {"content": content})
        if code >= 400:
            return tool_text(f"discord webhook HTTP {code}: {resp}", is_error=True)
        return tool_text(f"discord ok ({code}): {resp[:500]}")
    return tool_text(f"unknown tool: {name}", is_error=True)


if __name__ == "__main__":
    run_stdio(TOOLS, dispatch, "panda-registry-slack-discord")

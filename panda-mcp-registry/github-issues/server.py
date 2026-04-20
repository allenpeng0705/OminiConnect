#!/usr/bin/env python3
"""MCP: GitHub REST v3 issues (list + create) using GITHUB_TOKEN."""
from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "common"))
from mcp_stdio import run_stdio, tool_text  # noqa: E402

TOOLS = [
    {
        "name": "gh_list_issues",
        "description": "List open issues for a repository.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "owner": {"type": "string"},
                "repo": {"type": "string"},
                "per_page": {"type": "integer", "default": 15},
            },
            "required": ["owner", "repo"],
        },
    },
    {
        "name": "gh_get_issue",
        "description": "Fetch one issue by number.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "owner": {"type": "string"},
                "repo": {"type": "string"},
                "number": {"type": "integer"},
            },
            "required": ["owner", "repo", "number"],
        },
    },
    {
        "name": "gh_create_issue",
        "description": "Create a new issue (needs repo scope).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "owner": {"type": "string"},
                "repo": {"type": "string"},
                "title": {"type": "string"},
                "body": {"type": "string", "description": "Markdown body"},
            },
            "required": ["owner", "repo", "title"],
        },
    },
]


def gh_token() -> str:
    t = os.environ.get("GITHUB_TOKEN", "").strip()
    if not t:
        raise RuntimeError("set GITHUB_TOKEN (classic PAT or fine-grained with issues read/write)")
    return t


def github_request(
    path: str,
    method: str = "GET",
    body: dict | None = None,
) -> tuple[int, Any]:
    url = "https://api.github.com" + path
    data = None if body is None else json.dumps(body).encode("utf-8")
    headers = {
        "Authorization": f"Bearer {gh_token()}",
        "Accept": "application/vnd.github+json",
        "User-Agent": "panda-mcp-registry-github",
    }
    if data is not None:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            raw = resp.read().decode("utf-8")
            if not raw:
                return resp.status, {}
            return resp.status, json.loads(raw)
    except urllib.error.HTTPError as e:
        err = e.read().decode("utf-8", errors="replace")
        try:
            parsed = json.loads(err)
            msg = parsed.get("message", err)
        except json.JSONDecodeError:
            msg = err
        raise RuntimeError(f"GitHub HTTP {e.code}: {msg}") from e


def dispatch(name: str, args: dict) -> dict:
    owner = args.get("owner", "").strip()
    repo = args.get("repo", "").strip()
    if name == "gh_list_issues":
        n = int(args.get("per_page") or 15)
        _, data = github_request(f"/repos/{owner}/{repo}/issues?state=open&per_page={n}")
        if not isinstance(data, list):
            return tool_text(str(data), is_error=True)
        lines = [f"#{it['number']}\t{it.get('title','')}" for it in data if "pull_request" not in it]
        return tool_text("\n".join(lines) or "(no issues)")
    if name == "gh_get_issue":
        num = int(args["number"])
        _, it = github_request(f"/repos/{owner}/{repo}/issues/{num}")
        title = it.get("title", "")
        body = it.get("body") or ""
        return tool_text(f"{title}\n\n{body}")
    if name == "gh_create_issue":
        title = args["title"].strip()
        body = args.get("body") or ""
        _, it = github_request(
            f"/repos/{owner}/{repo}/issues",
            method="POST",
            body={"title": title, "body": body},
        )
        return tool_text(f"created #{it.get('number')}: {it.get('html_url','')}")
    return tool_text(f"unknown tool: {name}", is_error=True)


if __name__ == "__main__":
    run_stdio(TOOLS, dispatch, "panda-registry-github")

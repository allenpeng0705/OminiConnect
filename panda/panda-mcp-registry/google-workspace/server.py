#!/usr/bin/env python3
"""MCP: Google Calendar + Gmail read helpers using a user OAuth access token (stdlib)."""
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
        "name": "gcal_list_calendars",
        "description": "List calendars for the authenticated user (needs Calendar scope).",
        "inputSchema": {"type": "object", "properties": {}},
    },
    {
        "name": "gcal_list_events",
        "description": "List upcoming events from a calendar.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "calendar_id": {
                    "type": "string",
                    "description": "e.g. primary or email group calendar id",
                    "default": "primary",
                },
                "max_results": {"type": "integer", "default": 10},
            },
        },
    },
    {
        "name": "gmail_search",
        "description": "Search mailbox (Gmail query syntax).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "q": {"type": "string", "description": "Gmail search query"},
                "max_results": {"type": "integer", "default": 10},
            },
            "required": ["q"],
        },
    },
]


def token() -> str:
    t = os.environ.get("GOOGLE_ACCESS_TOKEN", "").strip()
    if not t:
        raise RuntimeError(
            "set GOOGLE_ACCESS_TOKEN to a valid OAuth2 access token "
            "(Calendar + https://www.googleapis.com/auth/gmail.readonly as needed)"
        )
    return t


def gapi_get(url: str) -> Any:
    req = urllib.request.Request(
        url,
        headers={"Authorization": f"Bearer {token()}"},
        method="GET",
    )
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            return json.loads(resp.read().decode("utf-8"))
    except urllib.error.HTTPError as e:
        err = e.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"Google API HTTP {e.code}: {err[:800]}") from e


def dispatch(name: str, args: dict) -> dict:
    if name == "gcal_list_calendars":
        data = gapi_get("https://www.googleapis.com/calendar/v3/users/me/calendarList?maxResults=50")
        items = data.get("items") or []
        lines = [f"{it.get('id')}\t{it.get('summary','')}" for it in items]
        return tool_text("\n".join(lines) or "(empty)")
    if name == "gcal_list_events":
        cal = (args.get("calendar_id") or "primary").strip()
        n = int(args.get("max_results") or 10)
        from urllib.parse import quote

        cid = quote(cal, safe="")
        url = (
            f"https://www.googleapis.com/calendar/v3/calendars/{cid}/events"
            f"?maxResults={n}&singleEvents=true&orderBy=startTime&timeMin=2020-01-01T00:00:00Z"
        )
        data = gapi_get(url)
        items = data.get("items") or []
        lines = []
        for it in items:
            st = it.get("start", {})
            start = st.get("dateTime") or st.get("date", "")
            lines.append(f"{start}\t{it.get('summary','')}")
        return tool_text("\n".join(lines) or "(no events)")
    if name == "gmail_search":
        q = args["q"].strip()
        n = int(args.get("max_results") or 10)
        from urllib.parse import quote

        url = (
            "https://gmail.googleapis.com/gmail/v1/users/me/messages"
            f"?q={quote(q)}&maxResults={n}"
        )
        data = gapi_get(url)
        mids = [m["id"] for m in (data.get("messages") or [])]
        if not mids:
            return tool_text("(no messages)")
        lines = []
        for mid in mids:
            u = f"https://gmail.googleapis.com/gmail/v1/users/me/messages/{mid}?format=metadata&metadataHeaders=Subject&metadataHeaders=From"
            meta = gapi_get(u)
            hdrs = {h["name"]: h["value"] for h in meta.get("payload", {}).get("headers", [])}
            subj = hdrs.get("Subject", "")
            frm = hdrs.get("From", "")
            lines.append(f"{mid}\t{frm}\t{subj}")
        return tool_text("\n".join(lines))
    return tool_text(f"unknown tool: {name}", is_error=True)


if __name__ == "__main__":
    run_stdio(TOOLS, dispatch, "panda-registry-google")

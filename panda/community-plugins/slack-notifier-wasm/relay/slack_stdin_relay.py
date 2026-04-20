#!/usr/bin/env python3
"""Forward high-cost Panda signals from stdin to a Slack Incoming Webhook (stdlib only).

Pipe container logs or access logs that include x-panda-slack-notify / high-cost hints.

  export SLACK_WEBHOOK_URL='https://hooks.slack.com/services/...'
  docker compose logs -f panda 2>&1 | python3 slack_stdin_relay.py
"""
from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request

TRIGGERS = (
    "x-panda-slack-notify",
    "high-cost",
    "x-panda-slack-reason",
)


def main() -> None:
    url = os.environ.get("SLACK_WEBHOOK_URL", "").strip()
    if not url:
        sys.stderr.write("slack_stdin_relay: set SLACK_WEBHOOK_URL\n")
        # Still drain stdin so pipes do not break.
        for _ in sys.stdin:
            pass
        sys.exit(1)

    for line in sys.stdin:
        if not any(t in line for t in TRIGGERS):
            continue
        text = line.strip()
        if len(text) > 2000:
            text = text[:1997] + "..."
        payload = json.dumps({"text": f":warning: Panda high-cost signal\n```{text}```"}).encode(
            "utf-8"
        )
        req = urllib.request.Request(
            url,
            data=payload,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            urllib.request.urlopen(req, timeout=15)
        except urllib.error.URLError as e:
            sys.stderr.write(f"slack_stdin_relay: post failed: {e}\n")


if __name__ == "__main__":
    main()

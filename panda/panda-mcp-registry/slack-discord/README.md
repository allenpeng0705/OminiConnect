# Slack / Discord Notifier MCP

Uses **incoming webhooks** only (no OAuth). Pass webhook URLs from a secret store or agent args.

## One-click

```bash
python3 server.py
```

## Panda snippet

```yaml
mcp:
  enabled: true
  servers:
    - name: alerts
      command: python3
      args: ["/absolute/path/to/panda-mcp-registry/slack-discord/server.py"]
```

Tools: `slack_post` (`webhook_url`, `text`), `discord_post` (`webhook_url`, `content`).

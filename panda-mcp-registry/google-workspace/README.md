# Google Calendar / Gmail MCP

Read-only helpers against Google REST APIs. You supply a **short-lived OAuth access token** (not a service account for consumer Gmail).

## Token

1. Create an OAuth client in Google Cloud Console (Desktop or Web).
2. Add scopes: `https://www.googleapis.com/auth/calendar.readonly`, `https://www.googleapis.com/auth/gmail.readonly`.
3. Complete the OAuth consent flow and copy the **access token**.

```bash
export GOOGLE_ACCESS_TOKEN='ya29....'
python3 server.py
```

## One-click

```bash
export GOOGLE_ACCESS_TOKEN='...'
python3 server.py
```

## Panda snippet

```yaml
mcp:
  enabled: true
  servers:
    - name: google_workspace
      command: python3
      args: ["/absolute/path/to/panda-mcp-registry/google-workspace/server.py"]
      env:
        GOOGLE_ACCESS_TOKEN: "${GOOGLE_ACCESS_TOKEN}"
```

For production, inject the token from your secret manager instead of a static YAML value.

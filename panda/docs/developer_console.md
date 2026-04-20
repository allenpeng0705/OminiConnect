# Panda Developer Console

This guide explains how to run and use Panda's optional Developer Console for live request debugging.

The console is intended for development/troubleshooting. It is disabled by default.

## 1) Enable the console

Run Panda with:

```bash
PANDA_DEV_CONSOLE_ENABLED=true cargo run -p panda-server -- panda.yaml
```

Endpoints:

- UI: `GET /console`
- WebSocket stream: `GET /console/ws`

Example local URLs:

- `http://127.0.0.1:8080/console`
- `ws://127.0.0.1:8080/console/ws`

If `PANDA_DEV_CONSOLE_ENABLED` is not set, both endpoints return `404`.

## 2) Secure the console (recommended)

Console auth reuses Panda ops auth behavior.

In `panda.yaml`:

```yaml
observability:
  admin_auth_header: x-panda-admin-secret
  admin_secret_env: PANDA_OPS_SECRET
```

In shell:

```bash
export PANDA_OPS_SECRET='change-me'
```

When `observability.admin_secret_env` is configured, requests to `/console` and `/console/ws` must include the configured header/value.

Example:

```bash
curl -i http://127.0.0.1:8080/console \
  -H "x-panda-admin-secret: change-me"
```

## 3) What the console shows

The `/console` page is a **Live Trace** UI: a sidebar of recent `request_id`s, a **timeline** of events for the selected request, and a **thought stream** panel that shows the latest rolling excerpt from streaming assistant output (`llm_trace` events during SSE).

Event coverage:

- Request lifecycle:
  - `request_started`
  - `request_finished`
  - `request_failed`
- MCP tool calls:
  - `mcp_call` with `payload.phase = "start"` and `payload.phase = "finish"`
- Streaming assistant preview (SSE only, throttled):
  - `llm_trace` with `payload.text_tail` and `payload.chars_total`

Core event fields:

- `version`
- `request_id`
- `ts_unix_ms`
- `stage`
- `kind`
- `method`
- `route`
- `status` (when available)
- `elapsed_ms` (when available)
- `payload` (event-specific details)

## 4) Event format examples

Request start:

```json
{
  "version": "v1",
  "request_id": "d4f4d909-8cb3-4ac5-8f2f-4ef0bcb2f0c3",
  "trace_id": null,
  "ts_unix_ms": 1774941095123,
  "stage": "ingress",
  "kind": "request_started",
  "method": "POST",
  "route": "/v1/chat/completions",
  "status": null,
  "elapsed_ms": null
}
```

MCP start:

```json
{
  "version": "v1",
  "request_id": "d4f4d909-8cb3-4ac5-8f2f-4ef0bcb2f0c3",
  "trace_id": null,
  "ts_unix_ms": 1774941095341,
  "stage": "mcp",
  "kind": "mcp_call",
  "method": "POST",
  "route": "/v1/chat/completions",
  "status": null,
  "elapsed_ms": null,
  "payload": {
    "phase": "start",
    "round": 1,
    "server": "filesystem",
    "tool": "read_file",
    "arguments_preview": "{\"path\":\"src/main.rs\",\"api_key\":\"[REDACTED]\"}",
    "arguments_redacted": true
  }
}
```

MCP finish:

```json
{
  "version": "v1",
  "request_id": "d4f4d909-8cb3-4ac5-8f2f-4ef0bcb2f0c3",
  "trace_id": null,
  "ts_unix_ms": 1774941095398,
  "stage": "mcp",
  "kind": "mcp_call",
  "method": "POST",
  "route": "/v1/chat/completions",
  "status": null,
  "elapsed_ms": 57,
  "payload": {
    "phase": "finish",
    "round": 1,
    "server": "filesystem",
    "tool": "read_file",
    "status": "success",
    "duration_ms": 57,
    "error": null
  }
}
```

## 5) Quick verification flow

1. Enable console and start Panda.
2. Open `/console` in browser.
3. Send a chat request:

```bash
curl -s http://127.0.0.1:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model":"gpt-4o-mini",
    "messages":[{"role":"user","content":"hello"}]
  }'
```

4. Confirm live events appear in the UI.

## 6) Operational notes

- The event channel is bounded. Slow viewers can miss older events under load, but the connection remains active.
- Argument redaction is best-effort key-based masking; treat console output as sensitive operational data.
- For production-like environments, keep console disabled unless actively debugging.

## 7) Troubleshooting

### `404` on `/console` or `/console/ws`

Cause:

- Console is disabled.

Checks:

- Ensure Panda started with `PANDA_DEV_CONSOLE_ENABLED=true`.
- Confirm you are calling the correct host/port from `panda.yaml` `listen`.

### `401` on `/console` or `/console/ws`

Cause:

- Ops auth is enabled and secret header is missing/invalid.

Checks:

- Verify `observability.admin_secret_env` in `panda.yaml`.
- Verify that env var is exported in the Panda process environment.
- Verify request header name matches `observability.admin_auth_header` (default `x-panda-admin-secret`).

Quick check:

```bash
curl -i http://127.0.0.1:8080/console \
  -H "x-panda-admin-secret: $PANDA_OPS_SECRET"
```

### Browser says WebSocket failed

Cause:

- Wrong scheme/port, auth mismatch, or intermediate proxy strips upgrade headers.

Checks:

- Use `ws://` for HTTP and `wss://` for HTTPS.
- If ops auth is enabled, ensure browser request to `/console/ws` includes required header path (or test through a local trusted path first).
- If running behind another gateway/proxy, ensure WebSocket upgrade is allowed and forwarded.

Manual handshake test:

```bash
curl -i -N \
  -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Version: 13" \
  -H "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" \
  http://127.0.0.1:8080/console/ws
```

### Console opens but no events appear

Cause:

- No traffic is flowing through Panda, or all requests are blocked before normal pipeline paths.

Checks:

- Send a known test request to `/v1/chat/completions`.
- Verify logs for policy/identity rejections that may short-circuit requests.
- Confirm your request reaches this Panda instance (especially in multi-replica setups).

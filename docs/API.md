# OminiConnect REST API

Base URL: `http://localhost:9000` (or your deployed domain)

All endpoints require a `Bearer` API key in the `Authorization` header:

```
Authorization: Bearer sk-xxxxx
```

---

## Authentication

### Create API Key

Create a named API key for an agent or application.

```
POST /auth/apikey
```

**Request body:**
```json
{ "label": "github-writer-agent" }
```

**Response `201`:**
```json
{
  "key": "sk_om_abc123...",  // shown ONLY once — store securely
  "label": "github-writer-agent",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### List API Keys

List all API keys (the raw key is never returned after creation).

```
GET /auth/apikey
```

**Response `200`:**
```json
[
  {
    "key_hash": "$2b$12$...",
    "label": "github-writer-agent",
    "created_at": "2024-01-15T10:00:00Z"
  }
]
```

### Revoke API Key

```
DELETE /auth/apikey/{key_hash}
```

**Response `200`:**
```json
{ "ok": true }
```

---

## Connectors

### List Connected Platforms

```
GET /api/connectors
```

**Response `200`:**
```json
[
  {
    "platform": "github",
    "enabled": true,
    "scopes": ["repo", "user:email", "read:org"],
    "created_at": "2024-01-10T08:00:00Z"
  },
  {
    "platform": "slack",
    "enabled": true,
    "scopes": ["chat:write", "channels:read"],
    "created_at": "2024-01-12T14:30:00Z"
  }
]
```

### Delete Connector

```
DELETE /api/connectors/{platform}
```

**Response `200`:**
```json
{ "ok": true }
```

---

## Maton-Style Direct Call

Call any connected platform's API directly — OminiConnect handles auth token injection.

```
POST /api/call/{platform}
```

**Request body:**
```json
{
  "method": "GET",
  "path": "/user/repos",
  "params": { "sort": "updated", "per_page": "10" },
  "body": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `method` | string | HTTP method: `GET`, `POST`, `PUT`, `PATCH`, `DELETE` |
| `path` | string | API path (e.g., `/user/repos`) |
| `params` | object | Query string parameters (optional) |
| `body` | any | Request body for POST/PUT/PATCH (optional) |

**Response:** The upstream API's raw response is returned directly.

```json
[
  { "id": 123, "name": "my-repo", "full_name": "user/my-repo" }
]
```

**Example — GitHub:**
```bash
curl -X POST http://localhost:9000/api/call/github \
  -H "Authorization: Bearer sk-xxxxx" \
  -H "Content-Type: application/json" \
  -d '{"method": "GET", "path": "/user/repos", "params": {"sort": "updated"}}'
```

**Example — Slack:**
```bash
curl -X POST http://localhost:9000/api/call/slack \
  -H "Authorization: Bearer sk-xxxxx" \
  -H "Content-Type: application/json" \
  -d '{"method": "POST", "path": "/api/chat.postMessage", "body": {"channel": "C0123", "text": "Hello!"}}'
```

---

## Tools

Tools are structured, scope-checked wrappers around platform APIs. Each tool has a slug, description, and typed input schema.

### List Tools

```
GET /api/tools
GET /api/tools?platform=github
```

**Response `200`:**
```json
{
  "toolkits": [
    {
      "slug": "github",
      "name": "GitHub",
      "logo": "/images/template-logos/github.svg",
      "provider": "github",
      "tools": [
        {
          "slug": "github_list_repos",
          "name": "List Repositories",
          "description": "List repositories for the authenticated user",
          "method": "GET",
          "endpoint": "/user/repos",
          "scopes": ["repo"],
          "scope_satisfied": "yes",
          "tags": ["github", "repository"]
        }
      ]
    }
  ]
}
```

### Search Tools

```
GET /api/tools/search?q=list+repos
GET /api/tools/search?q=create+issue&platform=github
GET /api/tools/search?q=message&filter_scope=yes
```

| Query param | Description |
|-------------|-------------|
| `q` | Search query (matches name, description, tags, slug) |
| `platform` | Filter by platform |
| `filter_scope` | `yes` (only tools with satisfied scopes), `no` (only unsatisfied), `any` (all) |

**Response `200`:**
```json
{
  "tools": [
    {
      "slug": "github_list_repos",
      "name": "List Repositories",
      "description": "List repositories for the authenticated user",
      "method": "GET",
      "endpoint": "/user/repos",
      "scopes": ["repo"],
      "scope_satisfied": "yes",
      "tags": ["github", "repository"]
    }
  ],
  "query": "list repos"
}
```

### Execute Tool

```
POST /api/tools/execute
```

**Request body:**
```json
{
  "tool_slug": "github_list_repos",
  "platform": "github",
  "arguments": { "sort": "updated" },
  "callback_url": null
}
```

| Field | Type | Description |
|-------|------|-------------|
| `tool_slug` | string | The tool identifier |
| `platform` | string | The platform (e.g., `github`) |
| `arguments` | object | Tool-specific arguments |
| `callback_url` | string | Optional. If provided, returns 202 immediately and POSTs result to this URL |

**Response (sync):**
```json
{
  "ok": true,
  "body": [ { "id": 123, "name": "my-repo" } ],
  "call_id": "call_abc123",
  "status": "completed",
  "duration_ms": 234
}
```

**Response (async — when `callback_url` provided):**
```json
{
  "call_id": "call_abc123",
  "status": "pending"
}
```

Your webhook receives the result when ready.

---

## MCP (JSON-RPC)

OminiConnect also supports MCP JSON-RPC protocol:

```
POST /api/mcp
```

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

See [MCP.md](./MCP.md) for full MCP documentation.

---

## Error Responses

All errors follow a consistent format:

| Status | Meaning |
|--------|---------|
| `400` | Bad request — invalid parameters |
| `401` | Unauthorized — missing or invalid API key |
| `403` | Forbidden — connector disabled |
| `404` | Not found — tool or connector not found |
| `429` | Rate limited — back off and retry |
| `500` | Internal server error |

**Error body:**
```json
{ "error": "platform not configured" }
```

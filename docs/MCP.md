# OminiConnect MCP (Model Context Protocol)

OminiConnect implements the MCP standard so AI agents can discover and execute tools through a standardized interface. MCP clients (Claude Desktop, Cursor, Windsurf, etc.) can connect directly to OminiConnect without any code.

---

## What is MCP?

MCP (Model Context Protocol) is a standardized protocol for AI models to discover tools and call them. Instead of hard-coding tool definitions, the AI queries the MCP server to find out what tools are available and what arguments each tool accepts.

OminiConnect's MCP server:
- Exposes all connected platforms as tools
- Enforces OAuth scopes per tool
- Supports both synchronous execution and async callback-based execution
- Uses JSON-RPC 2.0 over HTTP

---

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/mcp` | JSON-RPC 2.0 request/response |
| `GET` | `/api/mcp/sse` | SSE stream for async push notifications |

---

## Authentication

Include your API key as a Bearer token:

```
Authorization: Bearer sk-xxxxx
```

The MCP server authenticates every request and only returns tools for platforms the user has connected.

---

## JSON-RPC Methods

### `tools/list`

Returns all tools available to the authenticated user. Tools are scoped to the user's connected platforms.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "github_list_repos",
        "description": "List repositories for the authenticated user",
        "input_schema": {
          "type": "object",
          "properties": {
            "sort": {
              "type": "string",
              "description": "Sort order: created, updated, pushed, full_name",
              "enum": ["created", "updated", "pushed", "full_name"]
            },
            "per_page": {
              "type": "integer",
              "description": "Results per page (max 100)"
            },
            "page": {
              "type": "integer",
              "description": "Page number"
            }
          }
        },
        "scope_satisfied": "yes"
      },
      {
        "name": "github_create_issue",
        "description": "Create a new issue in a repository",
        "input_schema": {
          "type": "object",
          "properties": {
            "owner": { "type": "string", "description": "Repository owner" },
            "repo": { "type": "string", "description": "Repository name" },
            "title": { "type": "string", "description": "Issue title" },
            "body": { "type": "string", "description": "Issue body text" }
          },
          "required": ["owner", "repo", "title"]
        },
        "scope_satisfied": "yes"
      }
    ]
  }
}
```

**`scope_satisfied` field:**

| Value | Meaning |
|-------|---------|
| `"yes"` | The user has granted all scopes required by this tool |
| `"no"` | The user has connected the platform but is missing required scopes |
| `"unknown"` | Scope status could not be determined |

### `tools/call`

Execute a tool by name with structured arguments.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "github_list_repos",
    "arguments": { "sort": "updated", "per_page": 10 }
  }
}
```

**Response (sync execution):**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "ok": true,
    "body": [
      { "id": 123, "name": "my-repo", "full_name": "user/my-repo", "updated_at": "2024-01-15T10:30:00Z" }
    ],
    "call_id": "abc123",
    "status": "completed",
    "duration_ms": 234
  }
}
```

**Response (error):**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32602,
    "message": "Insufficient scopes for this tool"
  }
}
```

**With `callback_url` for async execution:**

Pass a `callback_url` in the arguments to have the result POSTed to your URL:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "long_running_tool",
    "arguments": {},
    "callback_url": "https://your-app.com/webhook/tool-result"
  }
}
```

Response (immediate):
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "ok": true,
    "call_id": "call_abc123",
    "status": "pending"
  }
}
```

Your webhook receives the result:
```json
{
  "call_id": "call_abc123",
  "tool_slug": "long_running_tool",
  "platform": "github",
  "status": "completed",
  "result": "...",
  "duration_ms": 5432
}
```

---

## Error Codes

MCP uses standard JSON-RPC error codes plus application-specific codes:

| Code | Meaning |
|------|---------|
| `-32600` | Invalid Request — malformed JSON-RPC |
| `-32601` | Method not found — unknown MCP method |
| `-32602` | Invalid params — bad arguments or scope insufficient |
| `-32603` | Internal error — OminiConnect server error |

---

## SSE Endpoint (`/api/mcp/sse`)

For clients that want push notifications (long-running tasks, webhooks), connect to the SSE endpoint:

```bash
curl -N http://localhost:9000/api/mcp/sse \
  -H "Authorization: Bearer sk-xxxxx"
```

The SSE stream sends events as JSON:

```
event: message
data: {"type":"connected","owner":"alice"}

event: message
data: {"type":"tool_result","call_id":"abc123","status":"completed","result":{...}}
```

The SSE connection also receives `tools/call` results when using callback-based async execution.

---

## Using with AI Clients

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "OminiConnect": {
      "command": "curl",
      "args": [
        "-X", "POST", "http://localhost:9000/api/mcp",
        "-H", "Authorization: Bearer sk-xxxxx",
        "-H", "Content-Type: application/json",
        "-d", "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\",\"params\":{}}"
      ]
    }
  }
}
```

For production, use an MCP client library instead of curl.

### Cursor

In Cursor settings → MCP → Add server:

```
Name: OminiConnect
URL: http://localhost:9000/api/mcp
Headers: Authorization: Bearer sk-xxxxx
```

### Windsurf

Settings → MCP → Add:

```
Name: OminiConnect
URL: http://localhost:9000/api/mcp
Headers: Authorization: Bearer sk-xxxxx
```

---

## Scopes and Security

Each tool requires specific OAuth scopes. When a tool's `scope_satisfied` is `"no"`, the user needs to re-authorize the platform with additional scopes.

Tools are executed with the OAuth token stored for that platform — the AI agent never sees the raw token, only the tool result.

All tool executions are logged with:
- Timestamp
- Agent ID (from the API key)
- Tool name and platform
- Arguments passed
- Response body (truncated to 64KB)
- Duration

---

## Platform Coverage

OminiConnect MCP exposes tools for all connected platforms. Use `tools/list` to discover available tools for each platform.

Tools are defined in `tools/registry/` (YAML) and can be extended by adding new tool definitions.

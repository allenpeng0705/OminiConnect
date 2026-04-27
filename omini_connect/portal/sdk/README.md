# OminiConnect SDKs

Client libraries for the OminiConnect API — connect AI agents to 700+ external platforms through a unified interface.

---

## Language SDKs

### Python

**Requires:** Python 3.10+

```bash
pip install ominiconnect
```

```python
from ominiconnect import OminiConnect, AuthError

client = OminiConnect(api_key="sk-xxxxx")

# Check what platforms are connected
print(client.connectors.list())

# Call any API directly — Maton style (simplest way)
user = client.call("github", "GET", "/user")
repos = client.call("github", "GET", "/user/repos", params={"sort": "updated"})

# POST with body
client.call("slack", "POST", "/api/chat.postMessage", body={
    "channel": "C0123",
    "text": "Hello!"
})

# Use tools (discoverable + scope-checked)
result = client.tools.execute("github_list_repos", {"sort": "updated"})
if result["ok"]:
    print(result["body"])

# Search tools
tools = client.tools.search("list repos", platform="github")

# Create named API keys — one per agent
key = client.api_keys.create("pr-reviewer-agent")
print(key["key"])  # shown once — store securely!
```

### Error handling

```python
from ominiconnect import (
    AuthError,
    ConnectorNotFoundError,
    UpstreamError,
    NetworkError,
)

try:
    client.call("github", "GET", "/user")
except AuthError:
    print("Invalid API key")
except ConnectorNotFoundError:
    print("Platform not connected — use portal to connect first")
except UpstreamError as e:
    print(f"GitHub error {e.status_code}: {e.body}")
except NetworkError:
    print("Connection failed — check your network")
```

### From source

```bash
cd sdk/python
pip install -e .
```

---

### JavaScript / TypeScript

**Requires:** Node.js 16+ or any `fetch`-compatible runtime.

```bash
npm install @ominiconnect/sdk
```

```typescript
import { OminiConnect } from "@ominiconnect/sdk";

const client = new OminiConnect({ apiKey: "sk-xxxxx" });

// Check connected platforms
console.log(await client.connectors.list());

// Call any API directly — Maton style
const user = await client.call("github", "GET", "/user");
const repos = await client.call("github", "GET", "/user/repos", {
  params: { sort: "updated" }
});
await client.call("slack", "POST", "/api/chat.postMessage", {
  body: { channel: "C0123", text: "Hello!" }
});

// Use tools
const result = await client.tools.execute("github_list_repos", { sort: "updated" });

// Create named API keys
const { key } = await client.apiKeys.create("pr-reviewer-agent");
console.log(key); // shown once
```

### TypeScript types

```typescript
import type {
  Connector,
  ToolkitsResponse,
  ToolExecuteResult,
  ApiKeyCreated,
} from "@ominiconnect/sdk";
```

### Error handling

```typescript
try {
  await client.call("github", "GET", "/user");
} catch (e) {
  if (e instanceof OminiConnectError) {
    if (e.name === "AuthError") { /* invalid key */ }
    if (e.name === "ConnectorNotFoundError") { /* not connected */ }
    if (e instanceof UpstreamError) { /* e.statusCode, e.body */ }
  }
}
```

### From source

```bash
cd sdk/js
npm install
npm run build
```

---

### Go

**Requires:** Go 1.21+

```bash
go get github.com/ominiconnect/go-sdk/ominiconnect
```

```go
package main

import (
    "fmt"
    "log"

    "github.com/ominiconnect/go-sdk/ominiconnect"
)

func main() {
    client := ominiconnect.New("sk-xxxxx", nil)

    // List connected platforms
    connectors, err := client.Connectors.List()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(connectors)

    // Call GitHub API directly — Maton style
    user, err := client.Call(context.Background(), "github", "GET", "/user", nil, nil)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(user)

    // Create a named API key
    key, err := client.ApiKeys.Create("pr-reviewer-agent")
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(key.Key) // shown once — store securely
}
```

### Error handling

```go
import "errors"

_, err := client.Call(ctx, "github", "GET", "/user", nil, nil)
if err != nil {
    if errors.Is(err, ominiconnect.ErrAuth) {
        println("invalid API key")
    }
    var upstream *ominiconnect.UpstreamError
    if errors.As(err, &upstream) {
        fmt.Printf("upstream error %d: %s\n", upstream.StatusCode, upstream.Message)
    }
}
```

---

### Rust

**Requires:** Rust 1.70+

Add to your `Cargo.toml`:

```toml
[dependencies]
omini-connect-sdk = "0.1"
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
```

```rust
use omini_connect_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("sk-xxxxx", None);

    // List connected platforms
    let connectors = client.connectors().list().await?;
    println!("{:?}", connectors);

    // Call GitHub API directly — Maton style
    let user = client
        .call("github", "GET", "/user", None, None)
        .await?;
    println!("{}", user);

    // Create a named API key
    let key = client.api_keys().create("pr-reviewer-agent").await?;
    println!("{}", key.key); // shown once — store securely

    Ok(())
}
```

---

## REST API

All SDKs are thin wrappers around the REST API. Use raw HTTP if preferred:

| SDK method | HTTP | Body |
|---|---|---|
| `client.call(platform, method, path)` | `POST /api/call/{platform}` | `{"method", "path", "params", "body"}` |
| `client.connectors.list()` | `GET /api/connectors` | — |
| `client.connectors.delete(platform)` | `DELETE /api/connectors/{platform}` | — |
| `client.tools.list(platform?)` | `GET /api/tools?platform=...` | — |
| `client.tools.search(q, platform?)` | `GET /api/tools/search?q=...&platform=...` | — |
| `client.tools.execute(slug, args)` | `POST /api/tools/execute` | `{"tool_slug", "arguments", "callback_url?"}` |
| `client.apiKeys.create(label)` | `POST /auth/apikey` | `{"label"}` |
| `client.apiKeys.list()` | `GET /auth/apikey` | — |
| `client.apiKeys.delete(key_hash)` | `DELETE /auth/apikey/{key_hash}` | — |

### Maton-style call

The simplest way to use OminiConnect — just pass through to the platform API:

```bash
curl -X POST http://localhost:9000/api/call/github \
  -H "Authorization: Bearer sk-xxxxx" \
  -H "Content-Type: application/json" \
  -d '{"method": "GET", "path": "/user/repos", "params": {"sort": "updated"}}'
```

### Authentication

All API calls require a `Bearer` token in the `Authorization` header:

```
Authorization: Bearer sk-xxxxx
```

API keys are created via the portal UI or `POST /auth/apikey`.

---

## MCP (Model Context Protocol)

OminiConnect exposes an MCP server so AI agents can discover and call tools natively.

### MCP endpoint

- **JSON-RPC**: `POST /api/mcp` — send JSON-RPC requests, get JSON-RPC responses
- **SSE stream**: `GET /api/mcp/sse` — Server-Sent Events for async push

### MCP methods

| Method | Description |
|--------|-------------|
| `tools/list` | Returns all tools available to the authenticated user |
| `tools/call` | Executes a tool by name with arguments |

### Example: tools/list

```json
// POST /api/mcp
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}

// Response
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
              "description": "Sort order: created, updated, pushed, full_name"
            }
          }
        },
        "scope_satisfied": "yes"
      }
    ]
  }
}
```

### Example: tools/call

```json
// POST /api/mcp
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "github_list_repos",
    "arguments": { "sort": "updated" }
  }
}

// Response (sync)
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "ok": true,
    "body": { ... },
    "duration_ms": 234
  }
}
```

### Using with Claude Desktop

Add to your `claude_desktop_config.json`:

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

For production use, use an MCP client library instead of curl.

---

## LLM Integration (Natural Language API Selection)

OminiConnect provides a natural language interface so AI agents can call APIs without knowing the underlying REST endpoints. Just describe what you want in plain English.

### REST API

| SDK method | HTTP | Body |
|---|---|---|
| `client.llm.execute(query)` | `POST /api/llm` | `{"query", "platform?"}` |
| `client.llm.listAvailableTools()` | `GET /api/llm/tools` | — |

### Example: Natural language query

```bash
curl -X POST http://localhost:9000/api/llm \
  -H "Authorization: Bearer sk-xxxxx" \
  -H "Content-Type: application/json" \
  -d '{"query": "list my github repos sorted by updated"}'
```

### Response

```json
{
  "ok": true,
  "tool": "github_list_repos",
  "tool_name": "List Repositories",
  "arguments": { "sort": "updated" },
  "explanation": "Selected github_list_repos because your query mentions 'repos' and 'github'.",
  "result": {
    "ok": true,
    "body": [{ "name": "my-repo", "updated_at": "2024-01-15T..." }],
    "call_id": "call_abc123",
    "status": "completed",
    "duration_ms": 234
  }
}
```

### When the query is ambiguous

```json
{
  "ok": false,
  "error": "ambiguous",
  "message": "Your query could match multiple tools. Did you mean one of:",
  "candidates": [
    { "tool": "github_list_repos", "name": "List Repositories", "match_reason": "score 0.85" },
    { "tool": "github_search_code", "name": "Search Code", "match_reason": "score 0.72" }
  ]
}
```

### SDK usage

```python
from ominiconnect import OminiConnect

client = OminiConnect(api_key="sk-xxxxx")

# Natural language — the SDK picks the right tool and executes it
result = client.llm.execute("list my github repos sorted by updated")
print(result.result["body"])

# Optional platform hint for faster routing
result = client.llm.execute("create an issue", platform="github")

# See what tools are available for external LLM selection
catalog = client.llm.list_available_tools()
print(catalog.platforms["github"]["tools"][0]["example_queries"])
```

```typescript
const client = new OminiConnect({ apiKey: "sk-xxxxx" });

// Natural language query
const result = await client.llm.execute("list my github repos sorted by updated");
console.log(result.result);

// With platform hint
const result2 = await client.llm.execute("create an issue", { platform: "github" });

// Get available tools for external LLM
const catalog = await client.llm.listAvailableTools();
```

```go
client := ominiconnect.New("sk-xxxxx", nil)

result, err := client.Llm().Execute(context.Background(), "list my github repos sorted by updated", nil)
if err != nil {
    log.Fatal(err)
}
fmt.Println(result.Result)

// With platform hint
platform := "github"
result, err = client.Llm().Execute(context.Background(), "create an issue", &platform)
```

```rust
let client = Client::new("sk-xxxxx", None);

let result = client.llm()
    .execute("list my github repos sorted by updated", None)
    .await?;
println!("{:?}", result.result);

// With platform hint
let result = client.llm()
    .execute("create an issue", Some("github"))
    .await?;
```

### How it works

The LLM endpoint uses **rule-based keyword matching** — no external LLM API needed:

1. **Platform detection** — infers platform from query ("github" → GitHub connector)
2. **Tool scoring** — scores each available tool by action verb match, keyword match, and example query similarity
3. **Argument extraction** — parses query for common parameters (sort, per_page, etc.)
4. **Execution** — calls the selected tool via the existing `/api/tools/execute` flow

If the query is ambiguous (multiple tools have similar scores), returns a `candidates` list for disambiguation.

---

## Architecture

```
OminiConnect client
    │
    ├── .call(platform, method, path)   → POST /api/call/{platform}  (Maton-style)
    ├── .connectors                     → /api/connectors
    ├── .tools                          → /api/tools, /api/tools/search, /api/tools/execute
    ├── .api_keys                       → /auth/apikey
    ├── .llm                            → /api/llm, /api/llm/tools, /api/llm/explain
    └── MCP                            → POST /api/mcp, GET /api/mcp/sse
```

**One client, all capabilities.** Pass any API key — it works for both tools and direct calls.

**Three ways to call APIs:**

1. **Maton style** (`client.call(platform, method, path)`) — direct REST passthrough. You handle the API shape, OminiConnect handles auth and token management.

2. **Tool style** (`client.tools.execute(slug, args)`) — structured, scope-checked, discoverable. OminiConnect validates scopes before forwarding. Better for AI agents that need guided tool selection.

3. **LLM style** (`client.llm.execute("list my repos")`) — natural language. The AI picks the right tool and arguments automatically. Best for conversational AI agents that reason in natural language.

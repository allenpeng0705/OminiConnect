# LLM Integration Design — Natural Language API Selection

## Problem Statement

Today, an AI agent using OminiConnect must:
1. Know which platform to call (e.g., "github")
2. Know which endpoint to use (e.g., `/user/repos`)
3. Know the HTTP method and arguments

This requires the agent to have knowledge of the underlying APIs. The goal is to let an agent say:

> "List my GitHub repositories sorted by last updated date"

And have OminiConnect automatically select and execute the correct tool (`github_list_repos`), returning the result.

## Architecture

```
LLM Agent
    │
    │  "list my github repos sorted by updated"
    ▼
POST /api/llm  { query, platform?, context? }
    │
    ├──► Tool Registry (search for matching tools)
    │    ├── github_list_repos (description: "List repositories for the authenticated user")
    │    ├── github_create_issue
    │    └── ...
    │
    ├──► Decision Engine
    │    ├── Option A: Rule-based keyword matching (no external LLM needed)
    │    └── Option B: Claude-assisted tool selection (via SDK/Proxy)
    │
    ├──► Execute selected tool with resolved arguments
    │
    ▼
{ tool, arguments, result, explanation }
```

## REST API Design

### `POST /api/llm`

**Request:**
```json
{
  "query": "list my github repos sorted by updated",
  "platform": "github",           // optional hint
  "connected_platforms": ["github", "slack"],  // optional, auto-detected if omitted
  "context": {                    // optional extra context
    "current_file": "README.md"
  }
}
```

**Response (success):**
```json
{
  "ok": true,
  "tool": "github_list_repos",
  "tool_name": "List Repositories",
  "arguments": { "sort": "updated" },
  "explanation": "Selected github_list_repos because your query mentions 'repos' and 'github'. The sort=updated matches your request to sort by update time.",
  "result": {
    "ok": true,
    "body": [ { "name": "my-repo", "updated_at": "2024-01-15T..." } ],
    "call_id": "call_abc123",
    "status": "completed",
    "duration_ms": 234
  }
}
```

**Response (ambiguous query):**
```json
{
  "ok": false,
  "error": "ambiguous",
  "message": "Your query could match multiple tools. Did you mean one of:",
  "candidates": [
    { "tool": "github_list_repos", "name": "List Repositories", "match_reason": "query mentions 'repos'" },
    { "tool": "github_search_code", "name": "Search Code", "match_reason": "query mentions 'search'" }
  ],
  "suggestion": "Try being more specific, e.g., 'list my GitHub repositories' or 'search code in my GitHub repos'"
}
```

**Response (no match):**
```json
{
  "ok": false,
  "error": "no_matching_tool",
  "message": "No tool found matching your query.",
  "available_tools_hint": "You have GitHub and Slack connected. Available GitHub tools include: list_repos, create_issue, search_code..."
}
```

**Response (no connector):**
```json
{
  "ok": false,
  "error": "platform_not_connected",
  "message": "GitHub is not connected. Connect it at /dashboard first."
}
```

### `GET /api/llm/tools`

Returns available tools with descriptions for a connected platform — useful for an external LLM to do tool selection client-side.

```json
{
  "platforms": {
    "github": {
      "connected": true,
      "tools": [
        {
          "slug": "github_list_repos",
          "name": "List Repositories",
          "description": "List repositories for the authenticated user",
          "example_queries": ["list my repos", "show my github repositories", "what repos do I have"],
          "scopes": ["repo"],
          "scope_satisfied": "yes"
        }
      ]
    }
  }
}
```

### `POST /api/llm/explain`

Given a natural language query and selected tool, return what arguments would be used:

```json
{
  "query": "create an issue in my omniconnect repo",
  "tool": "github_create_issue"
}

// Response
{
  "arguments": {
    "owner": "my-username",
    "repo": "omniconnect",
    "title": "Bug: issue title",
    "body": "issue body"
  },
  "missing_params": ["title", "body"],
  "explanation": "I extracted owner=my-username and repo=omniconnect from your query. Please provide title and body."
}
```

## SDK Design

Each SDK gets an `Llm` manager with a single `execute(query)` method.

### Python

```python
from ominiconnect import OminiConnect

client = OminiConnect(api_key="sk-xxxxx")

# Natural language query
result = client.llm("list my github repos sorted by updated")
print(result["result"]["body"])

# With platform hint (faster routing)
result = client.llm("list my repos", platform="github")

# Check what tools are available for natural language selection
catalog = client.llm.list_available_tools()
print(catalog["platforms"]["github"]["tools"][0]["example_queries"])
```

### TypeScript

```typescript
const client = new OminiConnect({ apiKey: "sk-xxxxx" });

// Natural language query
const result = await client.llm("list my github repos sorted by updated");
console.log(result.result?.body);

// With platform hint
const result2 = await client.llm("create an issue", { platform: "github" });

// Get available tools for external LLM selection
const catalog = await client.llm.listAvailableTools();
```

### Go

```go
client := ominiconnect.New("sk-xxxxx", nil)

// Natural language query
result, err := client.Llm().Execute("list my github repos sorted by updated", nil)
if err != nil {
    log.Fatal(err)
}
fmt.Println(result.Result.Body)

// With platform hint
result, err = client.Llm().Execute("create an issue", map[string]string{"platform": "github"})
```

### Rust

```rust
let client = Client::new("sk-xxxxx", None);

// Natural language query
let result = client.llm()
    .execute("list my github repos sorted by updated", None)
    .await?;
println!("{:?}", result.result);

// With platform hint
let result = client.llm()
    .execute("create an issue", Some(LlQueryOptions { platform: Some("github".into()), .. }))
    .await?;
```

### Flutter

```dart
final client = OminiConnect(apiKey: 'sk-xxxxx');

// Natural language query
final result = await client.llm.execute('list my github repos sorted by updated');
print(result.result?.body);
```

### Android (Kotlin)

```kotlin
val client = OminiConnect("sk-xxxxx")

// Natural language query
val result = client.llm.execute("list my github repos sorted by updated")
println(result.result?.body)
```

### iOS (Swift)

```swift
let client = OminiConnect(apiKey: "sk-xxxxx")

// Natural language query
let result = try await client.llm.execute("list my github repos sorted by updated")
print(result.result?.body)
```

## Implementation Strategy

### Phase 1: Rule-Based Tool Selection (No External LLM)

Implement tool selection using keyword/embedding similarity. This works for many queries without calling any external LLM API.

**Algorithm:**
1. Tokenize query (lowercase, remove stop words)
2. Score each tool by:
   - Keyword match in description/name (30%)
   - Platform match (20%)
   - Action verb match (GET → list/search, POST → create, DELETE → remove/delete) (30%)
   - Query similarity to `example_queries` (20%)
3. If top score > threshold → use that tool
4. If top score < threshold → return candidates for disambiguation
5. If multiple tools tied → return ambiguous error

**Tool definition gets new field:**
```yaml
# tools/registry/github.yaml
- slug: github_list_repos
  name: List Repositories
  description: List repositories for the authenticated user
  provider: github
  endpoint: /user/repos
  method: GET
  example_queries:
    - "list my repos"
    - "show my github repositories"
    - "what repos do I have"
  ...
```

**Argument extraction:**
- Parse query for known path parameters (owner, repo names)
- Use default values for optional params
- Return `missing_params` if required args can't be extracted

### Phase 2: Claude-Assisted Selection (Optional, Configurable)

For complex queries where rule-based matching fails, optionally call Claude:

**Configuration:**
```bash
ANTHROPIC_API_KEY=sk-ant-...  # optional, enables AI-assisted selection
LLM_FALLBACK_THRESHOLD=0.5    # use Claude if top score < 0.5
```

**Flow:**
1. If `ANTHROPIC_API_KEY` is set and rule-based confidence < threshold:
   - Build prompt with query + available tools (from `/api/llm/tools`)
   - Call Claude with tool-use system prompt
   - Extract tool selection and arguments from Claude response
   - Execute tool
2. Otherwise, return rule-based result or ambiguous error

### Phase 3: Streaming Responses

For long-running tool executions, stream results back via SSE:

```json
// GET /api/llm/sse
event: tool_selected
data: {"tool": "github_list_repos", "confidence": 0.92}

event: executing
data: {"status": "calling_github_api"}

event: result
data: {"ok": true, "body": [...], "duration_ms": 234}
```

## File Structure Changes

```
portal/src/
  ├── api/
  │   ├── llm.rs          # NEW: LLM integration endpoint
  │   ├── mod.rs
  │   └── ...
  ├── tools.rs            # Add example_queries field to Tool
  └── ...

sdk/
  ├── python/ominiconnect/
  │   ├── llm.py          # NEW: LlmManager
  │   └── client.py       # Add client.llm property
  ├── js/src/
  │   ├── llm.ts          # NEW: LlmManager
  │   └── client.ts       # Add client.llm property
  ├── go/ominiconnect/
  │   ├── llm.go          # NEW: LlmManager
  │   └── client.go       # Add client.Llm() method
  ├── rust/src/
  │   ├── llm.rs          # NEW: LlmManager
  │   └── lib.rs          # Add client.llm() method
  ├── flutter/lib/
  │   ├── llm.dart        # NEW: LlmManager
  │   └──ominiconnect.dart
  ├── android/ominiconnect/src/main/java/ai/ominiconnect/
  │   ├── LlmManager.kt  # NEW
  │   └── OminiConnect.kt # Add llm property
  └── ios/Sources/OminiConnect/
      ├── LlmManager.swift  # NEW
      └── OminiConnect.swift # Add llm property
```

## Configuration

```bash
# .env
ANTHROPIC_API_KEY=           # Optional: enables Claude-assisted selection
LLM_FALLBACK_THRESHOLD=0.5  # Rule-based confidence threshold to trigger Claude
LLM_DEFAULT_PLATFORM=        # Default platform if not specified in query
```

## Security Considerations

1. **Scope enforcement**: The LLM endpoint still respects OAuth scopes — if the connected account doesn't have `repo` scope, `github_list_repos` returns `scope_satisfied: no` and execution fails
2. **No new attack surface**: The endpoint uses existing auth (Bearer token), existing tool execution, existing proxy — no new privileged operations
3. **Audit logging**: Log all LLM queries and tool selections for security review
4. **Rate limiting**: Apply existing rate limits to LLM endpoint

## Backwards Compatibility

- Existing tools API (`/api/tools`, `/api/tools/search`, `/api/tools/execute`) unchanged
- Existing MCP endpoint unchanged
- SDK `llm()` manager is additive — no breaking changes to existing APIs

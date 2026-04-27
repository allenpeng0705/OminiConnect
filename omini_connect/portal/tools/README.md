# Tool Registry

This directory contains tool definitions and generation tooling for OminiConnect's LLM-accessible API registry.

## Structure

```
tools/
├── registry/          # Tool YAML definitions (loaded at runtime)
│   └── github.yaml    # GitHub tools (9 allowlisted operations)
├── generator/        # OpenAPI → tool generator binary
│   └── src/main.rs
└── specs/            # Downloaded OpenAPI specs for reference
```

## Tool Registry Format

Tools are defined in YAML files, one per provider:

```yaml
- slug: github_list_repos
  name: List Repositories
  description: |
    List repositories for the authenticated user...
  provider: github
  endpoint: /user/repos
  method: GET
  scopes:
    - repo
  tags:
    - repositories
    - code
  input_schema:
    type: object
    properties:
      sort:
        type: string
        enum: [created, updated, pushed, full_name]
    required: []
```

## Generating Tools from OpenAPI

Use the `generate-tools` binary to generate tool YAML from OpenAPI specs:

```bash
# Generate with explicit operation ID allowlist
cargo run --bin generate-tools -- \
  --spec ./specs/github.json \
  --provider github \
  --output ./registry/github.yaml \
  --operation-ids "list_repos,get_user,create_issue"

# Generate with tag filter
cargo run --bin generate-tools -- \
  --spec ./specs/slack.json \
  --provider slack \
  --output ./registry/slack.yaml \
  --tags "channels,messages,users"

# Generate with prefix filter
cargo run --bin generate-tools -- \
  --spec ./specs/github.json \
  --provider github \
  --output ./registry/github.yaml \
  --prefixes "list_,get_,create_"
```

## Scope Filtering

Tools have a `scope_satisfied` field in the API response:

- `"yes"` - Connector has all required scopes
- `"no"` - Connector is missing required scopes
- `"unknown"` - No connector configured or scopes unknown

## API Endpoints

- `GET /api/tools?platform=github` - List tools for a platform (supports `?platform=` filter)
- `GET /api/tools/search?q=list` - Search tools by name, description, or tags
- `POST /api/tools/execute` - Execute a tool (sync or async via `callback_url`)
- `POST /api/mcp` - MCP JSON-RPC endpoint (`tools/list`, `tools/call`)
- `GET /api/mcp/sse` - SSE stream for async push to connected clients

## Tool Execution Audit

Every `POST /api/tools/execute` call is recorded in the `tool_executions` table with:
- `call_id`, `agent_id`, `tool_slug`, `platform`, `arguments`
- `status` (success/error), `duration_ms`, `result_body`
- `created_at` timestamp

Async calls (with `callback_url`) are recorded immediately and updated when the callback fires.

## Adding a New Provider

1. Obtain the provider's OpenAPI spec
2. Store it in `specs/` for reference
3. Run the generator with an appropriate allowlist
4. The generated YAML is loaded automatically at startup

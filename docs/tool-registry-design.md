# Tool Registry Design for OminiConnect

## Goal
Make OminiConnect APIs accessible to LLMs via a Composio-inspired Tool Registry.

---

## Alignment with OminiConnect (read this first)

**Identity (replace `connected_account_id`)**  
The portal proxy today resolves **`Authorization: Bearer <API key>` → API key owner → `connectors` row**. There is no `connected_account_id` in the data model. For tools, use:

- **`platform`** — the connector’s `platform` string (e.g. `github`, `github-abc12` for a managed slug, or `feishu` for native). This is the same segment used in the proxy path.
- **Optional: `owner_username`** — only if you add a different auth model later; for v1, the API key implies the owner.

**Discovery and execution** should therefore take `platform` (and the same Bearer API key) rather than a separate opaque account id, unless you introduce a new id in the DB on purpose.

**Proxy URL and HTTP method**  
The live route is `GET,POST /api/ominiconnect/proxy/:platform/*path` (see `omini_connect/portal/src/api/mod.rs` and `proxy::forward`).

- The **path segment** `platform` must match a configured connector for that API key.
- The **HTTP method** for the upstream call must be the tool’s `method` (`GET` for `list repos`, etc.). The design must not assume everything is `POST` to the proxy; the execute handler should call the same forwarder with the tool’s method.
- Nango engines use `connector.provider_key` + `connection_ref`; native engines use the OAuth vault. The tool’s `provider` field is best documented as the **proxy `platform` value** (and Nango *provider key* when they match, e.g. `github`).

**Path templates**  
OpenAPI paths use `{param}` placeholders. Execution must **substitute** `arguments` into the path, query, and body per operation (the illustrative generator below does not do this; implement explicitly).

**Scope filtering (realistic v1)**  
“Verify all scopes granted” requires reading **effective granted scopes** from Nango and/or the token. If that is expensive or missing, v1 can: (a) filter using **connector-configured** scopes from the DB, (b) return tools with a `scope_satisfied: "unknown" | "yes" | "no"` flag, or (c) skip filtering and document the risk. The doc below assumes the strict case once data exists.

**YAML location**  
Recommend a single repo path, e.g. `tools/registry/*.yaml` or `omini_connect/portal/tools/*.yaml`, and load at startup; document it when implementing Phase 1.

---

## Core Concepts

### Tool Definition
Inspired by Composio's `Tool` schema:

```typescript
interface Tool {
  slug: string;                    // "github_list_repos"
  name: string;                   // "List Repositories"
  description: string;            // "List all repositories for the authenticated user..."
  provider: string;                // Proxy :platform (often matches Nango provider key when not a slug)
  endpoint: string;               // "/user/repos" — may include {placeholders} for path params
  method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
  inputSchema: JSONSchema;         // OpenAPI parameter schema
  outputSchema?: JSONSchema;       // Response schema
  scopes: string[];               // Required OAuth scopes
  tags?: string[];                // ["code", "repository"]
  icon_url?: string;               // Provider logo
}
```

### Toolkit Concept
Groups tools by provider (like Composio):
```typescript
interface Toolkit {
  slug: string;        // "github"
  name: string;        // "GitHub"
  logo?: string;       // "/images/template-logos/github.svg"
  provider: string;    // Same as tool.provider / proxy :platform
}
```

---

## Tool Storage

### Option A: YAML Files (Initial/Simple)
Store tool definitions in YAML files, one per provider:
```
tools/
  github.yaml    # All GitHub tools
  slack.yaml     # All Slack tools
  notion.yaml    # All Notion tools
```

**Pros**: Simple, version controllable, easy to audit
**Cons**: Manual maintenance for 700+ providers

### Option B: Database (Scalable)
Store in PostgreSQL with tooling for generation:
```sql
CREATE TABLE tools (
  id UUID PRIMARY KEY,
  slug VARCHAR(255) UNIQUE NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  provider VARCHAR(100) NOT NULL,
  endpoint VARCHAR(500) NOT NULL,
  method VARCHAR(10) NOT NULL,
  input_schema JSONB,
  output_schema JSONB,
  scopes TEXT[],
  tags TEXT[],
  icon_url VARCHAR(500),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE toolkits (
  slug VARCHAR(100) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  logo VARCHAR(500),
  provider VARCHAR(100) NOT NULL
);
-- Add REFERENCES providers(key) only if you introduce a real providers table.
```

### Recommended: Start with YAML, migrate to DB later

---

## Tool Discovery API

### List tools for a configured connector
```
GET /api/tools?platform=github
```

Same **`Authorization: Bearer <API key>`** as the proxy. Optional query **`platform`**: if omitted, return toolkits for **all** connectors that exist for this key (or return 400 with a message—pick one behavior and document it).

Returns tools whose **required** scopes are satisfied (see *Scope filtering* under *Alignment*). If scope verification is not implemented yet, return all tools for that provider and optionally a `scope_filter: "none"` in the response meta.

```json
{
  "toolkits": [
    {
      "slug": "github",
      "name": "GitHub",
      "logo": "/images/template-logos/github.svg",
      "tools": [
        {
          "slug": "github_list_repos",
          "name": "List Repositories",
          "description": "List all repositories for the authenticated user",
          "method": "GET",
          "endpoint": "/user/repos",
          "inputSchema": {
            "type": "object",
            "properties": {
              "sort": { "type": "string", "enum": ["created", "updated", "full_name"] }
            }
          }
        }
      ]
    }
  ]
}
```

### Search tools
```
GET /api/tools/search?q=list+repos&platform=github
```

Same auth as above. `q` matches name, description, slug, and/or tags (define which in implementation).

### Get specific tool
```
GET /api/tools/github_list_repos
```

Decide whether this requires the same API key and whether metadata is public; typically **metadata is not sensitive** but documenting slugs is still useful for agents.

---

## Tool Execution

Leverage existing proxy infrastructure:

```
POST /api/tools/execute
Authorization: Bearer <API key>
{
  "tool_slug": "github_list_repos",
  "platform": "github",
  "arguments": { "sort": "updated" }
}
```

**`platform`** must match a connector the key may use (same as proxy). For a single GitHub connection, it is the stored connector `platform` (e.g. `github` or a managed `github-xyz`).

Backend flow:
1. Authenticate API key → owner (same as `proxy::forward`).
2. Load connector for `(owner, platform)`; ensure enabled; ensure engine/token path matches tool (`nango` vs native) — same preconditions as the proxy.
3. Look up tool by slug → `endpoint`, `method`, `scopes`, path/query/body binding rules.
4. (Optional) Verify scopes if data is available; else enforce documented v1 policy.
5. Build **native path and body**: substitute `arguments` into path templates; append query; serialize JSON body for POST/PUT/PATCH.
6. **Invoke the same internal forward logic** as `GET/POST /api/ominiconnect/proxy/{platform}/{native_path}` with the tool’s **method** (not always POST).
7. Return upstream status + body (and normalize errors for LLM clients).

---

## OpenAPI → Tool Generation Pipeline

### For providers with OpenAPI specs:
1. Fetch OpenAPI spec (e.g., from SwaggerHub, provider docs)
2. Parse operations → generate tool definitions
3. Cross-reference with Nango `available_scopes`
4. Filter tools to only those matchable to scopes

### For providers without specs:
- Start with minimal tool definitions (endpoint + method only)
- Gradually enrich descriptions manually

### Generation Script (conceptual)
Illustrative only — OpenAPI 3’s `paths` is an object; operations live under `paths[pathKey][method]`. You must:
- allowlist paths/tags/operationIds so GitHub-scale specs do not create thousands of tools;
- merge `parameters` and requestBody into a single `inputSchema` (or split by `in`);
- map path `{vars}` to JSON Schema `arguments` properties;
- produce stable **snake_case** slugs, e.g. `github_list_repos` (document one convention; avoid mixing with `GITHUB_LIST_REPOS`).

```typescript
// Pseudocode — do not copy literally
function generateToolsFromOpenAPI(
  spec: OpenAPI3,
  provider: string,
  scopeMappings: Record<string, string[]>
): Tool[] {
  // iterate Object.entries(spec.paths), then each operation get/post/...
  // build path template, method, combined inputSchema, scopes, tags
  return [];
}
```

---

## Implementation Plan

### Phase 1: Core Infrastructure ✅ (Done)
1. ✅ Create tool types (`src/tools.rs`): `Tool`, `Toolkit`, `ToolRegistry`, `HttpMethod`
2. ✅ Fixed YAML directory: `tools/registry/*.yaml`, loaded at startup with fail-fast validation
3. ✅ Created `github.yaml` with 9 allowlisted tools (list repos, list issues, get user, etc.)
4. ✅ Implemented `GET /api/tools?platform=xxx` endpoint (Bearer auth, returns toolkits + tools)
5. ✅ Implemented `POST /api/tools/execute` (uses existing proxy patterns with correct HTTP method, path substitution, query/body routing)

**Files created:**
- `src/tools.rs` - Tool types and registry loader
- `src/api/tools.rs` - API handlers (list, execute)
- `tools/registry/github.yaml` - GitHub tool definitions (9 tools)

### Phase 2: Tool Generation ✅ (Done)
1. ✅ Built OpenAPI → tool generator (`tools/generator/src/main.rs`) with allowlist support:
   - `--operation-ids` for explicit operation ID filtering
   - `--tags` for tag-based filtering
   - `--prefixes` for operationId prefix filtering
2. ✅ Scope filtering implemented:
   - `scope_satisfied: "yes" | "no" | "unknown"` added to tool response
   - Uses connector-configured scopes from DB
3. ✅ Expand to all providers — 886 provider YAML files in `tools/registry/`
   - Generator used to produce tool definitions for the full provider catalog
   - All providers now have tool definitions loaded at startup

### Phase 3: LLM UX ✅ (Partial)
1. ✅ Implemented search endpoint `GET /api/tools/search?q=...` (keyword search across name, description, tags, slug)
2. ✅ MCP server endpoint at `POST /api/mcp` with JSON-RPC 2.0:
   - `tools/list` - Returns available tools in MCP format
   - `tools/call` - Executes a tool
   - Same auth as other endpoints (Bearer token)
3. ⏳ **LLM-optimized tool descriptions** — pending
   - Descriptions are currently minimal (endpoint paths, basic summaries)
   - Need to enrich descriptions across all 886 provider YAML files
   - Use LLM to generate clear descriptions that help LLM agents select the right tool

---

## Composio Patterns to Reuse

1. **Tool slug format**: use one convention — e.g. lowercase snake `github_list_repos` (matches JSON examples above; avoid UPPER_SCREAMING unless you switch all examples)
2. **Toolkit grouping**: Tools grouped by provider
3. **Scope-based filtering**: Only show tools user has scopes for
4. **SessionContext for execution**: Track which tools were called in a session

## Composio Patterns to Adapt

1. **Input parameters**: Use JSON Schema (like Composio)
2. **Tool execution**: Leverage existing proxy, don't reinvent auth
3. **No version control initially**: Start simple, add versions if needed

# OminiConnect User Stories

This document captures real-world use cases to validate whether OminiConnect's design and implementation can support them. Each story includes the user goal, how they interact with OminiConnect, and the technical flow.

---

## User Personas

1. **Developer** — Building an application that needs OAuth for providers (Slack, GitHub, etc.)
2. **AI/LLM Developer** — Building AI agents that need to call external services
3. **Enterprise IT** — Managing API access for compliance and security
4. **End User** — Connecting their accounts (GitHub, Slack, etc.) to be used by agents

---

## Story 1: Developer connects Slack for a simple bot

### Goal
Get a Slack token so my backend can post messages to my company's Slack workspace.

### Flow
```
1. Developer registers at OminiConnect portal
2. Goes to "Connectors" → "Slack"
3. OAuth flow: authorizes OminiConnect to access "chat:write"
4. OminiConnect stores token
5. Developer calls: GET /api/token/slack
   → Returns { access_token: "xoxb-xxx", scope: "chat:write", expires_in: 7200 }
6. Developer uses token to call Slack API directly
7. Later: token expires → call again → OminiConnect auto-refreshes
```

### OminiConnect Value
- OAuth flow handled (no need to implement Slack OAuth yourself)
- Token auto-refresh (OminiConnect keeps token valid)
- Scope filtering (only got `chat:write`, not `admin`)

### Requirements Met
- ✅ OAuth handling
- ✅ Token storage + auto-refresh
- ✅ Scope filtering at token issuance
- ❌ Audit logging (optional for this case)

---

## Story 2: AI Agent discovers and uses tools via MCP

### Goal
Build an AI agent that can help users with GitHub and Slack tasks. The agent should discover available tools and call them without human intervention.

### Flow
```
1. End User connects GitHub and Slack via OminiConnect portal (OAuth)
2. AI Developer writes agent code:
   ```python
   from ominiconnect import OminiConnect

   client = OminiConnect(api_key="agent-key")

   # Agent asks: what tools do I have?
   tools = client.tools.list()
   # Returns: github_list_repos, github_create_issue, slack_post_message, ...

   # Agent decides to use github_list_repos
   result = client.tools.execute("github_list_repos", {"sort": "updated"})

   # Agent posts to Slack
   result = client.tools.execute("slack_post_message", {
     "channel": "#engineering",
     "text": "New issues found!"
   })
   ```

3. Agent uses MCP endpoint directly:
   - `tools/list` → discovers available tools (with natural language descriptions)
   - `tools/call` → executes selected tool

### OminiConnect Value
- Tool discovery (LLM knows what's available)
- Scope enforcement (agent can only use tools for connected accounts)
- Token management (doesn't need to handle OAuth)
- Natural language descriptions (LLM understands what each tool does)

### Requirements Met
- ✅ OAuth + token management
- ✅ Tool registry with descriptions
- ✅ MCP endpoint for discovery/execution
- ✅ Scope enforcement at execution
- ✅ Multi-provider in single interface

---

## Story 3: Developer uses OminiConnect as API Gateway

### Goal
Build a frontend that calls GitHub API. Can't call GitHub directly due to CORS.

### Flow
```
1. User connects GitHub (OAuth)
2. Frontend calls:
   POST /api/call/github
   { "method": "GET", "path": "/user/repos", "params": {"sort": "updated"} }

   OminiConnect:
   - Injects stored token as Authorization header
   - Forwards to GitHub
   - Returns response

3. Frontend gets data without CORS issues
```

### OminiConnect Value
- CORS workaround (provider doesn't support cross-origin)
- Token injection (don't expose token to frontend)
- Same interface for different providers

### Requirements Met
- ✅ API proxy with token injection
- ✅ Token management

---

## Story 4: Enterprise enforces data residency

### Goal
Company policy: No user data can leave specific regions. Need PII filtering and data residency enforcement.

### Flow
```
1. IT Admin configures OminiConnect with:
   - Data residency: "US only"
   - PII filter: mask phone numbers, emails
   - Allowed scopes per department

2. Employee connects their Google account
   - Token stored with scope "email,contacts"
   - Employee is in "sales" department → gets additional scope "drive.read"

3. Agent calls tool:
   POST /api/tools/execute
   { "tool_slug": "google_list_contacts" }

   OminiConnect:
   - Checks scope (employee has "contacts" scope → allowed)
   - Executes Google API call
   - PII filter: masks phone numbers, emails
   - Audit log: who called what, when, result
   - Response: contact list with PII masked

4. Compliance team reviews audit logs
```

### OminiConnect Value
- Scope enforcement per user/department
- PII scrubbing (wasm_policies)
- Data residency rules
- Full audit trail

### Requirements Met
- ✅ OAuth + token management
- ✅ Scope enforcement
- ✅ Audit logging
- ✅ wasm_policies (PII filter, content moderation)
- ✅ Tool execution with transformations

---

## Story 5: LLM selects best tool from multiple providers

### Goal
LLM needs to handle a complex task that requires multiple tools from different providers. LLM should figure out which tools to use based on natural language.

### Flow
```
User: "Notify the team about the latest GitHub issues in our repo"

LLM → OminiConnect:
POST /api/llm
{ "query": "notify team about latest GitHub issues" }

OminiConnect (rule-based):
1. Detects platform: "github" (keyword match)
2. Finds tools: github_list_issues (score: 0.9), github_create_issue (0.6)
3. Selects: github_list_issues
4. Extracts arguments: { repo: "my-org/my-repo", state: "open" }
5. Executes tool
6. Gets issues
7. Formats notification

Returns:
{
  "tool": "github_list_issues",
  "arguments": {"repo": "my-org/my-repo", "state": "open"},
  "result": {
    "ok": true,
    "body": [{"title": "Fix bug", "number": 123}, {"title": "Add feature", "number": 124}]
  }
}

LLM then posts to Slack using slack_post_message tool
```

### OminiConnect Value
- Natural language tool selection
- Multi-provider orchestration (LLM can discover across all connected providers)
- Ambiguity handling (if multiple tools match, returns candidates for disambiguation)

### Requirements Met
- ✅ Tool registry
- ✅ Natural language descriptions
- ✅ LLM endpoint for tool selection
- ✅ Multi-provider discovery
- ✅ Tool execution

---

## Story 6: User creates multiple API keys for different agents

### Goal
I have multiple AI agents (code reviewer, social media manager, customer support). Each should have its own API key with different scopes.

### Flow
```
1. User logs into OminiConnect portal
2. Connects: GitHub, Slack, Gmail

3. Creates API keys:
   - "code-reviewer" → allowed tools: github_list_repos, github_create_issue
   - "social-media" → allowed tools: twitter_post, linkedin_post, facebook_post
   - "support-agent" → allowed tools: gmail_read, zendesk_create_ticket

4. Each agent gets its own API key:
   agent1 = OminiConnect(api_key="sk-om-code-reviewer-xxx")
   agent2 = OminiConnect(api_key="sk-om-social-media-xxx")
   agent3 = OminiConnect(api_key="sk-om-support-agent-xxx")

5. code-reviewer agent tries to post to Twitter:
   → Rejected! Scope not allowed for this API key

6. support-agent reads Gmail:
   → Allowed! Gmail connected and scope permitted
```

### OminiConnect Value
- Named API keys per agent/use case
- Tool-level permissions per API key
- Audit: know which agent did what

### Requirements Met
- ✅ API key management with labels
- ✅ Tool-level permissions
- ✅ Audit logging
- ✅ MCP/tool discovery with scope filtering

---

## Story 7: Developer builds custom tool via tool registry

### Goal
Developer has a custom internal API. They want to register it in OminiConnect so agents can use it.

### Flow
```
1. Developer writes OpenAPI spec for their internal API
2. Creates tool definition:
   ```yaml
   # portal/tools/registry/my-internal-api.yaml
   - slug: my_api_create_ticket
     name: Create Support Ticket
     description: Create a ticket in our internal support system
     provider: my-internal
     endpoint: /tickets
     method: POST
     input_schema:
       type: object
       properties:
         title: { type: string }
         description: { type: string }
         priority: { type: string, enum: ["low", "medium", "high"] }
     scopes: ["tickets:write"]
   ```

3. Registers via API or portal UI

4. Agent discovers and uses:
   tools = client.tools.list()
   # Includes my_api_create_ticket

   result = client.tools.execute("my_api_create_ticket", {
     "title": "Cannot login",
     "priority": "high"
   })
```

### OminiConnect Value
- Unified tool discovery interface
- Custom tools alongside OAuth-connected providers
- Scope enforcement for custom tools too

### Requirements Met
- ✅ Tool registry
- ✅ Custom tool registration
- ✅ MCP/tool execution for custom tools

---

## Story 8: Token-first for trusted agents

### Goal
Trusted internal agent should call provider APIs directly for performance. OminiConnect just delivers tokens.

### Flow
```
1. User connects GitHub (scopes: repo, user)

2. Agent starts, gets token:
   token = client.tokens.get("github")
   # Returns: { access_token: "ghs_xxx", expires_in: 3600 }

3. Agent calls GitHub directly:
   requests.get("https://api.github.com/user/repos",
     headers={"Authorization": f"Bearer {token}"})

4. If token expires:
   token = client.tokens.get("github")  # OminiConnect auto-refreshes
   # Returns: { access_token: "ghs_xxx_new", expires_in: 3600 }
```

### OminiConnect Value
- Token vault without proxy overhead
- Performance: no bottleneck
- Full control: agent handles rate limits, retries

### Requirements Met
- ✅ OAuth + token management
- ✅ Token delivery endpoint (no proxy)
- ✅ Token auto-refresh

---

## Summary: Capability Matrix

| Capability | Story 1 | Story 2 | Story 3 | Story 4 | Story 5 | Story 6 | Story 7 | Story 8 |
|-----------|---------|---------|---------|---------|---------|---------|---------|---------|
| OAuth + Token Storage | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | | ✅ |
| Token Auto-Refresh | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | | ✅ |
| API Proxy | | | ✅ | | | | | |
| Tool Registry | | ✅ | | | ✅ | ✅ | ✅ | |
| MCP Endpoint | | ✅ | | | ✅ | ✅ | ✅ | |
| LLM Endpoint | | | | | ✅ | | | |
| Scope Enforcement | ✅ | ✅ | | ✅ | ✅ | ✅ | ✅ | |
| Audit Logging | | ✅ | | ✅ | | ✅ | | |
| wasm_policies (PII) | | | | ✅ | | | | |
| Custom Tool Registration | | | | | | | ✅ | |
| Multi-Provider | | ✅ | | | ✅ | ✅ | | |

---

## Questions to Validate

### 1. Token Delivery vs Proxy
- Stories 1, 8 want token-only (direct to provider)
- Stories 2, 4, 5 want proxy (scope enforcement, audit)
- **Decision needed**: Do we support both modes?

### 2. Tool Scope vs API Key Scope
- Story 6 uses API key permissions
- Story 4 uses user/department scopes
- **Current state**: Tools have `scopes` field, API keys are separate
- **Question**: Should API key permissions override tool permissions?

### 3. Custom Tools
- Story 7: User registers custom tools
- **Current state**: Tool registry exists in `portal/tools/registry/`
- **Question**: Is the custom tool registration flow user-friendly enough?

### 4. LLM Integration
- Story 5: Natural language tool selection
- **Current state**: Rule-based, works for simple cases
- **Question**: Is rule-based sufficient or do we need actual LLM (Claude) for complex cases?

---

## Architecture Recommendations

### Two Modes for API Access

**Mode A: Token Delivery** (for trusted clients)
```
GET /api/token/{platform}
→ Returns fresh token (auto-refresh if needed)
→ Client calls provider directly
```
Use case: Story 1, Story 8

**Mode B: Tool Execution** (for LLMs and untrusted clients)
```
POST /api/tools/execute
→ OminiConnect executes with scope enforcement, audit
→ Returns result
```
Use case: Story 2, Story 4, Story 5, Story 6

### API Gateway Mode (Optional)
```
POST /api/call/{platform}
→ Proxy with token injection
→ Optional transformations
```
Use case: Story 3 (when CORS not supported)

### Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| OAuth + Token Storage | ✅ | oauth_vault crate |
| Token Auto-Refresh | ✅ | oauth_vault handles this |
| Tool Registry | ✅ | portal/tools/registry/*.yaml |
| MCP Endpoint | ✅ | POST /api/mcp |
| LLM Endpoint | ✅ | POST /api/llm |
| API Proxy | ✅ | POST /api/call/{platform} |
| Scope Enforcement | ✅ | At tool execution time |
| Audit Logging | ✅ | audit_logger crate |
| wasm_policies | ✅ | wasm_policies crate |
| Custom Tool Registration | ⚠️ | Manual YAML, no UI |
| Multi-tenant/API Keys | ⚠️ | Basic support, needs refinement |

---

*Last updated: 2026-04-27*
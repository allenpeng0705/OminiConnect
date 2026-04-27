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

## Real World Scenarios

### Scenario A: AI Code Review Agent

**Setup:**
- Company uses GitHub Enterprise (on-premise)
- Development team: 50 engineers
- Need automated code review on PRs

**Without OminiConnect:**
- Build OAuth integration for GitHub Enterprise (complex, on-premise auth)
- Handle token storage and refresh for 50 users
- Scope management per user/department
- Build tool registry so agent knows what it can do

**With OminiConnect:**
```
1. IT Admin registers GitHub Enterprise as connector
2. Engineers connect their GitHub accounts via OAuth
3. AI Agent code:
   tools = client.tools.list(platform="github")
   # Discovers: github_list_pulls, github_create_review_comment, ...

   result = client.tools.execute("github_list_pulls", {
     "owner": "my-company",
     "repo": "backend-api",
     "state": "open"
   })
```

**Value:**
- 50+ OAuth integrations handled by OminiConnect
- Token auto-refresh (no expired token errors)
- Scope enforcement (intern can't access private repos outside company)

---

### Scenario B: Sales AI Assistant with Multi-Platform

**Setup:**
- Sales team uses: Salesforce (CRM), Slack (comms), Gmail (email)
- AI assistant helps: prepare meeting notes, update CRM, send follow-ups

**Without OminiConnect:**
- Integrate Salesforce OAuth + API
- Integrate Slack OAuth + API
- Integrate Gmail OAuth + API
- Each has different auth flow, token format, refresh logic

**With OminiConnect:**
```
1. Sales rep connects all three via OminiConnect portal (one OAuth per platform)
2. AI Agent discovers tools across platforms:
   tools = client.tools.list()
   # Returns: salesforce_get_lead, salesforce_update_opportunity,
   #          slack_post_message, gmail_send_email, ...

3. Agent workflow:
   # Get lead info
   lead = client.tools.execute("salesforce_get_lead", {"id": "00Qxx0000001234"})

   # Create Slack message
   client.tools.execute("slack_post_message", {
     "channel": "#sales-team",
     "text": f"Meeting prep: {lead['Name']} - {lead['Company']}"
   })

   # Log activity in Salesforce
   client.tools.execute("salesforce_update_opportunity", {
     "id": "006xx0000012345",
     "stage": "Closed Won"
   })
```

**Value:**
- Single interface for all platforms
- Agent doesn't need to know about Salesforce vs Gmail API differences
- Scope: sales rep can only access their assigned leads

---

### Scenario C: Enterprise Compliance Dashboard

**Setup:**
- Fortune 500 company
- Must audit all API calls for SOC2 compliance
- Data residency: EU only
- PII must be filtered

**Without OminiConnect:**
- Build logging system for every API call
- Store all API responses with timestamps
- Filter PII manually in each integration
- Ensure no data leaves EU

**With OminiConnect:**
```
1. IT configures in portal:
   - Data residency: EU
   - PII filter: email, phone, SSN
   - Audit: all tool executions logged

2. Employee connects their Google Workspace
   - OminiConnect stores token with scopes: "email.read", "contacts.read"

3. Agent calls tool:
   POST /api/tools/execute
   { "tool_slug": "google_list_contacts" }

4. OminiConnect:
   - Checks scope (email.read → allowed)
   - Calls Google API
   - PII filter: masks {email}, {phone}
   - Audit log: { "user": "john@company.com", "tool": "google_list_contacts", "timestamp": "..." }
   - Ensures response stays in EU data center

5. Compliance team reviews audit trail
```

**Value:**
- Automatic audit logging for every tool execution
- PII scrubbing without building custom filters
- Scope enforcement per employee
- Data residency enforcement

---

### Scenario D: Claude Desktop Integration

**Setup:**
- Developer uses Claude Desktop
- Wants Claude to help with GitHub issues and Slack notifications

**Without OminiConnect:**
- Configure Claude Desktop MCP manually for each service
- Manage multiple MCP server configurations
- No scope control (Claude has full token access)

**With OminiConnect:**
```
1. User connects GitHub and Slack via portal
2. Adds to claude_desktop_config.json:
   {
     "mcpServers": {
       "OminiConnect": {
         "command": "npx",
         "args": ["-y", "@ominiconnect/sdk-mcp"]
       }
     }
   }

3. Claude Desktop automatically discovers tools via MCP:
   - tools/list → returns all available tools
   - tools/call → executes with token management

4. User asks Claude:
   "Create a GitHub issue for the login bug and notify the team on Slack"

   Claude → OminiConnect MCP:
   - tools/call: github_create_issue → success
   - tools/call: slack_post_message → success
```

**Value:**
- One MCP server for all connected platforms
- Scope: limited to what user authorized
- Token auto-refresh handled
- No need to configure multiple MCP servers

---

### Scenario E: China Market AI Agent

**Setup:**
- Company expanding to China
- Need AI agent that works with: Feishu, DingTalk, WeChat Work

**Without OminiConnect:**
- Implement OAuth for each Chinese platform (different flows, different UAs)
- Handle token storage and refresh
- Build Chinese-language tool descriptions for LLM

**With OminiConnect:**
```
1. Admin connects company accounts:
   - Feishu (飞书) - enterprise IM
   - DingTalk (钉钉) - enterprise comms
   - WeChat Work (企业微信) - enterprise WeChat

2. AI Agent discovers Chinese platform tools:
   tools = client.tools.list()
   # Returns: feishu_send_message, dingtalk_create_task,
   #          wechatwork_send_message, ...

3. Agent responds to user in Chinese:
   "好的，我会在飞书群发送会议提醒"

   client.tools.execute("feishu_send_message", {
     "chat_id": "oc_xxxxx",
     "content": "会议提醒：明天下午3点"
   })
```

**Value:**
- Built-in support for Chinese platforms (Feishu, DingTalk, WeChat Work)
- Handles domestic OAuth flows (different from Western platforms)
- Chinese-language tool descriptions for LLM
- No need to understand Chinese platform API quirks

---

### Scenario F: Multi-Agent Orchestration

**Setup:**
- Company has 3 AI agents:
  - Code Reviewer: GitHub access only
  - Social Media Manager: Twitter, LinkedIn, Facebook
  - Customer Support: Zendesk, Gmail

**Without OminiConnect:**
- Each agent needs its own OAuth flow and token management
- No centralized audit of which agent did what
- Hard to revoke access per agent

**With OminiConnect:**
```
1. Admin creates 3 API keys in portal:
   - code-reviewer: tools=[github_*]
   - social-media: tools=[twitter_*, linkedin_*, facebook_*]
   - support-agent: tools=[zendesk_*, gmail_*]

2. Each agent gets its own key:
   code_reviewer = OminiConnect(api_key="sk-om-code-reviewer-xxx")
   social_manager = OminiConnect(api_key="sk-om-social-xxx")
   support_agent = OminiConnect(api_key="sk-om-support-xxx")

3. code-reviewer tries to post to Twitter:
   → Blocked! Tool not in code-reviewer key's allowed list

4. support_agent reads Gmail:
   → Allowed! Gmail in support-agent key's allowed list

5. Admin reviews audit logs:
   - "support-agent called gmail_read at 2024-04-27 14:32"
   - "social-media posted to Twitter at 2024-04-27 14:45"
```

**Value:**
- Per-agent API keys with tool restrictions
- Centralized audit log
- Instant revocation (just delete API key)
- No token sharing between agents

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

## Positioning: How OminiConnect Compares to Others

OminiConnect combines features from multiple categories into one platform:

### Comparison Matrix

| Feature | Nango | Maton | Composio | OminiConnect |
|---------|-------|-------|----------|---------------|
| OAuth + Token Management | ✅ 300+ providers | ❌ | ❌ | ✅ 700+ via Nango + built-in |
| Unified API Call (Maton-style) | ❌ | ✅ | ❌ | ✅ |
| LLM Tool Registry | ❌ | ❌ | ✅ | ✅ |
| MCP Endpoint | ⚠️ | ❌ | ✅ | ✅ |
| Natural Language Tool Selection | ❌ | ❌ | ❌ | ✅ |
| Scope Enforcement | ❌ | ❌ | ❌ | ✅ |
| Audit Logging | ❌ | ❌ | ❌ | ✅ |
| wasm_policies (PII, content) | ❌ | ❌ | ❌ | ✅ |

### OminiConnect is Layered

```
┌─────────────────────────────────────────────────┐
│  Layer 4: LLM Natural Language                 │
│  "Notify team about GitHub issues" → Tool     │
├─────────────────────────────────────────────────┤
│  Layer 3: Tool Registry + MCP                  │
│  Discover & execute tools (like Composio)     │
├─────────────────────────────────────────────────┤
│  Layer 2: API Gateway (Maton-style)           │
│  POST /api/call/{platform} - unified call     │
├─────────────────────────────────────────────────┤
│  Layer 1: OAuth + Token Management (like Nango)│
│  Connect once, we handle tokens + refresh      │
└─────────────────────────────────────────────────┘
```

### How Users Choose What to Use

| Use Case | Use OminiConnect For | Example |
|----------|---------------------|---------|
| Need OAuth for Slack/GitHub | Token Delivery | Story 1 |
| Building AI agent | MCP + Tool Registry | Story 2, Story 6 |
| Can't call API due to CORS | API Gateway | Story 3 |
| Need audit/compliance | Tool Execution + Audit | Story 4 |
| Natural language to API | LLM Endpoint | Story 5 |
| Custom internal API | Custom Tool Registry | Story 7 |
| Trusted agent (performance) | Token Delivery | Story 8 |

### The Unique Value

**vs Nango alone:**
- Nango handles OAuth but has no tool discovery for LLMs
- OminiConnect adds: MCP endpoint, tool registry, LLM natural language selection
- User connects their actual accounts (not test accounts)

**vs Maton alone:**
- Maton provides unified call interface but no OAuth/token management
- OminiConnect wraps with: token injection, scope enforcement, audit

**vs Composio alone:**
- Composio gives tools but uses YOUR API keys
- OminiConnect connects USER'S accounts via OAuth → agent works with user's actual data

**vs building it yourself:**
- OAuth for each provider is complex (every provider different)
- Token refresh logic is error-prone
- Scope enforcement requires tracking per-user permissions
- OminiConnect handles all of this

---

## Implementation Gaps

Based on the user stories and scenarios, here are the gaps between current implementation and requirements:

### Critical Gaps (Must Have)

#### 1. Token Delivery Endpoint (Story 1, Story 8, Scenario F)
**Gap:** No `GET /api/token/{platform}` endpoint exists.

**Current state:** Tokens are used internally for tool execution and proxy, but there's no endpoint to just retrieve a fresh token for direct API calls.

**Needed:**
```
GET /api/token/{platform}
Authorization: Bearer sk-xxx

Response: { access_token: "...", expires_in: 3600, scope: "repo,user" }
```

**Why critical:** Story 1 (token-first for trusted agents) and Story 8 (performance-critical direct calls) require this. Also needed for Scenario F (multi-agent with per-agent tokens).

---

#### 2. Per-Agent Tool Restrictions (Scenario F)
**Gap:** API keys exist but have no tool-level permissions.

**Current state:** API key in `auth/middleware.rs` validates the key exists, but doesn't filter which tools the agent can use.

**Scenarios affected:** Scenario F (multi-agent), Story 6

**Needed:**
- API keys with `allowed_tools: ["github_*"]` field
- Middleware checks API key's allowed tools before tool execution
- UI to configure tool restrictions per API key

**Current code to check:** `portal/src/auth/middleware.rs` - needs `allowed_tools` check in `validate_api_key()`.

---

#### 3. Scope Enforcement for Chinese Platforms (Scenario E)
**Gap:** Feishu, DingTalk, WeChat Work OAuth is implemented in `oauth_vault`, but scope enforcement at tool execution is unclear.

**Current state:** `connectors/` crate has feishu, dingtalk, wechatwork, maton - but are they integrated with tool execution scope checking?

**Needed:**
- Verify scopes from OAuth are stored correctly
- Verify tool execution checks scopes against token's allowed scopes
- Chinese platform OAuth flows work correctly

---

#### 4. Custom Tool Registration UI (Story 7)
**Gap:** Custom tools require manual YAML creation in `portal/tools/registry/`.

**Current state:** Developer must create YAML files manually, no portal UI.

**Needed:**
- UI to add custom tools (name, description, endpoint, method, input schema)
- Or at minimum: API endpoint to register tools
- Tool registry auto-reloads when new tools added

---

### Important Gaps (Should Have)

#### 5. Data Residency Enforcement (Scenario C)
**Gap:** No data residency configuration or enforcement.

**Current state:** `wasm_policies` has PII filtering, but no data residency rules.

**Needed:**
- Portal UI to set data residency: "US only", "EU only", "China only"
- Middleware checks response data center based on config
- Tool execution respects residency rules

---

#### 6. Department/Role-based Scopes (Story 4, Scenario C)
**Gap:** User scopes are from OAuth, but no per-department scope mapping.

**Current state:** Token stores OAuth scopes directly, no transformation based on user department/role.

**Needed:**
- Admin can map departments to additional scopes
- Example: "sales" department gets "drive.read" in addition to OAuth scopes
- Token issuance applies department scope mapping

---

#### 7. Audit Log UI (Story 4, Scenario C, Scenario F)
**Gap:** `audit_logger` crate exists, but no UI to view audit logs.

**Current state:** Logs are stored, but compliance team has no way to view them via portal.

**Needed:**
- Audit log page in portal
- Filters: by user, by tool, by date range
- Export to CSV/JSON for compliance

---

### Minor Gaps (Nice to Have)

#### 8. LLM Tool Selection for Complex Queries (Story 5)
**Gap:** Current LLM endpoint uses rule-based keyword matching.

**Current state:** Works for simple queries like "list my repos". May fail for complex multi-step reasoning.

**Needed:**
- Optional Claude integration for complex queries
- Threshold-based fallback: if rule-based confidence < X, use Claude
- Config: `ANTHROPIC_API_KEY`, `LLM_FALLBACK_THRESHOLD`

---

#### 9. MCP SDK Package (Scenario D)
**Gap:** No `@ominiconnect/sdk-mcp` npm package for Claude Desktop.

**Current state:** Claude Desktop config example uses `npx -y @ominiconnect/sdk-mcp` which doesn't exist.

**Needed:**
- Publish `@ominiconnect/sdk-mcp` npm package
- Implements MCP protocol client for OminiConnect
- Handles `tools/list` and `tools/call` JSON-RPC

---

#### 10. Tool Execution Callback/Async (Story 4)
**Gap:** Tool execution is synchronous. Long-running tools block.

**Current state:** `POST /api/tools/execute` returns result immediately.

**Needed:**
- `callback_url` parameter: if provided, execute async and POST result to callback
- `status` field: "completed", "pending", "failed"
- SSE stream for real-time status updates

---

## Quick Wins (Already Mostly There)

| Feature | Status | Notes |
|---------|--------|-------|
| OAuth + Token Storage | ✅ | `oauth_vault` crate works |
| Token Auto-Refresh | ✅ | Handled in `oauth_vault` |
| Tool Registry | ✅ | `portal/tools/registry/*.yaml` |
| MCP Endpoint | ✅ | `POST /api/mcp` works |
| LLM Endpoint | ✅ | `POST /api/llm` rule-based works |
| API Proxy (Maton-style) | ✅ | `POST /api/call/{platform}` |
| Scope Enforcement | ⚠️ | Works for tools, but not per-API-key restrictions |
| wasm_policies (PII) | ✅ | Implemented |
| Audit Logging | ⚠️ | Logs stored, no UI |
| Chinese Platforms | ⚠️ | OAuth works, tool execution unclear |

---

## Recommended Priority Order

1. **Token Delivery Endpoint** — enables many use cases (Story 1, 8, F)
2. **Per-Agent Tool Restrictions** — critical for multi-agent (Scenario F)
3. **Audit Log UI** — compliance requirement (Scenario C)
4. **Custom Tool Registration API** — developer experience (Story 7)
5. **MCP SDK Package** — enables Claude Desktop (Scenario D)
6. **Data Residency Config** — enterprise requirement (Scenario C)
7. **Department Scopes** — enterprise requirement (Story 4)

---

*Last updated: 2026-04-27*
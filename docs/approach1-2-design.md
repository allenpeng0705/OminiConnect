# Approach 1 & 2: LLM Tool Selection with Optional Panda Gateway

## Overview

This document describes two approaches for LLM-powered tool selection in OminiConnect:

- **Approach 1**: Caller-controlled LLM (OminiConnect as tool registry + executor)
- **Approach 2**: OminiConnect-managed LLM orchestration (OminiConnect + Panda + LiteLLM)

Both approaches share the same principle: **only expose connected providers' tools to minimize LLM context**.

**Key design principle: Panda and LiteLLM are both optional and pluggable.**

- **LiteLLM**: Always used as the LLM abstraction layer (unified interface to 100+ providers)
- **Panda**: Optional infrastructure layer (AI GW + MCP GW + API GW)

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              User                                        │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         OminiConnect                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                     │
│  │    Auth     │  │   Scope     │  │    Tool     │                     │
│  │  (Bearer)   │  │  Checking   │  │   Registry   │                     │
│  └─────────────┘  └─────────────┘  └─────────────┘                     │
│                                                                        │
│  Approach 1: /api/tools/registry (LLM-friendly tool list)               │
│  Approach 2: /api/llm (natural language → tool selection)              │
└─────────────────────────────────────────────────────────────────────────┘
           │                           │                      │
           ▼                           ▼                      ▼
┌──────────────────┐        ┌──────────────────┐    ┌──────────────────┐
│   Panda API GW   │        │  Panda AI GW     │    │  Panda MCP GW    │
│   (Optional)     │        │  (Optional)       │    │  (Optional)      │
│                  │        │                  │    │                  │
│  Rate limiting   │        │  LLM calls       │    │  Tool execution  │
│  Observability   │        │  Tool loops      │    │  Multi-step      │
│  Auth proxy      │        │  Streaming       │    │  orchestration   │
└──────────────────┘        └────────┬─────────┘    └──────────────────┘
           │                         │                      │
           ▼                         ▼                      ▼
    Provider APIs          ┌────────────────┐         OminiConnect
                            │    LiteLLM    │        MCP tools
                            │  (100+ models)│
                            └───────┬────────┘
                                    │
              ┌─────────────────────┼─────────────────────┐
              ▼                     ▼                     ▼
       ┌────────────┐        ┌────────────┐        ┌────────────┐
       │  Ollama    │        │  Claude    │        │   GPT-4    │
       │ (local)    │        │ (Anthropic)│        │  (OpenAI)  │
       └────────────┘        └────────────┘        └────────────┘
```

### LLM Flow by Mode

| Mode | LLM Flow | Panda Used? |
|------|----------|-------------|
| **Simple** | OminiConnect → LiteLLM → LLM | ❌ No |
| **Enterprise** | OminiConnect → Panda AI GW → LiteLLM → LLM | ✅ Yes |

### LiteLLM Role

LiteLLM is the **LLM abstraction layer** that provides:

- **100+ provider support**: OpenAI, Anthropic, Ollama, vLLM, Azure, AWS Bedrock, Google Vertex, HuggingFace, etc.
- **Unified API**: OpenAI-compatible `/v1/chat/completions` interface
- **Fallback support**: Automatic retry and fallback between providers
- **Cost tracking**: Per-model cost logging

LiteLLM runs as a proxy server:
```bash
# Example LiteLLM startup
litellm --port 4000

# With config file
litellm --config litellm_config.yaml
```

### Panda Role (Optional)

Panda is the **infrastructure layer** that provides:

| Component | Purpose |
|-----------|---------|
| **AI Gateway** | Rate limiting, TPM budgets, semantic caching, observability |
| **MCP Gateway** | Tool loops, multi-step orchestration, MCP server management |
| **API Gateway** | Request routing, auth proxy, rate limiting for direct API calls |

**Panda uses LiteLLM as upstream when enabled:**
```yaml
# Panda config - use LiteLLM as upstream
routes:
  - path_prefix: /v1/chat
    upstream: http://litellm:4000/v1
    type: openai  # LiteLLM is OpenAI-compatible
```

This means Panda's built-in provider support is **optional** - we prefer LiteLLM for provider abstraction.

## Configuration

Each approach has independent Panda integration settings. LiteLLM is always the LLM abstraction.

```rust
struct AppState {
    // Approach 1: Direct API mode
    approach1: Approach1Config,

    // Approach 2: OminiConnect-managed LLM
    approach2: Approach2Config,
}

struct Approach1Config {
    /// Enable/disable Approach 1
    enabled: bool,

    /// Optional Panda API Gateway for rate limiting and observability
    panda_api_gateway: PandaGatewayConfig,
}

struct Approach2Config {
    /// Enable/disable Approach 2
    enabled: bool,

    /// LiteLLM configuration (always used as LLM abstraction)
    litellm: LiteLLMConfig,

    /// Panda AI Gateway for LLM calls and tool loops (optional)
    panda_ai_gateway: PandaGatewayConfig,

    /// Panda MCP Gateway for MCP tool execution (optional)
    panda_mcp_gateway: PandaGatewayConfig,
}

struct LiteLLMConfig {
    /// LiteLLM server URL (e.g., "http://litellm:4000")
    url: String,

    /// API key for LiteLLM (if required)
    api_key: Option<String>,

    /// Default model to use
    default_model: String,
}

struct PandaGatewayConfig {
    /// Enable/disable this Panda gateway
    enabled: bool,

    /// Panda server URL (e.g., "http://panda:8080")
    url: Option<String>,

    /// Panda API key for authentication
    api_key: Option<String>,
}
```

### Configuration Examples

```yaml
# Simple case - Approach 1 only, no Panda
ominiconnect:
  approach1:
    enabled: true
  approach2:
    enabled: false

# Medium case - Approach 1 + Panda API GW for rate limiting
ominiconnect:
  approach1:
    enabled: true
    panda_api_gateway:
      enabled: true
      url: "http://panda:8080"
  approach2:
    enabled: false

# Full case - Both approaches, Panda for all gateways
ominiconnect:
  approach1:
    enabled: true
    panda_api_gateway:
      enabled: true
      url: "http://panda:8080"
  approach2:
    enabled: true
    panda_ai_gateway:
      enabled: true
      url: "http://panda:8080"
    panda_mcp_gateway:
      enabled: true
      url: "http://panda:8080"
```

## Approach 1: Caller-Controlled LLM

### Principle
OminiConnect provides a clean **tool registry** that the caller (using their own LLM) can consume. No orchestration logic in OminiConnect.

### Endpoints

#### `GET /api/tools/registry`
Returns all connected platforms with their tools in LLM-friendly format.

**Query Parameters:**
- `platform` (optional): Filter by platform (e.g., `github`, `slack`)

**Response:**
```json
{
  "platforms": {
    "github": {
      "connected": true,
      "tools": [
        {
          "name": "github_list_repos",
          "description": "List repositories for the authenticated user",
          "method": "GET",
          "endpoint": "/user/repos",
          "input_schema": {
            "type": "object",
            "properties": {
              "sort": { "type": "string" },
              "per_page": { "type": "integer" }
            },
            "required": []
          },
          "example_queries": [
            "list my github repos",
            "show my repositories"
          ],
          "scope_satisfied": "yes"
        }
      ]
    }
  }
}
```

#### `POST /api/tools/execute`
Executes a single tool with given arguments.

**Request:**
```json
{
  "tool_slug": "github_list_repos",
  "platform": "github",
  "arguments": {
    "per_page": 10
  }
}
```

**Response:** Direct tool result from provider API.

### How It Works

```
1. Caller → GET /api/tools/registry
   └─→ OminiConnect returns connected platforms + tools

2. Caller → Caller's LLM: "Which tool should I use for listing repos?"

3. Caller → POST /api/tools/execute
   └─→ OminiConnect validates auth, checks scopes, proxies to GitHub
   └─→ Returns result

4. Repeat steps 2-3 for each tool call
```

### Implementation Status

✅ **Implemented** - `GET /api/tools/registry` in `portal/src/api/tools.rs`

## Approach 2: OminiConnect-Managed LLM

### Principle
User sends natural language query. OminiConnect orchestrates LLM calls via Panda to select and execute tools.

### Endpoints

#### `POST /api/llm`
Natural language tool selection and execution.

**Request:**
```json
{
  "query": "notify team about latest GitHub issues",
  "platform": "github",  // optional hint
  "context": {}          // optional context
}
```

**Response (sync):**
```json
{
  "ok": true,
  "tool": "github_list_issues",
  "arguments": {
    "repo": "my-org/my-repo",
    "state": "open"
  },
  "result": { ... }
}
```

**Response (async with callback):**
```json
{
  "ok": true,
  "call_id": "abc-123",
  "status": "pending"
}
```

### How It Works

```
1. User → POST /api/llm { "query": "notify team about issues" }

2. OminiConnect:
   a. Checks if Panda AI GW is enabled
   b. If yes → route to Panda AI GW
   c. If no → check if LLM_FALLBACK_THRESHOLD exceeded

3. Panda AI GW (if enabled):
   a. Calls LLM with user's connected tools
   b. LLM decides: call github_list_issues first, then slack_post_message
   c. Panda MCP GW → OminiConnect → execute each tool

4. Panda MCP GW → OminiConnect:
   a. POST /api/mcp { method: "tools/call", params: { name, arguments } }
   b. OminiConnect validates scopes, executes tool
   c. Returns result to Panda

5. Panda AI GW → LLM:
   a. Receives tool results
   b. If more steps needed → continue loop (up to max_tool_rounds)
   c. If done → format final response

6. OminiConnect → User: final response
```

### Multi-Step Orchestration

```
User: "Notify team about latest GitHub issues"

Panda AI GW → LLM:
  System: "Given these tools, what's the plan?"
  Tools: [github_list_issues, slack_post_message, ...]

LLM → Panda:
  Step 1: call github_list_issues { repo: "org/repo", state: "open" }

Panda MCP GW → OminiConnect → GitHub API
  └─→ Returns: [{ title: "Bug 1", ... }, { title: "Bug 2", ... }]

Panda AI GW → LLM:
  Tool result: [issues list]
  Continue? Yes → Format message for Slack

LLM → Panda:
  Step 2: call slack_post_message { channel: "team", message: "Latest issues: ..." }

Panda MCP GW → OminiConnect → Slack API
  └─→ Returns: { ok: true }

Panda AI GW → User:
  Done. Team notified about 2 GitHub issues.
```

## Panda Integration

### When Panda is Used

| Component | Approach 1 | Approach 2 |
|-----------|------------|------------|
| Panda AI GW | ❌ Never | ✅ When `approach2.panda_ai_gateway.enabled` |
| Panda MCP GW | ❌ Never | ✅ When `approach2.panda_mcp_gateway.enabled` |
| Panda API GW | ✅ Optional | ✅ Optional |

### LiteLLM Configuration (Always Required for Approach 2)

```bash
# LiteLLM server URL
LITELLM_URL=http://litellm:4000

# Optional LiteLLM API key
LITELLM_API_KEY=xxx

# Default model (can be overridden per request)
LITELLM_DEFAULT_MODEL=gpt-4o-mini

# Example: Use local Ollama
# LITELLM_DEFAULT_MODEL=ollama/llama3

# Example: Use Claude via LiteLLM
# LITELLM_DEFAULT_MODEL=claude-3-sonnet-20240229
```

LiteLLM config file example (`litellm_config.yaml`):
```yaml
model_list:
  - model_name: gpt-4o-mini
    litellm_params:
      model: gpt-4o-mini
      api_key: os.environ/GPT4_API_KEY

  - model_name: claude-3-sonnet
    litellm_params:
      model: anthropic/claude-3-sonnet-20240229
      api_key: os.environ/ANTHROPIC_API_KEY

  - model_name: llama3
    litellm_params:
      model: ollama/llama3
      api_base: http://ollama:11434
```

### Panda Configuration in OminiConnect

Environment variables:
```bash
# Approach 1 - Optional Panda API GW
PANDA_API_GATEWAY_ENABLED=true
PANDA_API_GATEWAY_URL=http://panda:8080
PANDA_API_GATEWAY_API_KEY=xxx

# Approach 2 - LiteLLM (required) + Optional Panda
LITELLM_URL=http://litellm:4000
LITELLM_API_KEY=xxx
LITELLM_DEFAULT_MODEL=gpt-4o-mini

# Panda AI GW (optional, for tool loops + observability)
PANDA_AI_GATEWAY_ENABLED=true
PANDA_AI_GATEWAY_URL=http://panda:8080
PANDA_AI_GATEWAY_API_KEY=xxx

# Panda MCP GW (optional, for orchestration)
PANDA_MCP_GATEWAY_ENABLED=true
PANDA_MCP_GATEWAY_URL=http://panda:8080
PANDA_MCP_GATEWAY_API_KEY=xxx
```

### OminiConnect as Panda MCP Server

OminiConnect exposes `/api/mcp` for Panda to connect as MCP client:

```yaml
# Panda side (panda.yaml)
mcp:
  enabled: true
  servers:
    - name: ominiconnect
      enabled: true
      command: "npx"
      args: ["-y", "@ominiconnect/sdk-mcp"]
      env:
        API_KEY: "${OMINICONNECT_API_KEY}"
        BASE_URL: "${OMINICONNECT_BASE_URL}"
```

Or via stdio adapter:
```yaml
mcp:
  enabled: true
  servers:
    - name: ominiconnect
      enabled: true
      command: "python"
      args: ["/path/to/ominiconnect_mcp_stdio_adapter.py"]
```

## Security Considerations

### Token Isolation
- OminiConnect never exposes raw OAuth tokens to Panda
- Panda only sees OminiConnect's API, not underlying provider credentials
- All auth happens through OminiConnect's auth layer

### Scope Enforcement
- OminiConnect enforces scope checking before tool execution
- Even if Panda is compromised, attacker cannot exceed user's granted scopes

### Rate Limiting
- Panda API GW handles rate limiting for direct API calls
- Panda AI GW handles rate limiting for LLM calls

## Implementation Status

| Component | Status |
|-----------|--------|
| Approach 1: `GET /api/tools/registry` | ✅ Implemented |
| Approach 1: `POST /api/tools/execute` | ✅ Implemented |
| Approach 2: LiteLLM client in OminiConnect | ✅ Implemented |
| Approach 2: `POST /api/llm` with LiteLLM | ✅ Implemented |
| Approach 2: Panda AI GW integration | ✅ Implemented |
| Approach 2: Panda MCP GW integration | ✅ Implemented |
| Approach 2: Panda API GW integration | ✅ Implemented |
| Configuration management | ✅ Implemented |

## Implementation Details

### LiteLLM Client (`src/llm.rs`)

```rust
pub struct LiteLLMConfig {
    pub url: String,           // LITELLM_URL
    pub api_key: Option<String>, // LITELLM_API_KEY
    pub default_model: String, // LITELLM_DEFAULT_MODEL
    pub timeout_secs: u64,
}

pub struct LiteLLMClient {
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, LLMError>
    pub async fn chat_simple(&self, message: &str) -> Result<String, LLMError>
}
```

### Panda Client (`src/panda.rs`)

```rust
pub struct PandaConfig {
    pub ai_gateway: PandaGatewayConfig,
    pub mcp_gateway: PandaGatewayConfig,
    pub api_gateway: PandaGatewayConfig,
}

pub struct PandaAIGatewayClient { /* routes LLM calls through Panda */ }
pub struct PandaMCPGatewayClient { /* routes tool execution through Panda */ }
pub struct PandaAPIGatewayClient { /* routes API calls through Panda */ }
```

### AppState Integration

```rust
pub struct AppState {
    // ... existing fields ...
    pub llm_config: LiteLLMConfig,
    pub llm_client: LiteLLMClient,
    pub panda_config: PandaConfig,
    pub panda_ai_gateway: Option<PandaAIGatewayClient>,
    pub panda_mcp_gateway: Option<PandaMCPGatewayClient>,
    pub panda_api_gateway: Option<PandaAPIGatewayClient>,
}
```

### `/api/llm` Flow

1. Check if LLM is available (LiteLLM or Panda AI GW)
2. If available: use LLM-based execution
   - Build tool list from connected platforms
   - Call LLM with query + tools
   - Parse tool call from response
3. If not available: fall back to rule-based execution

## Next Steps

### Phase 4: Multi-step Orchestration ✅ Implemented
- [x] Design tool loop with context passing
- [x] Implement max_tool_rounds limit
- [x] Handle async tool execution with callbacks

### Configuration

```bash
# LiteLLM configuration
LITELLM_URL=http://litellm:4000
LITELLM_API_KEY=xxx
LITELLM_DEFAULT_MODEL=gpt-4o-mini
LITELLM_MAX_TOOL_ROUNDS=4
```

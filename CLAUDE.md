# OminiConnect

**AI Agent API Gateway** — gives LLMs and AI agents access to 700+ external APIs (GitHub, Slack, Salesforce, Feishu, DingTalk, QCC, etc.) through a unified tool interface.

## Project Overview

- **Portal**: `/portal` — Rust/Axum backend + React/TypeScript frontend
- **Nango**: `/third_party/nango/` — OAuth/token management for 700+ providers
- **Tool Registry**: `/portal/tools/registry/*.yaml` — YAML definitions for LLM-accessible tools
- **Goal**: Agents call tools via REST or MCP instead of managing API keys directly

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  AI Agent / Claude Code / Cursor                              │
│     │  Bearer <api-key>                                       │
│     ▼                                                         │
│  OminiConnect Portal (port 9000)                              │
│     │                                                         │
│     ├── /api/tools      — List/search/execute tools           │
│     ├── /api/mcp        — MCP JSON-RPC endpoint               │
│     ├── /api/llm        — LLM query with auto-tool-selection  │
│     └── /api/ominiconnect/proxy/:platform/* — Proxy to upstream│
│           │                                                   │
│           ▼                                                   │
│     Nango (port 3003) — OAuth flow + token storage            │
│           │                                                   │
│           ▼                                                   │
│  Upstream APIs: GitHub, Slack, QCC, Feishu, etc.              │
└──────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
OminiConnect/
├── portal/
│   ├── src/                    # Rust backend
│   │   ├── api/                # HTTP handlers (llm.rs, tools.rs, proxy.rs, mcp.rs, etc.)
│   │   ├── auth/               # Session/API key authentication
│   │   ├── db/                 # Database models (connectors, api_keys, tools)
│   │   ├── tools.rs            # Tool/Toolkit/ToolRegistry types
│   │   ├── nango.rs            # Nango client integration
│   │   └── argument_extractor.rs # Extract params from LLM natural language
│   ├── frontend/src/           # React frontend
│   │   ├── pages/              # Dashboard, Connectors, Catalog pages
│   │   ├── components/         # TestPanel, ScopeSelector, etc.
│   │   └── api/                # API client functions
│   └── tools/
│       ├── registry/*.yaml     # Tool definitions (886 provider YAML files)
│       └── generator/          # OpenAPI → tool generator
├── third_party/nango/          # Vendored Nango (OAuth provider)
├── docs/
│   ├── tool-registry-design.md # Tool registry architecture & protocol types
│   └── API.md                  # Full API reference
└── Makefile                    # `make dev` starts portal + nango
```

## Key Files

| File | Purpose |
|------|---------|
| `portal/src/tools.rs` | `Tool`, `Toolkit`, `ToolRegistry`, `ToolProtocol` (Rest/Mcp/GraphQL/WebSocket) |
| `portal/src/api/llm.rs` | LLM query handler, auto-detect provider, tool execution with MCP support |
| `portal/src/api/tools.rs` | `GET /api/tools`, `POST /api/tools/execute` |
| `portal/src/api/mcp.rs` | MCP JSON-RPC server (`tools/list`, `tools/call`) |
| `portal/src/api/proxy.rs` | `GET,POST /api/ominiconnect/proxy/:platform/*` |
| `portal/src/nango.rs` | Nango SDK wrapper for OAuth & token refresh |
| `portal/tools/registry/*.yaml` | Tool definitions (886 provider files) |
| `docs/tool-registry-design.md` | Full design doc with protocol types, execution flow |

## Authentication

- **Session cookie**: `ominiconnect_session` (website login)
- **API key**: `Authorization: Bearer <api-key>` (for agents/LLM tools)
- Cookie `Secure` flag auto-detected from `PORTAL_BASE_URL` (https = true)

## Tool Protocol Types

Tools can use different execution protocols (defined in `tools.rs`):

| Protocol | Description | Example |
|----------|-------------|---------|
| `rest` (default) | Standard REST API | GitHub, Slack |
| `mcp` | JSON-RPC over HTTP SSE | QCC (企查查) |

Add new protocols via `ToolProtocol` enum + handler in `llm.rs`.

## Current Status (2026-05-06)

### Completed
- ✅ Tool Registry: YAML-based tool definitions for 886 providers
- ✅ Tool APIs: list, search, execute via REST
- ✅ MCP endpoint: `POST /api/mcp` with `tools/list` and `tools/call`
- ✅ Protocol abstraction: Rest + MCP support
- ✅ LLM query endpoint: `POST /api/llm` with auto-detect provider selection
- ✅ QCC MCP integration working
- ✅ Test Panel frontend (React component for natural language queries)

### In Progress / Known Issues
- 🔄 PostgreSQL must be running for Nango (`make dev` fails if DB down)
- 🔄 Auto-detect provider selection: tags added to QCC tools for Chinese company queries

### TODO (from Plan.md)
- LinkedIn Image Upload (multi-step asset upload flow)
- MCP Connectors for each platform
- Schema Registry for LLM tool understanding
- Facebook Posting
- Feishu Custom App support (Enterprise Internal Apps don't support user OAuth)
- Content Moderation / PII Filtering
- Developer SDK

## Make Commands

```bash
make dev          # Start portal (9000) + nango (3003) + postgres
make dev_portal   # Start portal only
make logs         # Tail portal logs
```

## Development Notes

- **Git push workflow**: If remote has commits ahead:
  ```bash
  git stash && git pull --rebase && git stash pop && git push
  ```
- **User-Agent header**: Required for GitHub API — added in `execute_tool_direct`
- **QCC MCP URL format**: `https://agent.qcc.com/mcp/{server}/stream` where server extracted from endpoint slug
- **LLM per-request credentials**: API key sent from browser in request body, not stored server-side

## Adding a New Provider

1. Add tool YAML to `portal/tools/registry/<provider>.yaml`
2. Set `protocol: rest` (default) or `protocol: mcp` if MCP-based
3. For MCP, add handler case in `llm.rs` matching on `ToolProtocol::Mcp`
4. Add tags for auto-detect (e.g., `company`, `business`, `risk` for QCC)
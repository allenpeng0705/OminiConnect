# OminiConnect Design Document

> **⚠️ Historical — Pre-Nango Integration**
> This document is the initial design from 2026-04-20 and predates the Nango integration.
> Many sections describe approaches that were later revised. The current architecture
> uses Nango as the connector engine for global providers.

**Date:** 2026-04-20
**Status:** Draft
**Parent Project:** Panda (https://github.com/allenpeng0705/panda)

---

## 1. Executive Summary

**OminiConnect** is a next-generation AI Gateway and Tool Orchestrator that bridges LLMs with the Chinese enterprise software ecosystem (Feishu, DingTalk, WeChat Work). It is built on top of Panda's proven Rust-based gateway architecture.

**Core Value Proposition:** Secure, authenticated, standardized "hands" for AI agents to manipulate Chinese SaaS platforms via MCP (Model Context Protocol) — without the complexity of manual OAuth management.

---

## 2. Background: Why OminiConnect?

### 2.1 The Problem

Chinese enterprises face three critical bottlenecks:

1. **OAuth Fragmentation** — Each platform (Feishu, DingTalk, WeChat Work) has its own OAuth flow. Agents must manage multiple credential systems.

2. **Compliance Complexity** — PIPL regulations require data localization, PII scrubbing, and audit trails. Cloud-only solutions fail for SOEs.

3. **Tool Discovery** — LLMs need a standardized way to discover and invoke enterprise tools. MCP provides this, but MCP servers for Chinese platforms don't exist.

### 2.2 The Opportunity

The "Last Mile" of Enterprise AI Integration — connecting AI agents to existing enterprise workflows — is the highest-value unsolved problem in the Chinese AI market.

---

## 3. Architecture Overview

### 3.1 High-Level Architecture

```
┌─────────┐     ┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   LLM   │────▶│  OminiConnect    │────▶│  OminiConnect    │────▶│  Feishu/DingTalk│
│         │◀────│  (Panda Core)   │◀────│  Connectors      │◀────│  /WeChat APIs   │
└─────────┘     └─────────────────┘     └──────────────────┘     └─────────────────┘
                      ▲                        ▲
                      │                        │
               Tool Discovery          OAuth2 Token
               (tools/list)             Vault
```

### 3.2 OminiConnect Components

```
OminiConnect/
├── panda/              (Forked: Panda gateway core)
│   ├── crates/
│   │   ├── panda-proxy/   # MCP runtime, AI gateway, Wasm engine
│   │   ├── panda-server/  # Binary entry
│   │   ├── panda-config/ # Configuration types
│   │   └── panda-pdk/    # Wasm policy SDK
│   └── panda.example.yaml
├── omini_connect/               (OminiConnect value layer)
│   ├── oauth_vault/    # Token storage + refresh
│   ├── connectors/     # Platform-specific connectors
│   │   ├── feishu/     # Feishu (Lark) API connector
│   │   ├── dingtalk/   # DingTalk API connector
│   │   └── wechatwork/ # WeChat Work API connector
│   └── wasm_policies/  # Compliance Wasm modules
│   └── audit_logger/   # Audit logging for PIPL compliance
│   └── sdk/            # Developer SDK
│   └── skills/         # Skill marketplace
└── dashboard/          # Monitoring dashboard
```

---

## 4. OminiConnect vs Panda: What We Use

### 4.1 Components We Use from Panda

| Panda Component | Purpose in OminiConnect |
|----------------|------------------------|
| `panda-proxy` (MCP runtime) | Tool discovery and execution |
| `panda-proxy` (Wasm engine) | Compliance policy sandbox |
| `panda-proxy` (AI gateway) | Optional: for AI routing |
| `panda-config` | YAML configuration |
| `panda-pdk` | Wasm plugin SDK |

### 4.2 Components We Skip from Panda

| Panda Feature | Reason |
|--------------|--------|
| TPM/token budgets | Not applicable for tool-only use |
| Semantic cache | Not v1 priority |
| Model failover | Not applicable for tool-only use |
| Upstream LLM proxy | Agents use cloud LLMs directly |

### 4.3 Components We Add (Our Value)

| New Component | Purpose |
|--------------|---------|
| `oauth_vault` | Centralized OAuth2 token storage + auto-refresh for Chinese SaaS |
| `connectors/feishu` | Feishu MCP connector (Bitable, Calendar, Messaging) |
| `connectors/dingtalk` | DingTalk MCP connector (Workflow, Messaging) |
| `connectors/wechatwork` | WeChat Work MCP connector (Customer Management) |
| `wasm_policies` | Compliance modules (PII scrub, keyword filter, data residency) |
| `audit_logger` | Audit logging for PIPL compliance |

---

## 5. Data Flow

### 5.1 Tool Discovery Flow

```
1. LLM sends: POST /v1/chat/completions (with tools wanted)
2. Panda MCP runtime: returns OminiConnect tools in OpenAI format
3. Tools appear to LLM as standard function definitions
```

### 5.2 Tool Execution Flow

```
1. LLM calls tool (e.g., feishu_calendar_list)
2. Panda routes to OminiConnect connector
3. Connector fetches OAuth2 token from vault
4. Connector transforms args + calls Feishu API
5. Result returned through Panda to LLM
```

### 5.3 Compliance Flow

```
1. Tool result passes through Wasm compliance layer
2. Wasm policy scans for PII, restricted keywords
3. If violation: redact/block + log to audit trail
4. Clean result returned to LLM
```

### 5.4 Connector Integration with Panda

OminiConnect connectors run as standalone MCP HTTP servers. Panda connects to them via `remote_mcp_url`:

```
┌─────────┐     ┌─────────────────┐     ┌──────────────────┐
│   LLM   │────▶│     Panda       │────▶│  Connector HTTP   │
│         │◀────│  (MCP Gateway)  │◀────│    Servers        │
└─────────┘     └─────────────────┘     └──────────────────┘
                      │                        │
                      │  remote_mcp_url:       │
                      │  http://host:8090/mcp  │
                      │  http://host:8091/mcp  │
                      │  http://host:8092/mcp  │
                      ▼                        ▼
              api_gateway.egress          OAuth2 Vault
              (allowlist config)          (per-connector)
```

**Configuration (`omini_connect.yaml`):**
```yaml
api_gateway:
  egress:
    enabled: true
    allowlist:
      allow_hosts: ["127.0.0.1"]

mcp:
  enabled: true
  servers:
    - name: feishu
      remote_mcp_url: "http://127.0.0.1:8090/mcp"
    - name: dingtalk
      remote_mcp_url: "http://127.0.0.1:8091/mcp"
    - name: wechatwork
      remote_mcp_url: "http://127.0.0.1:8092/mcp"
```

**Running the System:**
1. Start each connector server:
   ```bash
   cargo run -p omini-connect-feishu --bin feishu_server --port=8090
   cargo run -p omini-connect-dingtalk --bin dingtalk_server --port=8091
   cargo run -p omini-connect-wechatwork --bin wechatwork_server --port=8092
   ```
2. Start Panda with the OminiConnect configuration:
   ```bash
   cargo run -p panda-server -- --config omini_connect.yaml
   ```

### 5.5 Hybrid Inference Configuration

OminiConnect supports hybrid inference routing to route sensitive requests to local LLM (Ollama/vLLM) while forwarding general requests to cloud providers. This is configured via the `hybrid_inference` section in Panda's configuration.

**Request Flow:**
```
Request → Hybrid Router (evaluates rules) → Local LLM or Cloud Provider
```

**Configuration Options:**

#### Cloud-Only Mode
```yaml
hybrid_inference:
  enabled: true
  local_llm:
    enabled: false
  targets:
    - name: openai
      provider: openai
      upstream: "https://api.openai.com/v1"
      api_key_name: "OPENAI_API_KEY"
  cloud_fallback:
    enabled: true
    target: "openai"
  rules:
    - name: cloud_only
      priority: 1
      conditions:
        - type: always
      action: route_to_cloud
```

#### Local-Only Mode
```yaml
hybrid_inference:
  enabled: true
  local_llm:
    enabled: true
    endpoint: "http://127.0.0.1:11434/v1/chat/completions"
    model: "llama3"
    timeout_ms: 30000
  cloud_fallback:
    enabled: false
  rules:
    - name: local_only
      priority: 1
      conditions:
        - type: always
      action: route_to_local
```

#### Hybrid Mode (Both Local and Cloud)
```yaml
hybrid_inference:
  enabled: true
  local_llm:
    enabled: true
    endpoint: "http://127.0.0.1:11434/v1/chat/completions"
    model: "llama3"
  targets:
    - name: openai
      provider: openai
      upstream: "https://api.openai.com/v1"
      api_key_name: "OPENAI_API_KEY"
  cloud_fallback:
    enabled: true
    target: "openai"
  rules:
    - name: sensitive_data
      priority: 1
      conditions:
        - type: content_contains_pii
      action: route_to_local
    - name: general
      priority: 0
      conditions:
        - type: always
      action: route_to_cloud
```

**Rule Conditions:**
- `content_contains_pii` - Content contains PII (email, phone, etc.)
- `content_matches` - Content matches keyword or regex patterns
- `tool_is` - Request uses specific tool(s)
- `tenant_is` - Request from specific tenant(s)
- `user_in_group` - User in specific group(s)
- `wasm_sensitivity_score_gte` - Wasm sensitivity score >= threshold
- `always` - Always match

**Rule Actions:**
- `route_to_local` - Route to local LLM
- `route_to_cloud` - Route to cloud target
- `route_to_target` - Route to specific named target
- `ask_wasm_policy` - Defer to Wasm policy decision

---

## 6. Priority Roadmap

### Phase 1: Foundation (Weeks 1-4)
- [x] Fork Panda into OminiConnect structure
- [x] Build OAuth2 vault (token storage + refresh)
- [x] Implement Feishu connector (Bitable, Calendar, Messaging)
- [x] Basic compliance Wasm (PII scrub, keyword filter)

### Phase 2: Tool Expansion (Weeks 5-8)
- [x] DingTalk connector (OAuth2 + MCP server)
- [x] WeChat Work connector (OAuth2 + MCP server)
- [x] Schema registry for LLM tool discovery
- [x] Documentation auto-generation

### Phase 3: Advanced Features & Compliance (Weeks 9-12)
- [x] Data localization (PIPL compliance)
- [x] Content moderation Wasm
- [x] Hybrid inference (local LLM for sensitive tasks)
- [x] Audit logging + export

### Phase 4: Scaling & Ecosystem (Ongoing)
- [x] Developer SDK
- [x] Skill marketplace (pre-packaged MCP tool sets)
- [x] Monitoring dashboard

---

## 7. Strategic Considerations

### 7.1 The "Stateful" Execution Problem

Chinese enterprise workflows (especially Feishu/DingTalk) are highly stateful and asynchronous. Future work includes implementing **Async Webhook Handlers** that can "park" agent requests, listen for callbacks, and re-inject results.

**Status:** Deferred to v2

### 7.2 Semantic Caching

Caching tool results semantically can reduce API quota usage and latency. When an agent asks "What's on my schedule today?" multiple times, Panda should return cached results.

**Status:** Deferred to v2

### 7.3 Compliance Wasm (Priority)

Compliance requirements in China change rapidly. Wasm modules provide hot-reloadable compliance policies without binary restarts. This is the **highest priority** differentiator.

**Status:** Primary v1 focus

**Example Use Cases:**
- Real-time content moderation (sensitive keyword filtering)
- PII scrubbing for outgoing messages
- Data residency checks before API calls
- Audit trail logging

### 7.4 Local-First Edge (Future Differentiation)

Allow enterprises to host OminiConnect on-premise. OAuth tokens never leave the internal network. The "agent" might be a cloud model, but it communicates with a local instance that sanitizes data before going to the cloud.

**Status:** Deferred — sales differentiator for later

---

## 8. The "Skill" as an Asset

Beyond providing "Feishu API access," OminiConnect should package **Skills** — pre-defined sets of MCP tools, system prompts, and Wasm policies.

**Example:**
- "Feishu Project Manager Skill" = Feishu connector + calendar tool + approval workflow + compliance policies
- "WeChat Customer Service Skill" = WeChat Work connector + message templates + PII scrubber

This transforms OminiConnect from a utility (low margin) to an enterprise solution (high margin).

---

## 9. Technical Decisions

### 9.1 Why Fork Panda?

- Full control over branding (OminiConnect vs Panda)
- Can strip unnecessary Panda features (TPM, semantic cache, AI gateway)
- Clear sync path for upstream Panda updates
- Independent release versioning

### 9.2 Update Strategy

When Panda updates:
```bash
cd OminiConnect
git remote add panda <panda-repo>
git fetch panda
git merge panda/main
# Resolve conflicts, test, release
```

### 9.3 Wasm Policy Model

```
┌─────────────────────────────────────────┐
│ Wasm Policy Guest                        │
│ - panda_on_tool_result()                │
│ - Scan for PII / keywords / data types  │
│ - Return: pass / redact / block         │
└─────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────┐
│ Host (Panda Wasm Runtime)               │
│ - Load policy .wasm files               │
│ - Enforce memory limits                 │
│ - Log all decisions                     │
└─────────────────────────────────────────┘
```

---

## 10. References

### Panda Documentation
- High-Level Design: `docs/high_level_design.md`
- Architecture: `docs/architecture_two_pillars.md`
- MCP + API Gateway: `docs/design_api_gateway_and_mcp_gateway.md`
- Implementation Plan: `docs/implementation_plan_mcp_api_gateway.md`
- Data Flow: `docs/panda_data_flow.md`

### Panda Repository
https://github.com/allenpeng0705/panda

---

## 11. Implementation: Fork Structure

### 11.1 Project Layout

```
OminiConnect/
├── panda/                    # Forked from Panda (subtree)
│   ├── crates/
│   │   ├── panda-proxy/      # MCP runtime, AI gateway, Wasm engine
│   │   ├── panda-server/     # Binary entry
│   │   ├── panda-config/    # Configuration types
│   │   ├── panda-pdk/       # Wasm policy SDK
│   │   └── panda-wasm/      # Wasm runtime
│   ├── docs/                # Panda documentation (read-only reference)
│   └── panda.example.yaml   # Example configuration
├── omini_connect/                     # OminiConnect value layer
│   ├── oauth_vault/         # OAuth2 token storage + refresh
│   ├── schema_registry/     # Tool schema registry
│   ├── connectors/
│   │   ├── feishu/          # Feishu (Lark) MCP connector
│   │   ├── dingtalk/        # DingTalk MCP connector
│   │   └── wechatwork/      # WeChat Work MCP connector
│   ├── wasm_policies/       # Compliance Wasm modules
│   ├── audit_logger/        # Audit logging for PIPL
│   ├── sdk/                 # Developer SDK
│   └── skills/              # Skill marketplace
├── dashboard/                # Monitoring dashboard
├── docs/
│   └── design.md            # This document
├── Cargo.toml               # Workspace manifest
└── README.md
```

### 11.2 Workspace Members

| Crate | Source | Purpose |
|-------|--------|---------|
| `panda-proxy` | Forked | MCP runtime, AI gateway, Wasm engine |
| `panda-server` | Forked | Binary entry point |
| `panda-config` | Forked | YAML configuration parsing |
| `panda-pdk` | Forked | Wasm plugin SDK |
| `omini-connect-oauth-vault` | New | OAuth2 token storage and auto-refresh |
| `omini-connect-feishu` | New | Feishu API connector |
| `omini-connect-dingtalk` | New | DingTalk API connector |
| `omini-connect-wechatwork` | New | WeChat Work API connector |
| `omini-connect-wasm-policies` | New | Compliance Wasm policies |
| `omini-connect-schema-registry` | New | Tool schema registry |
| `omini-connect-portal` | New | Operator portal (OAuth + connectors) |
| `omini-connect-hybrid-inference` | New | Hybrid inference routing |
| `omini-connect-audit-logger` | New | Audit logging for PIPL compliance |
| `omini-connect-sdk` | New | Developer SDK |
| `omini-connect-skills` | New | Skill marketplace |
| `omini-connect-dashboard` | New | Monitoring dashboard |

### 11.3 Update Strategy

When Panda releases updates, sync using git subtree:

```bash
# Fetch latest Panda
git fetch panda main

# Merge updates into OminiConnect
git subtree pull --prefix=panda panda main --squash

# After resolving conflicts, commit
git add .
git commit -m "Sync panda updates"
```

---

## 12. Document History

| Date | Author | Changes |
|------|--------|---------|
| 2026-04-20 | Claude | Initial draft from design discussions |

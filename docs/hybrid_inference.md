# Hybrid Inference Design

**Date:** 2026-04-20
**Status:** Design Draft
**Parent Project:** OminiConnect

---

## 1. Overview

Hybrid inference allows sensitive AI requests to be processed by a local LLM instead of cloud providers, ensuring:

- **PIPL Compliance** - Chinese citizen data never leaves the network
- **Data Sovereignty** - Enterprise data stays on-premise
- **Latency Optimization** - Local inference for sensitive tasks

```
┌─────────────────────────────────────────────────────────────────┐
│                      Panda Gateway                               │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐   │
│  │  Request    │───▶│  Sensitivity │───▶│  Routing        │   │
│  │  Incoming   │    │  Classifier  │    │  Decision       │   │
│  └─────────────┘    └──────────────┘    └────────┬────────┘   │
│                           │                      │             │
│         ┌─────────────────┼──────────────────────┤             │
│         ▼                 ▼                      ▼             │
│  ┌─────────────┐  ┌──────────────┐    ┌─────────────────┐   │
│  │  Content    │  │  Wasm        │    │  Tool/Identity   │   │
│  │  Analysis   │  │  Policy      │    │  Rules           │   │
│  └─────────────┘  └──────────────┘    └─────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
     ┌─────────────────┐             ┌─────────────────┐
     │   Local LLM     │             │   Cloud LLM      │
     │   (Ollama)     │             │ (OpenAI/Claude)  │
     └─────────────────┘             └─────────────────┘
```

---

## 2. Sensitivity Rules Engine

### 2.1 Rule Definition

```rust
pub struct SensitivityRule {
    /// Unique rule name
    pub name: String,
    /// Rule priority (higher = evaluated first)
    pub priority: u8,
    /// Conditions that must match
    pub conditions: Vec<Condition>,
    /// Action when rule matches
    pub action: RoutingAction,
}

pub enum Condition {
    /// Content contains PII
    ContentContainsPii,
    /// Content matches pattern (regex or keyword list)
    ContentMatches(Pattern),
    /// Request uses specific tool(s)
    ToolIs(Vec<ToolId>),
    /// Request from specific tenant(s)
    TenantIs(Vec<TenantId>),
    /// Request from user in group(s)
    UserInGroup(Vec<GroupId>),
    /// Request has sensitivity flag from Wasm
    WasmSensitivityScoreGte(u8),
    /// Always match (default fallback)
    Always,
}

pub enum RoutingAction {
    /// Route to local LLM
    RouteToLocal,
    /// Route to cloud LLM
    RouteToCloud,
    /// Ask Wasm policy for decision
    AskWasmPolicy,
}

pub enum Pattern {
    /// Simple keyword list
    Keywords(Vec<String>),
    /// Regex pattern
    Regex(String),
}
```

### 2.2 Rule Evaluation

Rules are evaluated in priority order (highest first). First matching rule wins.

```
1. Check ContentContainsPii → RouteToLocal
2. Check ToolIs(["hr_get_salary", "customer_get_id"]) → RouteToLocal
3. Check TenantIs(["soe_enterprise"]) → RouteToLocal
4. Check WasmSensitivityScoreGte(70) → AskWasmPolicy
5. Default → RouteToCloud
```

### 2.2 Example Configuration

```yaml
hybrid_inference:
  enabled: true

  # Local LLM configuration
  local_llm:
    provider: "ollama"  # or "llama.cpp", "vllm"
    endpoint: "http://localhost:11434/v1/chat/completions"
    model: "qwen2.5:7b-instruct"
    timeout_ms: 30000
    # For Chinese enterprise: better to use Qwen, DeepSeek, or Yi models
    # model: "deepseek-r1:7b"
    # model: "yi:latest"

  # Sensitivity rules (evaluated in priority order)
  rules:
    - name: "pii_block"
      priority: 100
      conditions:
        - type: "content_contains_pii"
      action: "route_to_local"

    - name: "sensitive_tools"
      priority: 90
      conditions:
        - type: "tool_is"
          tools: ["hr_get_salary", "employee_get_id", "customer_get_id"]
      action: "route_to_local"

    - name: "soe_tenants"
      priority: 80
      conditions:
        - type: "tenant_is"
          tenants: ["soe_enterprise_1", "government_office"]
      action: "route_to_local"

    - name: "high_sensitivity_wasm"
      priority: 70
      conditions:
        - type: "wasm_sensitivity_score_gte"
          score: 70
      action: "route_to_local"

    - name: "financial_data"
      priority: 60
      conditions:
        - type: "content_matches"
          pattern:
            type: "keywords"
            values: ["工资", "薪酬", "社保", "公积金", "银行账户"]
      action: "route_to_local"

    - name: "default_cloud"
      priority: 1
      conditions:
        - type: "always"
      action: "route_to_cloud"
```

---

## 3. Wasm Integration

### 3.1 Existing Wasm Policies

Current Wasm policies already scan content:

| Policy | Checks | Sets Header |
|--------|--------|-------------|
| `pii_scrubber` | Phone, ID, email | `x-omini-connect-pii-detected` |
| `keyword_filter` | Restricted keywords | `x-omini-connect-keyword-blocked` |
| `content_moderation` | Adult/hate/violence | `x-omini-connect-policy` |
| `data_residency` | Chinese PII | `x-omini-connect-blocked` |

### 3.2 New Wasm Policy: Sensitivity Scorer

```rust
// New: omini_connect/hybrid_inference/src/wasm_sensitivity.rs

#[no_mangle]
pub extern "C" fn panda_on_request_body(ptr: i32, len: i32) -> i32 {
    let input = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

    // Calculate sensitivity score (0-100)
    let score = calculate_sensitivity_score(input);

    // Set sensitivity header
    set_header(b"x-omini-connect-sensitivity-score", score.to_string().as_bytes());

    // If score >= threshold, set high sensitivity marker
    if score >= 70 {
        set_header(b"x-omini-connect-high-sensitivity", b"true");
    }

    RC_ALLOW  // Don't block, just score
}

fn calculate_sensitivity_score(content: &[u8]) -> u8 {
    let mut score: u8 = 0;

    // PII detection (0-30 points)
    if contains_chinese_phone(content) { score += 10; }
    if contains_chinese_id(content) { score += 20; }
    if contains_email(content) { score += 5; }

    // Keyword detection (0-30 points)
    if contains_keywords(content, SENSITIVE_KEYWORDS) { score += 30; }

    // Content pattern detection (0-20 points)
    if contains_financial_terms(content) { score += 20; }
    if contains_medical_terms(content) { score += 15; }
    if contains_government_terms(content) { score += 20; }

    // Name detection (0-20 points)
    if contains_potential_name(content) { score += 10; }

    score.min(100)
}

static SENSITIVE_KEYWORDS: &[&str] = &[
    "工资", "薪酬", "银行账户", "身份证", "社保", "公积金",
    "病假", "医疗", "人事", "绩效", "奖金", "股票",
    // ... more sensitive terms
];
```

### 3.3 Wasm → Hybrid Gateway Communication

```
Request → Wasm Policy (scans + scores)
        → Sets headers:
          - x-omini-connect-sensitivity-score: 0-100
          - x-omini-connect-pii-types: phone,id,email
          - x-omini-connect-keyword-categories: financial,medical
        → Hybrid Inference Gateway reads headers
        → Applies routing rules
```

---

## 4. Local LLM Integration

### 4.1 Provider Support

| Provider | API | Notes |
|----------|-----|-------|
| **Ollama** | OpenAI-compatible | Easiest to deploy |
| **llama.cpp server** | OpenAI-compatible | More control, no runtime overhead |
| **vLLM** | OpenAI-compatible | High throughput, better for A100/H100 |
| **DeepSeek** | OpenAI-compatible | Chinese-optimized models |

### 4.2 Local LLM Module

```rust
// omini_connect/hybrid_inference/src/local_llm.rs

pub struct LocalLlmClient {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl LocalLlmClient {
    pub async fn complete(&self, request: ChatRequest) -> Result<ChatResponse> {
        // Transform to OpenAI-compatible format
        let oai_request = OpenAIRequest::from(request);

        let response = self.client
            .post(&self.endpoint)
            .json(&oai_request)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        // Parse response
        let oai_response: OpenAIResponse = response.json().await?;
        Ok(ChatResponse::from(oai_response))
    }
}
```

### 4.3 Model Selection for Chinese Enterprise

Recommended models for Chinese enterprise compliance:

| Model | Size | Strengths |
|-------|------|-----------|
| **Qwen2.5-7B** | 7B | Strong Chinese, good reasoning |
| **DeepSeek-R1-7B** | 7B | Excellent for code/analysis |
| **Yi-6B** | 6B | Bilingual Chinese/English |
| **GLM-4-9B** | 9B | Strong Chinese understanding |

**Requirements:**
- Minimum 8GB VRAM for 7B models (FP16)
- 16GB+ RAM recommended
- Chinese enterprise: Qwen or DeepSeek preferred

---

## 5. Routing Decision Flow

```
┌──────────────────────────────────────────────────────────────┐
│                     Incoming Request                          │
│                  (with content + context)                    │
└─────────────────────────┬────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────────┐
│                    Wasm Policy Layer                         │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  panda_on_request_body()                              │  │
│  │  - Scan for PII, keywords                              │  │
│  │  - Calculate sensitivity_score (0-100)                 │  │
│  │  - Set headers:                                        │  │
│  │    x-omini-connect-sensitivity-score                           │  │
│  │    x-omini-connect-pii-types                                   │  │
│  │    x-omini-connect-keyword-categories                          │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────┬────────────────────────────────────┘
                          │
                          ▼
┌──────────────────────────────────────────────────────────────┐
│                 Hybrid Inference Gateway                      │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Evaluate Rules (priority order):                       │  │
│  │  1. PII detected? → RouteToLocal                       │  │
│  │  2. Sensitive tool? → RouteToLocal                     │  │
│  │  3. High-sensitivity tenant? → RouteToLocal            │  │
│  │  4. Wasm score >= 70? → RouteToLocal                  │  │
│  │  5. Keyword match? → RouteToLocal                     │  │
│  │  6. Default → RouteToCloud                             │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────┬────────────────────────────────────┘
                          │
            ┌─────────────┴─────────────┐
            │                           │
            ▼                           ▼
┌─────────────────────┐     ┌─────────────────────┐
│     Local LLM       │     │      Cloud LLM       │
│   (Ollama/vLLM)     │     │ (OpenAI/Claude)     │
│                     │     │                     │
│ - No data egress    │     │ - Higher quality     │
│ - PIPL compliant    │     │ - More capable       │
│ - Fast for short    │     │ - Global reach      │
│   contexts          │     │                     │
└─────────────────────┘     └─────────────────────┘
```

---

## 6. Configuration

### 6.1 Full Configuration Example

```yaml
# omini_connect.yaml

hybrid_inference:
  enabled: true

  # Local LLM settings
  local_llm:
    enabled: true
    endpoint: "http://localhost:11434/v1/chat/completions"
    model: "qwen2.5:7b-instruct"
    timeout_ms: 30000

  # Targets: define routing targets with their credentials
  targets:
    # General cloud LLM (for non-sensitive requests)
    # Set the corresponding environment variable: export OPENAI_API_KEY=sk-...
    - name: "general"
      provider: "openai"  # or "anthropic", "deepseek", etc.
      upstream: "https://api.openai.com/v1"
      api_key_name: "OPENAI_API_KEY"  # Env var name - must be set before starting

    - name: "anthropic_cloud"
      provider: "anthropic"
      upstream: "https://api.anthropic.com/v1"
      api_key_name: "ANTHROPIC_API_KEY"

    - name: "deepseek_cloud"
      provider: "openai_compatible"
      upstream: "https://api.deepseek.com/v1"
      api_key_name: "DEEPSEEK_API_KEY"

  # Cloud fallback (when local fails - uses "general" target)
  cloud_fallback:
    enabled: true
    target: "general"  # Reference to targets[].name

  # Routing rules
  rules:
    # Highest priority: Block all PII with local processing
    - name: "pii_always_local"
      priority: 100
      conditions:
        - type: "content_contains_pii"
      action: "route_to_local"

    # High priority: Sensitive tools always use local
    - name: "sensitive_tools_local"
      priority: 90
      conditions:
        - type: "tool_is"
          tools:
            - "hr_get_salary"
            - "hr_get_bonus"
            - "employee_get_id"
            - "customer_get_id"
            - "finance_get_salary"
      action: "route_to_local"

    # SOE enterprises: all local
    - name: "soe_local_only"
      priority: 85
      conditions:
        - type: "tenant_is"
          tenants: ["soe_*", "government_*"]

  # Routing rules
  rules:
    # Highest priority: Block all PII with local processing
    - name: "pii_always_local"
      priority: 100
      conditions:
        - type: "content_contains_pii"
      action: "route_to_local"
      audit: true

    # High priority: Sensitive tools always use local
    - name: "sensitive_tools_local"
      priority: 90
      conditions:
        - type: "tool_is"
          tools:
            - "hr_get_salary"
            - "hr_get_bonus"
            - "employee_get_id"
            - "customer_get_id"
            - "finance_get_salary"
      action: "route_to_local"

    # SOE enterprises: all local
    - name: "soe_local_only"
      priority: 85
      conditions:
        - type: "tenant_is"
          tenants: ["soe_*", "government_*"]
      action: "route_to_local"

    # High Wasm sensitivity score
    - name: "high_sensitivity"
      priority: 70
      conditions:
        - type: "wasm_sensitivity_score_gte"
          score: 70
      action: "route_to_local"

    # Financial keywords trigger local
    - name: "financial_keywords"
      priority: 60
      conditions:
        - type: "content_matches"
          pattern:
            type: "keywords"
            values:
              - "工资"
              - "薪酬"
              - "社保"
              - "公积金"
              - "银行账户"
              - "奖金"
      action: "route_to_local"

    # Medical/health data
    - name: "medical_data"
      priority: 55
      conditions:
        - type: "content_matches"
          pattern:
            type: "keywords"
            values:
              - "病假"
              - "医疗"
              - "体检"
              - "疾病"
      action: "route_to_local"

    # Default: cloud (with fallback to local on cloud failure)
    - name: "default"
      priority: 1
      conditions:
        - type: "always"
      # Route to "general" target (cloud), fallback to local on error
      target: "general"
      fallback_to_local: true

  # Observability
  observability:
    log_routing_decisions: true
    metrics_enabled: true
    audit_log_path: "/var/log/omini_connect/hybrid_audit.log"
```

### 6.2 Target Configuration

Each target defines a possible routing destination with its credentials:

```yaml
targets:
  - name: "general"                    # Unique name (used by rules)
    provider: "openai"                 # Provider type
    upstream: "https://api.openai.com/v1"  # API endpoint
    api_key_name: "OPENAI_API_KEY"   # Environment variable name

  - name: "anthropic"
    provider: "anthropic"
    upstream: "https://api.anthropic.com/v1"
    api_key_name: "ANTHROPIC_API_KEY"
```

**How API Keys Work:**
1. Set environment variable before starting: `export OPENAI_API_KEY=sk-...`
2. Gateway reads `api_key_name` to find the env var
3. API key is injected into requests to that target

**Provider Types:**
| Provider | API Style | Notes |
|----------|----------|-------|
| `openai` | OpenAI Chat Completions | Direct |
| `anthropic` | Anthropic Messages | Body transformation |
| `openai_compatible` | OpenAI format | DeepSeek, Qwen, etc. |
| `ollama` | OpenAI-compatible | Local models |

### 6.3 Per-Tenant Override

```yaml
tenants:
  - id: "enterprise_a"
    name: "Enterprise A"
    hybrid_mode: "cloud_only"  # Override: never use local

  - id: "enterprise_b"
    name: "Enterprise B"
    hybrid_mode: "local_preferred"  # Prefer local, fallback to cloud

  - id: "enterprise_c"
    name: "Enterprise C (SOE)"
    hybrid_mode: "local_only"  # Never use cloud
```

---

## 7. Implementation Plan

### Phase 1: Core Engine (MVP)
1. Create `omini_connect/hybrid_inference` crate
2. Implement sensitivity rules engine
3. Add local LLM client (Ollama)
4. Basic routing decision

### Phase 2: Wasm Integration
1. Create sensitivity scoring Wasm module
2. Wire Wasm headers to routing engine
3. Add PII/keyword conditions

### Phase 3: Advanced Rules
1. Tenant-based routing
2. Tool-based routing
3. Audit logging

### Phase 4: Production Hardening
1. Fallback mechanisms
2. Circuit breakers
3. Metrics and observability

---

## 8. Files to Create/Modify

### Create:
- `omini_connect/hybrid_inference/Cargo.toml`
- `omini_connect/hybrid_inference/src/lib.rs`
- `omini_connect/hybrid_inference/src/rules.rs`
- `omini_connect/hybrid_inference/src/router.rs`
- `omini_connect/hybrid_inference/src/local_llm.rs`
- `omini_connect/hybrid_inference/src/config.rs`
- `omini_connect/hybrid_inference/src/wasm_sensitivity.rs` (Wasm module)

### Modify:
- `omini_connect/connectors/*/tools.rs` - Add `sensitive: bool` flag to tools
- `panda/crates/panda-proxy` - Add hybrid inference middleware
- `docs/design.md` - Update with hybrid inference section

---

## 9. Metrics

```rust
// Metrics to emit
hybrid_routing_decisions_total{rule="...", action="local|cloud"}
hybrid_local_latency_ms{histogram}
hybrid_cloud_fallback_total{}
hybrid_wasm_score_distribution{histogram}
```

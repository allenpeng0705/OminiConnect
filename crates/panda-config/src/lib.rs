//! GitOps-style configuration: parse and validate YAML without starting the server.
//!
//! Kept separate from the proxy so unit tests can cover invalid files and defaults
//! without binding to a network stack.
//!
//! # Product pillars (field groupings)
//!
//! - **Inbound (all-in-one — Panda API gateway + MCP):** MCP and agent session config—[`McpConfig`](crate::McpConfig),
//!   [`AgentSessionsConfig`](crate::AgentSessionsConfig)—and [`TrustedGatewayConfig`](crate::TrustedGatewayConfig) when **external**
//!   L7 wraps Panda (ingress to Panda is then attested). Target: Panda’s own API gateway ingress/egress; see `docs/panda_data_flow.md`.
//! - **Outbound (AI gateway):** `default_backend`, [`routes`](crate::RouteConfig), [`AdapterConfig`](crate::AdapterConfig),
//!   [`SemanticCacheConfig`](crate::SemanticCacheConfig), [`TpmConfig`](crate::TpmConfig),
//!   [`ModelFailoverConfig`](crate::ModelFailoverConfig), [`RoutingConfig`](crate::RoutingConfig),
//!   [`RateLimitFallbackConfig`](crate::RateLimitFallbackConfig), [`ContextManagementConfig`](crate::ContextManagementConfig).
//! - **Cross-cutting:** `listen`, `server`, `tls`, [`ObservabilityConfig`](crate::ObservabilityConfig), [`PluginsConfig`](crate::PluginsConfig),
//!   [`IdentityConfig`](crate::IdentityConfig), [`AuthConfig`](crate::AuthConfig),
//!   [`PromptSafetyConfig`](crate::PromptSafetyConfig), [`PiiConfig`](crate::PiiConfig),
//!   [`ConsoleOidcConfig`](crate::ConsoleOidcConfig), [`BudgetHierarchyConfig`](crate::BudgetHierarchyConfig).
//!
//! See **`docs/architecture_two_pillars.md`**, **`docs/mcp_gateway_phase1.md`** (minimal MCP + API gateway first),
//! and **`docs/protocol_evolution.md`** (future protocols).

use std::path::Path;

use http::header::HeaderName;
use serde::Deserialize;

fn format_listen_address(addr: &str, port: u16) -> String {
    if addr.contains(':') && !addr.starts_with('[') {
        format!("[{addr}]:{port}")
    } else {
        format!("{addr}:{port}")
    }
}

/// Optional “Kong handshake”: attested edge injects identity headers; Panda verifies before trusting them.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TrustedGatewayConfig {
    /// Header the edge sets to a shared secret (must match env `PANDA_TRUSTED_GATEWAY_SECRET`).
    #[serde(default)]
    pub attestation_header: Option<String>,
    /// End-user subject after OIDC at the edge (e.g. `X-User-Id`).
    #[serde(default)]
    pub subject_header: Option<String>,
    #[serde(default)]
    pub tenant_header: Option<String>,
    /// Space- or comma-separated scopes (e.g. `X-User-Scopes`).
    #[serde(default)]
    pub scopes_header: Option<String>,
}

impl TrustedGatewayConfig {
    pub(crate) fn validate(&self) -> anyhow::Result<()> {
        for name in [
            self.attestation_header.as_deref(),
            self.subject_header.as_deref(),
            self.tenant_header.as_deref(),
            self.scopes_header.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if name.trim().is_empty() {
                anyhow::bail!("trusted_gateway header names must be non-empty when set");
            }
            HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| anyhow::anyhow!("invalid trusted_gateway header name: {name:?}"))?;
        }
        Ok(())
    }
}

/// EU AI Act–style audit export (stub: local signed JSONL; S3/GCS in `docs/compliance_export.md`).
#[derive(Debug, Clone, Deserialize)]
pub struct ComplianceExportConfig {
    #[serde(default)]
    pub enabled: bool,
    /// `off` | `local_jsonl`. Object-store modes are design-only until wired.
    #[serde(default = "default_compliance_export_mode")]
    pub mode: String,
    /// Directory for `local_jsonl` daily files (`panda-compliance-YYYYMMDD.jsonl`).
    #[serde(default)]
    pub local_path: String,
    /// Optional env var holding shared secret for HMAC-SHA256 over each record (hex in output).
    #[serde(default)]
    pub signing_secret_env: Option<String>,
}

fn default_compliance_export_mode() -> String {
    "off".to_string()
}

impl Default for ComplianceExportConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: default_compliance_export_mode(),
            local_path: String::new(),
            signing_secret_env: None,
        }
    }
}

/// W3C `traceparent` and generated IDs (see `ensure_correlation_id` in proxy).
#[derive(Debug, Clone, Deserialize)]
pub struct ObservabilityConfig {
    #[serde(default = "default_correlation_header")]
    pub correlation_header: String,
    /// Header name used for optional ops endpoint authentication.
    #[serde(default = "default_admin_auth_header")]
    pub admin_auth_header: String,
    /// Optional env var name for shared secret protecting ops endpoints.
    #[serde(default)]
    pub admin_secret_env: Option<String>,
    #[serde(default)]
    pub compliance_export: ComplianceExportConfig,
}

fn default_correlation_header() -> String {
    "x-request-id".to_string()
}

fn default_admin_auth_header() -> String {
    "x-panda-admin-secret".to_string()
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            correlation_header: default_correlation_header(),
            admin_auth_header: default_admin_auth_header(),
            admin_secret_env: None,
            compliance_export: ComplianceExportConfig::default(),
        }
    }
}

/// TPM backends: optional Redis for multi-instance totals (`PANDA_REDIS_URL` overrides `redis_url`).
#[derive(Debug, Clone, Deserialize)]
pub struct TpmConfig {
    #[serde(default)]
    pub redis_url: Option<String>,
    /// If true, enforce token budgets per identity bucket.
    #[serde(default)]
    pub enforce_budget: bool,
    /// Max prompt-token estimate per rolling minute per bucket.
    #[serde(default = "default_tpm_budget_tokens_per_minute")]
    pub budget_tokens_per_minute: u64,
    /// Optional fixed `Retry-After` (seconds) for 429 responses; when unset, derive from window.
    #[serde(default)]
    pub retry_after_seconds: Option<u64>,
    /// When Redis is configured but unreachable at startup, treat TPM as **degraded** (stricter cap).
    #[serde(default = "default_true")]
    pub redis_unavailable_degraded_limits: bool,
    /// When a Redis TPM command fails at runtime, enter degraded mode until a command succeeds again.
    #[serde(default = "default_true")]
    pub redis_command_error_degraded_limits: bool,
    /// Effective budget multiplier while degraded (e.g. `0.5` ≈ 50% “safe mode”).
    #[serde(default = "default_tpm_degraded_limit_ratio")]
    pub redis_degraded_limit_ratio: f64,
}

fn default_tpm_budget_tokens_per_minute() -> u64 {
    60_000
}

fn default_tpm_degraded_limit_ratio() -> f64 {
    0.5
}

fn default_true() -> bool {
    true
}

impl Default for TpmConfig {
    fn default() -> Self {
        Self {
            redis_url: None,
            enforce_budget: false,
            budget_tokens_per_minute: default_tpm_budget_tokens_per_minute(),
            retry_after_seconds: None,
            redis_unavailable_degraded_limits: true,
            redis_command_error_degraded_limits: true,
            redis_degraded_limit_ratio: default_tpm_degraded_limit_ratio(),
        }
    }
}

/// Wasm plugins: load `*.wasm` from `directory` (Phase 2).
#[derive(Debug, Clone, Deserialize)]
pub struct PluginsConfig {
    /// Directory containing `.wasm` guest modules (must exist when set).
    #[serde(default)]
    pub directory: Option<String>,
    /// Max request body bytes buffered for Wasm body hooks.
    #[serde(default = "default_wasm_max_request_body_bytes")]
    pub max_request_body_bytes: usize,
    /// Best-effort per-hook timeout in milliseconds.
    #[serde(default = "default_wasm_execution_timeout_ms")]
    pub execution_timeout_ms: u64,
    /// If true, plugin failures reject request path; if false, fail open and continue.
    #[serde(default)]
    pub fail_closed: bool,
    /// If true, watch plugin directory and hot-swap runtime on changes.
    #[serde(default)]
    pub hot_reload: bool,
    /// Poll interval for hot-reload scans (milliseconds).
    #[serde(default = "default_wasm_reload_interval_ms")]
    pub reload_interval_ms: u64,
    /// Debounce window for filesystem change storms (milliseconds).
    #[serde(default = "default_wasm_reload_debounce_ms")]
    pub reload_debounce_ms: u64,
    /// Max successful hot reloads allowed per rolling minute.
    #[serde(default = "default_wasm_max_reloads_per_minute")]
    pub max_reloads_per_minute: u64,
}

fn default_wasm_max_request_body_bytes() -> usize {
    256 * 1024
}

fn default_wasm_execution_timeout_ms() -> u64 {
    25
}

fn default_wasm_reload_interval_ms() -> u64 {
    2000
}

fn default_wasm_reload_debounce_ms() -> u64 {
    500
}

fn default_wasm_max_reloads_per_minute() -> u64 {
    30
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            directory: None,
            max_request_body_bytes: default_wasm_max_request_body_bytes(),
            execution_timeout_ms: default_wasm_execution_timeout_ms(),
            fail_closed: false,
            hot_reload: false,
            reload_interval_ms: default_wasm_reload_interval_ms(),
            reload_debounce_ms: default_wasm_reload_debounce_ms(),
            max_reloads_per_minute: default_wasm_max_reloads_per_minute(),
        }
    }
}

/// Unified edge auth block (merged into [`IdentityConfig`] at load time).
///
/// Use this for JWKS-backed verification; legacy HS256 remains under `identity`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AuthConfig {
    /// Optional marker (e.g. `"jwt"`); reserved for future auth kinds.
    #[serde(default, rename = "type")]
    pub auth_type: Option<String>,
    /// When set, RSA tokens (`RS256`/`RS384`/`RS512`) are verified using keys from this JWKS URL.
    #[serde(default)]
    pub jwks_url: Option<String>,
    /// When true, sets [`IdentityConfig::require_jwt`] so every proxied route requires a bearer JWT.
    #[serde(default)]
    pub enforce_on_all_routes: bool,
}

/// Optional identity controls for Phase 3 entry.
#[derive(Debug, Clone, Deserialize)]
pub struct IdentityConfig {
    /// If true, require a bearer JWT on proxied requests.
    #[serde(default)]
    pub require_jwt: bool,
    /// Env var name containing HS256 secret (default: PANDA_JWT_HS256_SECRET).
    #[serde(default = "default_jwt_hs256_secret_env")]
    pub jwt_hs256_secret_env: String,
    /// When set, asymmetric JWTs are verified with keys loaded from this JWKS document (HTTPS GET).
    #[serde(default)]
    pub jwks_url: Option<String>,
    /// How long to cache a fetched JWKS before refresh (seconds).
    #[serde(default = "default_jwks_cache_ttl_seconds")]
    pub jwks_cache_ttl_seconds: u64,
    /// Expected issuer (`iss`) values. Empty means issuer is not checked.
    #[serde(default)]
    pub accepted_issuers: Vec<String>,
    /// Expected audience (`aud`) values. Empty means audience is not checked.
    #[serde(default)]
    pub accepted_audiences: Vec<String>,
    /// Required scopes (all must be present). Empty means no scope check.
    #[serde(default)]
    pub required_scopes: Vec<String>,
    /// Route-specific scope requirements (matched by path prefix).
    #[serde(default)]
    pub route_scope_rules: Vec<RouteScopeRule>,
    /// If true, mint a scoped agent token and forward in `x-panda-agent-token`.
    #[serde(default)]
    pub enable_token_exchange: bool,
    /// Env var name containing HS256 secret for minted agent tokens.
    #[serde(default = "default_agent_token_secret_env")]
    pub agent_token_secret_env: String,
    /// Lifetime for minted agent tokens.
    #[serde(default = "default_agent_token_ttl_seconds")]
    pub agent_token_ttl_seconds: u64,
    /// Scopes to embed in minted agent tokens (space-delimited in `scope` claim).
    #[serde(default)]
    pub agent_token_scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RouteScopeRule {
    pub path_prefix: String,
    #[serde(default)]
    pub required_scopes: Vec<String>,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            require_jwt: false,
            jwt_hs256_secret_env: default_jwt_hs256_secret_env(),
            jwks_url: None,
            jwks_cache_ttl_seconds: default_jwks_cache_ttl_seconds(),
            accepted_issuers: vec![],
            accepted_audiences: vec![],
            required_scopes: vec![],
            route_scope_rules: vec![],
            enable_token_exchange: false,
            agent_token_secret_env: default_agent_token_secret_env(),
            agent_token_ttl_seconds: default_agent_token_ttl_seconds(),
            agent_token_scopes: vec![],
        }
    }
}

fn default_jwks_cache_ttl_seconds() -> u64 {
    3600
}

fn default_jwt_hs256_secret_env() -> String {
    "PANDA_JWT_HS256_SECRET".to_string()
}

fn default_agent_token_secret_env() -> String {
    "PANDA_AGENT_TOKEN_HS256_SECRET".to_string()
}

fn default_agent_token_ttl_seconds() -> u64 {
    300
}

/// Starter prompt-safety controls (Phase 3).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PromptSafetyConfig {
    /// If true, run deny-pattern scan on request path/query/body.
    #[serde(default)]
    pub enabled: bool,
    /// If true, log would-block events but do not reject traffic.
    #[serde(default)]
    pub shadow_mode: bool,
    /// Case-insensitive literal patterns that trigger 403 when found.
    #[serde(default)]
    pub deny_patterns: Vec<String>,
}

/// Starter PII scrubbing controls (Phase 3).
#[derive(Debug, Clone, Deserialize)]
pub struct PiiConfig {
    /// If true, redact request body using regex patterns.
    #[serde(default)]
    pub enabled: bool,
    /// If true, detect/log redaction candidates but do not mutate request body.
    #[serde(default)]
    pub shadow_mode: bool,
    /// Regex patterns to redact from buffered request body.
    #[serde(default)]
    pub redact_patterns: Vec<String>,
    /// Replacement text used for each matched PII region.
    #[serde(default = "default_pii_replacement")]
    pub replacement: String,
}

fn default_pii_replacement() -> String {
    "[REDACTED]".to_string()
}

impl Default for PiiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            shadow_mode: false,
            redact_patterns: vec![],
            replacement: default_pii_replacement(),
        }
    }
}

/// Phase 4: semantic cache controls (MVP in-memory backend).
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticCacheConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Backend kind: `memory` (default) or `redis` (Redis-compatible; works with Dragonfly).
    #[serde(default = "default_semantic_cache_backend")]
    pub backend: String,
    /// Optional Redis URL for `backend=redis`; env `PANDA_SEMANTIC_CACHE_REDIS_URL` also supported.
    #[serde(default)]
    pub redis_url: Option<String>,
    /// Similarity threshold placeholder for future embedding/vector backend.
    #[serde(default = "default_semantic_cache_similarity_threshold")]
    pub similarity_threshold: f32,
    /// Max number of cached responses kept in-memory.
    #[serde(default = "default_semantic_cache_max_entries")]
    pub max_entries: usize,
    /// Time-to-live for cache entries in seconds.
    #[serde(default = "default_semantic_cache_ttl_seconds")]
    pub ttl_seconds: u64,
    /// When `true`, the in-memory backend may return a cached body for a **non-identical** prompt
    /// that passes the Jaccard similarity check (same model/tools contract). **Default `false`**
    /// (exact key match only) is safer for multi-tenant and agent workloads. Redis backend is
    /// always exact-key.
    #[serde(default = "default_semantic_cache_similarity_fallback")]
    pub similarity_fallback: bool,
    /// When `true`, append the TPM identity bucket (subject/tenant/session hash) to the cache key
    /// so chat completions do not share entries across principals. Recommended when
    /// `semantic_cache` is enabled for untrusted tenants or agents.
    #[serde(default)]
    pub scope_keys_with_tpm_bucket: bool,
    /// When `true` with `backend: memory`, after an exact key miss, call the embeddings API and
    /// search in-process entries by cosine similarity (uses `similarity_threshold`). Requires
    /// `embedding_url` and `embedding_api_key_env`.
    #[serde(default)]
    pub embedding_lookup_enabled: bool,
    #[serde(default)]
    pub embedding_url: Option<String>,
    #[serde(default = "default_semantic_cache_embedding_model")]
    pub embedding_model: String,
    #[serde(default = "default_semantic_cache_embedding_api_key_env")]
    pub embedding_api_key_env: String,
    #[serde(default = "default_semantic_cache_embedding_timeout_ms")]
    pub embedding_timeout_ms: u64,
    #[serde(default = "default_semantic_cache_embedding_max_prompt_chars")]
    pub embedding_max_prompt_chars: usize,
}

fn default_semantic_cache_embedding_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_semantic_cache_embedding_api_key_env() -> String {
    "OPENAI_API_KEY".to_string()
}

fn default_semantic_cache_embedding_timeout_ms() -> u64 {
    15_000
}

fn default_semantic_cache_embedding_max_prompt_chars() -> usize {
    8192
}

fn default_semantic_cache_similarity_fallback() -> bool {
    false
}

fn default_semantic_cache_similarity_threshold() -> f32 {
    0.92
}

fn default_semantic_cache_backend() -> String {
    "memory".to_string()
}

fn default_semantic_cache_max_entries() -> usize {
    10_000
}

fn default_semantic_cache_ttl_seconds() -> u64 {
    300
}

impl Default for SemanticCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: default_semantic_cache_backend(),
            redis_url: None,
            similarity_threshold: default_semantic_cache_similarity_threshold(),
            max_entries: default_semantic_cache_max_entries(),
            ttl_seconds: default_semantic_cache_ttl_seconds(),
            similarity_fallback: default_semantic_cache_similarity_fallback(),
            scope_keys_with_tpm_bucket: false,
            embedding_lookup_enabled: false,
            embedding_url: None,
            embedding_model: default_semantic_cache_embedding_model(),
            embedding_api_key_env: default_semantic_cache_embedding_api_key_env(),
            embedding_timeout_ms: default_semantic_cache_embedding_timeout_ms(),
            embedding_max_prompt_chars: default_semantic_cache_embedding_max_prompt_chars(),
        }
    }
}

/// Phase 4: universal adapter target provider (expand toward **maximum upstream coverage**—not only OpenAI-shaped backends).
#[derive(Debug, Clone, Deserialize)]
pub struct AdapterConfig {
    /// Backend provider protocol to target from OpenAI-compatible ingress.
    /// **`anthropic`** — native Messages API mapping in `panda-proxy`.
    /// **Any label in [`OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS`]** — same runtime as `openai` (passthrough to
    /// `backend_base`); use a **specific label** (e.g. `groq`, `together`) for clarity in ops and docs.
    /// **Native** non–OpenAI APIs (e.g. some cloud APIs) require new adapters — see `docs/provider_adapters.md`.
    #[serde(default = "default_adapter_provider")]
    pub provider: String,
    /// Anthropic API version header when provider is `anthropic`.
    #[serde(default = "default_adapter_anthropic_version")]
    pub anthropic_version: String,
}

fn default_adapter_provider() -> String {
    "openai".to_string()
}

fn default_adapter_anthropic_version() -> String {
    "2023-06-01".to_string()
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            provider: default_adapter_provider(),
            anthropic_version: default_adapter_anthropic_version(),
        }
    }
}

/// Labels for `adapter.provider` and per-route `type` (`adapter_type`) that use **OpenAI-shaped**
/// HTTP to `backend_base` (passthrough; no request-body mapping). Same idea as multi-provider
/// gateways (e.g. [Portkey AI Gateway](https://github.com/Portkey-AI/gateway)): one client surface,
/// many upstreams that speak `/v1/chat/completions`-style JSON.
///
/// **`anthropic`** is not listed here: it selects the native Anthropic Messages adapter in `panda-proxy`.
///
/// Includes **`gemini`**, **`vertex`**, **`bedrock`** as labels for OpenAI-compatible endpoints documented by
/// Google/AWS; auth and base URLs are described in **`docs/provider_gemini_bedrock_vertex.md`**.
pub const OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS: &[&str] = &[
    "openai",
    "openai_compatible",
    "groq",
    "together",
    "mistral",
    "ollama",
    "openrouter",
    "perplexity",
    "fireworks",
    "deepinfra",
    "anyscale",
    "xai",
    "deepseek",
    "replicate",
    "lambda",
    "moonshot",
    "hyperbolic",
    "siliconflow",
    "novita",
    "azure_openai",
    // Gemini / Vertex / Bedrock: passthrough like `openai` when `backend_base` is the vendor's OpenAI-compatible URL — see `docs/provider_gemini_bedrock_vertex.md`.
    "gemini",
    "vertex",
    "bedrock",
];

fn ensure_adapter_provider_allowed(name: &str) -> anyhow::Result<()> {
    let n = name.trim();
    if n.is_empty() {
        anyhow::bail!("adapter provider must be non-empty");
    }
    if n == "anthropic" {
        return Ok(());
    }
    if OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS.contains(&n) {
        return Ok(());
    }
    anyhow::bail!(
        "adapter.provider must be `anthropic` or an OpenAI-shaped label (openai, gemini, vertex, bedrock, ...); see docs/provider_adapters.md and docs/provider_gemini_bedrock_vertex.md"
    );
}

/// Tuning for MCP **Streamable HTTP** in-memory sessions and GET listener SSE ([spec](https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http)).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct McpStreamableHttpConfig {
    /// Max SSE `message` events retained per session for `Last-Event-ID` replay on GET listener.
    #[serde(default = "default_mcp_streamable_sse_ring_max_events")]
    pub sse_ring_max_events: usize,
    /// How long idle session metadata and replay buffers may live (seconds).
    #[serde(default = "default_mcp_streamable_session_ttl_seconds")]
    pub session_ttl_seconds: u64,
    /// Interval between SSE comment keepalives on the GET listener stream (seconds).
    #[serde(default = "default_mcp_streamable_sse_keepalive_interval_seconds")]
    pub sse_keepalive_interval_seconds: u64,
}

fn default_mcp_streamable_sse_ring_max_events() -> usize {
    64
}

fn default_mcp_streamable_session_ttl_seconds() -> u64 {
    86_400
}

fn default_mcp_streamable_sse_keepalive_interval_seconds() -> u64 {
    20
}

impl Default for McpStreamableHttpConfig {
    fn default() -> Self {
        Self {
            sse_ring_max_events: default_mcp_streamable_sse_ring_max_events(),
            session_ttl_seconds: default_mcp_streamable_session_ttl_seconds(),
            sse_keepalive_interval_seconds: default_mcp_streamable_sse_keepalive_interval_seconds(),
        }
    }
}

/// Phase 4: MCP tool servers (stub wiring; real transports added incrementally).
#[derive(Debug, Clone, Deserialize)]
pub struct McpConfig {
    /// When true, load MCP server entries; the proxy runtime is built in the `panda-proxy` crate.
    #[serde(default)]
    pub enabled: bool,
    /// If true, MCP client errors do not fail the overall request path (until chat/tool loop is wired).
    #[serde(default = "default_mcp_fail_open")]
    pub fail_open: bool,
    #[serde(default = "default_mcp_tool_timeout_ms")]
    pub tool_timeout_ms: u64,
    #[serde(default = "default_mcp_max_tool_payload_bytes")]
    pub max_tool_payload_bytes: usize,
    /// Maximum number of non-streaming tool-call follow-up rounds per request.
    #[serde(default = "default_mcp_max_tool_rounds")]
    pub max_tool_rounds: usize,
    /// Streaming MCP first-round probe budget in bytes before passthrough fallback.
    #[serde(default = "default_mcp_stream_probe_bytes")]
    pub stream_probe_bytes: usize,
    /// Rolling window size in seconds for MCP probe status snapshots.
    #[serde(default = "default_mcp_probe_window_seconds")]
    pub probe_window_seconds: u64,
    /// When true, discovered tools may be advertised to the model (OpenAI-style `tools` field).
    #[serde(default)]
    pub advertise_tools: bool,
    /// Proof-of-intent mode for tool calls: `off`, `audit`, or `enforce`.
    #[serde(default = "default_mcp_proof_of_intent_mode")]
    pub proof_of_intent_mode: String,
    /// Optional allowlist rules: which tool names are allowed for each intent label.
    #[serde(default)]
    pub intent_tool_policies: Vec<McpIntentToolPolicy>,
    #[serde(default)]
    pub servers: Vec<McpServerConfig>,
    /// Optional human-in-the-loop gate before selected MCP tools run (HTTP approval callback).
    #[serde(default)]
    pub hitl: McpHitlConfig,
    /// Declarative tool name patterns → allow/deny (first match wins). See `docs/ai_routing_strategy.md`.
    #[serde(default)]
    pub tool_routes: McpToolRoutesConfig,
    /// Optional cache for deterministic MCP tool results (allowlist + TTL + identity scope).
    #[serde(default)]
    pub tool_cache: McpToolCacheConfig,
    /// Streamable HTTP session store / SSE replay (ingress MCP when clients negotiate streamable transport).
    #[serde(default)]
    pub streamable_http: McpStreamableHttpConfig,
}

/// Cache deterministic MCP tool results by `(scope, server.tool, args_hash)` to reduce repeated token/tool cost.
#[derive(Debug, Clone, Deserialize)]
pub struct McpToolCacheConfig {
    #[serde(default)]
    pub enabled: bool,
    /// MVP backend: `memory` (process-local). Future: `redis`.
    #[serde(default = "default_mcp_tool_cache_backend")]
    pub backend: String,
    /// Default TTL for allowlisted tools unless overridden.
    #[serde(default = "default_mcp_tool_cache_ttl_seconds")]
    pub default_ttl_seconds: u64,
    #[serde(default = "default_mcp_tool_cache_max_value_bytes")]
    pub max_value_bytes: usize,
    /// When `true` and `observability.compliance_export` is enabled, append **`miss`** rows to
    /// `panda.compliance.tool_cache.v1` for allowlisted tools (high volume; default off).
    #[serde(default)]
    pub compliance_log_misses: bool,
    #[serde(default)]
    pub allow: Vec<McpToolCacheAllowRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpToolCacheAllowRule {
    pub server: String,
    pub tool: String,
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
}

/// Pause high-risk MCP tool calls until an external approval endpoint returns success.
#[derive(Debug, Clone, Deserialize)]
pub struct McpHitlConfig {
    #[serde(default)]
    pub enabled: bool,
    /// HTTPS URL that accepts POST JSON and returns `{"approved": true}` or `{"status":"approved"}`.
    #[serde(default)]
    pub approval_url: String,
    #[serde(default = "default_mcp_hitl_timeout_ms")]
    pub timeout_ms: u64,
    /// When set, send `Authorization: Bearer $TOKEN` using this env var (optional).
    #[serde(default)]
    pub bearer_token_env: Option<String>,
    /// Tool keys to gate: OpenAI function name (`server_tool`) and/or `server.tool`.
    #[serde(default)]
    pub tools: Vec<String>,
    /// If true, proceed with the tool when the approval call fails or times out.
    #[serde(default)]
    pub fail_open: bool,
}

fn default_mcp_hitl_timeout_ms() -> u64 {
    120_000
}

impl Default for McpHitlConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            approval_url: String::new(),
            timeout_ms: default_mcp_hitl_timeout_ms(),
            bearer_token_env: None,
            tools: vec![],
            fail_open: false,
        }
    }
}

fn default_mcp_fail_open() -> bool {
    true
}

fn default_mcp_tool_timeout_ms() -> u64 {
    30_000
}

fn default_mcp_max_tool_payload_bytes() -> usize {
    1_048_576
}

fn default_mcp_max_tool_rounds() -> usize {
    4
}

fn default_mcp_stream_probe_bytes() -> usize {
    16 * 1024
}

fn default_mcp_probe_window_seconds() -> u64 {
    60
}

fn default_mcp_proof_of_intent_mode() -> String {
    "off".to_string()
}

fn default_mcp_tool_routes_unmatched() -> String {
    "allow".to_string()
}

fn default_mcp_tool_cache_backend() -> String {
    "memory".to_string()
}

fn default_mcp_tool_cache_ttl_seconds() -> u64 {
    120
}

fn default_mcp_tool_cache_max_value_bytes() -> usize {
    65_536
}

/// Pattern-based filtering for MCP tools advertised and executed (`mcp.tool_routes`).
#[derive(Debug, Clone, Deserialize)]
pub struct McpToolRoutesConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Evaluated in order; first pattern match applies.
    #[serde(default)]
    pub rules: Vec<McpToolRouteRule>,
    /// When no rule matches: `allow` keeps the tool; `deny` drops it.
    #[serde(default = "default_mcp_tool_routes_unmatched")]
    pub unmatched: String,
}

/// One rule: `pattern` uses `*` as a substring wildcard; matched against OpenAI `mcp_server_tool` and `server.tool`.
#[derive(Debug, Clone, Deserialize)]
pub struct McpToolRouteRule {
    pub pattern: String,
    /// `allow` or `deny`
    pub action: String,
    /// For `allow` only: if non-empty, the tool’s MCP server must be listed here.
    #[serde(default)]
    pub servers: Vec<String>,
}

impl Default for McpToolRoutesConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rules: vec![],
            unmatched: default_mcp_tool_routes_unmatched(),
        }
    }
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            fail_open: default_mcp_fail_open(),
            tool_timeout_ms: default_mcp_tool_timeout_ms(),
            max_tool_payload_bytes: default_mcp_max_tool_payload_bytes(),
            max_tool_rounds: default_mcp_max_tool_rounds(),
            stream_probe_bytes: default_mcp_stream_probe_bytes(),
            probe_window_seconds: default_mcp_probe_window_seconds(),
            advertise_tools: false,
            proof_of_intent_mode: default_mcp_proof_of_intent_mode(),
            intent_tool_policies: vec![],
            servers: vec![],
            hitl: McpHitlConfig::default(),
            tool_routes: McpToolRoutesConfig::default(),
            tool_cache: McpToolCacheConfig::default(),
            streamable_http: McpStreamableHttpConfig::default(),
        }
    }
}

impl Default for McpToolCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: default_mcp_tool_cache_backend(),
            default_ttl_seconds: default_mcp_tool_cache_ttl_seconds(),
            max_value_bytes: default_mcp_tool_cache_max_value_bytes(),
            compliance_log_misses: false,
            allow: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpServerConfig {
    /// Stable id for routing (e.g. `filesystem`, `corp-sql`).
    pub name: String,
    #[serde(default = "default_mcp_server_enabled")]
    pub enabled: bool,
    /// When set, spawn this process and speak MCP over stdin/stdout (JSON-RPC, one object per line).
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    /// Declarative REST tool via [`ApiGatewayEgressConfig`] (mutually exclusive with non-empty `command`).
    #[serde(default)]
    pub http_tool: Option<McpHttpToolConfig>,
    /// Multiple declarative REST tools on one logical server (mutually exclusive with `http_tool` and non-empty `command`).
    #[serde(default)]
    pub http_tools: Vec<McpHttpToolConfig>,
    /// Full URL of a remote MCP server speaking JSON-RPC 2.0 over HTTP POST (same shape as Panda ingress MCP).
    /// Requires [`ApiGatewayEgressConfig::enabled`] and allowlist coverage for that URL. Mutually exclusive with `command`, `http_tool`, and `http_tools`.
    #[serde(default)]
    pub remote_mcp_url: Option<String>,
    /// Optional [`ApiGatewayEgressProfile::name`] for headers on `remote_mcp_url` requests.
    #[serde(default)]
    pub remote_mcp_egress_profile: Option<String>,
}

/// Single tool backed by one HTTP call through the API gateway egress client (`api_gateway.egress`).
#[derive(Debug, Clone, Deserialize)]
pub struct McpHttpToolConfig {
    /// HTTP method (default `GET`). `POST` / `PUT` / `PATCH` send `arguments` as JSON body.
    #[serde(default = "default_mcp_http_tool_method")]
    pub method: String,
    /// Path joined with `api_gateway.egress.corporate.default_base` (must start with `/`).
    pub path: String,
    /// MCP tool name for this server (OpenAI function name uses `mcp_{server}_{tool_name}`).
    #[serde(default = "default_mcp_http_tool_tool_name")]
    pub tool_name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Optional [`ApiGatewayEgressProfile::name`] for extra `default_headers` merged after global egress headers.
    #[serde(default)]
    pub egress_profile: Option<String>,
}

fn default_mcp_http_tool_method() -> String {
    "GET".to_string()
}

fn default_mcp_http_tool_tool_name() -> String {
    "call".to_string()
}

fn default_mcp_server_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpIntentToolPolicy {
    /// Intent label (example: `general`, `data_read`, `filesystem`).
    pub intent: String,
    /// Tool names allowed for this intent.
    /// Entries may be full OpenAI function names (`mcp_server_tool`) or canonical `server.tool`.
    #[serde(default)]
    pub allowed_tools: Vec<String>,
}

/// When `agent_profile` matches and the ingress path starts with `path_prefix`, use `backend_base` for that hop
/// (longest matching `path_prefix` wins). Optional `mcp_max_tool_rounds` tightens the MCP tool-followup cap for that request.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentProfileBackendRule {
    pub profile: String,
    pub backend_base: String,
    /// Must start with `/`. Default targets OpenAI-style chat completions.
    #[serde(default = "default_agent_profile_upstream_path_prefix")]
    pub path_prefix: String,
    /// When set, `min(global mcp.max_tool_rounds, this)` for matching requests (non-TPM delegation cap).
    #[serde(default)]
    pub mcp_max_tool_rounds: Option<usize>,
}

fn default_agent_profile_upstream_path_prefix() -> String {
    "/v1/chat".to_string()
}

/// Correlate multi-step agent traffic and optionally isolate TPM buckets per session. See `docs/ai_routing_strategy.md` §3.5.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentSessionsConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Client (or edge) header carrying the agent session id; echoed on responses when present.
    #[serde(default = "default_agent_sessions_header")]
    pub header: String,
    /// Optional header for agent profile / planner id (echoed when present). Applied after JWT claims if configured.
    #[serde(default = "default_agent_sessions_profile_header")]
    pub profile_header: String,
    /// JWT claim name (top-level or in `extra`) for agent session id when Bearer token is valid. Header overrides when both are set.
    #[serde(default)]
    pub jwt_session_claim: Option<String>,
    /// JWT claim for agent profile. Header overrides when both are set.
    #[serde(default)]
    pub jwt_profile_claim: Option<String>,
    /// When set and `agent_session` is present, MCP tool rounds are capped at `min(mcp.max_tool_rounds, this)`.
    #[serde(default)]
    pub mcp_max_tool_rounds_with_session: Option<usize>,
    /// Per-profile backend overrides (planner routing). Requires `agent_profile` from JWT and/or `profile_header`.
    #[serde(default)]
    pub profile_backend_rules: Vec<AgentProfileBackendRule>,
    /// When true, TPM prompt budget keys include a short hash of the session id (separate cap per session).
    #[serde(default = "default_agent_sessions_tpm_isolated")]
    pub tpm_isolated_buckets: bool,
}

fn default_agent_sessions_header() -> String {
    "x-panda-agent-session".to_string()
}

fn default_agent_sessions_profile_header() -> String {
    "x-panda-agent-profile".to_string()
}

fn default_agent_sessions_tpm_isolated() -> bool {
    true
}

impl Default for AgentSessionsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            header: default_agent_sessions_header(),
            profile_header: default_agent_sessions_profile_header(),
            jwt_session_claim: None,
            jwt_profile_claim: None,
            mcp_max_tool_rounds_with_session: None,
            profile_backend_rules: Vec::new(),
            tpm_isolated_buckets: default_agent_sessions_tpm_isolated(),
        }
    }
}

/// Terminate TLS on the **same** `listen` socket (HTTP disabled for that process).
#[derive(Debug, Clone, Deserialize)]
pub struct TlsListenConfig {
    pub cert_pem: String,
    pub key_pem: String,
    /// If set, require a client certificate issued by this CA (PEM path).
    #[serde(default)]
    pub client_ca_pem: Option<String>,
}

/// Optional `server:` block: nested listen/port/TLS (alternative to top-level `listen` / `tls`).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServerSection {
    /// Full `host:port` or `[ipv6]:port` (overrides top-level `listen` when set).
    #[serde(default)]
    pub listen: Option<String>,
    /// When `listen` is unset, combined with `port` (default address `127.0.0.1`).
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub tls: Option<TlsListenConfig>,
}

/// Per-route HTTP rate limit (requests per second, shared across clients for that route).
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct RouteRateLimitConfig {
    pub rps: u32,
}

/// Optional path-based backend override (Kong-style routing light).
///
/// The first matching route with the **longest** `path_prefix` wins; otherwise the top-level
/// [`PandaConfig::default_backend`] is used. Optional [`RouteConfig::methods`] restricts HTTP verbs for
/// that prefix (405 + `Allow` when not listed).
#[derive(Debug, Clone, Deserialize)]
pub struct RouteConfig {
    /// Must start with `/`. Request paths that start with this prefix use `backend_base` for that hop.
    #[serde(alias = "path")]
    pub path_prefix: String,
    pub backend_base: String,
    #[serde(default)]
    pub rate_limit: Option<RouteRateLimitConfig>,
    /// Override TPM budget (tokens/minute) for this path; inherits [`TpmConfig::budget_tokens_per_minute`] when unset.
    #[serde(default)]
    pub tpm_limit: Option<u64>,
    /// Override semantic cache: `None` = use global `semantic_cache.enabled`; `false` = never cache this path.
    #[serde(default)]
    pub semantic_cache: Option<bool>,
    /// Restrict MCP tool exposure/calls to these server names (must exist in global `mcp.servers`).
    #[serde(default)]
    pub mcp_servers: Option<Vec<String>>,
    /// Override adapter for this path (`openai` | `anthropic`); YAML key `type`.
    #[serde(default, rename = "type")]
    pub adapter_type: Option<String>,
    /// If non-empty, only these HTTP methods may be used for this path prefix (e.g. `GET`, `POST`, `PUT`, `PATCH`, `DELETE`).
    /// Omitted or empty → all methods allowed. Values are normalized to canonical form at load time.
    #[serde(default)]
    pub methods: Vec<String>,
    /// Optional overrides for global `routing` (AI routing pipeline).
    #[serde(default)]
    pub routing: Option<RouteRoutingOverrides>,
    /// When set, overrides global [`crate::McpConfig::advertise_tools`] for `POST /v1/chat/completions`
    /// on this path prefix (longest-prefix match). Use `false` for pure HTTP proxy routes (no merged MCP
    /// tool list); `true` to enable advertisement when global is `false`. `None` = inherit global.
    #[serde(default)]
    pub mcp_advertise_tools: Option<bool>,
}

/// On backend HTTP 429, retry the same logical chat request against a secondary provider.
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitFallbackConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Base URL for `anthropic` (e.g. `https://api.anthropic.com`) or full request URL for `openai_compatible`.
    #[serde(default)]
    pub backend_base: String,
    /// `anthropic`: map OpenAI chat JSON → Anthropic Messages API. `openai_compatible`: POST the same JSON to `backend_base`.
    #[serde(default = "default_rate_limit_fallback_provider")]
    pub provider: String,
    /// Env var for the fallback API key (`ANTHROPIC_API_KEY`, Azure `api-key`, etc.).
    #[serde(default = "default_rate_limit_fallback_api_key_env")]
    pub api_key_env: String,
    /// For `openai_compatible` on Azure-style hosts: send `api-key` instead of `Authorization: Bearer`.
    #[serde(default)]
    pub use_api_key_header: bool,
}

fn default_rate_limit_fallback_provider() -> String {
    "anthropic".to_string()
}

fn default_rate_limit_fallback_api_key_env() -> String {
    "ANTHROPIC_API_KEY".to_string()
}

impl Default for RateLimitFallbackConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend_base: String::new(),
            provider: default_rate_limit_fallback_provider(),
            api_key_env: default_rate_limit_fallback_api_key_env(),
            use_api_key_header: false,
        }
    }
}

/// Compress long OpenAI-style chat histories via a summarizer model before forwarding to the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct ContextManagementConfig {
    #[serde(default)]
    pub enabled: bool,
    /// When `messages` length exceeds this, run summarization (after keeping a tail).
    #[serde(default = "default_context_max_messages")]
    pub max_messages: usize,
    /// Recent turns to keep verbatim after summarizing the prefix.
    #[serde(default = "default_context_keep_recent_messages")]
    pub keep_recent_messages: usize,
    /// OpenAI-compatible base URL for the summarizer (e.g. `https://api.openai.com`).
    #[serde(default)]
    pub summarizer_backend_base: String,
    #[serde(default)]
    pub summarizer_model: String,
    /// Env var holding the bearer token for `summarizer_backend_base`.
    #[serde(default = "default_context_summarizer_api_key_env")]
    pub summarizer_api_key_env: String,
    #[serde(default = "default_context_summarizer_timeout_ms")]
    pub request_timeout_ms: u64,
    #[serde(default = "default_context_summary_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_context_summarization_max_tokens")]
    pub summarization_max_tokens: u32,
}

fn default_context_max_messages() -> usize {
    48
}

fn default_context_keep_recent_messages() -> usize {
    16
}

fn default_context_summarizer_api_key_env() -> String {
    "PANDA_SUMMARIZER_API_KEY".to_string()
}

fn default_context_summarizer_timeout_ms() -> u64 {
    60_000
}

fn default_context_summary_system_prompt() -> String {
    "Summarize the following chat messages into a concise factual summary for continuing the conversation. Preserve important entities, decisions, constraints, and open tasks. Do not invent details."
        .to_string()
}

fn default_context_summarization_max_tokens() -> u32 {
    2048
}

impl Default for ContextManagementConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_messages: default_context_max_messages(),
            keep_recent_messages: default_context_keep_recent_messages(),
            summarizer_backend_base: String::new(),
            summarizer_model: String::new(),
            summarizer_api_key_env: default_context_summarizer_api_key_env(),
            request_timeout_ms: default_context_summarizer_timeout_ms(),
            system_prompt: default_context_summary_system_prompt(),
            summarization_max_tokens: default_context_summarization_max_tokens(),
        }
    }
}

/// OIDC login for the Developer Console (Okta, Entra, etc.). Off by default (Core track).
#[derive(Debug, Clone, Deserialize)]
pub struct ConsoleOidcConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Issuer base URL (e.g. `https://login.microsoftonline.com/{tenant}/v2.0`).
    #[serde(default)]
    pub issuer_url: String,
    #[serde(default)]
    pub client_id: String,
    /// Env var holding the OIDC client secret (confidential client).
    #[serde(default)]
    pub client_secret_env: String,
    /// Browser redirect URI path on this host (must match IdP app registration).
    #[serde(default = "default_console_oidc_redirect_path")]
    pub redirect_path: String,
    /// Public base URL for redirects (e.g. `https://panda.example.com`). If empty, derived from request `Host` (dev only).
    #[serde(default)]
    pub redirect_base_url: String,
    #[serde(default = "default_console_oidc_scopes")]
    pub scopes: Vec<String>,
    #[serde(default = "default_console_oidc_cookie_name")]
    pub cookie_name: String,
    #[serde(default = "default_console_oidc_session_ttl_seconds")]
    pub session_ttl_seconds: u64,
    /// Env var with secret bytes for signing console session cookies (HS256).
    #[serde(default = "default_console_oidc_signing_secret_env")]
    pub signing_secret_env: String,
    /// Optional JWT claim name on the IdP `id_token` for RBAC (string or array of strings).
    #[serde(default)]
    pub roles_claim: String,
    /// If non-empty, require at least one of these role strings in `roles_claim`.
    #[serde(default)]
    pub required_roles: Vec<String>,
    /// How to evaluate `required_roles` against the IdP token: `any` (default) = at least one match; `all` = every listed role must be present.
    #[serde(default = "default_console_oidc_required_roles_mode")]
    pub required_roles_mode: String,
    /// Require PKCE S256 in OIDC login/callback flow (recommended for public/confidential clients).
    #[serde(default = "default_console_oidc_require_pkce")]
    pub require_pkce: bool,
    /// Include and validate OIDC nonce claim in `id_token`.
    #[serde(default = "default_console_oidc_require_nonce")]
    pub require_nonce: bool,
    /// Force `Secure` session cookie attribute. When false, HTTPS `redirect_base_url` still enables Secure.
    #[serde(default)]
    pub force_secure_cookie: bool,
}

fn default_console_oidc_redirect_path() -> String {
    "/console/oauth/callback".to_string()
}

fn default_console_oidc_scopes() -> Vec<String> {
    vec![
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ]
}

fn default_console_oidc_cookie_name() -> String {
    "panda_console_session".to_string()
}

fn default_console_oidc_session_ttl_seconds() -> u64 {
    86400
}

fn default_console_oidc_signing_secret_env() -> String {
    "PANDA_CONSOLE_SESSION_SECRET".to_string()
}

fn default_console_oidc_require_pkce() -> bool {
    true
}

fn default_console_oidc_require_nonce() -> bool {
    true
}

fn default_console_oidc_required_roles_mode() -> String {
    "any".to_string()
}

impl Default for ConsoleOidcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            issuer_url: String::new(),
            client_id: String::new(),
            client_secret_env: String::new(),
            redirect_path: default_console_oidc_redirect_path(),
            redirect_base_url: String::new(),
            scopes: default_console_oidc_scopes(),
            cookie_name: default_console_oidc_cookie_name(),
            session_ttl_seconds: default_console_oidc_session_ttl_seconds(),
            signing_secret_env: default_console_oidc_signing_secret_env(),
            roles_claim: String::new(),
            required_roles: vec![],
            required_roles_mode: default_console_oidc_required_roles_mode(),
            require_pkce: default_console_oidc_require_pkce(),
            require_nonce: default_console_oidc_require_nonce(),
            force_secure_cookie: false,
        }
    }
}

/// Per-department prompt-token budget (rolling minute), plus optional org-wide cap (Enterprise).
#[derive(Debug, Clone, Deserialize)]
pub struct BudgetHierarchyDepartmentLimit {
    pub department: String,
    pub prompt_tokens_per_minute: u64,
}

/// Hierarchical token budgets keyed by a JWT claim (e.g. `department`). Requires Redis.
#[derive(Debug, Clone, Deserialize)]
pub struct BudgetHierarchyConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Top-level claim name on the JWT payload (e.g. `department` or `cost_center`).
    #[serde(default = "default_budget_hierarchy_jwt_claim")]
    pub jwt_claim: String,
    /// Optional org-wide prompt budget per rolling minute (all departments share this cap).
    #[serde(default)]
    pub org_prompt_tokens_per_minute: Option<u64>,
    #[serde(default)]
    pub departments: Vec<BudgetHierarchyDepartmentLimit>,
    /// Override Redis URL; defaults to `tpm.redis_url` / `PANDA_REDIS_URL` when unset.
    #[serde(default)]
    pub redis_url: Option<String>,
    /// If true, Redis evaluation failures allow traffic (fail-open). Defaults to fail-closed.
    #[serde(default)]
    pub fail_open: bool,
    /// Optional blend rate for **estimated** USD of prompt tokens in the current hierarchy window (`GET /tpm/status`). Does not enforce spend caps by itself.
    #[serde(default)]
    pub usd_per_million_prompt_tokens: Option<f64>,
}

fn default_budget_hierarchy_jwt_claim() -> String {
    "department".to_string()
}

impl Default for BudgetHierarchyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            jwt_claim: default_budget_hierarchy_jwt_claim(),
            org_prompt_tokens_per_minute: None,
            departments: vec![],
            redis_url: None,
            fail_open: false,
            usd_per_million_prompt_tokens: None,
        }
    }
}

/// Wire protocol for a failover hop (ingress remains OpenAI-shaped unless transformed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFailoverProtocol {
    /// OpenAI Chat Completions, Embeddings, Responses, etc. (same JSON/path as client).
    #[serde(alias = "openai")]
    OpenaiCompatible,
    /// Anthropic Messages API — Panda maps OpenAI chat JSON → `/v1/messages` for this hop only.
    Anthropic,
}

impl Default for ModelFailoverProtocol {
    fn default() -> Self {
        Self::OpenaiCompatible
    }
}

/// Which logical API this backend accepts for failover (empty `supports` = defaults for [`ModelFailoverProtocol`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFailoverOperation {
    ChatCompletions,
    Embeddings,
    Responses,
    Images,
    Audio,
}

/// One physical backend in a model parity / failover chain.
#[derive(Debug, Clone, Deserialize)]
pub struct ModelFailoverBackend {
    pub backend_base: String,
    /// When set, replace `Authorization` (or `api-key`) for this hop only.
    #[serde(default)]
    pub api_key_env: Option<String>,
    #[serde(default)]
    pub use_api_key_header: bool,
    #[serde(default)]
    pub protocol: ModelFailoverProtocol,
    /// Empty: infer from `protocol` (OpenAI: chat + embeddings + responses; Anthropic: chat only).
    #[serde(default)]
    pub supports: Vec<ModelFailoverOperation>,
    /// When `false`, this backend is skipped for streaming chat requests. Default: accept streaming.
    #[serde(default)]
    pub supports_streaming: Option<bool>,
}

/// Map request `model` names to an ordered list of backends (model parity / failover).
#[derive(Debug, Clone, Deserialize)]
pub struct ModelFailoverGroup {
    /// If empty, matches any `model` on the chat path.
    #[serde(default)]
    pub match_models: Vec<String>,
    #[serde(default)]
    pub backends: Vec<ModelFailoverBackend>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelFailoverConfig {
    #[serde(default)]
    pub enabled: bool,
    /// Prefix for OpenAI-style chat (e.g. `/v1/chat` matches `/v1/chat/completions`).
    #[serde(default = "default_model_failover_path_prefix")]
    pub path_prefix: String,
    /// Optional prefix for `/v1/embeddings` (or compatible) failover. When unset, embeddings are not in the failover map.
    #[serde(default)]
    pub embeddings_path_prefix: Option<String>,
    /// Optional prefix for OpenAI Responses API failover (`/v1/responses`). Pass-through only (no Anthropic mapping).
    #[serde(default)]
    pub responses_path_prefix: Option<String>,
    /// Optional prefix for OpenAI Images API failover (`/v1/images`).
    #[serde(default)]
    pub images_path_prefix: Option<String>,
    /// Optional prefix for OpenAI Audio API failover (`/v1/audio`).
    #[serde(default)]
    pub audio_path_prefix: Option<String>,
    /// When `true`, streaming OpenAI-style SSE chat responses under model failover are **fully
    /// buffered** (up to `midstream_sse_max_buffer_bytes`) so Panda can retry the **next** backend
    /// if the winning upstream connection fails mid-body. **TTFT increases** (no incremental bytes
    /// until the full response is collected or a backend succeeds). Anthropic adapter streaming is
    /// excluded. When `false`, behavior is unchanged (no failover after response headers on 200).
    #[serde(default)]
    pub allow_failover_after_first_byte: bool,
    /// Max bytes to buffer per SSE response when `allow_failover_after_first_byte` applies.
    #[serde(default = "default_model_failover_midstream_max_buffer_bytes")]
    pub midstream_sse_max_buffer_bytes: usize,
    /// Enable local in-process failover circuit breaker.
    #[serde(default)]
    pub circuit_breaker_enabled: bool,
    /// Open circuit after this many consecutive retryable failures per backend.
    #[serde(default = "default_model_failover_cb_failure_threshold")]
    pub circuit_breaker_failure_threshold: u32,
    /// Keep circuit open for this many seconds.
    #[serde(default = "default_model_failover_cb_open_seconds")]
    pub circuit_breaker_open_seconds: u64,
    #[serde(default)]
    pub groups: Vec<ModelFailoverGroup>,
}

fn default_model_failover_path_prefix() -> String {
    "/v1/chat".to_string()
}

fn default_model_failover_cb_failure_threshold() -> u32 {
    3
}

fn default_model_failover_cb_open_seconds() -> u64 {
    30
}

fn default_model_failover_midstream_max_buffer_bytes() -> usize {
    16 * 1024 * 1024
}

impl Default for ModelFailoverConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            path_prefix: default_model_failover_path_prefix(),
            embeddings_path_prefix: None,
            responses_path_prefix: None,
            images_path_prefix: None,
            audio_path_prefix: None,
            allow_failover_after_first_byte: false,
            midstream_sse_max_buffer_bytes: default_model_failover_midstream_max_buffer_bytes(),
            circuit_breaker_enabled: false,
            circuit_breaker_failure_threshold: default_model_failover_cb_failure_threshold(),
            circuit_breaker_open_seconds: default_model_failover_cb_open_seconds(),
            groups: vec![],
        }
    }
}

/// Per-path overrides for [`RoutingConfig`] (unset fields inherit from global `routing`).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RouteRoutingOverrides {
    /// When set, overrides [`RoutingConfig::enabled`] for this route’s prefix.
    #[serde(default)]
    pub enabled: Option<bool>,
    /// When set, overrides [`RoutingConfig::shadow_mode`].
    #[serde(default)]
    pub shadow_mode: Option<bool>,
    /// When set, overrides [`SemanticRoutingConfig::enabled`] for this path (still requires effective routing on).
    #[serde(default)]
    pub semantic_enabled: Option<bool>,
}

/// Named backend for semantic routing (`embed`, `classifier`, or `llm_judge`).
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticRouteTarget {
    /// Short id for logs and router JSON (`target` must match this exactly).
    pub name: String,
    /// Embed mode: non-empty text embedded at warmup. Classifier / `llm_judge`: optional hint shown to the router model.
    pub routing_text: String,
    /// OpenAI-compatible API base for chat when this target wins (e.g. `https://api.openai.com/v1`).
    pub backend_base: String,
}

/// Semantic / model-intent routing stage (embeddings, classifier, or LLM judge); executed in `panda-proxy` for chat JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticRoutingConfig {
    #[serde(default)]
    pub enabled: bool,
    /// `off` | `embed` | `classifier` | `llm_judge` (see [`PandaConfig::validate`]).
    #[serde(default = "default_semantic_routing_mode")]
    pub mode: String,
    /// OpenAI-compatible base URL for embedding requests (e.g. `https://api.openai.com/v1`).
    #[serde(default)]
    pub embed_backend_base: String,
    /// Env var for bearer key used with `embed_backend_base` when `mode` is `embed`.
    #[serde(default)]
    pub embed_api_key_env: String,
    /// Chat model id for `/v1/chat/completions` when `mode` is `classifier` or `llm_judge`.
    #[serde(default = "default_semantic_routing_router_model")]
    pub router_model: String,
    /// When true, send `response_format: {type: json_object}` on router chat (OpenAI-compatible only).
    #[serde(default)]
    pub router_response_json: bool,
    /// Model id for `/v1/embeddings` (e.g. `text-embedding-3-small`).
    #[serde(default = "default_semantic_routing_embed_model")]
    pub embed_model: String,
    /// Minimum cosine similarity (after L2 normalization) to select a target; else static upstream is used.
    #[serde(default = "default_semantic_routing_similarity_threshold")]
    pub similarity_threshold: f32,
    #[serde(default)]
    pub targets: Vec<SemanticRouteTarget>,
    /// HTTP base for classifier or LLM-judge routing when `mode` is `classifier` or `llm_judge`.
    #[serde(default)]
    pub router_backend_base: String,
    #[serde(default)]
    pub router_api_key_env: String,
    #[serde(default = "default_semantic_routing_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default = "default_semantic_routing_cache_ttl_seconds")]
    pub cache_ttl_seconds: u64,
    #[serde(default = "default_semantic_routing_max_prompt_chars")]
    pub max_prompt_chars: usize,
}

fn default_semantic_routing_mode() -> String {
    "off".to_string()
}

fn default_semantic_routing_timeout_ms() -> u64 {
    5000
}

fn default_semantic_routing_cache_ttl_seconds() -> u64 {
    3600
}

fn default_semantic_routing_max_prompt_chars() -> usize {
    4096
}

fn default_semantic_routing_embed_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_semantic_routing_router_model() -> String {
    "gpt-4o-mini".to_string()
}

fn default_semantic_routing_similarity_threshold() -> f32 {
    0.55
}

impl Default for SemanticRoutingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: default_semantic_routing_mode(),
            embed_backend_base: String::new(),
            embed_api_key_env: String::new(),
            embed_model: default_semantic_routing_embed_model(),
            similarity_threshold: default_semantic_routing_similarity_threshold(),
            targets: vec![],
            router_backend_base: String::new(),
            router_api_key_env: String::new(),
            router_model: default_semantic_routing_router_model(),
            router_response_json: false,
            timeout_ms: default_semantic_routing_timeout_ms(),
            cache_ttl_seconds: default_semantic_routing_cache_ttl_seconds(),
            max_prompt_chars: default_semantic_routing_max_prompt_chars(),
        }
    }
}

/// Pluggable AI routing pipeline (beyond static path → upstream). See `docs/ai_routing_strategy.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct RoutingConfig {
    /// Master switch for non-static routing stages (semantic, future tool/model routing).
    #[serde(default)]
    pub enabled: bool,
    /// When true, log or trace routing decisions without changing the effective upstream (when implemented).
    #[serde(default)]
    pub shadow_mode: bool,
    /// `static` — on failure, use path-based upstream only. `deny` — fail closed (503/502 when implemented).
    #[serde(default = "default_routing_fallback")]
    pub fallback: String,
    #[serde(default)]
    pub semantic: SemanticRoutingConfig,
}

fn default_routing_fallback() -> String {
    "static".to_string()
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            shadow_mode: false,
            fallback: default_routing_fallback(),
            semantic: SemanticRoutingConfig::default(),
        }
    }
}

/// Panda built-in API gateway (ingress / egress). Phase A: flags and config surface only; see
/// `docs/design_api_gateway_and_mcp_gateway.md` and `docs/implementation_plan_mcp_api_gateway.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayConfig {
    #[serde(default)]
    pub ingress: ApiGatewayIngressConfig,
    #[serde(default)]
    pub egress: ApiGatewayEgressConfig,
}

/// Optional Redis for **shared** per-second RPS counters (`INCR` + short TTL) for
/// [`RouteConfig::rate_limit`] and [`ApiGatewayIngressRoute::rate_limit`]. When unset or Redis
/// unavailable, limits fall back to process-local windows.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiGatewayIngressRateLimitRedisConfig {
    /// Env var holding Redis URL (same pattern as `PANDA_REDIS_URL` elsewhere).
    #[serde(default)]
    pub url_env: Option<String>,
    #[serde(default = "default_api_gateway_ingress_rl_redis_prefix")]
    pub key_prefix: String,
}

fn default_api_gateway_ingress_rl_redis_prefix() -> String {
    "panda:gw:ingress_rps".to_string()
}

impl Default for ApiGatewayIngressRateLimitRedisConfig {
    fn default() -> Self {
        Self {
            url_env: None,
            key_prefix: default_api_gateway_ingress_rl_redis_prefix(),
        }
    }
}

/// Ingress sits in front of MCP + chat handlers (TLS, routing, rate limits — phased implementation).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayIngressConfig {
    /// When true, only paths matching [`ApiGatewayIngressConfig::routes`] (or the built-in default table
    /// when `routes` is empty) reach existing handlers; others get 404.
    #[serde(default)]
    pub enabled: bool,
    /// Longest `path_prefix` wins. When empty and `enabled`, Panda uses built-in prefixes for `/v1`, `/mcp`,
    /// `/console`, `/health`, `/metrics`, and other ops endpoints (see `docs/implementation_plan_mcp_api_gateway.md`).
    #[serde(default)]
    pub routes: Vec<ApiGatewayIngressRoute>,
    /// Optional Redis-backed RPS counters for ingress + top-level [`RouteConfig::rate_limit`].
    #[serde(default)]
    pub rate_limit_redis: ApiGatewayIngressRateLimitRedisConfig,
}

/// JWT / bearer policy for a single ingress row (overrides global `identity.require_jwt` for that prefix).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ApiGatewayIngressAuthMode {
    /// Use global [`crate::IdentityConfig::require_jwt`].
    #[default]
    Inherit,
    /// Require a valid Bearer JWT for this prefix even when global `require_jwt` is false.
    Required,
    /// Do not enforce JWT on this prefix even when global `require_jwt` is true (public or internal-only paths).
    Optional,
}

/// One row in the ingress route table (`path_prefix` longest match wins).
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ApiGatewayIngressRoute {
    /// When set (non-empty after trim), this row applies only to requests whose resolved tenant matches
    /// ([`crate::ControlPlaneConfig::tenant_resolution_header`]). Empty/absent = global (all tenants).
    #[serde(default)]
    pub tenant_id: Option<String>,
    pub path_prefix: String,
    pub backend: ApiGatewayIngressBackend,
    /// When non-empty, only these HTTP methods reach the backend; others get **405** (with `Allow`).
    #[serde(default)]
    pub methods: Vec<String>,
    /// When set and `backend` is **`ai`**, use this URL as the backend base for matching paths (overrides `routes` / top-level `default_backend` resolution for that hop).
    #[serde(default)]
    pub backend_base: Option<String>,
    /// Optional per-prefix RPS cap (same semantics as top-level `routes[].rate_limit`).
    #[serde(default)]
    pub rate_limit: Option<RouteRateLimitConfig>,
    /// Per-prefix JWT requirement (see [`ApiGatewayIngressAuthMode`]).
    #[serde(default)]
    pub auth: ApiGatewayIngressAuthMode,
}

impl Default for ApiGatewayIngressRoute {
    fn default() -> Self {
        Self {
            tenant_id: None,
            path_prefix: String::new(),
            backend: ApiGatewayIngressBackend::Ai,
            methods: Vec::new(),
            backend_base: None,
            rate_limit: None,
            auth: ApiGatewayIngressAuthMode::default(),
        }
    }
}

/// Shared validation for [`ApiGatewayIngressRoute`] (static YAML and control-plane HTTP upsert/import).
pub fn validate_ingress_route_row(r: &ApiGatewayIngressRoute) -> anyhow::Result<()> {
    let p = r.path_prefix.trim();
    if p.is_empty() {
        anyhow::bail!("path_prefix must not be empty");
    }
    if !p.starts_with('/') {
        anyhow::bail!("path_prefix must start with `/`: {:?}", r.path_prefix);
    }
    for (j, mth) in r.methods.iter().enumerate() {
        let m = mth.trim();
        if m.is_empty() {
            anyhow::bail!("methods[{j}] must not be empty when methods list is non-empty");
        }
        http::Method::from_bytes(m.as_bytes()).map_err(|_| {
            anyhow::anyhow!("methods[{j}] invalid HTTP method: {:?}", mth)
        })?;
    }
    let u = r
        .backend_base
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    if let Some(base) = u {
        if r.backend != ApiGatewayIngressBackend::Ai {
            anyhow::bail!("backend_base is only valid when backend is `ai`");
        }
        let uri: http::Uri = base
            .parse()
            .map_err(|e| anyhow::anyhow!("backend_base: invalid URL: {e}"))?;
        if uri.scheme_str() != Some("https") && uri.scheme_str() != Some("http") {
            anyhow::bail!("backend_base must use http or https");
        }
        if uri.host().is_none() {
            anyhow::bail!("backend_base must include a host");
        }
    }
    if let Some(ref rl) = r.rate_limit {
        if rl.rps == 0 {
            anyhow::bail!("rate_limit.rps must be > 0 when set");
        }
    }
    Ok(())
}

impl ApiGatewayIngressRoute {
    /// Validate a single row for control-plane upsert/import (same rules as [`validate_ingress_route_row`]).
    pub fn validate_for_control_plane(&self) -> anyhow::Result<()> {
        validate_ingress_route_row(self)
    }
}

/// Logical backend for an ingress path (maps to existing dispatch branches in `panda-proxy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiGatewayIngressBackend {
    /// OpenAI-shaped proxy (`forward_to_upstream`).
    Ai,
    /// Native MCP HTTP surface (JSON-RPC POST; minimal streamable HTTP when requested).
    Mcp,
    /// Health, metrics, console, TPM/MCP status JSON, etc.
    Ops,
    /// Explicit deny (403).
    Deny,
    /// **410 Gone** (deprecated or removed surface).
    Gone,
    /// **404 Not Found** for this prefix (explicit tombstone vs unmatched path).
    NotFound,
}

/// Optional TLS for the corporate egress HTTPS client (mTLS + private PKI roots).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiGatewayEgressTlsConfig {
    /// PEM file path for the client certificate chain (mutual TLS). Requires [`Self::client_key_pem`].
    pub client_cert_pem: Option<String>,
    /// PEM file path for the client private key. Requires [`Self::client_cert_pem`].
    pub client_key_pem: Option<String>,
    /// Optional PEM file with one or more extra CA certificates (e.g. corporate issuing CA).
    /// Mozilla WebPKI roots are always loaded in addition.
    pub extra_ca_pem: Option<String>,
    /// Minimum TLS version for outbound connections: `tls12` or `tls13` (case-insensitive). Empty = rustls default (1.2+).
    pub min_protocol_version: Option<String>,
    /// Unix only: reload client certs / trust roots from disk on `SIGHUP` (re-read PEM paths from config).
    pub reload_on_sighup: bool,
    /// When > 0, poll PEM file mtimes on this interval (ms) and reload the egress HTTPS client if any changed.
    pub watch_reload_ms: u64,
    /// Optional cipher suite allow-list (IANA / rustls variant names, or `0x1301` hex). Empty = rustls defaults.
    #[serde(default)]
    pub cipher_suites: Vec<String>,
}

impl Default for ApiGatewayEgressTlsConfig {
    fn default() -> Self {
        Self {
            client_cert_pem: None,
            client_key_pem: None,
            extra_ca_pem: None,
            min_protocol_version: None,
            reload_on_sighup: false,
            watch_reload_ms: 0,
            cipher_suites: Vec::new(),
        }
    }
}

fn ensure_rustls_ring_provider_for_validate() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("install rustls ring crypto provider for config validation");
    });
}

fn load_pem_certificate_chain(path: &std::path::Path, ctx: &str) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    use anyhow::Context;
    use rustls::pki_types::CertificateDer;
    use std::fs::File;
    use std::io::BufReader;
    let mut reader = BufReader::new(
        File::open(path).with_context(|| format!("{ctx}: open {}", path.display()))?,
    );
    let mut out = Vec::new();
    for item in rustls_pemfile::certs(&mut reader) {
        let der = item.with_context(|| format!("{ctx}: invalid certificate PEM in {}", path.display()))?;
        out.push(CertificateDer::from(der));
    }
    anyhow::ensure!(
        !out.is_empty(),
        "{ctx}: no certificates found in {}",
        path.display()
    );
    Ok(out)
}

fn load_pem_private_key(path: &std::path::Path, ctx: &str) -> anyhow::Result<rustls::pki_types::PrivateKeyDer<'static>> {
    use anyhow::Context;
    use std::fs::File;
    use std::io::BufReader;
    let mut reader = BufReader::new(
        File::open(path).with_context(|| format!("{ctx}: open {}", path.display()))?,
    );
    rustls_pemfile::private_key(&mut reader)
        .context(format!("{ctx}: read private key PEM from {}", path.display()))?
        .with_context(|| format!("{ctx}: no usable private key in {}", path.display()))
}

fn webpki_root_store_arc() -> std::sync::Arc<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    std::sync::Arc::new(roots)
}

/// Resolve egress TLS cipher names to rustls `SupportedCipherSuite` values (ring provider).
pub fn resolve_egress_cipher_suite_names(
    names: &[String],
) -> anyhow::Result<Vec<rustls::SupportedCipherSuite>> {
    use rustls::crypto::ring::ALL_CIPHER_SUITES;
    use rustls::CipherSuite;
    let mut out = Vec::new();
    for n in names {
        let t = n.trim();
        if t.is_empty() {
            anyhow::bail!("empty cipher suite entry");
        }
        let found: Option<rustls::SupportedCipherSuite> =
            if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                let v = u16::from_str_radix(hex, 16)?;
                let cs = CipherSuite::from(v);
                ALL_CIPHER_SUITES.iter().find(|s| s.suite() == cs).copied()
            } else {
                ALL_CIPHER_SUITES
                    .iter()
                    .find(|s| {
                        s.suite().as_str().map(|x| x == t).unwrap_or(false)
                            || format!("{:?}", s.suite()).eq_ignore_ascii_case(t)
                    })
                    .copied()
            };
        let Some(s) = found else {
            anyhow::bail!("unknown or unsupported cipher suite: {t:?}");
        };
        out.push(s);
    }
    Ok(out)
}

impl ApiGatewayEgressTlsConfig {
    /// Path consistency, file existence, and **PEM parse checks** (extra CA trust anchors; client cert + key usable as `rustls` client identity).
    pub fn validate(&self) -> anyhow::Result<()> {
        if let Some(ref v) = self.min_protocol_version {
            let t = v.trim().to_ascii_lowercase();
            if !t.is_empty() && t != "tls12" && t != "tls13" {
                anyhow::bail!(
                    "api_gateway.egress.tls.min_protocol_version must be tls12, tls13, or empty (got {v:?})"
                );
            }
        }
        if self.watch_reload_ms > 86_400_000 {
            anyhow::bail!("api_gateway.egress.tls.watch_reload_ms must be <= 86400000 (24 hours)");
        }
        let cert = self
            .client_cert_pem
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let key = self
            .client_key_pem
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        match (cert, key) {
            (Some(_), Some(_)) | (None, None) => {}
            _ => anyhow::bail!(
                "api_gateway.egress.tls: set both client_cert_pem and client_key_pem for mTLS, or omit both"
            ),
        }
        if let Some(p) = cert {
            let path = std::path::Path::new(p);
            if !path.is_file() {
                anyhow::bail!(
                    "api_gateway.egress.tls.client_cert_pem: not a readable file: {p:?}"
                );
            }
        }
        if let Some(p) = key {
            let path = std::path::Path::new(p);
            if !path.is_file() {
                anyhow::bail!(
                    "api_gateway.egress.tls.client_key_pem: not a readable file: {p:?}"
                );
            }
        }
        if let Some(p) = self
            .extra_ca_pem
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if !std::path::Path::new(p).is_file() {
                anyhow::bail!("api_gateway.egress.tls.extra_ca_pem: not a readable file: {p:?}");
            }
        }

        let extra_path = self
            .extra_ca_pem
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let has_client_auth = cert.is_some() && key.is_some();
        if extra_path.is_none() && !has_client_auth && self.cipher_suites.is_empty() {
            return Ok(());
        }

        ensure_rustls_ring_provider_for_validate();

        if !self.cipher_suites.is_empty() {
            resolve_egress_cipher_suite_names(&self.cipher_suites).map_err(|e| {
                anyhow::anyhow!("api_gateway.egress.tls.cipher_suites: {e}")
            })?;
        }

        if extra_path.is_none() && !has_client_auth {
            return Ok(());
        }

        if let Some(p) = extra_path {
            let path = std::path::Path::new(p);
            let cas = load_pem_certificate_chain(path, "api_gateway.egress.tls.extra_ca_pem")?;
            let mut roots = rustls::RootCertStore::empty();
            for c in cas {
                roots.add(c).map_err(|e| {
                    anyhow::anyhow!(
                        "api_gateway.egress.tls.extra_ca_pem: not a valid trust anchor ({path:?}): {e}"
                    )
                })?;
            }
        }

        if let (Some(cp), Some(kp)) = (cert, key) {
            let cert_path = std::path::Path::new(cp);
            let key_path = std::path::Path::new(kp);
            let certs =
                load_pem_certificate_chain(cert_path, "api_gateway.egress.tls.client_cert_pem")?;
            let key = load_pem_private_key(key_path, "api_gateway.egress.tls.client_key_pem")?;
            rustls::ClientConfig::builder()
                .with_root_certificates(webpki_root_store_arc())
                .with_client_auth_cert(certs, key)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "api_gateway.egress.tls: client certificate and private key cannot be used as a TLS client identity: {e}"
                    )
                })?;
        }

        Ok(())
    }
}

/// Redis-backed shared counters for `api_gateway.egress.rate_limit.max_rps` (optional).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiGatewayEgressRateLimitRedisConfig {
    /// Environment variable name that must resolve to a Redis URL when set (same pattern as ingress RPS Redis).
    pub url_env: Option<String>,
    pub key_prefix: String,
}

fn default_api_gateway_egress_rl_redis_prefix() -> String {
    "panda:gw:egress_rps".to_string()
}

impl Default for ApiGatewayEgressRateLimitRedisConfig {
    fn default() -> Self {
        Self {
            url_env: None,
            key_prefix: default_api_gateway_egress_rl_redis_prefix(),
        }
    }
}

/// Per `route_label` (egress HTTP `EgressHttpRequest.route_label`) RPS cap (fixed 1s window).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayEgressPerRouteRateLimit {
    pub route_label: String,
    pub max_rps: u32,
}

/// Process-local and optional Redis-backed caps for the corporate egress HTTP client (Phase G3).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiGatewayEgressRateLimitConfig {
    /// Max concurrent egress HTTP `request` operations per process (see `EgressClient` in `panda-proxy`)
    /// (held from first attempt through final response, including retries). `0` = unlimited.
    /// Fail-fast when the cap is reached (no queue).
    pub max_in_flight: u32,
    /// Max egress `request` calls per second (fixed 1 second window). `0` = unlimited.
    /// When [`ApiGatewayEgressRateLimitRedisConfig::url_env`] resolves, this limit is enforced cluster-wide in Redis.
    pub max_rps: u32,
    #[serde(default)]
    pub redis: ApiGatewayEgressRateLimitRedisConfig,
    /// Additional per-`route_label` RPS caps (local or Redis when `redis` is configured).
    #[serde(default)]
    pub per_route: Vec<ApiGatewayEgressPerRouteRateLimit>,
}

impl Default for ApiGatewayEgressRateLimitConfig {
    fn default() -> Self {
        Self {
            max_in_flight: 0,
            max_rps: 0,
            redis: ApiGatewayEgressRateLimitRedisConfig::default(),
            per_route: Vec::new(),
        }
    }
}

/// Egress sits behind MCP for HTTP/tool calls toward corporate API gateways (Phase B+).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayEgressConfig {
    /// When true, HTTP tool paths may use the egress client (Phase B+).
    #[serde(default)]
    pub enabled: bool,
    /// Total per-request budget (milliseconds) for connect + response; `0` means use internal default (30s).
    #[serde(default = "default_api_gateway_egress_timeout_ms")]
    pub timeout_ms: u64,
    /// Hyper client pool idle timeout (milliseconds). `0` disables explicit idle timeout (library default).
    #[serde(default = "default_api_gateway_egress_pool_idle_timeout_ms")]
    pub pool_idle_timeout_ms: u64,
    /// Headers merged on every egress request before per-call headers (per-call wins on duplicate keys). Use `value_env` for secrets.
    #[serde(default)]
    pub default_headers: Vec<ApiGatewayEgressDefaultHeader>,
    /// Retry policy for transient upstream failures and **429 / 502 / 503 / 504** responses.
    #[serde(default)]
    pub retry: ApiGatewayEgressRetryConfig,
    /// Named header sets merged after global [`Self::default_headers`] for MCP `http_tool` / `http_tools` that set `egress_profile`.
    #[serde(default)]
    pub profiles: Vec<ApiGatewayEgressProfile>,
    #[serde(default)]
    pub corporate: ApiGatewayEgressCorporateConfig,
    #[serde(default)]
    pub allowlist: ApiGatewayEgressAllowlistConfig,
    #[serde(default)]
    pub tls: ApiGatewayEgressTlsConfig,
    #[serde(default)]
    pub rate_limit: ApiGatewayEgressRateLimitConfig,
}

impl ApiGatewayEgressConfig {
    /// Redis URL when `rate_limit.redis.url_env` names a non-empty environment variable.
    pub fn effective_rate_limit_redis_url(&self) -> Option<String> {
        let env_name = self
            .rate_limit
            .redis
            .url_env
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())?;
        std::env::var(env_name)
            .ok()
            .filter(|s| !s.trim().is_empty())
    }
}

/// Named egress auth / header profile (referenced by `mcp.servers[].http_tool.egress_profile`).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayEgressProfile {
    pub name: String,
    #[serde(default)]
    pub default_headers: Vec<ApiGatewayEgressDefaultHeader>,
}

/// Static or environment-backed header on every corporate egress request.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayEgressDefaultHeader {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
    /// Read at startup from `std::env::var` — mutually exclusive with `value`.
    #[serde(default)]
    pub value_env: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiGatewayEgressRetryConfig {
    /// Extra attempts after the first try (`0` disables retries).
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
}

impl Default for ApiGatewayEgressRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 0,
            initial_backoff_ms: 50,
            max_backoff_ms: 2000,
        }
    }
}

fn default_api_gateway_egress_timeout_ms() -> u64 {
    30_000
}

fn default_api_gateway_egress_pool_idle_timeout_ms() -> u64 {
    90_000
}

/// Corporate API entry (joined with relative paths from tools).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ApiGatewayEgressCorporateConfig {
    /// Base URL (`https` recommended). Trailing slashes are ignored when joining paths.
    #[serde(default)]
    pub default_base: Option<String>,
    /// Optional round-robin pool of base URLs for **relative** egress targets (e.g. `http_tool` paths).
    /// When non-empty, each relative request uses the next base (per process). Every entry must satisfy
    /// the same allowlist rules as [`Self::default_base`]. When empty, only `default_base` is used (legacy).
    #[serde(default)]
    pub pool_bases: Vec<String>,
}

/// Host / path guardrails for egress (SSRF mitigation).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiGatewayEgressAllowlistConfig {
    /// Allowed `host` or `host:port` entries (case-insensitive host). When a port is omitted, only the
    /// scheme default port (`443` for `https`, `80` for `http`) is accepted for that host.
    #[serde(default)]
    pub allow_hosts: Vec<String>,
    /// Request paths must start with one of these prefixes (longest match). Empty after load means `/` only.
    #[serde(default)]
    pub allow_path_prefixes: Vec<String>,
}

impl Default for ApiGatewayEgressAllowlistConfig {
    fn default() -> Self {
        Self {
            allow_hosts: Vec::new(),
            allow_path_prefixes: Vec::new(),
        }
    }
}

impl Default for ApiGatewayIngressConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            routes: Vec::new(),
            rate_limit_redis: ApiGatewayIngressRateLimitRedisConfig::default(),
        }
    }
}

impl ApiGatewayIngressConfig {
    pub fn validate_rate_limit_redis(&self) -> anyhow::Result<()> {
        if let Some(env) = self
            .rate_limit_redis
            .url_env
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if self.rate_limit_redis.key_prefix.trim().is_empty() {
                anyhow::bail!(
                    "api_gateway.ingress.rate_limit_redis.key_prefix must be non-empty when url_env is set"
                );
            }
            if env.contains(|c: char| c.is_whitespace()) {
                anyhow::bail!("api_gateway.ingress.rate_limit_redis.url_env must not contain whitespace");
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        self.validate_rate_limit_redis()?;
        if !self.enabled {
            return Ok(());
        }
        let mut seen = std::collections::HashSet::new();
        for (i, r) in self.routes.iter().enumerate() {
            validate_ingress_route_row(r)
                .map_err(|e| anyhow::anyhow!("api_gateway.ingress.routes[{i}]: {e}"))?;
            let p = r.path_prefix.trim();
            let t = r.tenant_id.as_deref().map(str::trim).unwrap_or("").to_string();
            if !seen.insert((t, p.to_string())) {
                anyhow::bail!(
                    "api_gateway.ingress.routes duplicate (tenant_id, path_prefix): tenant={:?} path={:?}",
                    r.tenant_id,
                    p
                );
            }
        }
        Ok(())
    }
}

impl Default for ApiGatewayEgressConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            timeout_ms: default_api_gateway_egress_timeout_ms(),
            pool_idle_timeout_ms: default_api_gateway_egress_pool_idle_timeout_ms(),
            default_headers: Vec::new(),
            retry: ApiGatewayEgressRetryConfig::default(),
            profiles: Vec::new(),
            corporate: ApiGatewayEgressCorporateConfig::default(),
            allowlist: ApiGatewayEgressAllowlistConfig::default(),
            tls: ApiGatewayEgressTlsConfig::default(),
            rate_limit: ApiGatewayEgressRateLimitConfig::default(),
        }
    }
}

impl ApiGatewayConfig {
    fn validate_egress_default_header_entry(
        ctx: &str,
        h: &ApiGatewayEgressDefaultHeader,
    ) -> anyhow::Result<()> {
        let name = h.name.trim();
        if name.is_empty() {
            anyhow::bail!("{ctx}.name must not be empty");
        }
        if name.eq_ignore_ascii_case("host") {
            anyhow::bail!("{ctx}.name must not be Host (set via URL / default_base)");
        }
        http::HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| anyhow::anyhow!("{ctx}.name invalid header name: {:?}", h.name))?;
        let has_v = h.value.as_ref().is_some_and(|v| !v.trim().is_empty());
        let has_e = h.value_env.as_ref().is_some_and(|v| !v.trim().is_empty());
        if !(has_v ^ has_e) {
            anyhow::bail!("{ctx}: set exactly one of `value` or `value_env`");
        }
        if has_e {
            let k = h.value_env.as_ref().unwrap().trim();
            if std::env::var(k).map(|v| v.is_empty()).unwrap_or(true) {
                anyhow::bail!("{ctx}.value_env: environment variable {k:?} missing or empty");
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        self.ingress.validate()?;
        if !self.egress.enabled {
            return Ok(());
        }
        let hosts: Vec<String> = self
            .egress
            .allowlist
            .allow_hosts
            .iter()
            .map(|h| h.trim().to_string())
            .filter(|h| !h.is_empty())
            .collect();
        if hosts.is_empty() {
            anyhow::bail!(
                "api_gateway.egress.allowlist.allow_hosts must be non-empty when api_gateway.egress.enabled=true"
            );
        }
        for h in &hosts {
            if h.contains('/') {
                anyhow::bail!(
                    "api_gateway.egress.allowlist.allow_hosts entries must be hostname or hostname:port, got {h:?}"
                );
            }
        }
        let mut path_prefixes: Vec<String> = self
            .egress
            .allowlist
            .allow_path_prefixes
            .iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
        if path_prefixes.is_empty() {
            path_prefixes.push("/".to_string());
        }
        for p in &path_prefixes {
            if !p.starts_with('/') {
                anyhow::bail!(
                    "api_gateway.egress.allowlist.allow_path_prefixes entries must start with '/': {p:?}"
                );
            }
        }
        let validate_corp_base = |raw: &str, ctx: &str| -> anyhow::Result<()> {
            let b = raw.trim();
            if b.is_empty() {
                anyhow::bail!("{ctx} must be non-empty when set");
            }
            let u: http::Uri = b
                .parse()
                .map_err(|e| anyhow::anyhow!("{ctx}: invalid URL: {e}"))?;
            if u.scheme_str() != Some("https") && u.scheme_str() != Some("http") {
                anyhow::bail!("{ctx} must use http or https");
            }
            let Some(host) = u.host() else {
                anyhow::bail!("{ctx} must include a host");
            };
            let port = u.port_u16().unwrap_or_else(|| {
                if u.scheme_str() == Some("https") {
                    443
                } else {
                    80
                }
            });
            if !Self::egress_allowlist_contains_host(&hosts, host, port, u.scheme_str().unwrap_or("")) {
                anyhow::bail!(
                    "{ctx} host {host}:{port} is not covered by api_gateway.egress.allowlist.allow_hosts"
                );
            }
            Ok(())
        };
        if let Some(ref base) = self.egress.corporate.default_base {
            validate_corp_base(
                base,
                "api_gateway.egress.corporate.default_base",
            )?;
        }
        for (i, pb) in self.egress.corporate.pool_bases.iter().enumerate() {
            validate_corp_base(
                pb,
                &format!("api_gateway.egress.corporate.pool_bases[{i}]"),
            )?;
        }
        if self.egress.retry.max_retries > 10 {
            anyhow::bail!("api_gateway.egress.retry.max_retries must be <= 10");
        }
        if self.egress.retry.max_retries > 0 {
            if self.egress.retry.initial_backoff_ms == 0 {
                anyhow::bail!(
                    "api_gateway.egress.retry.initial_backoff_ms must be > 0 when max_retries > 0"
                );
            }
            if self.egress.retry.max_backoff_ms < self.egress.retry.initial_backoff_ms {
                anyhow::bail!(
                    "api_gateway.egress.retry.max_backoff_ms must be >= initial_backoff_ms"
                );
            }
        }
        const EGRESS_RL_CAP: u32 = 1_000_000;
        if self.egress.rate_limit.max_in_flight > EGRESS_RL_CAP {
            anyhow::bail!(
                "api_gateway.egress.rate_limit.max_in_flight must be <= {EGRESS_RL_CAP}"
            );
        }
        if self.egress.rate_limit.max_rps > EGRESS_RL_CAP {
            anyhow::bail!("api_gateway.egress.rate_limit.max_rps must be <= {EGRESS_RL_CAP}");
        }
        if self
            .egress
            .rate_limit
            .redis
            .key_prefix
            .trim()
            .is_empty()
        {
            anyhow::bail!("api_gateway.egress.rate_limit.redis.key_prefix must be non-empty");
        }
        let mut rl_seen = std::collections::HashSet::<String>::new();
        for (i, pr) in self.egress.rate_limit.per_route.iter().enumerate() {
            let label = pr.route_label.trim();
            if label.is_empty() {
                anyhow::bail!("api_gateway.egress.rate_limit.per_route[{i}].route_label must be non-empty");
            }
            if !rl_seen.insert(label.to_string()) {
                anyhow::bail!(
                    "api_gateway.egress.rate_limit.per_route duplicate route_label: {:?}",
                    pr.route_label
                );
            }
            if pr.max_rps == 0 {
                anyhow::bail!(
                    "api_gateway.egress.rate_limit.per_route[{i}].max_rps must be > 0"
                );
            }
            if pr.max_rps > EGRESS_RL_CAP {
                anyhow::bail!(
                    "api_gateway.egress.rate_limit.per_route[{i}].max_rps must be <= {EGRESS_RL_CAP}"
                );
            }
        }
        for (i, h) in self.egress.default_headers.iter().enumerate() {
            Self::validate_egress_default_header_entry(
                &format!("api_gateway.egress.default_headers[{i}]"),
                h,
            )?;
        }
        let mut prof_seen = std::collections::HashSet::<String>::new();
        for (pi, prof) in self.egress.profiles.iter().enumerate() {
            let pn = prof.name.trim();
            if pn.is_empty() {
                anyhow::bail!("api_gateway.egress.profiles[{pi}].name must not be empty");
            }
            if !prof_seen.insert(pn.to_string()) {
                anyhow::bail!(
                    "api_gateway.egress.profiles duplicate name: {:?}",
                    prof.name
                );
            }
            for (hi, h) in prof.default_headers.iter().enumerate() {
                Self::validate_egress_default_header_entry(
                    &format!("api_gateway.egress.profiles[{pi}].default_headers[{hi}]"),
                    h,
                )?;
            }
        }
        self.egress.tls.validate()?;
        Ok(())
    }

    fn egress_allowlist_contains_host(
        allow_hosts: &[String],
        host: &str,
        port: u16,
        scheme: &str,
    ) -> bool {
        let host_lc = host.to_ascii_lowercase();
        let default_port = if scheme == "https" { 443 } else { 80 };
        for entry in allow_hosts {
            let e = entry.trim();
            if let Some(colon) = e.rfind(':') {
                let tail = &e[colon + 1..];
                if !tail.is_empty() && tail.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(p) = tail.parse::<u16>() {
                        let hostname = e[..colon].to_ascii_lowercase();
                        if hostname == host_lc && p == port {
                            return true;
                        }
                        continue;
                    }
                }
            }
            if e.to_ascii_lowercase() == host_lc && port == default_port {
                return true;
            }
        }
        false
    }
}

impl Default for ApiGatewayConfig {
    fn default() -> Self {
        Self {
            ingress: ApiGatewayIngressConfig::default(),
            egress: ApiGatewayEgressConfig::default(),
        }
    }
}

/// Where dynamic ingress routes are persisted (when [`ControlPlaneConfig::enabled`]).
///
/// ### Managed PostgreSQL (configurable today)
///
/// [`ControlPlaneStoreKind::Postgres`] is **not** tied to a single vendor: any PostgreSQL-compatible
/// server reached with a normal `postgres://` / `postgresql://` URL works, including:
///
/// - **AWS:** RDS for PostgreSQL, Aurora PostgreSQL (TCP URL, usually `sslmode=require` or equivalent).
/// - **Azure:** Azure Database for PostgreSQL (Flexible Server / Single Server style endpoints).
/// - **GCP:** Cloud SQL for PostgreSQL (TCP or Unix socket / Cloud SQL Auth Proxy host parameter in the URL).
///
/// There is **no** separate `kind` per cloud; use [`ControlPlaneStoreConfig::database_url`] with the
/// connection string your provider documents (TLS, IAM auth tokens, etc., go in the URL or env as
/// supported by the client stack — Panda uses **sqlx** + **rustls** for Postgres TLS when built with
/// `control-plane-sql`).
///
/// ### Not implemented (different SQL engines)
///
/// - **MySQL** (e.g. RDS for MySQL, Aurora MySQL, Azure Database for MySQL): doable with a future
///   `mysql` store kind and MySQL-specific upsert SQL; not shipped yet.
/// - **Microsoft SQL Server** / **Azure SQL Database**: different wire protocol and SQL dialect; would
///   need a dedicated backend (e.g. sqlx MSSQL or another client), not the current Postgres driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneStoreKind {
    /// In-process only (lost on restart).
    #[default]
    Memory,
    /// JSON bundle on disk (`ControlPlaneStoreConfig::json_file`).
    JsonFile,
    /// SQLite file or in-memory URL (`ControlPlaneStoreConfig::database_url`, e.g. `sqlite://./panda_control.db`).
    Sqlite,
    /// PostgreSQL wire protocol — **AWS RDS/Aurora Postgres, Azure Postgres, GCP Cloud SQL Postgres**, or self-hosted.
    ///
    /// Use `postgresql://` or `postgres://` in [`ControlPlaneStoreConfig::database_url`]. Enable TLS
    /// as required by the provider (`sslmode=require`, etc.).
    Postgres,
}

/// Storage for control-plane dynamic ingress routes.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ControlPlaneStoreConfig {
    #[serde(default)]
    pub kind: ControlPlaneStoreKind,
    /// When [`Self::kind`] is [`ControlPlaneStoreKind::Postgres`], open a dedicated listener and reload
    /// dynamic ingress on `NOTIFY` (writers call `pg_notify` after each change). Use with multiple replicas
    /// sharing one database; requires `panda-proxy` built with `control-plane-sql`.
    #[serde(default)]
    pub postgres_listen: bool,
    /// Path for [`ControlPlaneStoreKind::JsonFile`] (atomic rewrite on change).
    #[serde(default)]
    pub json_file: Option<String>,
    /// DSN for [`ControlPlaneStoreKind::Sqlite`] or [`ControlPlaneStoreKind::Postgres`].
    ///
    /// Examples: `sqlite://./cp.db`; `postgresql://user:pass@db.xxx.rds.amazonaws.com:5432/panda` (RDS);
    /// Azure Postgres host from the Azure portal; Cloud SQL via TCP or proxy host query parameters.
    #[serde(default)]
    pub database_url: Option<String>,
}

impl Default for ControlPlaneStoreConfig {
    fn default() -> Self {
        Self {
            kind: ControlPlaneStoreKind::Memory,
            postgres_listen: false,
            json_file: None,
            database_url: None,
        }
    }
}

fn default_control_plane_redis_reload_channel() -> String {
    "panda:control_plane:ingress_reload".to_string()
}

/// Optional Redis pub/sub so every replica reloads dynamic ingress after a mutating control-plane call on any instance.
#[derive(Debug, Clone, Deserialize)]
pub struct ControlPlaneReloadPubSubConfig {
    /// Environment variable name holding a `redis://` or `rediss://` URL (same pattern as other Redis features).
    pub redis_url_env: String,
    #[serde(default = "default_control_plane_redis_reload_channel")]
    pub channel: String,
}

fn default_control_plane_api_key_header() -> String {
    "x-panda-control-api-key".to_string()
}

fn default_control_plane_api_key_prefix() -> String {
    "panda:cp:apikey:".to_string()
}

fn default_cp_auth_console_roles_mode() -> String {
    "any".to_string()
}

/// Optional auth extensions for control-plane HTTP routes (OIDC session, Redis-backed API keys).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ControlPlaneAuthConfig {
    /// Accept a valid developer-console OIDC session cookie (see [`ConsoleOidcConfig`]). Requires `console_oidc.enabled` at runtime.
    #[serde(default)]
    pub allow_console_oidc_session: bool,
    /// If non-empty, the session JWT must satisfy these roles (same semantics as `console_oidc.required_roles` / `required_roles_mode`).
    #[serde(default)]
    pub required_console_roles: Vec<String>,
    #[serde(default = "default_cp_auth_console_roles_mode")]
    pub required_console_roles_mode: String,
    /// Env var for Redis URL used to validate/issue/revoke control-plane API keys.
    #[serde(default)]
    pub api_keys_redis_url_env: Option<String>,
    /// Key prefix for `EXISTS` / `SET` / `DEL`; token is stored at `{prefix}{sha256_hex(token)}`.
    #[serde(default = "default_control_plane_api_key_prefix")]
    pub api_keys_redis_key_prefix: String,
    /// Request header carrying the raw API key (checked when Redis URL is configured).
    #[serde(default = "default_control_plane_api_key_header")]
    pub api_key_header: String,
    /// When non-empty, a valid console OIDC session whose roles match **any** (or **all**, see
    /// [`Self::oidc_read_only_roles_mode`]) of these grants **read-only** control-plane access,
    /// unless the session already satisfies [`Self::required_console_roles`] for read/write.
    #[serde(default)]
    pub oidc_read_only_roles: Vec<String>,
    #[serde(default = "default_cp_auth_console_roles_mode")]
    pub oidc_read_only_roles_mode: String,
}

impl Default for ControlPlaneAuthConfig {
    fn default() -> Self {
        Self {
            allow_console_oidc_session: false,
            required_console_roles: Vec::new(),
            required_console_roles_mode: default_cp_auth_console_roles_mode(),
            api_keys_redis_url_env: None,
            api_keys_redis_key_prefix: default_control_plane_api_key_prefix(),
            api_key_header: default_control_plane_api_key_header(),
            oidc_read_only_roles: Vec::new(),
            oidc_read_only_roles_mode: default_cp_auth_console_roles_mode(),
        }
    }
}

/// Optional admin API for dynamic config (Epic E). When disabled, no extra routes are exposed.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ControlPlaneConfig {
    /// When true, serve read-only control endpoints under [`Self::path_prefix`] (before ingress routing).
    pub enabled: bool,
    /// URL path prefix without trailing slash (e.g. `/ops/control`). Empty uses `/ops/control`.
    pub path_prefix: String,
    /// When set to a positive value, reload in-memory dynamic ingress from the backing store on this interval.
    /// Helps replicas (or sidecars) pick up changes written by another instance (`json_file`, `sqlite`, `postgres`).
    /// Ignored for [`ControlPlaneStoreKind::Memory`]; invalid to set when the store kind is memory and control plane is enabled.
    #[serde(default)]
    pub reload_from_store_ms: Option<u64>,
    /// When non-empty, control-plane routes accept the [`crate::ObservabilityConfig::admin_auth_header`] value if it matches
    /// **`std::env::var`** for any listed env var **or** the primary [`crate::ObservabilityConfig::admin_secret_env`] (when set).
    /// Lets you issue multiple service-style secrets without sharing the global ops secret.
    #[serde(default)]
    pub additional_admin_secret_envs: Vec<String>,
    /// HTTP header name (case-insensitive per HTTP) used to resolve the active tenant for **ingress** classification
    /// against dynamic rows that set [`ApiGatewayIngressRoute::tenant_id`]. When unset, only global dynamic rows apply for tenant scoping.
    #[serde(default)]
    pub tenant_resolution_header: Option<String>,
    /// Environment variable names whose values match [`crate::ObservabilityConfig::admin_auth_header`] for **read-only**
    /// control-plane access (GET status, GET ingress routes, GET export). Mutations require the primary ops secret,
    /// [`Self::additional_admin_secret_envs`], OIDC session, or Redis API keys.
    #[serde(default)]
    pub read_only_secret_envs: Vec<String>,
    /// OIDC session + Redis API keys for control-plane routes.
    #[serde(default)]
    pub auth: ControlPlaneAuthConfig,
    /// Subscribe to Redis channel and reload dynamic ingress on each message; writers **`PUBLISH`** after successful mutations.
    #[serde(default)]
    pub reload_pubsub: Option<ControlPlaneReloadPubSubConfig>,
    #[serde(default)]
    pub store: ControlPlaneStoreConfig,
}

impl Default for ControlPlaneConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            path_prefix: "/ops/control".to_string(),
            reload_from_store_ms: None,
            additional_admin_secret_envs: Vec::new(),
            tenant_resolution_header: None,
            read_only_secret_envs: Vec::new(),
            auth: ControlPlaneAuthConfig::default(),
            reload_pubsub: None,
            store: ControlPlaneStoreConfig::default(),
        }
    }
}

/// Root gateway configuration (e.g. `panda.yaml`).
///
/// Organized by **inbound (MCP gateway + API gateway)** vs **outbound (AI gateway)** vs **shared**—see crate-level docs and
/// `docs/architecture_two_pillars.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct PandaConfig {
    /// Bind address. Leave empty when using [`PandaConfig::server`] `listen` or `port`.
    #[serde(default)]
    pub listen: String,
    #[serde(default)]
    pub server: Option<ServerSection>,
    /// Default HTTP backend base when no [`RouteConfig`] matches (scheme + authority + optional path prefix).
    pub default_backend: String,
    /// Per-path backend bases; see [`RouteConfig`]. Empty preserves single-backend behavior.
    #[serde(default)]
    pub routes: Vec<RouteConfig>,
    #[serde(default)]
    pub trusted_gateway: TrustedGatewayConfig,
    #[serde(default)]
    pub observability: ObservabilityConfig,
    #[serde(default)]
    pub tpm: TpmConfig,
    #[serde(default)]
    pub tls: Option<TlsListenConfig>,
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub identity: IdentityConfig,
    /// Merged into `identity` (JWKS URL, `enforce_on_all_routes` → `require_jwt`).
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub prompt_safety: PromptSafetyConfig,
    #[serde(default)]
    pub pii: PiiConfig,
    #[serde(default)]
    pub mcp: McpConfig,
    #[serde(default)]
    pub api_gateway: ApiGatewayConfig,
    #[serde(default)]
    pub control_plane: ControlPlaneConfig,
    #[serde(default)]
    pub semantic_cache: SemanticCacheConfig,
    #[serde(default)]
    pub adapter: AdapterConfig,
    #[serde(default)]
    pub rate_limit_fallback: RateLimitFallbackConfig,
    #[serde(default)]
    pub context_management: ContextManagementConfig,
    #[serde(default)]
    pub console_oidc: ConsoleOidcConfig,
    #[serde(default)]
    pub budget_hierarchy: BudgetHierarchyConfig,
    #[serde(default)]
    pub model_failover: ModelFailoverConfig,
    #[serde(default)]
    pub routing: RoutingConfig,
    #[serde(default)]
    pub agent_sessions: AgentSessionsConfig,
}

impl PandaConfig {
    pub fn load_from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        Self::from_yaml_str(&raw)
    }

    pub fn from_yaml_str(raw: &str) -> anyhow::Result<Self> {
        let mut cfg: Self = serde_yaml::from_str(raw)?;
        cfg.resolve_server_section()?;
        cfg.merge_auth_section();
        cfg.normalize_route_methods()?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Parse and canonicalize [`RouteConfig::methods`] (dedupe, validate tokens as HTTP methods).
    fn normalize_route_methods(&mut self) -> anyhow::Result<()> {
        use http::Method;
        for r in &mut self.routes {
            let mut out = Vec::new();
            for s in std::mem::take(&mut r.methods) {
                let t = s.trim();
                if t.is_empty() {
                    anyhow::bail!("routes.methods entry must not be empty or whitespace");
                }
                let m = Method::from_bytes(t.as_bytes())
                    .map_err(|_| anyhow::anyhow!("routes.methods invalid HTTP method: {:?}", s))?;
                // Canonical form for matching (HTTP methods are uppercase in practice; `http` may keep
                // lowercase tokens as extension methods).
                let canon = m.as_str().to_ascii_uppercase();
                if !out.contains(&canon) {
                    out.push(canon);
                }
            }
            r.methods = out;
        }
        Ok(())
    }

    /// Merge [`AuthConfig`] into [`IdentityConfig`] (JWKS URL, `enforce_on_all_routes` → `require_jwt`).
    fn merge_auth_section(&mut self) {
        if let Some(ref u) = self.auth.jwks_url {
            let t = u.trim();
            if !t.is_empty() {
                self.identity.jwks_url = Some(t.to_string());
            }
        }
        if self.auth.enforce_on_all_routes {
            self.identity.require_jwt = true;
        }
    }

    /// Merge [`PandaConfig::server`] into `listen` / `tls` when present.
    ///
    /// - Non-empty `server.listen` overrides the top-level `listen` string.
    /// - `server.port` (+ optional `server.address`) is used **only** when `listen` is still empty
    ///   after that, so `listen: "127.0.0.1:9000"` with `server: { port: 8080, tls: ... }` keeps
    ///   `9000` and only applies nested TLS.
    fn resolve_server_section(&mut self) -> anyhow::Result<()> {
        let Some(ref s) = self.server else {
            return Ok(());
        };
        if let Some(ref l) = s.listen {
            if !l.trim().is_empty() {
                self.listen = l.clone();
            }
        }
        if self.listen.trim().is_empty() {
            if let Some(port) = s.port {
                let addr = s.address.as_deref().unwrap_or("127.0.0.1");
                self.listen = format_listen_address(addr, port);
            }
        }
        if let Some(ref tls) = s.tls {
            self.tls = Some(tls.clone());
        }
        Ok(())
    }

    pub fn listen_addr(&self) -> anyhow::Result<std::net::SocketAddr> {
        if let Ok(override_listen) = std::env::var("PANDA_LISTEN_OVERRIDE") {
            let t = override_listen.trim();
            if !t.is_empty() {
                return t.parse().map_err(|e| {
                    anyhow::anyhow!("invalid PANDA_LISTEN_OVERRIDE (expected host:port): {e}")
                });
            }
        }
        Ok(self.listen.parse()?)
    }

    fn validate_semantic_routing_config(semantic: &SemanticRoutingConfig) -> anyhow::Result<()> {
        let mode = semantic.mode.to_ascii_lowercase();
        if mode == "off" {
            anyhow::bail!(
                "routing.semantic.mode must not be \"off\" when semantic routing is enabled"
            );
        }
        match mode.as_str() {
            "embed" => {
                if semantic.embed_backend_base.trim().is_empty() {
                    anyhow::bail!("routing.semantic.embed_backend_base is required when routing.semantic.mode=embed");
                }
                let u: http::Uri = semantic
                    .embed_backend_base
                    .trim()
                    .parse()
                    .map_err(|e| anyhow::anyhow!("routing.semantic.embed_backend_base invalid URI: {e}"))?;
                if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                    anyhow::bail!("routing.semantic.embed_backend_base must use http or https");
                }
                if semantic.embed_api_key_env.trim().is_empty() {
                    anyhow::bail!(
                        "routing.semantic.embed_api_key_env must be non-empty when routing.semantic.mode=embed"
                    );
                }
                if semantic.embed_model.trim().is_empty() {
                    anyhow::bail!("routing.semantic.embed_model must be non-empty when routing.semantic.mode=embed");
                }
                if !(0.0..=1.0).contains(&semantic.similarity_threshold) {
                    anyhow::bail!("routing.semantic.similarity_threshold must be within [0.0, 1.0]");
                }
                if semantic.targets.is_empty() {
                    anyhow::bail!("routing.semantic.targets must not be empty when routing.semantic.mode=embed");
                }
                let mut seen = std::collections::HashSet::<String>::new();
                for t in &semantic.targets {
                    if t.name.trim().is_empty() {
                        anyhow::bail!("routing.semantic.targets.name must be non-empty");
                    }
                    if !seen.insert(t.name.clone()) {
                        anyhow::bail!("routing.semantic.targets names must be unique: {:?}", t.name);
                    }
                    if t.routing_text.trim().is_empty() {
                        anyhow::bail!("routing.semantic.targets.routing_text must be non-empty");
                    }
                    if t.backend_base.trim().is_empty() {
                        anyhow::bail!("routing.semantic.targets.backend_base must be non-empty");
                    }
                    let tu: http::Uri = t
                        .backend_base
                        .trim()
                        .parse()
                        .map_err(|e| anyhow::anyhow!("routing.semantic.targets.backend_base invalid URI: {e}"))?;
                    if tu.scheme_str() != Some("http") && tu.scheme_str() != Some("https") {
                        anyhow::bail!("routing.semantic.targets.backend_base must use http or https");
                    }
                }
            }
            "classifier" | "llm_judge" => {
                if semantic.router_backend_base.trim().is_empty() {
                    anyhow::bail!(
                        "routing.semantic.router_backend_base is required when routing.semantic.mode is classifier or llm_judge"
                    );
                }
                let u: http::Uri = semantic
                    .router_backend_base
                    .trim()
                    .parse()
                    .map_err(|e| anyhow::anyhow!("routing.semantic.router_backend_base invalid URI: {e}"))?;
                if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                    anyhow::bail!("routing.semantic.router_backend_base must use http or https");
                }
                if semantic.router_api_key_env.trim().is_empty() {
                    anyhow::bail!(
                        "routing.semantic.router_api_key_env must be non-empty for classifier / llm_judge modes"
                    );
                }
                if semantic.router_model.trim().is_empty() {
                    anyhow::bail!(
                        "routing.semantic.router_model must be non-empty for classifier / llm_judge modes"
                    );
                }
                if !(0.0..=1.0).contains(&semantic.similarity_threshold) {
                    anyhow::bail!("routing.semantic.similarity_threshold must be within [0.0, 1.0]");
                }
                if semantic.targets.is_empty() {
                    anyhow::bail!(
                        "routing.semantic.targets must not be empty when routing.semantic.mode is classifier or llm_judge"
                    );
                }
                let mut seen = std::collections::HashSet::<String>::new();
                for t in &semantic.targets {
                    if t.name.trim().is_empty() {
                        anyhow::bail!("routing.semantic.targets.name must be non-empty");
                    }
                    if !seen.insert(t.name.clone()) {
                        anyhow::bail!("routing.semantic.targets names must be unique: {:?}", t.name);
                    }
                    if t.backend_base.trim().is_empty() {
                        anyhow::bail!("routing.semantic.targets.backend_base must be non-empty");
                    }
                    let tu: http::Uri = t
                        .backend_base
                        .trim()
                        .parse()
                        .map_err(|e| anyhow::anyhow!("routing.semantic.targets.backend_base invalid URI: {e}"))?;
                    if tu.scheme_str() != Some("http") && tu.scheme_str() != Some("https") {
                        anyhow::bail!("routing.semantic.targets.backend_base must use http or https");
                    }
                }
            }
            _ => anyhow::bail!(
                "routing.semantic.mode must be one of: off, embed, classifier, llm_judge (got {:?})",
                semantic.mode
            ),
        }
        if semantic.timeout_ms == 0 {
            anyhow::bail!(
                "routing.semantic.timeout_ms must be > 0 when semantic routing is enabled"
            );
        }
        if semantic.cache_ttl_seconds == 0 {
            anyhow::bail!(
                "routing.semantic.cache_ttl_seconds must be > 0 when semantic routing is enabled"
            );
        }
        if semantic.max_prompt_chars == 0 {
            anyhow::bail!(
                "routing.semantic.max_prompt_chars must be > 0 when semantic routing is enabled"
            );
        }
        Ok(())
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.listen.trim().is_empty() {
            anyhow::bail!(
                "`listen` must be set (top-level `listen`, or `server.listen` / `server.port`)"
            );
        }
        if self.default_backend.trim().is_empty() {
            anyhow::bail!("`default_backend` must not be empty");
        }
        let _: http::Uri = self
            .default_backend
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid `default_backend` URL: {e}"))?;
        self.trusted_gateway.validate()?;
        if self.observability.correlation_header.trim().is_empty() {
            anyhow::bail!("`observability.correlation_header` must not be empty");
        }
        HeaderName::from_bytes(self.observability.correlation_header.as_bytes())
            .map_err(|_| anyhow::anyhow!("invalid observability.correlation_header token"))?;
        if self.observability.admin_auth_header.trim().is_empty() {
            anyhow::bail!("`observability.admin_auth_header` must not be empty");
        }
        HeaderName::from_bytes(self.observability.admin_auth_header.as_bytes())
            .map_err(|_| anyhow::anyhow!("invalid observability.admin_auth_header token"))?;
        if self
            .observability
            .admin_secret_env
            .as_ref()
            .is_some_and(|v| v.trim().is_empty())
        {
            anyhow::bail!("observability.admin_secret_env must be non-empty when set");
        }
        if self.control_plane.enabled {
            let p = self.control_plane.path_prefix.trim();
            if !p.is_empty() && !p.starts_with('/') {
                anyhow::bail!("control_plane.path_prefix must start with `/` when set");
            }
            use ControlPlaneStoreKind as Csk;
            match self.control_plane.store.kind {
                Csk::JsonFile => {
                    if self
                        .control_plane
                        .store
                        .json_file
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .is_none()
                    {
                        anyhow::bail!(
                            "control_plane.store.json_file is required when control_plane.store.kind is json_file"
                        );
                    }
                }
                Csk::Sqlite | Csk::Postgres => {
                    if self
                        .control_plane
                        .store
                        .database_url
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .is_none()
                    {
                        anyhow::bail!(
                            "control_plane.store.database_url is required when control_plane.store.kind is sqlite or postgres"
                        );
                    }
                }
                Csk::Memory => {}
            }
            if self
                .control_plane
                .reload_from_store_ms
                .is_some_and(|n| n > 0)
                && matches!(self.control_plane.store.kind, Csk::Memory)
            {
                anyhow::bail!(
                    "control_plane.reload_from_store_ms is set but control_plane.store.kind is memory; use json_file, sqlite, or postgres"
                );
            }
            if self.control_plane.store.postgres_listen
                && !matches!(self.control_plane.store.kind, Csk::Postgres)
            {
                anyhow::bail!(
                    "control_plane.store.postgres_listen is true but store.kind is not postgres"
                );
            }
            if let Some(ref ps) = self.control_plane.reload_pubsub {
                if ps.redis_url_env.trim().is_empty() {
                    anyhow::bail!("control_plane.reload_pubsub.redis_url_env must be non-empty when reload_pubsub is set");
                }
                if ps.channel.trim().is_empty() {
                    anyhow::bail!("control_plane.reload_pubsub.channel must be non-empty when reload_pubsub is set");
                }
            }
            if let Some(h) = self
                .control_plane
                .tenant_resolution_header
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                http::HeaderName::from_bytes(h.as_bytes()).map_err(|_| {
                    anyhow::anyhow!("control_plane.tenant_resolution_header is not a valid HTTP header name")
                })?;
            }
            let a = &self.control_plane.auth;
            if a.api_key_header.trim().is_empty() {
                anyhow::bail!("control_plane.auth.api_key_header must not be empty");
            }
            http::HeaderName::from_bytes(a.api_key_header.trim().as_bytes()).map_err(|_| {
                anyhow::anyhow!("control_plane.auth.api_key_header is not a valid HTTP header name")
            })?;
            if let Some(env) = a.api_keys_redis_url_env.as_deref() {
                if env.trim().is_empty() {
                    anyhow::bail!(
                        "control_plane.auth.api_keys_redis_url_env must be non-empty when set"
                    );
                }
            }
            if a.api_keys_redis_key_prefix.trim().is_empty() {
                anyhow::bail!("control_plane.auth.api_keys_redis_key_prefix must not be empty");
            }
        }
        if self.agent_sessions.enabled {
            if self.agent_sessions.header.trim().is_empty() {
                anyhow::bail!(
                    "agent_sessions.header must not be empty when agent_sessions.enabled=true"
                );
            }
            HeaderName::from_bytes(self.agent_sessions.header.as_bytes())
                .map_err(|_| anyhow::anyhow!("invalid agent_sessions.header token"))?;
            if self.agent_sessions.profile_header.trim().is_empty() {
                anyhow::bail!("agent_sessions.profile_header must not be empty when agent_sessions.enabled=true");
            }
            HeaderName::from_bytes(self.agent_sessions.profile_header.as_bytes())
                .map_err(|_| anyhow::anyhow!("invalid agent_sessions.profile_header token"))?;
            if let Some(ref c) = self.agent_sessions.jwt_session_claim {
                if c.trim().is_empty() {
                    anyhow::bail!("agent_sessions.jwt_session_claim must not be empty when set");
                }
            }
            if let Some(ref c) = self.agent_sessions.jwt_profile_claim {
                if c.trim().is_empty() {
                    anyhow::bail!("agent_sessions.jwt_profile_claim must not be empty when set");
                }
            }
            if let Some(n) = self.agent_sessions.mcp_max_tool_rounds_with_session {
                if n == 0 {
                    anyhow::bail!(
                        "agent_sessions.mcp_max_tool_rounds_with_session must be > 0 when set"
                    );
                }
            }
        }
        for (i, rule) in self
            .agent_sessions
            .profile_backend_rules
            .iter()
            .enumerate()
        {
            if rule.profile.trim().is_empty() {
                anyhow::bail!(
                    "agent_sessions.profile_backend_rules[{i}].profile must not be empty"
                );
            }
            let u = rule.backend_base.trim();
            if u.is_empty() {
                anyhow::bail!(
                    "agent_sessions.profile_backend_rules[{i}].backend_base must not be empty"
                );
            }
            let _: http::Uri = u.parse().map_err(|e| {
                anyhow::anyhow!(
                    "invalid agent_sessions.profile_backend_rules[{i}].backend_base URL: {e}"
                )
            })?;
            let pref = rule.path_prefix.trim();
            if pref.is_empty() || !pref.starts_with('/') {
                anyhow::bail!(
                    "agent_sessions.profile_backend_rules[{i}].path_prefix must be non-empty and start with '/'"
                );
            }
            if let Some(m) = rule.mcp_max_tool_rounds {
                if m == 0 {
                    anyhow::bail!(
                        "agent_sessions.profile_backend_rules[{i}].mcp_max_tool_rounds must be > 0 when set"
                    );
                }
            }
        }
        if let Some(ref t) = self.tls {
            if !Path::new(&t.cert_pem).is_file() {
                anyhow::bail!("tls.cert_pem not a file: {}", t.cert_pem);
            }
            if !Path::new(&t.key_pem).is_file() {
                anyhow::bail!("tls.key_pem not a file: {}", t.key_pem);
            }
            if let Some(ref ca) = t.client_ca_pem {
                if !Path::new(ca).is_file() {
                    anyhow::bail!("tls.client_ca_pem not a file: {ca}");
                }
            }
        }
        if let Some(ref d) = self.plugins.directory {
            if d.trim().is_empty() {
                anyhow::bail!("`plugins.directory` must not be empty when set");
            }
            if !Path::new(d).is_dir() {
                anyhow::bail!("plugins.directory is not a directory: {d}");
            }
        }
        if self.plugins.max_request_body_bytes == 0 {
            anyhow::bail!("plugins.max_request_body_bytes must be > 0");
        }
        if self.plugins.execution_timeout_ms == 0 {
            anyhow::bail!("plugins.execution_timeout_ms must be > 0");
        }
        if self.plugins.reload_interval_ms == 0 {
            anyhow::bail!("plugins.reload_interval_ms must be > 0");
        }
        if self.plugins.reload_debounce_ms == 0 {
            anyhow::bail!("plugins.reload_debounce_ms must be > 0");
        }
        if self.plugins.max_reloads_per_minute == 0 {
            anyhow::bail!("plugins.max_reloads_per_minute must be > 0");
        }
        if self.tpm.enforce_budget && self.tpm.budget_tokens_per_minute == 0 {
            anyhow::bail!("tpm.budget_tokens_per_minute must be > 0 when enforce_budget=true");
        }
        if self.tpm.retry_after_seconds.is_some_and(|n| n == 0) {
            anyhow::bail!("tpm.retry_after_seconds must be > 0 when set");
        }
        if !(self.tpm.redis_degraded_limit_ratio > 0.0
            && self.tpm.redis_degraded_limit_ratio <= 1.0)
        {
            anyhow::bail!("tpm.redis_degraded_limit_ratio must be in (0, 1]");
        }
        let ce = &self.observability.compliance_export;
        if ce.enabled {
            let m = ce.mode.to_ascii_lowercase();
            if m != "local_jsonl" {
                anyhow::bail!(
                    "observability.compliance_export.mode must be \"local_jsonl\" when enabled (object-store modes are documented only)"
                );
            }
            if ce.local_path.trim().is_empty() {
                anyhow::bail!(
                    "observability.compliance_export.local_path is required when enabled"
                );
            }
        }
        if ce
            .signing_secret_env
            .as_ref()
            .is_some_and(|s| s.trim().is_empty())
        {
            anyhow::bail!(
                "observability.compliance_export.signing_secret_env must be non-empty when set"
            );
        }
        if self.identity.require_jwt {
            let has_hs256 = !self.identity.jwt_hs256_secret_env.trim().is_empty();
            let has_jwks = self
                .identity
                .jwks_url
                .as_ref()
                .is_some_and(|u| !u.trim().is_empty());
            if !has_hs256 && !has_jwks {
                anyhow::bail!(
                    "when identity.require_jwt=true, set identity.jwt_hs256_secret_env and/or identity.jwks_url (or auth.jwks_url)"
                );
            }
        }
        if let Some(ref u) = self.identity.jwks_url {
            if !u.trim().is_empty() {
                let _: http::Uri = u
                    .parse()
                    .map_err(|e| anyhow::anyhow!("identity.jwks_url invalid URI: {e}"))?;
                if self.identity.jwks_cache_ttl_seconds == 0 {
                    anyhow::bail!(
                        "identity.jwks_cache_ttl_seconds must be > 0 when jwks_url is set"
                    );
                }
            }
        }
        if self
            .identity
            .accepted_issuers
            .iter()
            .any(|v| v.trim().is_empty())
        {
            anyhow::bail!("identity.accepted_issuers entries must be non-empty");
        }
        if self
            .identity
            .accepted_audiences
            .iter()
            .any(|v| v.trim().is_empty())
        {
            anyhow::bail!("identity.accepted_audiences entries must be non-empty");
        }
        if self
            .identity
            .required_scopes
            .iter()
            .any(|v| v.trim().is_empty())
        {
            anyhow::bail!("identity.required_scopes entries must be non-empty");
        }
        for r in &self.identity.route_scope_rules {
            if r.path_prefix.trim().is_empty() {
                anyhow::bail!("identity.route_scope_rules.path_prefix must be non-empty");
            }
            if r.required_scopes.is_empty() {
                anyhow::bail!("identity.route_scope_rules.required_scopes must not be empty");
            }
            if r.required_scopes.iter().any(|v| v.trim().is_empty()) {
                anyhow::bail!(
                    "identity.route_scope_rules.required_scopes entries must be non-empty"
                );
            }
        }
        if self.identity.enable_token_exchange {
            if !self.identity.require_jwt {
                anyhow::bail!("identity.enable_token_exchange requires identity.require_jwt=true");
            }
            if self.identity.agent_token_secret_env.trim().is_empty() {
                anyhow::bail!("identity.agent_token_secret_env must be non-empty");
            }
            if self.identity.agent_token_ttl_seconds == 0 {
                anyhow::bail!("identity.agent_token_ttl_seconds must be > 0");
            }
            if self.identity.agent_token_scopes.is_empty() {
                anyhow::bail!(
                    "identity.agent_token_scopes must not be empty when token exchange is enabled"
                );
            }
            if self
                .identity
                .agent_token_scopes
                .iter()
                .any(|v| v.trim().is_empty())
            {
                anyhow::bail!("identity.agent_token_scopes entries must be non-empty");
            }
        }
        if self
            .prompt_safety
            .deny_patterns
            .iter()
            .any(|v| v.trim().is_empty())
        {
            anyhow::bail!("prompt_safety.deny_patterns entries must be non-empty");
        }
        {
            let sh = &self.mcp.streamable_http;
            if sh.sse_ring_max_events == 0 || sh.sse_ring_max_events > 4096 {
                anyhow::bail!("mcp.streamable_http.sse_ring_max_events must be 1..=4096");
            }
            if sh.session_ttl_seconds < 60 || sh.session_ttl_seconds > 604_800 {
                anyhow::bail!(
                    "mcp.streamable_http.session_ttl_seconds must be 60..=604800 (1 minute .. 7 days)"
                );
            }
            if sh.sse_keepalive_interval_seconds < 5 || sh.sse_keepalive_interval_seconds > 300 {
                anyhow::bail!(
                    "mcp.streamable_http.sse_keepalive_interval_seconds must be 5..=300"
                );
            }
        }
        if self.pii.enabled {
            if self.pii.replacement.trim().is_empty() {
                anyhow::bail!("pii.replacement must be non-empty when pii.enabled=true");
            }
            if self.pii.redact_patterns.is_empty() {
                anyhow::bail!("pii.redact_patterns must not be empty when pii.enabled=true");
            }
            for p in &self.pii.redact_patterns {
                if p.trim().is_empty() {
                    anyhow::bail!("pii.redact_patterns entries must be non-empty");
                }
                regex::Regex::new(p)
                    .map_err(|e| anyhow::anyhow!("invalid pii.redact_patterns regex {p:?}: {e}"))?;
            }
        }
        if self.mcp.enabled {
            if self.mcp.tool_timeout_ms == 0 {
                anyhow::bail!("mcp.tool_timeout_ms must be > 0 when mcp.enabled=true");
            }
            if self.mcp.max_tool_payload_bytes == 0 {
                anyhow::bail!("mcp.max_tool_payload_bytes must be > 0 when mcp.enabled=true");
            }
            if self.mcp.max_tool_rounds == 0 {
                anyhow::bail!("mcp.max_tool_rounds must be > 0 when mcp.enabled=true");
            }
            if self.mcp.stream_probe_bytes == 0 {
                anyhow::bail!("mcp.stream_probe_bytes must be > 0 when mcp.enabled=true");
            }
            if self.mcp.probe_window_seconds == 0 {
                anyhow::bail!("mcp.probe_window_seconds must be > 0 when mcp.enabled=true");
            }
            if self.mcp.servers.is_empty() {
                anyhow::bail!("mcp.servers must not be empty when mcp.enabled=true");
            }
            let mut seen = std::collections::HashSet::<String>::new();
            let mut any_http_tool = false;
            let mut any_remote_mcp = false;
            let egress_prof: std::collections::HashSet<String> = self
                .api_gateway
                .egress
                .profiles
                .iter()
                .map(|p| p.name.trim().to_string())
                .filter(|n| !n.is_empty())
                .collect();
            for s in &self.mcp.servers {
                if s.name.trim().is_empty() {
                    anyhow::bail!("mcp.servers entries must have non-empty name");
                }
                let cmd_nonempty = s
                    .command
                    .as_ref()
                    .is_some_and(|c| !c.trim().is_empty());
                if s.http_tool.is_some() && !s.http_tools.is_empty() {
                    anyhow::bail!(
                        "mcp.servers[{}]: set at most one of `http_tool` or `http_tools`",
                        s.name
                    );
                }
                if cmd_nonempty {
                    if s.http_tool.is_some() {
                        anyhow::bail!(
                            "mcp.servers[{}]: `command` and `http_tool` are mutually exclusive",
                            s.name
                        );
                    }
                    if !s.http_tools.is_empty() {
                        anyhow::bail!(
                            "mcp.servers[{}]: `command` and `http_tools` are mutually exclusive",
                            s.name
                        );
                    }
                }
                if let Some(ref c) = s.command {
                    if c.trim().is_empty() {
                        anyhow::bail!("mcp.servers command must be non-empty when set");
                    }
                }
                let remote_u = s
                    .remote_mcp_url
                    .as_deref()
                    .map(str::trim)
                    .filter(|u| !u.is_empty());
                if let Some(u) = remote_u {
                    if cmd_nonempty {
                        anyhow::bail!(
                            "mcp.servers[{}]: `remote_mcp_url` and `command` are mutually exclusive",
                            s.name
                        );
                    }
                    if s.http_tool.is_some() || !s.http_tools.is_empty() {
                        anyhow::bail!(
                            "mcp.servers[{}]: `remote_mcp_url` and http_tool/http_tools are mutually exclusive",
                            s.name
                        );
                    }
                    let uri: http::Uri = u.parse().map_err(|e| {
                        anyhow::anyhow!(
                            "mcp.servers[{}].remote_mcp_url: invalid URL: {e}",
                            s.name
                        )
                    })?;
                    if uri.scheme_str() != Some("http") && uri.scheme_str() != Some("https") {
                        anyhow::bail!(
                            "mcp.servers[{}].remote_mcp_url must use http or https",
                            s.name
                        );
                    }
                    if uri.host().is_none() {
                        anyhow::bail!(
                            "mcp.servers[{}].remote_mcp_url must include a host",
                            s.name
                        );
                    }
                    any_remote_mcp = true;
                    if let Some(ref ep) = s.remote_mcp_egress_profile {
                        let t = ep.trim();
                        if !t.is_empty() {
                            if !self.api_gateway.egress.enabled {
                                anyhow::bail!(
                                    "mcp.servers[{}]: remote_mcp_egress_profile requires api_gateway.egress.enabled=true",
                                    s.name
                                );
                            }
                            if !egress_prof.contains(t) {
                                anyhow::bail!(
                                    "mcp.servers[{}]: unknown api_gateway.egress.profiles name {:?}",
                                    s.name,
                                    t
                                );
                            }
                        }
                    }
                }
                let validate_http_tool_shape =
                    |ht: &McpHttpToolConfig, label: &str| -> anyhow::Result<()> {
                        let p = ht.path.trim();
                        if p.is_empty() || !p.starts_with('/') {
                            anyhow::bail!(
                                "mcp.servers[{}].{}.path must be non-empty and start with '/'",
                                s.name,
                                label
                            );
                        }
                        let m = ht.method.trim();
                        if m.is_empty() {
                            anyhow::bail!(
                                "mcp.servers[{}].{}.method must not be empty",
                                s.name,
                                label
                            );
                        }
                        http::Method::from_bytes(m.as_bytes()).map_err(|_| {
                            anyhow::anyhow!(
                                "mcp.servers[{}].{}.method invalid HTTP method: {:?}",
                                s.name,
                                label,
                                ht.method
                            )
                        })?;
                        if ht.tool_name.trim().is_empty() {
                            anyhow::bail!(
                                "mcp.servers[{}].{}.tool_name must not be empty",
                                s.name,
                                label
                            );
                        }
                        Ok(())
                    };
                if let Some(ref ht) = s.http_tool {
                    any_http_tool = true;
                    validate_http_tool_shape(ht, "http_tool")?;
                    if let Some(ref ep) = ht.egress_profile {
                        let t = ep.trim();
                        if !t.is_empty() {
                            if !self.api_gateway.egress.enabled {
                                anyhow::bail!(
                                    "mcp.servers[{}]: egress_profile requires api_gateway.egress.enabled=true",
                                    s.name
                                );
                            }
                            if !egress_prof.contains(t) {
                                anyhow::bail!(
                                    "mcp.servers[{}]: unknown api_gateway.egress.profiles name {:?}",
                                    s.name,
                                    t
                                );
                            }
                        }
                    }
                }
                if !s.http_tools.is_empty() {
                    any_http_tool = true;
                    let mut seen_tools = std::collections::HashSet::<String>::new();
                    for (i, ht) in s.http_tools.iter().enumerate() {
                        validate_http_tool_shape(ht, &format!("http_tools[{i}]"))?;
                        if let Some(ref ep) = ht.egress_profile {
                            let t = ep.trim();
                            if !t.is_empty() {
                                if !self.api_gateway.egress.enabled {
                                    anyhow::bail!(
                                        "mcp.servers[{}]: egress_profile requires api_gateway.egress.enabled=true",
                                        s.name
                                    );
                                }
                                if !egress_prof.contains(t) {
                                    anyhow::bail!(
                                        "mcp.servers[{}]: unknown api_gateway.egress.profiles name {:?}",
                                        s.name,
                                        t
                                    );
                                }
                            }
                        }
                        let tn = ht.tool_name.trim().to_string();
                        if !seen_tools.insert(tn.clone()) {
                            anyhow::bail!(
                                "mcp.servers[{}].http_tools duplicate tool_name: {:?}",
                                s.name,
                                tn
                            );
                        }
                    }
                }
                if !seen.insert(s.name.clone()) {
                    anyhow::bail!("mcp.servers names must be unique: duplicate {:?}", s.name);
                }
            }
            if any_remote_mcp && !self.api_gateway.egress.enabled {
                anyhow::bail!(
                    "mcp.servers remote_mcp_url requires api_gateway.egress.enabled=true"
                );
            }
            if any_http_tool {
                if !self.api_gateway.egress.enabled {
                    anyhow::bail!(
                        "mcp http_tool requires api_gateway.egress.enabled=true"
                    );
                }
                let corp = &self.api_gateway.egress.corporate;
                let has_pool = corp
                    .pool_bases
                    .iter()
                    .any(|s| !s.trim().is_empty());
                let has_default = corp
                    .default_base
                    .as_deref()
                    .map(str::trim)
                    .is_some_and(|s| !s.is_empty());
                if !has_pool && !has_default {
                    anyhow::bail!(
                        "mcp http_tool requires api_gateway.egress.corporate.default_base and/or non-empty pool_bases"
                    );
                }
            }
            if !self.mcp.servers.iter().any(|s| s.enabled) {
                anyhow::bail!(
                    "mcp.enabled requires at least one mcp.servers entry with enabled=true"
                );
            }
            match self.mcp.proof_of_intent_mode.as_str() {
                "off" | "audit" | "enforce" => {}
                _ => anyhow::bail!("mcp.proof_of_intent_mode must be one of: off, audit, enforce"),
            }
            for p in &self.mcp.intent_tool_policies {
                if p.intent.trim().is_empty() {
                    anyhow::bail!("mcp.intent_tool_policies.intent must be non-empty");
                }
                if p.allowed_tools.is_empty() {
                    anyhow::bail!("mcp.intent_tool_policies.allowed_tools must not be empty");
                }
                if p.allowed_tools.iter().any(|t| t.trim().is_empty()) {
                    anyhow::bail!(
                        "mcp.intent_tool_policies.allowed_tools entries must be non-empty"
                    );
                }
            }
            if self.mcp.tool_routes.enabled {
                match self
                    .mcp
                    .tool_routes
                    .unmatched
                    .trim()
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "allow" | "deny" => {}
                    _ => anyhow::bail!("mcp.tool_routes.unmatched must be one of: allow, deny"),
                }
                for r in &self.mcp.tool_routes.rules {
                    if r.pattern.trim().is_empty() {
                        anyhow::bail!("mcp.tool_routes.rules.pattern must be non-empty");
                    }
                    match r.action.trim().to_ascii_lowercase().as_str() {
                        "allow" | "deny" => {}
                        _ => anyhow::bail!(
                            "mcp.tool_routes.rules.action must be one of: allow, deny"
                        ),
                    }
                    if r.servers.iter().any(|s| s.trim().is_empty()) {
                        anyhow::bail!("mcp.tool_routes.rules.servers entries must be non-empty");
                    }
                }
            }
            if self.mcp.tool_cache.enabled {
                match self
                    .mcp
                    .tool_cache
                    .backend
                    .trim()
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "memory" => {}
                    _ => anyhow::bail!("mcp.tool_cache.backend must be 'memory' for MVP"),
                }
                if self.mcp.tool_cache.default_ttl_seconds == 0 {
                    anyhow::bail!("mcp.tool_cache.default_ttl_seconds must be > 0 when mcp.tool_cache.enabled=true");
                }
                if self.mcp.tool_cache.max_value_bytes == 0 {
                    anyhow::bail!("mcp.tool_cache.max_value_bytes must be > 0 when mcp.tool_cache.enabled=true");
                }
                if self.mcp.tool_cache.allow.is_empty() {
                    anyhow::bail!(
                        "mcp.tool_cache.allow must not be empty when mcp.tool_cache.enabled=true"
                    );
                }
                for r in &self.mcp.tool_cache.allow {
                    if r.server.trim().is_empty() {
                        anyhow::bail!("mcp.tool_cache.allow.server must be non-empty");
                    }
                    if r.tool.trim().is_empty() {
                        anyhow::bail!("mcp.tool_cache.allow.tool must be non-empty");
                    }
                    if r.ttl_seconds == Some(0) {
                        anyhow::bail!("mcp.tool_cache.allow.ttl_seconds must be > 0 when set");
                    }
                }
            }
            if self.mcp.hitl.enabled {
                if self.mcp.hitl.approval_url.trim().is_empty() {
                    anyhow::bail!("mcp.hitl.approval_url must be set when mcp.hitl.enabled=true");
                }
                let u: http::Uri = self
                    .mcp
                    .hitl
                    .approval_url
                    .trim()
                    .parse()
                    .map_err(|e| anyhow::anyhow!("mcp.hitl.approval_url invalid URI: {e}"))?;
                if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                    anyhow::bail!("mcp.hitl.approval_url must use http or https");
                }
                if self.mcp.hitl.timeout_ms == 0 {
                    anyhow::bail!("mcp.hitl.timeout_ms must be > 0 when mcp.hitl.enabled=true");
                }
                if self.mcp.hitl.tools.is_empty() {
                    anyhow::bail!("mcp.hitl.tools must not be empty when mcp.hitl.enabled=true");
                }
                if self.mcp.hitl.tools.iter().any(|t| t.trim().is_empty()) {
                    anyhow::bail!("mcp.hitl.tools entries must be non-empty");
                }
                if self
                    .mcp
                    .hitl
                    .bearer_token_env
                    .as_ref()
                    .is_some_and(|v| v.trim().is_empty())
                {
                    anyhow::bail!("mcp.hitl.bearer_token_env must be non-empty when set");
                }
            }
        } else if self.mcp.hitl.enabled {
            anyhow::bail!("mcp.hitl.enabled requires mcp.enabled=true");
        }
        if self.mcp.tool_routes.enabled && !self.mcp.enabled {
            anyhow::bail!("mcp.tool_routes.enabled requires mcp.enabled=true");
        }
        if self.mcp.tool_cache.enabled && !self.mcp.enabled {
            anyhow::bail!("mcp.tool_cache.enabled requires mcp.enabled=true");
        }
        if self.rate_limit_fallback.enabled {
            if self.rate_limit_fallback.backend_base.trim().is_empty() {
                anyhow::bail!("rate_limit_fallback.backend_base must be set when rate_limit_fallback.enabled=true");
            }
            if self.rate_limit_fallback.api_key_env.trim().is_empty() {
                anyhow::bail!("rate_limit_fallback.api_key_env must be non-empty when rate_limit_fallback.enabled=true");
            }
            match self.rate_limit_fallback.provider.as_str() {
                "anthropic" => {
                    let u: http::Uri =
                        self.rate_limit_fallback
                            .backend_base
                            .trim()
                            .parse()
                            .map_err(|e| {
                                anyhow::anyhow!("rate_limit_fallback.backend_base invalid URI: {e}")
                            })?;
                    if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                        anyhow::bail!("rate_limit_fallback.backend_base must use http or https");
                    }
                }
                "openai_compatible" => {
                    let u: http::Uri =
                        self.rate_limit_fallback
                            .backend_base
                            .trim()
                            .parse()
                            .map_err(|e| {
                                anyhow::anyhow!("rate_limit_fallback.backend_base invalid URI: {e}")
                            })?;
                    if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                        anyhow::bail!("rate_limit_fallback.backend_base must use http or https");
                    }
                }
                _ => anyhow::bail!(
                    "rate_limit_fallback.provider must be one of: anthropic, openai_compatible"
                ),
            }
        }
        if self.context_management.enabled {
            if self.context_management.max_messages == 0 {
                anyhow::bail!("context_management.max_messages must be > 0 when context_management.enabled=true");
            }
            if self.context_management.keep_recent_messages == 0 {
                anyhow::bail!(
                    "context_management.keep_recent_messages must be > 0 when context_management.enabled=true"
                );
            }
            if self.context_management.keep_recent_messages >= self.context_management.max_messages
            {
                anyhow::bail!(
                    "context_management.keep_recent_messages must be < context_management.max_messages"
                );
            }
            if self
                .context_management
                .summarizer_backend_base
                .trim()
                .is_empty()
            {
                anyhow::bail!(
                    "context_management.summarizer_backend_base must be set when context_management.enabled=true"
                );
            }
            let _: http::Uri = self
                .context_management
                .summarizer_backend_base
                .trim()
                .parse()
                .map_err(|e| {
                    anyhow::anyhow!("context_management.summarizer_backend_base invalid URI: {e}")
                })?;
            if self.context_management.summarizer_model.trim().is_empty() {
                anyhow::bail!(
                    "context_management.summarizer_model must be set when context_management.enabled=true"
                );
            }
            if self
                .context_management
                .summarizer_api_key_env
                .trim()
                .is_empty()
            {
                anyhow::bail!(
                    "context_management.summarizer_api_key_env must be non-empty when context_management.enabled=true"
                );
            }
            if self.context_management.request_timeout_ms == 0 {
                anyhow::bail!(
                    "context_management.request_timeout_ms must be > 0 when context_management.enabled=true"
                );
            }
            if self.context_management.summarization_max_tokens == 0 {
                anyhow::bail!(
                    "context_management.summarization_max_tokens must be > 0 when context_management.enabled=true"
                );
            }
            if self.context_management.system_prompt.trim().is_empty() {
                anyhow::bail!("context_management.system_prompt must be non-empty when context_management.enabled=true");
            }
        }
        if self.console_oidc.enabled {
            if self.console_oidc.issuer_url.trim().is_empty() {
                anyhow::bail!("console_oidc.issuer_url is required when console_oidc.enabled=true");
            }
            let iu: http::Uri = self
                .console_oidc
                .issuer_url
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("console_oidc.issuer_url invalid URI: {e}"))?;
            if iu.scheme_str() != Some("https") && iu.scheme_str() != Some("http") {
                anyhow::bail!("console_oidc.issuer_url must use http or https");
            }
            if self.console_oidc.client_id.trim().is_empty() {
                anyhow::bail!("console_oidc.client_id is required when console_oidc.enabled=true");
            }
            if self.console_oidc.client_secret_env.trim().is_empty() {
                anyhow::bail!(
                    "console_oidc.client_secret_env is required when console_oidc.enabled=true"
                );
            }
            if self.console_oidc.signing_secret_env.trim().is_empty() {
                anyhow::bail!(
                    "console_oidc.signing_secret_env is required when console_oidc.enabled=true"
                );
            }
            if self.console_oidc.redirect_path.trim().is_empty()
                || !self.console_oidc.redirect_path.starts_with('/')
            {
                anyhow::bail!("console_oidc.redirect_path must be a non-empty absolute path");
            }
            if self.console_oidc.session_ttl_seconds == 0 {
                anyhow::bail!(
                    "console_oidc.session_ttl_seconds must be > 0 when console_oidc.enabled=true"
                );
            }
            if self.console_oidc.cookie_name.trim().is_empty() {
                anyhow::bail!(
                    "console_oidc.cookie_name must be non-empty when console_oidc.enabled=true"
                );
            }
            if self.console_oidc.scopes.is_empty() {
                anyhow::bail!(
                    "console_oidc.scopes must not be empty when console_oidc.enabled=true"
                );
            }
            for r in &self.console_oidc.required_roles {
                if r.trim().is_empty() {
                    anyhow::bail!("console_oidc.required_roles entries must be non-empty");
                }
            }
            let rrm = self.console_oidc.required_roles_mode.to_ascii_lowercase();
            if rrm != "any" && rrm != "all" {
                anyhow::bail!("console_oidc.required_roles_mode must be \"any\" or \"all\"");
            }
            if !self.console_oidc.redirect_base_url.trim().is_empty() {
                let b: http::Uri =
                    self.console_oidc
                        .redirect_base_url
                        .trim()
                        .parse()
                        .map_err(|e| {
                            anyhow::anyhow!("console_oidc.redirect_base_url invalid URI: {e}")
                        })?;
                if b.scheme_str() != Some("https") && b.scheme_str() != Some("http") {
                    anyhow::bail!("console_oidc.redirect_base_url must use http or https");
                }
            }
        }
        if self.budget_hierarchy.enabled {
            let redis = self.effective_budget_hierarchy_redis_url();
            if redis.is_none() {
                anyhow::bail!(
                    "budget_hierarchy.enabled requires tpm.redis_url, budget_hierarchy.redis_url, or PANDA_REDIS_URL"
                );
            }
            if self.budget_hierarchy.jwt_claim.trim().is_empty() {
                anyhow::bail!("budget_hierarchy.jwt_claim must be non-empty when budget_hierarchy.enabled=true");
            }
            if self.budget_hierarchy.org_prompt_tokens_per_minute == Some(0) {
                anyhow::bail!("budget_hierarchy.org_prompt_tokens_per_minute must be > 0 when set");
            }
            if self.budget_hierarchy.departments.is_empty()
                && self.budget_hierarchy.org_prompt_tokens_per_minute.is_none()
            {
                anyhow::bail!(
                    "budget_hierarchy requires at least one departments[] entry or org_prompt_tokens_per_minute"
                );
            }
            for d in &self.budget_hierarchy.departments {
                if d.department.trim().is_empty() {
                    anyhow::bail!("budget_hierarchy.departments.department must be non-empty");
                }
                if d.prompt_tokens_per_minute == 0 {
                    anyhow::bail!(
                        "budget_hierarchy.departments.prompt_tokens_per_minute must be > 0"
                    );
                }
            }
            if !self.identity.require_jwt {
                anyhow::bail!("budget_hierarchy.enabled requires identity.require_jwt=true for JWT claim extraction");
            }
            if let Some(u) = self.budget_hierarchy.usd_per_million_prompt_tokens {
                if !u.is_finite() || u < 0.0 {
                    anyhow::bail!(
                        "budget_hierarchy.usd_per_million_prompt_tokens must be finite and >= 0 when set"
                    );
                }
            }
        }
        if self.model_failover.enabled {
            if self.model_failover.path_prefix.trim().is_empty()
                || !self.model_failover.path_prefix.starts_with('/')
            {
                anyhow::bail!(
                    "model_failover.path_prefix must be a non-empty path starting with /"
                );
            }
            if self.model_failover.groups.is_empty() {
                anyhow::bail!(
                    "model_failover.groups must not be empty when model_failover.enabled=true"
                );
            }
            for g in &self.model_failover.groups {
                if g.backends.is_empty() {
                    anyhow::bail!("each model_failover group must have at least one backend");
                }
                for m in &g.match_models {
                    if m.trim().is_empty() {
                        anyhow::bail!("model_failover.match_models entries must be non-empty");
                    }
                }
                for b in &g.backends {
                    if b.backend_base.trim().is_empty() {
                        anyhow::bail!("model_failover.backends[].backend_base must not be empty");
                    }
                    let u: http::Uri = b.backend_base.trim().parse().map_err(|e| {
                        anyhow::anyhow!("model_failover.backends[].backend_base invalid URI: {e}")
                    })?;
                    if u.scheme_str() != Some("http") && u.scheme_str() != Some("https") {
                        anyhow::bail!("model_failover.backends[].backend_base must use http or https");
                    }
                    if b.api_key_env.as_ref().is_some_and(|e| e.trim().is_empty()) {
                        anyhow::bail!(
                            "model_failover backend api_key_env must be non-empty when set"
                        );
                    }
                    if b.protocol == ModelFailoverProtocol::Anthropic {
                        for op in &b.supports {
                            if *op != ModelFailoverOperation::ChatCompletions {
                                anyhow::bail!(
                                    "model_failover: anthropic protocol backends only support chat_completions (got {:?})",
                                    op
                                );
                            }
                        }
                    }
                }
            }
            if let Some(ref e) = self.model_failover.embeddings_path_prefix {
                let t = e.trim();
                if t.is_empty() || !t.starts_with('/') {
                    anyhow::bail!("model_failover.embeddings_path_prefix must be a non-empty path starting with /");
                }
            }
            if let Some(ref r) = self.model_failover.responses_path_prefix {
                let t = r.trim();
                if t.is_empty() || !t.starts_with('/') {
                    anyhow::bail!("model_failover.responses_path_prefix must be a non-empty path starting with /");
                }
            }
            if let Some(ref r) = self.model_failover.images_path_prefix {
                let t = r.trim();
                if t.is_empty() || !t.starts_with('/') {
                    anyhow::bail!("model_failover.images_path_prefix must be a non-empty path starting with /");
                }
            }
            if let Some(ref r) = self.model_failover.audio_path_prefix {
                let t = r.trim();
                if t.is_empty() || !t.starts_with('/') {
                    anyhow::bail!(
                        "model_failover.audio_path_prefix must be a non-empty path starting with /"
                    );
                }
            }
            if self.model_failover.circuit_breaker_enabled {
                if self.model_failover.circuit_breaker_failure_threshold == 0 {
                    anyhow::bail!("model_failover.circuit_breaker_failure_threshold must be > 0");
                }
                if self.model_failover.circuit_breaker_open_seconds == 0 {
                    anyhow::bail!("model_failover.circuit_breaker_open_seconds must be > 0");
                }
            }
        }
        let fb = self.routing.fallback.to_ascii_lowercase();
        if fb != "static" && fb != "deny" {
            anyhow::bail!("routing.fallback must be one of: static, deny");
        }
        if self.routing.semantic.enabled && !self.routing.enabled {
            anyhow::bail!("routing.semantic.enabled requires routing.enabled=true");
        }
        let any_route_wants_semantic = self.routes.iter().any(|r| {
            r.routing
                .as_ref()
                .is_some_and(|o| o.semantic_enabled == Some(true))
        });
        if self.routing.semantic.enabled || any_route_wants_semantic {
            if any_route_wants_semantic && !self.routing.semantic.enabled {
                anyhow::bail!(
                    "routes[].routing.semantic_enabled=true requires routing.semantic.enabled=true (shared embed/router endpoints)"
                );
            }
            Self::validate_semantic_routing_config(&self.routing.semantic)?;
        }
        if self.semantic_cache.enabled {
            match self.semantic_cache.backend.as_str() {
                "memory" | "redis" => {}
                _ => anyhow::bail!("semantic_cache.backend must be one of: memory, redis"),
            }
            if self
                .semantic_cache
                .redis_url
                .as_ref()
                .is_some_and(|v| v.trim().is_empty())
            {
                anyhow::bail!("semantic_cache.redis_url must be non-empty when set");
            }
            if !(0.0..=1.0).contains(&self.semantic_cache.similarity_threshold) {
                anyhow::bail!("semantic_cache.similarity_threshold must be within [0.0, 1.0]");
            }
            if self.semantic_cache.max_entries == 0 {
                anyhow::bail!(
                    "semantic_cache.max_entries must be > 0 when semantic_cache.enabled=true"
                );
            }
            if self.semantic_cache.ttl_seconds == 0 {
                anyhow::bail!(
                    "semantic_cache.ttl_seconds must be > 0 when semantic_cache.enabled=true"
                );
            }
            if self.semantic_cache.embedding_lookup_enabled {
                if self.semantic_cache.backend.trim() != "memory" {
                    anyhow::bail!("semantic_cache.embedding_lookup_enabled requires semantic_cache.backend: memory");
                }
                let u = self
                    .semantic_cache
                    .embedding_url
                    .as_deref()
                    .unwrap_or("")
                    .trim();
                if u.is_empty() {
                    anyhow::bail!("semantic_cache.embedding_url must be non-empty when embedding_lookup_enabled=true");
                }
                if self.semantic_cache.embedding_api_key_env.trim().is_empty() {
                    anyhow::bail!("semantic_cache.embedding_api_key_env must be non-empty when embedding_lookup_enabled=true");
                }
            }
        }
        ensure_adapter_provider_allowed(&self.adapter.provider)?;
        if self.adapter.anthropic_version.trim().is_empty() {
            anyhow::bail!("adapter.anthropic_version must be non-empty");
        }
        let mut seen_prefixes = std::collections::HashSet::<String>::new();
        for r in &self.routes {
            if r.path_prefix.trim().is_empty() {
                anyhow::bail!("routes.path_prefix must be non-empty");
            }
            if !r.path_prefix.starts_with('/') {
                anyhow::bail!(
                    "routes.path_prefix must start with '/': {:?}",
                    r.path_prefix
                );
            }
            if r.backend_base.trim().is_empty() {
                anyhow::bail!("routes.backend_base must not be empty");
            }
            let _: http::Uri = r
                .backend_base
                .parse()
                .map_err(|e| anyhow::anyhow!("invalid routes.backend_base URL: {e}"))?;
            if !seen_prefixes.insert(r.path_prefix.clone()) {
                anyhow::bail!("routes.path_prefix must be unique: {:?}", r.path_prefix);
            }
            if let Some(ref rl) = r.rate_limit {
                if rl.rps == 0 {
                    anyhow::bail!("routes.rate_limit.rps must be > 0 when set");
                }
            }
            if let Some(n) = r.tpm_limit {
                if n == 0 {
                    anyhow::bail!("routes.tpm_limit must be > 0 when set");
                }
            }
            if r.semantic_cache == Some(true) && !self.semantic_cache.enabled {
                anyhow::bail!("routes.semantic_cache=true requires semantic_cache.enabled=true");
            }
            if let Some(ref t) = r.adapter_type {
                ensure_adapter_provider_allowed(t)?;
            }
            if let Some(ref names) = r.mcp_servers {
                if !names.is_empty() {
                    if !self.mcp.enabled {
                        anyhow::bail!("routes.mcp_servers requires mcp.enabled=true");
                    }
                    let known: std::collections::HashSet<_> =
                        self.mcp.servers.iter().map(|s| s.name.as_str()).collect();
                    for n in names {
                        if n.trim().is_empty() {
                            anyhow::bail!("routes.mcp_servers entries must be non-empty");
                        }
                        if !known.contains(n.as_str()) {
                            anyhow::bail!(
                                "routes.mcp_servers references unknown MCP server: {:?}",
                                n
                            );
                        }
                    }
                }
            }
            if r.mcp_advertise_tools == Some(true) && !self.mcp.enabled {
                anyhow::bail!(
                    "routes.mcp_advertise_tools=true requires mcp.enabled=true (path_prefix={:?})",
                    r.path_prefix
                );
            }
        }
        if self.api_gateway.ingress.enabled {
            let has_hs256 = !self.identity.jwt_hs256_secret_env.trim().is_empty();
            let has_jwks = self
                .identity
                .jwks_url
                .as_ref()
                .is_some_and(|u| !u.trim().is_empty());
            let can_validate_jwt = has_hs256 || has_jwks;
            for (i, r) in self.api_gateway.ingress.routes.iter().enumerate() {
                if r.auth == ApiGatewayIngressAuthMode::Required && !can_validate_jwt {
                    anyhow::bail!(
                        "api_gateway.ingress.routes[{i}].auth=required requires identity.jwt_hs256_secret_env and/or identity.jwks_url (or auth.jwks_url)"
                    );
                }
            }
        }
        self.api_gateway.validate()?;
        Ok(())
    }

    /// Longest-prefix matching route for an **ingress** path (client URI path).
    pub fn effective_route_for_path(&self, path: &str) -> Option<&RouteConfig> {
        let mut best: Option<(&RouteConfig, usize)> = None;
        for r in &self.routes {
            if path.starts_with(r.path_prefix.as_str()) {
                let len = r.path_prefix.len();
                if best.map(|(_, l)| len > l).unwrap_or(true) {
                    best = Some((r, len));
                }
            }
        }
        best.map(|(r, _)| r)
    }

    /// When the matching route sets [`RouteConfig::methods`], verifies the request method is allowed.
    ///
    /// Returns `Err(allow_list)` where `allow_list` is suitable for an HTTP `Allow` response header
    /// (comma-separated methods) when the method is not permitted; otherwise `Ok(())`.
    /// No matching route or empty `methods` → `Ok(())`.
    pub fn check_ingress_method(&self, path: &str, method: &http::Method) -> Result<(), String> {
        let Some(route) = self.effective_route_for_path(path) else {
            return Ok(());
        };
        if route.methods.is_empty() {
            return Ok(());
        }
        let m = method.as_str();
        if route
            .methods
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(m))
        {
            return Ok(());
        }
        Err(route.methods.join(", "))
    }

    /// Backend base URL for a request path: longest matching [`PandaConfig::routes`] entry, else [`PandaConfig::default_backend`].
    pub fn effective_backend_base(&self, path: &str) -> &str {
        self.effective_route_for_path(path)
            .map(|r| r.backend_base.as_str())
            .unwrap_or(self.default_backend.as_str())
    }

    /// TPM budget (tokens per minute) for an ingress path (per-route [`RouteConfig::tpm_limit`] or global).
    pub fn effective_tpm_budget_tokens_per_minute(&self, path: &str) -> u64 {
        self.effective_route_for_path(path)
            .and_then(|r| r.tpm_limit)
            .unwrap_or(self.tpm.budget_tokens_per_minute)
    }

    /// Whether semantic cache may be used for this ingress path.
    pub fn effective_semantic_cache_enabled_for_path(&self, path: &str) -> bool {
        let global_on = self.semantic_cache.enabled;
        match self
            .effective_route_for_path(path)
            .and_then(|r| r.semantic_cache)
        {
            None => global_on,
            Some(false) => false,
            Some(true) => global_on,
        }
    }

    /// Effective MCP tool advertisement for `POST /v1/chat/completions`: per-route [`RouteConfig::mcp_advertise_tools`]
    /// overrides global [`McpConfig::advertise_tools`] when set.
    pub fn effective_mcp_advertise_tools_for_path(&self, path: &str) -> bool {
        match self
            .effective_route_for_path(path)
            .and_then(|r| r.mcp_advertise_tools)
        {
            Some(v) => v,
            None => self.mcp.advertise_tools,
        }
    }

    /// Adapter provider for an ingress path (see [`OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS`] and `anthropic`).
    pub fn effective_adapter_provider(&self, path: &str) -> &str {
        self.effective_route_for_path(path)
            .and_then(|r| r.adapter_type.as_deref())
            .unwrap_or(self.adapter.provider.as_str())
    }

    /// When set, only these MCP server names are used for tools on this path (`Some(&[])` disables tools).
    pub fn effective_mcp_server_names(&self, path: &str) -> Option<&[String]> {
        self.effective_route_for_path(path)
            .and_then(|r| r.mcp_servers.as_ref())
            .map(|v| v.as_slice())
    }

    /// True when the default and every route `backend_base` parses as an HTTP URI.
    pub fn all_backend_base_uris_valid(&self) -> bool {
        if self.default_backend.parse::<http::Uri>().is_err() {
            return false;
        }
        self.routes
            .iter()
            .all(|r| r.backend_base.parse::<http::Uri>().is_ok())
    }

    /// Effective Redis URL: YAML, then env `PANDA_REDIS_URL`.
    pub fn effective_redis_url(&self) -> Option<String> {
        self.tpm
            .redis_url
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                std::env::var("PANDA_REDIS_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            })
    }

    /// Redis for hierarchical budgets: explicit `budget_hierarchy.redis_url`, else TPM/env Redis.
    pub fn effective_budget_hierarchy_redis_url(&self) -> Option<String> {
        self.budget_hierarchy
            .redis_url
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| self.effective_redis_url())
    }

    /// Effective semantic-cache Redis URL: YAML, then env `PANDA_SEMANTIC_CACHE_REDIS_URL`, then `PANDA_REDIS_URL`.
    pub fn effective_semantic_cache_redis_url(&self) -> Option<String> {
        self.semantic_cache
            .redis_url
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                std::env::var("PANDA_SEMANTIC_CACHE_REDIS_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| {
                std::env::var("PANDA_REDIS_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            })
    }

    /// Redis URL for shared gateway RPS counters: `api_gateway.ingress.rate_limit_redis.url_env` must name a non-empty env var.
    pub fn effective_api_gateway_ingress_rate_limit_redis_url(&self) -> Option<String> {
        let env_name = self
            .api_gateway
            .ingress
            .rate_limit_redis
            .url_env
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())?;
        std::env::var(env_name)
            .ok()
            .filter(|s| !s.trim().is_empty())
    }

    /// Effective `routing.enabled` for an ingress path (per-route override or global).
    pub fn effective_routing_enabled_for_path(&self, path: &str) -> bool {
        let base = self.routing.enabled;
        self.effective_route_for_path(path)
            .and_then(|r| r.routing.as_ref())
            .and_then(|o| o.enabled)
            .unwrap_or(base)
    }

    /// Effective `routing.shadow_mode` for an ingress path.
    pub fn effective_routing_shadow_mode_for_path(&self, path: &str) -> bool {
        let base = self.routing.shadow_mode;
        self.effective_route_for_path(path)
            .and_then(|r| r.routing.as_ref())
            .and_then(|o| o.shadow_mode)
            .unwrap_or(base)
    }

    /// Whether the semantic routing stage should run for this path (config only; proxy wiring may still gate on method/body).
    pub fn effective_semantic_routing_enabled_for_path(&self, path: &str) -> bool {
        if !self.effective_routing_enabled_for_path(path) {
            return false;
        }
        let base = self.routing.semantic.enabled;
        self.effective_route_for_path(path)
            .and_then(|r| r.routing.as_ref())
            .and_then(|o| o.semantic_enabled)
            .unwrap_or(base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Serialize tests that mutate process environment variables.
    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn rejects_empty_listen() {
        let err =
            PandaConfig::from_yaml_str("listen: ''\ndefault_backend: 'http://localhost'\n").unwrap_err();
        assert!(err.to_string().contains("listen"));
    }

    #[test]
    fn parses_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n",
        )
        .unwrap();
        assert!(cfg.listen_addr().is_ok());
        assert!(!cfg.agent_sessions.enabled);
        assert!(!cfg.api_gateway.ingress.enabled);
        assert!(!cfg.api_gateway.egress.enabled);
    }

    #[test]
    fn parses_api_gateway_flags() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: false
"#,
        )
        .unwrap();
        assert!(cfg.api_gateway.ingress.enabled);
        assert!(!cfg.api_gateway.egress.enabled);
    }

    #[test]
    fn rejects_egress_enabled_without_allow_hosts() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("allow_hosts"));
    }

    #[test]
    fn rejects_ingress_bad_route_prefix() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: 'v1'
        backend: ai
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("path_prefix"));
    }

    #[test]
    fn rejects_ingress_duplicate_route_prefix() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /v1
        backend: ai
      - path_prefix: /v1
        backend: ops
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("duplicate"));
    }

    #[test]
    fn accepts_ingress_enabled_empty_routes_uses_defaults_at_runtime() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
"#,
        )
        .unwrap();
        assert!(cfg.api_gateway.ingress.enabled);
        assert!(cfg.api_gateway.ingress.routes.is_empty());
    }

    #[test]
    fn accepts_egress_enabled_with_allowlist() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    timeout_ms: 5000
    pool_idle_timeout_ms: 60000
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts:
        - api.internal.example.com
      allow_path_prefixes:
        - /v1/
"#,
        )
        .unwrap();
        assert!(cfg.api_gateway.egress.enabled);
        assert_eq!(cfg.api_gateway.egress.timeout_ms, 5000);
        assert_eq!(
            cfg.api_gateway.egress.corporate.default_base.as_deref(),
            Some("https://api.internal.example.com")
        );
        assert_eq!(cfg.api_gateway.egress.allowlist.allow_hosts.len(), 1);
    }

    #[test]
    fn rejects_egress_validate_tls_partial_client_auth() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/']
    tls:
      client_cert_pem: '/tmp/panda-egress-client.pem'
"#,
        )
        .unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("client_key_pem") || msg.contains("both"),
            "unexpected: {msg}"
        );
    }

    #[test]
    fn rejects_egress_validate_tls_missing_cert_file() {
        let dir = tempfile::tempdir().unwrap();
        let key = dir.path().join("k.pem");
        std::fs::write(&key, b"dummy").unwrap();
        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/']
    tls:
      client_cert_pem: '{}'
      client_key_pem: '{}'
"#,
            dir.path().join("missing.crt").display(),
            key.display()
        );
        let err = PandaConfig::from_yaml_str(&yaml).unwrap_err();
        assert!(
            err.to_string().contains("client_cert_pem"),
            "unexpected: {err}"
        );
    }

    #[test]
    fn rejects_egress_validate_tls_non_pem_client_material() {
        let dir = tempfile::tempdir().unwrap();
        let cert = dir.path().join("c.pem");
        let key = dir.path().join("k.pem");
        std::fs::write(&cert, b"this is not a PEM certificate").unwrap();
        std::fs::write(&key, b"this is not a PEM key").unwrap();
        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/']
    tls:
      client_cert_pem: '{}'
      client_key_pem: '{}'
"#,
            cert.display(),
            key.display()
        );
        let err = PandaConfig::from_yaml_str(&yaml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("client_cert_pem") || msg.contains("certificate"),
            "unexpected: {msg}"
        );
    }

    #[test]
    fn rejects_egress_validate_tls_non_pem_extra_ca() {
        let dir = tempfile::tempdir().unwrap();
        let ca = dir.path().join("ca.pem");
        std::fs::write(&ca, b"-----BEGIN NOT A CERT-----\ndeadbeef\n-----END NOT A CERT-----\n").unwrap();
        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/']
    tls:
      extra_ca_pem: '{}'
"#,
            ca.display()
        );
        let err = PandaConfig::from_yaml_str(&yaml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("extra_ca_pem") || msg.contains("certificate"),
            "unexpected: {msg}"
        );
    }

    #[test]
    fn accepts_mcp_http_tools_with_egress() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/v1/']
mcp:
  enabled: true
  servers:
    - name: multi
      http_tools:
        - path: /v1/a
          tool_name: a
        - path: /v1/b
          method: POST
          tool_name: b
"#,
        )
        .unwrap();
        cfg.validate().unwrap();
        assert_eq!(cfg.mcp.servers[0].http_tools.len(), 2);
    }

    #[test]
    fn accepts_mcp_http_tool_with_corporate_pool_bases_only() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      pool_bases:
        - 'https://a.internal.example.com'
        - 'https://b.internal.example.com'
    allowlist:
      allow_hosts: [a.internal.example.com, b.internal.example.com]
      allow_path_prefixes: ['/api/']
mcp:
  enabled: true
  servers:
    - name: s1
      http_tool:
        path: /api/x
        method: GET
        tool_name: t
"#,
        )
        .unwrap();
        cfg.validate().unwrap();
    }

    #[test]
    fn accepts_ingress_route_with_methods() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
    routes:
      - path_prefix: /hooks
        backend: ai
        methods: [POST]
"#,
        )
        .unwrap();
        cfg.validate().unwrap();
        assert_eq!(cfg.api_gateway.ingress.routes[0].methods, vec!["POST".to_string()]);
    }

    #[test]
    fn validate_ingress_route_row_for_control_plane() {
        let ok = ApiGatewayIngressRoute {
            path_prefix: "/api".to_string(),
            backend: ApiGatewayIngressBackend::Mcp,
            methods: vec!["POST".to_string()],
            ..Default::default()
        };
        ok.validate_for_control_plane().unwrap();
        let bad_prefix = ApiGatewayIngressRoute {
            path_prefix: "relative".to_string(),
            ..Default::default()
        };
        assert!(bad_prefix.validate_for_control_plane().is_err());
        let bad_rl = ApiGatewayIngressRoute {
            path_prefix: "/z".to_string(),
            rate_limit: Some(RouteRateLimitConfig { rps: 0 }),
            ..Default::default()
        };
        assert!(bad_rl.validate_for_control_plane().is_err());
    }

    #[test]
    fn rejects_mcp_http_tool_and_http_tools_together() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://api.internal.example.com'
    allowlist:
      allow_hosts: [api.internal.example.com]
      allow_path_prefixes: ['/']
mcp:
  enabled: true
  servers:
    - name: x
      http_tool:
        path: /a
        tool_name: t1
      http_tools:
        - path: /b
          tool_name: t2
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("http_tool") && err.to_string().contains("http_tools"));
    }

    #[test]
    fn accepts_agent_sessions_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
agent_sessions:
  enabled: true
  header: x-my-agent-session
  tpm_isolated_buckets: false
"#,
        )
        .unwrap();
        assert!(cfg.agent_sessions.enabled);
        assert_eq!(cfg.agent_sessions.header, "x-my-agent-session");
        assert!(!cfg.agent_sessions.tpm_isolated_buckets);
    }

    #[test]
    fn rejects_control_plane_bad_path_prefix() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  path_prefix: no-leading-slash
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("control_plane.path_prefix"));
    }

    #[test]
    fn accepts_control_plane_enabled_default_prefix() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
"#,
        )
        .unwrap();
        assert!(cfg.control_plane.enabled);
        assert_eq!(cfg.control_plane.path_prefix, "/ops/control");
    }

    #[test]
    fn rejects_control_plane_json_file_without_path() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  store:
    kind: json_file
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("json_file"));
    }

    #[test]
    fn accepts_control_plane_sqlite_with_url() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  store:
    kind: sqlite
    database_url: 'sqlite://./cp.db'
"#,
        )
        .unwrap();
        assert!(matches!(
            cfg.control_plane.store.kind,
            ControlPlaneStoreKind::Sqlite
        ));
    }

    #[test]
    fn rejects_control_plane_reload_with_memory_store() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  reload_from_store_ms: 5000
  store:
    kind: memory
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("reload_from_store_ms"));
    }

    #[test]
    fn rejects_control_plane_postgres_listen_without_postgres() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  store:
    kind: sqlite
    database_url: 'sqlite://./cp.db'
    postgres_listen: true
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("postgres_listen"));
    }

    #[test]
    fn rejects_control_plane_reload_pubsub_without_redis_env() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  reload_pubsub:
    redis_url_env: '   '
    channel: cp
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("reload_pubsub"));
    }

    #[test]
    fn accepts_control_plane_reload_pubsub_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: true
  reload_pubsub:
    redis_url_env: PANDA_REDIS_URL
"#,
        )
        .unwrap();
        let ps = cfg.control_plane.reload_pubsub.as_ref().unwrap();
        assert_eq!(ps.redis_url_env, "PANDA_REDIS_URL");
        assert!(!ps.channel.is_empty());
    }

    #[test]
    fn rejects_console_oidc_bad_required_roles_mode() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
console_oidc:
  enabled: true
  issuer_url: 'https://issuer.example'
  client_id: cli
  client_secret_env: SEC
  signing_secret_env: SIG
  scopes: [openid]
  required_roles_mode: maybe
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("required_roles_mode"));
    }

    #[test]
    fn rejects_budget_hierarchy_bad_usd_rate() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
identity:
  require_jwt: true
tpm:
  redis_url: 'redis://127.0.0.1:6379'
budget_hierarchy:
  enabled: true
  org_prompt_tokens_per_minute: 1000
  usd_per_million_prompt_tokens: -1
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("usd_per_million_prompt_tokens"));
    }

    #[test]
    fn accepts_agent_sessions_jwt_and_profile_backend() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
agent_sessions:
  enabled: true
  jwt_session_claim: sid
  jwt_profile_claim: agent_profile
  mcp_max_tool_rounds_with_session: 8
  profile_backend_rules:
    - profile: research
      backend_base: "https://planner.example/v1"
      path_prefix: /v1/chat
      mcp_max_tool_rounds: 2
"#,
        )
        .unwrap();
        assert_eq!(cfg.agent_sessions.jwt_session_claim.as_deref(), Some("sid"));
        assert_eq!(
            cfg.agent_sessions.jwt_profile_claim.as_deref(),
            Some("agent_profile")
        );
        assert_eq!(cfg.agent_sessions.mcp_max_tool_rounds_with_session, Some(8));
        assert_eq!(cfg.agent_sessions.profile_backend_rules.len(), 1);
        assert_eq!(
            cfg.agent_sessions.profile_backend_rules[0].backend_base,
            "https://planner.example/v1"
        );
    }

    #[test]
    fn listen_addr_honors_panda_listen_override() {
        let _guard = ENV_TEST_LOCK.lock().unwrap();
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:9'\ndefault_backend: 'http://127.0.0.1:11434'\n",
        )
        .unwrap();
        unsafe {
            std::env::set_var("PANDA_LISTEN_OVERRIDE", "127.0.0.1:8088");
        }
        let a = cfg.listen_addr().unwrap();
        assert_eq!(a.port(), 8088);
        unsafe {
            std::env::remove_var("PANDA_LISTEN_OVERRIDE");
        }
    }

    #[test]
    fn rejects_bad_trusted_header_name() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:1'\n\
             trusted_gateway:\n  subject_header: 'bad name'\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid trusted_gateway"));
    }

    #[test]
    fn plugins_defaults_are_non_zero() {
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n",
        )
        .unwrap();
        assert!(cfg.plugins.max_request_body_bytes > 0);
        assert!(cfg.plugins.execution_timeout_ms > 0);
        assert!(!cfg.plugins.fail_closed);
        assert!(!cfg.plugins.hot_reload);
        assert!(cfg.plugins.reload_interval_ms > 0);
        assert!(cfg.plugins.reload_debounce_ms > 0);
        assert!(cfg.plugins.max_reloads_per_minute > 0);
        assert!(!cfg.tpm.enforce_budget);
        assert!(cfg.tpm.budget_tokens_per_minute > 0);
        assert!(cfg.tpm.retry_after_seconds.is_none());
        assert!(cfg.tpm.redis_unavailable_degraded_limits);
        assert!(cfg.tpm.redis_command_error_degraded_limits);
        assert_eq!(cfg.tpm.redis_degraded_limit_ratio, 0.5);
        assert!(!cfg.observability.compliance_export.enabled);
        assert_eq!(cfg.observability.compliance_export.mode, "off");
        assert!(!cfg.console_oidc.enabled);
        assert_eq!(cfg.console_oidc.required_roles_mode, "any");
        assert!(!cfg.budget_hierarchy.enabled);
        assert!(!cfg.model_failover.enabled);
        assert!(!cfg.model_failover.allow_failover_after_first_byte);
        assert!(cfg.model_failover.embeddings_path_prefix.is_none());
        assert!(cfg.model_failover.responses_path_prefix.is_none());
        assert_eq!(cfg.observability.admin_auth_header, "x-panda-admin-secret");
        assert!(cfg.observability.admin_secret_env.is_none());
        assert!(!cfg.identity.require_jwt);
        assert!(cfg.identity.jwks_url.is_none());
        assert_eq!(cfg.identity.jwks_cache_ttl_seconds, 3600);
        assert!(cfg.identity.accepted_issuers.is_empty());
        assert!(cfg.identity.accepted_audiences.is_empty());
        assert!(cfg.identity.required_scopes.is_empty());
        assert!(cfg.identity.route_scope_rules.is_empty());
        assert!(!cfg.identity.enable_token_exchange);
        assert_eq!(
            cfg.identity.agent_token_secret_env,
            "PANDA_AGENT_TOKEN_HS256_SECRET"
        );
        assert_eq!(cfg.identity.agent_token_ttl_seconds, 300);
        assert!(cfg.identity.agent_token_scopes.is_empty());
        assert!(!cfg.prompt_safety.enabled);
        assert!(!cfg.prompt_safety.shadow_mode);
        assert!(cfg.prompt_safety.deny_patterns.is_empty());
        assert!(!cfg.pii.enabled);
        assert!(!cfg.pii.shadow_mode);
        assert!(cfg.pii.redact_patterns.is_empty());
        assert_eq!(cfg.pii.replacement, "[REDACTED]");
        assert!(!cfg.mcp.enabled);
        assert!(cfg.mcp.fail_open);
        assert_eq!(cfg.mcp.tool_timeout_ms, 30_000);
        assert_eq!(cfg.mcp.max_tool_payload_bytes, 1_048_576);
        assert_eq!(cfg.mcp.max_tool_rounds, 4);
        assert_eq!(cfg.mcp.stream_probe_bytes, 16 * 1024);
        assert_eq!(cfg.mcp.probe_window_seconds, 60);
        assert!(!cfg.mcp.advertise_tools);
        assert_eq!(cfg.mcp.proof_of_intent_mode, "off");
        assert!(cfg.mcp.intent_tool_policies.is_empty());
        assert!(cfg.mcp.servers.is_empty());
        assert!(!cfg.mcp.hitl.enabled);
        assert!(!cfg.mcp.tool_cache.enabled);
        assert_eq!(cfg.mcp.tool_cache.backend, "memory");
        assert_eq!(cfg.mcp.tool_cache.default_ttl_seconds, 120);
        assert_eq!(cfg.mcp.tool_cache.max_value_bytes, 65_536);
        assert!(!cfg.mcp.tool_cache.compliance_log_misses);
        assert!(cfg.mcp.tool_cache.allow.is_empty());
        assert!(!cfg.rate_limit_fallback.enabled);
        assert!(!cfg.context_management.enabled);
        assert!(!cfg.semantic_cache.enabled);
        assert_eq!(cfg.semantic_cache.backend, "memory");
        assert!(cfg.semantic_cache.redis_url.is_none());
        assert_eq!(cfg.semantic_cache.similarity_threshold, 0.92);
        assert_eq!(cfg.semantic_cache.max_entries, 10_000);
        assert_eq!(cfg.semantic_cache.ttl_seconds, 300);
        assert!(!cfg.semantic_cache.similarity_fallback);
        assert!(!cfg.semantic_cache.scope_keys_with_tpm_bucket);
        assert_eq!(cfg.adapter.provider, "openai");
        assert_eq!(cfg.adapter.anthropic_version, "2023-06-01");
        assert!(cfg.routes.is_empty());
        assert!(cfg.server.is_none());
        assert!(!cfg.routing.enabled);
        assert!(!cfg.routing.shadow_mode);
        assert_eq!(cfg.routing.fallback, "static");
        assert!(!cfg.routing.semantic.enabled);
        assert_eq!(cfg.routing.semantic.mode, "off");
        assert_eq!(cfg.routing.semantic.embed_model, "text-embedding-3-small");
        assert_eq!(cfg.routing.semantic.router_model, "gpt-4o-mini");
        assert_eq!(cfg.routing.semantic.similarity_threshold, 0.55);
        assert!(cfg.routing.semantic.targets.is_empty());
        assert!(!cfg.agent_sessions.enabled);
        assert_eq!(cfg.agent_sessions.header, "x-panda-agent-session");
        assert!(cfg.agent_sessions.tpm_isolated_buckets);
    }

    #[test]
    fn rejects_mcp_enabled_without_servers() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp.servers"));
    }

    #[test]
    fn rejects_mcp_tool_cache_without_allow_entries() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  servers:\n    - name: fs\n  tool_cache:\n    enabled: true\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp.tool_cache.allow"));
    }

    #[test]
    fn accepts_mcp_enabled_with_one_server() {
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  servers:\n    - name: demo\n",
        )
        .unwrap();
        assert!(cfg.mcp.enabled);
        assert_eq!(cfg.mcp.servers.len(), 1);
        assert_eq!(cfg.mcp.servers[0].name, "demo");
    }

    #[test]
    fn rejects_mcp_remote_mcp_url_with_command() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:11434'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://x.example'
    allowlist:
      allow_hosts:
        - 'x.example'
        - 'mcp.example'
      allow_path_prefixes:
        - '/'
mcp:
  enabled: true
  servers:
    - name: r
      command: 'true'
      remote_mcp_url: 'https://mcp.example/mcp'
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("mutually exclusive"));
    }

    #[test]
    fn accepts_mcp_remote_mcp_url_with_egress() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:11434'
api_gateway:
  egress:
    enabled: true
    corporate:
      default_base: 'https://mcp.example'
    allowlist:
      allow_hosts:
        - 'mcp.example'
      allow_path_prefixes:
        - '/'
mcp:
  enabled: true
  servers:
    - name: r
      remote_mcp_url: 'https://mcp.example/mcp'
"#,
        )
        .unwrap();
        assert_eq!(
            cfg.mcp.servers[0].remote_mcp_url.as_deref(),
            Some("https://mcp.example/mcp")
        );
    }

    #[test]
    fn rejects_mcp_hitl_without_mcp_enabled() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  hitl:\n    enabled: true\n    approval_url: 'https://a.example/ok'\n    tools: ['x.y']\n",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("mcp.hitl.enabled requires mcp.enabled"));
    }

    #[test]
    fn accepts_mcp_hitl_when_mcp_enabled() {
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  servers:\n    - name: demo\n  hitl:\n    enabled: true\n    approval_url: 'https://a.example/approve'\n    tools: ['demo.drop']\n",
        )
        .unwrap();
        assert!(cfg.mcp.hitl.enabled);
        assert_eq!(cfg.mcp.hitl.tools, vec!["demo.drop".to_string()]);
    }

    #[test]
    fn rejects_rate_limit_fallback_bad_provider() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             rate_limit_fallback:\n  enabled: true\n  provider: acme\n  backend_base: 'https://api.example'\n  api_key_env: 'K'\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("rate_limit_fallback.provider"));
    }

    #[test]
    fn rejects_context_management_keep_recent_too_large() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             context_management:\n  enabled: true\n  max_messages: 10\n  keep_recent_messages: 10\n  summarizer_backend_base: 'https://api.openai.com'\n  summarizer_model: 'gpt-4o-mini'\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("keep_recent_messages"));
    }

    #[test]
    fn auth_block_merges_jwks_url_and_enforce_on_all_routes() {
        let cfg = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             auth:\n  type: jwt\n  jwks_url: 'https://issuer.example/.well-known/jwks.json'\n  enforce_on_all_routes: true\n",
        )
        .unwrap();
        assert!(cfg.identity.require_jwt);
        assert_eq!(
            cfg.identity.jwks_url.as_deref(),
            Some("https://issuer.example/.well-known/jwks.json")
        );
    }

    #[test]
    fn rejects_invalid_mcp_proof_of_intent_mode() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  proof_of_intent_mode: strict\n  servers:\n    - name: demo\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("proof_of_intent_mode"));
    }

    #[test]
    fn rejects_empty_mcp_command_when_set() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  servers:\n    - name: demo\n      command: ''\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("command"));
    }

    #[test]
    fn rejects_invalid_mcp_max_tool_rounds() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  max_tool_rounds: 0\n  servers:\n    - name: demo\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp.max_tool_rounds"));
    }

    #[test]
    fn rejects_invalid_mcp_stream_probe_bytes() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  stream_probe_bytes: 0\n  servers:\n    - name: demo\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp.stream_probe_bytes"));
    }

    #[test]
    fn rejects_invalid_mcp_probe_window_seconds() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             mcp:\n  enabled: true\n  probe_window_seconds: 0\n  servers:\n    - name: demo\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp.probe_window_seconds"));
    }

    #[test]
    fn rejects_invalid_semantic_cache_threshold() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             semantic_cache:\n  enabled: true\n  similarity_threshold: 1.2\n",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("semantic_cache.similarity_threshold"));
    }

    #[test]
    fn rejects_invalid_semantic_cache_backend() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             semantic_cache:\n  enabled: true\n  backend: dragonfly\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("semantic_cache.backend"));
    }

    #[test]
    fn accepts_semantic_cache_embedding_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: "127.0.0.1:0"
default_backend: "http://127.0.0.1:11434"
semantic_cache:
  enabled: true
  embedding_lookup_enabled: true
  embedding_url: "https://api.openai.com/v1/embeddings"
"#,
        )
        .expect("parse");
        assert!(cfg.semantic_cache.embedding_lookup_enabled);
        assert_eq!(
            cfg.semantic_cache.embedding_url.as_deref(),
            Some("https://api.openai.com/v1/embeddings")
        );
    }

    #[test]
    fn rejects_semantic_cache_embedding_with_redis_backend() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: "127.0.0.1:0"
default_backend: "http://127.0.0.1:11434"
semantic_cache:
  enabled: true
  backend: redis
  redis_url: "redis://127.0.0.1:6379"
  embedding_lookup_enabled: true
  embedding_url: "https://x.example/v1/embeddings"
"#,
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("semantic_cache.embedding_lookup_enabled requires"));
    }

    #[test]
    fn rejects_invalid_adapter_provider() {
        let err = PandaConfig::from_yaml_str(
            "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:11434'\n\
             adapter:\n  provider: unknown_provider\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("adapter.provider"));
    }

    #[test]
    fn accepts_groq_as_openai_shaped_adapter_label() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'https://api.groq.com/openai/v1'
adapter:
  provider: groq
"#,
        )
        .unwrap();
        assert_eq!(cfg.effective_adapter_provider("/v1/chat/completions"), "groq");
    }

    #[test]
    fn accepts_gemini_vertex_bedrock_labels() {
        for label in ["gemini", "vertex", "bedrock"] {
            let yaml = format!(
                "listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:9'\nadapter:\n  provider: {label}\n"
            );
            let cfg = PandaConfig::from_yaml_str(&yaml).unwrap_or_else(|e| panic!("{label}: {e}"));
            assert_eq!(
                cfg.effective_adapter_provider("/v1/chat/completions"),
                label,
                "{label}"
            );
        }
    }

    #[test]
    fn parses_routes_and_effective_backend_base_longest_prefix() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://default.example'
routes:
  - path_prefix: /v1
    backend_base: 'http://v1.example'
  - path_prefix: /v1/chat
    backend_base: 'http://chat.example'
"#,
        )
        .unwrap();
        assert_eq!(
            cfg.effective_backend_base("/other"),
            "http://default.example"
        );
        assert_eq!(
            cfg.effective_backend_base("/v1/models"),
            "http://v1.example"
        );
        assert_eq!(
            cfg.effective_backend_base("/v1/chat/completions"),
            "http://chat.example"
        );
    }

    #[test]
    fn rejects_duplicate_route_path_prefix() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api
    backend_base: 'http://a.example'
  - path_prefix: /api
    backend_base: 'http://b.example'
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("unique"));
    }

    #[test]
    fn rejects_route_path_prefix_without_leading_slash() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: api
    backend_base: 'http://a.example'
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("routes.path_prefix"));
    }

    #[test]
    fn rejects_invalid_route_backend_base_url() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /x
    backend_base: 'http://['
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("routes.backend_base"));
    }

    #[test]
    fn server_block_port_sets_listen() {
        let cfg = PandaConfig::from_yaml_str(
            r#"server:
  port: 8080
  address: "127.0.0.1"
default_backend: 'http://127.0.0.1:1'
"#,
        )
        .unwrap();
        assert_eq!(cfg.listen, "127.0.0.1:8080");
    }

    #[test]
    fn server_port_does_not_override_top_level_listen() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:9000'
server:
  port: 8080
  address: "127.0.0.1"
default_backend: 'http://127.0.0.1:1'
"#,
        )
        .unwrap();
        assert_eq!(cfg.listen, "127.0.0.1:9000");
    }

    #[test]
    fn server_listen_empty_falls_back_to_server_port() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: ''
server:
  listen: ''
  port: 3000
default_backend: 'http://127.0.0.1:1'
"#,
        )
        .unwrap();
        assert_eq!(cfg.listen, "127.0.0.1:3000");
    }

    #[test]
    fn route_path_alias_and_per_route_overrides() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://default'
semantic_cache:
  enabled: true
mcp:
  enabled: true
  servers:
    - name: a
routes:
  - path: /v1/chat
    backend_base: 'http://chat'
    rate_limit:
      rps: 50
    tpm_limit: 9000
    semantic_cache: false
    mcp_servers: [a]
    type: anthropic
"#,
        )
        .unwrap();
        assert_eq!(cfg.effective_backend_base("/v1/chat/x"), "http://chat");
        assert_eq!(cfg.effective_tpm_budget_tokens_per_minute("/v1/chat"), 9000);
        assert!(!cfg.effective_semantic_cache_enabled_for_path("/v1/chat"));
        assert_eq!(cfg.effective_adapter_provider("/v1/chat"), "anthropic");
        assert_eq!(
            cfg.effective_mcp_server_names("/v1/chat")
                .map(|s| s.to_vec()),
            Some(vec!["a".to_string()])
        );
    }

    #[test]
    fn route_methods_normalize_and_check_ingress() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api
    backend_base: 'http://api.example'
    methods: [GET, POST, PUT, PATCH, DELETE]
"#,
        )
        .unwrap();
        let r = cfg.effective_route_for_path("/api/items").unwrap();
        assert_eq!(r.methods, vec!["GET", "POST", "PUT", "PATCH", "DELETE"]);
        assert!(cfg.check_ingress_method("/api", &http::Method::GET).is_ok());
        assert!(cfg
            .check_ingress_method("/api/x", &http::Method::DELETE)
            .is_ok());
        assert!(cfg
            .check_ingress_method("/other", &http::Method::PATCH)
            .is_ok());
        assert!(cfg
            .check_ingress_method("/api", &http::Method::OPTIONS)
            .is_err());
    }

    #[test]
    fn route_mcp_advertise_tools_overrides_global() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: a
routes:
  - path_prefix: /v1/nomcp
    backend_base: 'http://api.example'
    mcp_advertise_tools: false
"#,
        )
        .unwrap();
        assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
        assert!(!cfg.effective_mcp_advertise_tools_for_path(
            "/v1/nomcp/chat/completions"
        ));
    }

    #[test]
    fn route_mcp_advertise_tools_true_when_global_false() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  advertise_tools: false
  servers:
    - name: a
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://llm.example'
    mcp_advertise_tools: true
"#,
        )
        .unwrap();
        assert!(!cfg.effective_mcp_advertise_tools_for_path("/v1/embeddings"));
        assert!(cfg.effective_mcp_advertise_tools_for_path("/v1/chat/completions"));
    }

    #[test]
    fn route_mcp_advertise_tools_true_requires_mcp_enabled() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: false
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://x'
    mcp_advertise_tools: true
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("mcp_advertise_tools"));
    }

    #[test]
    fn route_methods_rejects_invalid_token() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api
    backend_base: 'http://api.example'
    methods: ["BAD METHOD"]
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid HTTP method"));
    }

    #[test]
    fn rejects_routing_semantic_without_master_switch() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: false
  semantic:
    enabled: true
    mode: embed
    embed_backend_base: 'https://api.openai.com/v1'
    embed_api_key_env: 'OPENAI_API_KEY'
    targets:
      - name: a
        routing_text: test
        backend_base: 'https://api.openai.com/v1'
"#,
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("routing.semantic.enabled requires routing.enabled"));
    }

    #[test]
    fn accepts_routing_embed_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
  fallback: deny
  semantic:
    enabled: true
    mode: embed
    embed_backend_base: 'https://api.openai.com/v1'
    embed_api_key_env: 'OPENAI_API_KEY'
    targets:
      - name: general
        routing_text: general questions and chat
        backend_base: 'https://api.openai.com/v1'
"#,
        )
        .unwrap();
        assert!(cfg.routing.enabled);
        assert_eq!(cfg.routing.fallback, "deny");
        assert!(cfg.routing.semantic.enabled);
        assert_eq!(cfg.routing.semantic.targets.len(), 1);
        assert!(cfg.effective_routing_enabled_for_path("/v1/chat"));
        assert!(cfg.effective_semantic_routing_enabled_for_path("/v1/chat"));
    }

    #[test]
    fn accepts_routing_classifier_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
  fallback: static
  semantic:
    enabled: true
    mode: classifier
    router_backend_base: 'https://api.openai.com/v1'
    router_api_key_env: 'OPENAI_API_KEY'
    router_model: 'gpt-4o-mini'
    similarity_threshold: 0.7
    targets:
      - name: general
        routing_text: ''
        backend_base: 'https://api.openai.com/v1'
      - name: coding
        routing_text: programming and debugging tasks
        backend_base: 'https://api.anthropic.com/v1'
"#,
        )
        .unwrap();
        assert_eq!(cfg.routing.semantic.mode, "classifier");
        assert_eq!(cfg.routing.semantic.targets.len(), 2);
    }

    #[test]
    fn rejects_bad_routing_fallback() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  fallback: maybe
"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("routing.fallback"));
    }

    #[test]
    fn route_routing_overrides_effective_semantic() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
  semantic:
    enabled: true
    mode: embed
    embed_backend_base: 'https://api.openai.com/v1'
    embed_api_key_env: 'K'
    targets:
      - name: t1
        routing_text: general
        backend_base: 'https://api.openai.com/v1'
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://chat.example'
    routing:
      semantic_enabled: false
"#,
        )
        .unwrap();
        assert!(cfg.effective_semantic_routing_enabled_for_path("/other"));
        assert!(!cfg.effective_semantic_routing_enabled_for_path("/v1/chat"));
    }

    #[test]
    fn rejects_mcp_tool_routes_without_mcp_enabled() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: false
  tool_routes:
    enabled: true
    rules:
      - pattern: "*"
        action: deny
"#,
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("mcp.tool_routes.enabled requires mcp.enabled"));
    }

    #[test]
    fn accepts_mcp_tool_routes_minimal() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  tool_timeout_ms: 1000
  max_tool_payload_bytes: 1024
  servers:
    - name: a
  tool_routes:
    enabled: true
    unmatched: deny
    rules:
      - pattern: "mcp_a_*"
        action: allow
        servers: [a]
"#,
        )
        .unwrap();
        assert!(cfg.mcp.tool_routes.enabled);
        assert_eq!(cfg.mcp.tool_routes.rules.len(), 1);
    }

    #[test]
    fn rejects_route_semantic_enabled_without_global_semantic() {
        let err = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://chat.example'
    routing:
      semantic_enabled: true
"#,
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("routes[].routing.semantic_enabled"));
    }
}

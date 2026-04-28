//! Panda gateway client for AI GW, MCP GW, and API GW integration.
//!
//! Panda is an optional infrastructure layer that provides:
//! - **AI Gateway**: Rate limiting, TPM budgets, semantic caching, observability
//! - **MCP Gateway**: Tool loops, multi-step orchestration, MCP server management
//! - **API Gateway**: Request routing, auth proxy, rate limiting
//!
//! # Architecture
//!
//! When Panda is enabled:
//! ```text
//! OminiConnect → Panda AI GW → LiteLLM → LLM Provider
//! OminiConnect → Panda MCP GW → Tool Execution
//! OminiConnect → Panda API GW → Provider APIs
//! ```
//!
//! When Panda is disabled (simple mode):
//! ```text
//! OminiConnect → LiteLLM → LLM Provider
//! ```

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Panda gateway configuration.
#[derive(Debug, Clone, Default)]
pub struct PandaConfig {
    /// AI Gateway configuration
    pub ai_gateway: PandaGatewayConfig,
    /// MCP Gateway configuration
    pub mcp_gateway: PandaGatewayConfig,
    /// API Gateway configuration
    pub api_gateway: PandaGatewayConfig,
}

impl PandaConfig {
    /// Load from environment variables.
    pub fn from_env() -> Self {
        Self {
            ai_gateway: PandaGatewayConfig::from_env("PANDA_AI_GATEWAY"),
            mcp_gateway: PandaGatewayConfig::from_env("PANDA_MCP_GATEWAY"),
            api_gateway: PandaGatewayConfig::from_env("PANDA_API_GATEWAY"),
        }
    }

    /// Check if any Panda gateway is enabled.
    pub fn is_any_enabled(&self) -> bool {
        self.ai_gateway.enabled || self.mcp_gateway.enabled || self.api_gateway.enabled
    }

    /// Check if AI Gateway is enabled.
    pub fn is_ai_gateway_enabled(&self) -> bool {
        self.ai_gateway.enabled
    }

    /// Check if MCP Gateway is enabled.
    pub fn is_mcp_gateway_enabled(&self) -> bool {
        self.mcp_gateway.enabled
    }

    /// Check if API Gateway is enabled.
    pub fn is_api_gateway_enabled(&self) -> bool {
        self.api_gateway.enabled
    }
}

/// Configuration for a single Panda gateway.
#[derive(Debug, Clone)]
pub struct PandaGatewayConfig {
    /// Enable/disable this gateway
    pub enabled: bool,
    /// Panda server URL (e.g., "http://panda:8080")
    pub url: Option<String>,
    /// API key for Panda authentication
    pub api_key: Option<String>,
}

impl Default for PandaGatewayConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: None,
            api_key: None,
        }
    }
}

impl PandaGatewayConfig {
    /// Load from environment with a prefix.
    fn from_env(prefix: &str) -> Self {
        let enabled_var = format!("{}_ENABLED", prefix);
        let url_var = format!("{}_URL", prefix);
        let api_key_var = format!("{}_API_KEY", prefix);

        Self {
            enabled: std::env::var(enabled_var).ok().map(|s| s == "true").unwrap_or(false),
            url: std::env::var(url_var).ok().filter(|s| !s.is_empty()),
            api_key: std::env::var(api_key_var).ok().filter(|s| !s.is_empty()),
        }
    }
}

/// Panda AI Gateway client for LLM calls with tool support.
#[derive(Clone)]
pub struct PandaAIGatewayClient {
    config: PandaGatewayConfig,
    http_client: reqwest::Client,
}

impl PandaAIGatewayClient {
    /// Create a new Panda AI Gateway client.
    pub fn new(config: PandaGatewayConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            config,
            http_client,
        }
    }

    /// Check if the client is configured and enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.config.url.is_some()
    }

    /// Build the chat completions URL.
    fn chat_url(&self) -> Option<String> {
        self.config.url.as_ref().map(|url| format!("{}/v1/chat/completions", url.trim_end_matches('/')))
    }

    /// Send a chat completion request through Panda.
    pub async fn chat(&self, request: super::llm::ChatRequest) -> Result<super::llm::ChatResponse, PandaError> {
        let url = self.chat_url().ok_or(PandaError::NotConfigured)?;

        let mut req_builder = self.http_client.post(&url).header(CONTENT_TYPE, "application/json");

        if let Some(ref api_key) = self.config.api_key {
            req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", api_key));
        }

        let response = req_builder
            .json(&request)
            .send()
            .await
            .map_err(|e| PandaError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(PandaError::HttpError(status.as_u16(), body));
        }

        response
            .json::<super::llm::ChatResponse>()
            .await
            .map_err(|e| PandaError::ParseError(e.to_string()))
    }
}

/// Panda MCP Gateway client for tool execution.
#[derive(Clone)]
pub struct PandaMCPGatewayClient {
    config: PandaGatewayConfig,
    http_client: reqwest::Client,
}

impl PandaMCPGatewayClient {
    /// Create a new Panda MCP Gateway client.
    pub fn new(config: PandaGatewayConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            config,
            http_client,
        }
    }

    /// Check if the client is configured and enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.config.url.is_some()
    }

    /// Execute a tool call through Panda MCP Gateway.
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, PandaError> {
        if !self.is_enabled() {
            return Err(PandaError::NotConfigured);
        }

        let url = format!(
            "{}/mcp",
            self.config.url.as_ref().unwrap().trim_end_matches('/')
        );

        let mut req_builder = self.http_client.post(&url).header(CONTENT_TYPE, "application/json");

        if let Some(ref api_key) = self.config.api_key {
            req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", api_key));
        }

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": format!("panda-mcp-{}", uuid::Uuid::new_v4()),
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        });

        let response = req_builder
            .json(&request)
            .send()
            .await
            .map_err(|e| PandaError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(PandaError::HttpError(status.as_u16(), body));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| PandaError::ParseError(e.to_string()))?;

        // Extract result from JSON-RPC response
        body.get("result")
            .cloned()
            .ok_or_else(|| PandaError::UnexpectedResponse("No result in response".to_string()))
    }
}

/// Panda API Gateway client for rate-limited API calls.
#[derive(Clone)]
pub struct PandaAPIGatewayClient {
    config: PandaGatewayConfig,
    http_client: reqwest::Client,
}

impl PandaAPIGatewayClient {
    /// Create a new Panda API Gateway client.
    pub fn new(config: PandaGatewayConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            config,
            http_client,
        }
    }

    /// Check if the client is configured and enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.config.url.is_some()
    }

    /// Forward a request through Panda API Gateway.
    pub async fn forward_request(
        &self,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, PandaError> {
        if !self.is_enabled() {
            return Err(PandaError::NotConfigured);
        }

        let url = format!(
            "{}/{}",
            self.config.url.as_ref().unwrap().trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        let mut req_builder = match method {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            "PATCH" => self.http_client.patch(&url),
            _ => return Err(PandaError::InvalidMethod(method.to_string())),
        };

        req_builder = req_builder.header(CONTENT_TYPE, "application/json");

        if let Some(ref api_key) = self.config.api_key {
            req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", api_key));
        }

        if let Some(body) = body {
            req_builder = req_builder.json(&body);
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| PandaError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(PandaError::HttpError(status.as_u16(), body));
        }

        response
            .json()
            .await
            .map_err(|e| PandaError::ParseError(e.to_string()))
    }
}

/// Panda-related errors.
#[derive(Debug, thiserror::Error)]
pub enum PandaError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Not configured")]
    NotConfigured,

    #[error("Invalid method: {0}")]
    InvalidMethod(String),

    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panda_config_defaults() {
        let config = PandaConfig::default();
        assert!(!config.is_any_enabled());
    }

    #[test]
    fn test_panda_gateway_not_enabled_by_default() {
        let config = PandaGatewayConfig::default();
        assert!(!config.enabled);
        assert!(config.url.is_none());
    }
}

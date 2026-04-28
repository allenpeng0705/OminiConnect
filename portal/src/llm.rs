//! LLM client for LiteLLM integration.
//!
//! This module provides a client for calling LiteLLM as the LLM abstraction layer.
//! LiteLLM provides a unified interface to 100+ LLM providers (OpenAI, Anthropic, Ollama, etc.).
//!
//! # Architecture
//!
//! ```text
//! OminiConnect → LiteLLM → LLM Provider
//!                (unified interface)
//! ```
//!
//! When Panda AI GW is enabled:
//! ```text
//! OminiConnect → Panda AI GW → LiteLLM → LLM Provider
//! ```

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// LiteLLM client configuration.
#[derive(Debug, Clone)]
pub struct LiteLLMConfig {
    /// LiteLLM server URL (e.g., "http://litellm:4000")
    pub url: String,
    /// API key for LiteLLM (if required)
    pub api_key: Option<String>,
    /// Default model to use
    pub default_model: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Maximum number of tool call rounds (for multi-step orchestration)
    pub max_tool_rounds: u32,
    /// Whether LiteLLM is enabled
    pub enabled: bool,
}

impl Default for LiteLLMConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("LITELLM_URL")
                .unwrap_or_else(|_| "http://localhost:4000".to_string()),
            api_key: std::env::var("LITELLM_API_KEY").ok(),
            default_model: std::env::var("LITELLM_DEFAULT_MODEL")
                .unwrap_or_else(|_| "gpt-4o-mini".to_string()),
            timeout_secs: 60,
            max_tool_rounds: std::env::var("LITELLM_MAX_TOOL_ROUNDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            enabled: std::env::var("LITELLM_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
        }
    }
}

impl LiteLLMConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Check if LiteLLM is configured and enabled.
    pub fn is_available(&self) -> bool {
        self.enabled && !self.url.is_empty()
    }
}

/// A tool definition to send to the LLM.
#[derive(Debug, Clone, Serialize)]
pub struct LLMTool {
    /// Tool type (always "function" for OpenAI API)
    #[serde(rename = "type")]
    pub tool_type: String,
    /// The function definition
    pub function: LLMFunction,
}

/// A function definition within a tool.
#[derive(Debug, Clone, Serialize)]
pub struct LLMFunction {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// JSON Schema for input parameters
    pub parameters: serde_json::Value,
}

/// A tool result from a previous round.
#[derive(Debug, Clone, Serialize)]
pub struct ToolResult {
    pub call_id: String,
    pub tool_name: String,
    pub result: serde_json::Value,
}

/// Request to call LLM with tools.
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    /// Model to use (required by LiteLLM).
    pub model: String,
    /// Messages array
    pub messages: Vec<Message>,
    /// Tools to make available to the LLM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<LLMTool>>,
    /// Whether to stream the response
    #[serde(default)]
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tool_call_id")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tool_calls")]
    pub tool_calls: Option<Vec<MessageToolCall>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    #[serde(rename = "function")]
    pub function: MessageToolFunction,
}

impl MessageToolCall {
    /// Create a new tool call with type "function"
    pub fn new(id: String, name: String, arguments: String) -> Self {
        Self {
            id,
            call_type: "function".to_string(),
            function: MessageToolFunction { name, arguments },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageToolFunction {
    pub name: String,
    pub arguments: String,
}

/// Response from LiteLLM chat completion.
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    /// ID of the response
    pub id: Option<String>,
    /// Model used
    pub model: Option<String>,
    /// Response choices (contains message with tool_calls)
    pub choices: Vec<Choice>,
    /// Stop reason
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    /// Tool call ID
    pub id: Option<String>,
    /// Tool function to call
    #[serde(rename = "function")]
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolFunction {
    /// Tool name
    pub name: Option<String>,
    /// Arguments as JSON string
    pub arguments: Option<String>,
}

/// LiteLLM client for making chat completion requests.
#[derive(Clone)]
pub struct LiteLLMClient {
    config: LiteLLMConfig,
    http_client: reqwest::Client,
}

impl LiteLLMClient {
    /// Create a new LiteLLM client from config.
    pub fn new(config: LiteLLMConfig) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            config,
            http_client,
        }
    }

    /// Create from environment variables.
    pub fn from_env() -> Self {
        Self::new(LiteLLMConfig::default())
    }

    /// Check if LiteLLM is configured (URL is set).
    pub fn is_configured(&self) -> bool {
        !self.config.url.is_empty()
    }

    /// Build the chat completions URL.
    fn chat_url(&self) -> String {
        format!("{}/chat/completions", self.config.url.trim_end_matches('/'))
    }

/// Send a chat completion request.
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, LLMError> {
        let url = self.chat_url();
        let json_body = serde_json::to_string(&request).map_err(|e| LLMError::ParseError(e.to_string()))?;
        // Debug: print full JSON body to stderr
        eprintln!("[DEBUG] LiteLLM request: {}", json_body);
        tracing::info!("LiteLLM request body: {}", json_body);

        // Use curl subprocess to call LiteLLM (workaround for reqwest issues)
        let output = std::process::Command::new("curl")
            .args(&[
                "-s", "-X", "POST",
                &url,
                "-H", "Content-Type: application/json",
                "-d", &json_body,
            ])
            .output()
            .map_err(|e| LLMError::NetworkError(format!("curl failed: {}", e)))?;

        let body = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            let code = output.status.code().unwrap_or(500) as u16;
            return Err(LLMError::HttpError(code, body.to_string()));
        }

        let parsed: ChatResponse = serde_json::from_slice(&output.stdout)
            .map_err(|e| LLMError::ParseError(format!("{}: {}", e, body)))?;

        return Ok(parsed);
    }

    /// Send a chat completion with conversation context (for multi-step orchestration).
    /// This adds previous messages and tool results to maintain context across rounds.
    pub async fn chat_with_context(
        &self,
        messages: Vec<Message>,
        tools: Vec<LLMTool>,
        tool_results: Vec<ToolResult>,
    ) -> Result<ChatResponse, LLMError> {
        // Build messages with tool results appended
        let mut all_messages = messages;

// Add tool results as tool role messages (for MultiStep AI protocol)
        for tool_result in tool_results {
            let result_content = serde_json::to_string(&tool_result.result).unwrap_or_default();
            let tool_msg = Message {
                role: "tool".to_string(),
                content: Some(result_content),
                tool_call_id: Some(tool_result.call_id),
                name: None,
                tool_calls: None,
            };
            all_messages.push(tool_msg);
        }

        let request = ChatRequest {
            model: self.config.default_model.clone(),
            messages: all_messages,
            tools: Some(tools),
            stream: false,
        };

        self.chat(request).await
    }

    /// Call LLM with a simple text message (no tools).
    pub async fn chat_simple(&self, message: &str) -> Result<String, LLMError> {
        let request = ChatRequest {
            model: self.config.default_model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: Some(message.to_string()),
                tool_call_id: None,
                name: None,
                tool_calls: None,
            }],
            tools: None,
            stream: false,
        };

        let response = self.chat(request).await?;
        Ok(response.choices.first()
            .and_then(|c| c.message.content.as_ref())
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string())
    }
}

/// LLM-related errors.
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_litellm_config_from_env() {
        // Config should have defaults even without env vars
        let config = LiteLLMConfig::default();
        assert_eq!(config.url, "http://localhost:4000");
        assert_eq!(config.default_model, "gpt-4o-mini");
    }

    #[test]
    fn test_chat_url_construction() {
        let client = LiteLLMClient::new(LiteLLMConfig {
            url: "http://litellm:4000".to_string(),
            ..Default::default()
        });
        assert_eq!(client.chat_url(), "http://litellm:4000/chat/completions");
    }

    #[test]
    fn test_chat_url_trailing_slash() {
        let client = LiteLLMClient::new(LiteLLMConfig {
            url: "http://litellm:4000/".to_string(),
            ..Default::default()
        });
        assert_eq!(client.chat_url(), "http://litellm:4000/chat/completions");
    }
}

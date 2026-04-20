//! Local LLM client (Ollama/vLLM)

use crate::config::LocalLlmConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Local LLM errors
#[derive(Debug, Error)]
pub enum LocalLlmError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Timeout after {0:?}")]
    Timeout(Duration),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Local LLM not available: {0}")]
    NotAvailable(String),
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion request (OpenAI-compatible format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Chat completion response (OpenAI-compatible format)
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

/// Response choice
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Token usage
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// Local LLM client
#[derive(Debug, Clone)]
pub struct LocalLlmClient {
    config: LocalLlmConfig,
    client: Client,
}

impl LocalLlmClient {
    /// Create a new local LLM client
    pub fn new(config: LocalLlmConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_millis(config.timeout_ms))
                .build()
                .expect("Failed to create HTTP client"),
            config,
        }
    }

    /// Check if local LLM is available
    pub async fn is_available(&self) -> bool {
        // Try to ping the server
        let tags_url = format!("{}/api/tags", self.config.endpoint);

        match self.client.get(&tags_url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    /// Send a chat completion request
    pub async fn complete(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatCompletionResponse, LocalLlmError> {
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(2048),
            stream: Some(false),
        };

        let response = self
            .client
            .post(&self.config.endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    LocalLlmError::Timeout(Duration::from_millis(self.config.timeout_ms))
                } else {
                    LocalLlmError::Request(e)
                }
            })?;

        if !response.status().is_success() {
            return Err(LocalLlmError::NotAvailable(format!(
                "Server returned: {}",
                response.status()
            )));
        }

        let completion_response: ChatCompletionResponse =
            response.json().await.map_err(|e| {
                LocalLlmError::InvalidResponse(format!("Failed to parse response: {}", e))
            })?;

        Ok(completion_response)
    }

    /// Get the configured model name
    pub fn model(&self) -> &str {
        &self.config.model
    }

    /// Get the endpoint
    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }
}

/// Transform a generic chat request to local LLM format
pub fn to_local_llm_format(messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> LocalLlmConfig {
        LocalLlmConfig {
            enabled: true,
            endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "qwen2.5:7b-instruct".to_string(),
            timeout_ms: 5000,
        }
    }

    #[test]
    fn test_client_creation() {
        let config = test_config();
        let client = LocalLlmClient::new(config.clone());

        assert_eq!(client.model(), "qwen2.5:7b-instruct");
        assert!(client.endpoint().contains("localhost"));
    }

    #[test]
    fn test_message_format() {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ];

        let formatted = to_local_llm_format(messages.clone());
        assert_eq!(formatted.len(), messages.len());
    }
}

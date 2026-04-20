//! Hybrid inference router

use crate::config::{CloudFallbackConfig, HybridConfig, TargetConfig};
use crate::local_llm::{ChatMessage, LocalLlmClient, LocalLlmError};
use crate::rules::{RequestContext, RoutingDecision, RulesEngine, RuleResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

/// Router errors
#[derive(Debug, Error)]
pub enum RouterError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Local LLM error: {0}")]
    LocalLlm(#[from] LocalLlmError),

    #[error("Target not found: {0}")]
    TargetNotFound(String),

    #[error("No available target")]
    NoTargetAvailable,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid JSON response from {target}: {message}")]
    InvalidJsonResponse { target: String, message: String },
}

/// Routing destination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTarget {
    /// Route to local LLM
    Local,
    /// Route to cloud target
    Cloud(&'static str),
}

/// Router response
#[derive(Debug, Clone)]
pub struct RouterResponse {
    /// What we routed to
    pub target: RouteTarget,

    /// Target name (for cloud targets)
    pub target_name: Option<String>,

    /// Whether this was a fallback
    pub is_fallback: bool,

    /// Response content
    pub content: ResponseContent,
}

/// Response content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseContent {
    /// Local LLM response
    Local(LocalResponse),
    /// Cloud response (raw JSON)
    Cloud(serde_json::Value),
}

/// Local LLM response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalResponse {
    pub id: String,
    pub content: String,
    pub model: String,
    pub usage: LocalUsage,
}

/// Local token usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// Hybrid inference router
#[derive(Debug, Clone)]
pub struct Router {
    config: HybridConfig,
    rules_engine: RulesEngine,
    local_llm: Arc<RwLock<Option<LocalLlmClient>>>,
    cloud_clients: Arc<RwLock<HashMap<String, Client>>>,
}

impl Router {
    /// Create a new router
    pub async fn new(config: HybridConfig) -> Result<Self, RouterError> {
        let rules_engine = RulesEngine::new(&config);

        // Initialize local LLM client if enabled
        let local_llm = if config.local_llm.enabled {
            let client = LocalLlmClient::new(config.local_llm.clone());
            Some(client)
        } else {
            None
        };

        Ok(Self {
            config,
            rules_engine,
            local_llm: Arc::new(RwLock::new(local_llm)),
            cloud_clients: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Route a request and get a response
    pub async fn route(
        &self,
        ctx: RequestContext,
        messages: Vec<ChatMessage>,
    ) -> Result<RouterResponse, RouterError> {
        // Evaluate rules
        let result = self.rules_engine.evaluate(&ctx);

        // Decide where to route
        match result.action {
            RoutingDecision::RouteToLocal => {
                self.route_to_local(messages).await
            }
            RoutingDecision::RouteToTarget(target) => {
                self.route_to_cloud(target, messages).await
            }
            RoutingDecision::RouteToCloud => {
                // Use default target from config
                let target_name = self.config.cloud_fallback.target.clone();
                self.route_to_cloud(&target_name, messages).await
            }
            RoutingDecision::AskWasmPolicy => {
                // Wasm policy should have set headers - re-evaluate with higher threshold
                let mut ctx = ctx;
                ctx.wasm_sensitivity_score = Some(100); // Force local
                self.route_to_local(messages).await
            }
        }
    }

    /// Route to local LLM
    async fn route_to_local(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<RouterResponse, RouterError> {
        let local_llm = self.local_llm.read().await;

        match local_llm.as_ref() {
            Some(client) => {
                // Check availability
                if !client.is_available().await {
                    // Fallback to cloud if local not available
                    if self.config.cloud_fallback.enabled {
                        return self.route_to_cloud_fallback(messages).await;
                    }
                    return Err(RouterError::LocalLlm(LocalLlmError::NotAvailable(
                        "Local LLM not available".to_string(),
                    )));
                }

                // Call local LLM
                let response = client.complete(messages).await?;

                Ok(RouterResponse {
                    target: RouteTarget::Local,
                    target_name: Some(client.model().to_string()),
                    is_fallback: false,
                    content: ResponseContent::Local(LocalResponse {
                        id: response.id,
                        content: response.choices[0].message.content.clone(),
                        model: response.model,
                        usage: LocalUsage {
                            prompt_tokens: response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
                            completion_tokens: response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
                            total_tokens: response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
                        },
                    }),
                })
            }
            None => {
                // No local LLM configured - fallback to cloud
                if self.config.cloud_fallback.enabled {
                    self.route_to_cloud_fallback(messages).await
                } else {
                    Err(RouterError::Config("Local LLM disabled and no cloud fallback".to_string()))
                }
            }
        }
    }

    /// Route to a specific cloud target
    async fn route_to_cloud(
        &self,
        target_name: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<RouterResponse, RouterError> {
        // Find target config
        let target = self
            .config
            .get_target(target_name)
            .ok_or_else(|| RouterError::TargetNotFound(target_name.to_string()))?;

        // Get or create client for this target
        let client = self.get_cloud_client(&target).await?;

        // Build request
        let request_body = serde_json::json!({
            "model": "unknown", // Will be forwarded by cloud
            "messages": messages,
        });

        // Send request
        let response = client
            .post(&target.upstream)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            // Try fallback if not already a fallback
            if self.config.cloud_fallback.enabled && target.name != self.config.cloud_fallback.target {
                return self.route_to_cloud_fallback(messages).await;
            }
            return Err(RouterError::Request(response.error_for_status().unwrap_err()));
        }

        // Parse response body as JSON
        let body_text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            RouterError::InvalidJsonResponse {
                target: target.name.clone(),
                message: format!("{}: {}", e, &body_text[..body_text.len().min(200)]),
            }
        })?;

        Ok(RouterResponse {
            target: RouteTarget::Cloud(Box::leak(target_name.to_string().into_boxed_str())),
            target_name: Some(target_name.to_string()),
            is_fallback: false,
            content: ResponseContent::Cloud(json),
        })
    }

    /// Route to cloud fallback target
    async fn route_to_cloud_fallback(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<RouterResponse, RouterError> {
        let fallback_target_name = &self.config.cloud_fallback.target;

        // Find target config
        let target = self
            .config
            .get_target(fallback_target_name)
            .ok_or_else(|| RouterError::TargetNotFound(fallback_target_name.to_string()))?;

        // Get or create client for this target
        let client = self.get_cloud_client(&target).await?;

        // Build request
        let request_body = serde_json::json!({
            "model": "unknown",
            "messages": messages,
        });

        // Send request (no further fallback to prevent recursion)
        let response = client
            .post(&target.upstream)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RouterError::Request(response.error_for_status().unwrap_err()));
        }

        // Parse response body as JSON
        let body_text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            RouterError::InvalidJsonResponse {
                target: fallback_target_name.clone(),
                message: format!("{}: {}", e, &body_text[..body_text.len().min(200)]),
            }
        })?;

        Ok(RouterResponse {
            target: RouteTarget::Cloud(Box::leak(fallback_target_name.to_string().into_boxed_str())),
            target_name: Some(fallback_target_name.to_string()),
            is_fallback: true,
            content: ResponseContent::Cloud(json),
        })
    }

    /// Get or create HTTP client for a cloud target
    async fn get_cloud_client(&self, target: &TargetConfig) -> Result<Client, RouterError> {
        let mut clients = self.cloud_clients.write().await;

        if let Some(client) = clients.get(&target.name) {
            return Ok(client.clone());
        }

        // Get API key from environment
        let api_key = std::env::var(&target.api_key_name).map_err(|_| {
            RouterError::Config(format!(
                "Environment variable {} not set",
                target.api_key_name
            ))
        })?;

        // Create a new client with auth
        // In production, this would be cached properly
        let client_with_auth = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", api_key).parse().unwrap(),
                );
                headers
            })
            .build()
            .map_err(|e| RouterError::Config(format!("Failed to create client: {}", e)))?;

        clients.insert(target.name.clone(), client_with_auth.clone());

        Ok(client_with_auth)
    }

    /// Get the rules engine (for testing/debugging)
    pub fn rules_engine(&self) -> &RulesEngine {
        &self.rules_engine
    }

    /// Check if local LLM is available
    pub async fn is_local_available(&self) -> bool {
        let local_llm = self.local_llm.read().await;
        match local_llm.as_ref() {
            Some(client) => client.is_available().await,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> HybridConfig {
        serde_json::from_str(r#"{
            "enabled": true,
            "local_llm": {
                "enabled": true,
                "endpoint": "http://localhost:11434/v1/chat/completions",
                "model": "qwen2.5:7b-instruct",
                "timeout_ms": 5000
            },
            "targets": [
                {
                    "name": "general",
                    "provider": "openai",
                    "upstream": "https://api.openai.com/v1/chat/completions",
                    "api_key_name": "OPENAI_API_KEY"
                }
            ],
            "cloud_fallback": {
                "enabled": true,
                "target": "general"
            },
            "rules": [
                {
                    "name": "default",
                    "priority": 1,
                    "conditions": [{"type": "always"}],
                    "action": "route_to_cloud"
                }
            ]
        }"#).unwrap()
    }

    #[tokio::test]
    async fn test_router_creation() {
        let config = test_config();
        let router = Router::new(config).await;
        assert!(router.is_ok());
    }

    #[tokio::test]
    async fn test_rules_evaluation() {
        let config = test_config();
        let router = Router::new(config).await.unwrap();

        let ctx = RequestContext::default();
        let result = router.rules_engine().evaluate(&ctx);

        assert!(result.matched);
        assert!(matches!(result.action, RoutingDecision::RouteToCloud));
    }
}

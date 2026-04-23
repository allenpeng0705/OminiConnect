//! Connector execution abstraction.
//!
//! Phase 1 scaffold for Option B architecture:
//! - `omini_connect_native` executors use current OAuth vault + native proxy behavior
//! - `nango` executors will bridge to Nango-managed auth/proxy

use async_trait::async_trait;
use std::sync::Arc;

use crate::app::AppState;
use crate::oauth::models::ConnectorConfig;

/// Normalized test result returned by all connector engines.
#[derive(Debug, Clone)]
pub struct ConnectorTestResult {
    pub status: String,
    pub message: String,
}

/// Pluggable connector engine interface.
#[async_trait]
pub trait ConnectorExecutor: Send + Sync {
    async fn test_connection(&self, state: &Arc<AppState>, connector: &ConnectorConfig) -> anyhow::Result<ConnectorTestResult>;
}

/// Current built-in executor (existing OminiConnect behavior).
pub struct OminiConnectNativeExecutor;

#[async_trait]
impl ConnectorExecutor for OminiConnectNativeExecutor {
    async fn test_connection(&self, state: &Arc<AppState>, connector: &ConnectorConfig) -> anyhow::Result<ConnectorTestResult> {
        if is_api_key_platform(&connector.platform) {
            return Ok(if !connector.client_id.is_empty() {
                ConnectorTestResult {
                    status: "ok".to_string(),
                    message: "API key configured".to_string(),
                }
            } else {
                ConnectorTestResult {
                    status: "error".to_string(),
                    message: "API key not set".to_string(),
                }
            });
        }

        match state.oauth_vault.get_token(&connector.platform, "user").await {
            Ok(token) => {
                tracing::info!(
                    "Token test OK for {}: {}",
                    connector.platform,
                    &token[..token.len().min(10)]
                );
                Ok(ConnectorTestResult {
                    status: "ok".to_string(),
                    message: "Token retrieved successfully".to_string(),
                })
            }
            Err(e) => {
                tracing::warn!("Token test FAILED for {}: {}", connector.platform, e);
                Ok(ConnectorTestResult {
                    status: "error".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }
}

/// Nango bridge executor scaffold.
pub struct NangoExecutor;

#[async_trait]
impl ConnectorExecutor for NangoExecutor {
    async fn test_connection(&self, _state: &Arc<AppState>, connector: &ConnectorConfig) -> anyhow::Result<ConnectorTestResult> {
        if connector.connection_ref.trim().is_empty() {
            return Ok(ConnectorTestResult {
                status: "error".to_string(),
                message: "Nango connection_ref is empty (connect OAuth first)".to_string(),
            });
        }

        let Some((base, secret)) = crate::nango::nango_credentials() else {
            return Ok(ConnectorTestResult {
                status: "ok".to_string(),
                message: "Nango connection_ref set (NANGO_BASE_URL / NANGO_SECRET_KEY unset; remote probe skipped)".to_string(),
            });
        };

        let pk = connector.provider_key.trim();
        if pk.is_empty() {
            return Ok(ConnectorTestResult {
                status: "error".to_string(),
                message: "provider_key is empty (Nango integration unique key required)".to_string(),
            });
        }

        match crate::nango::probe_connection(&base, &secret, connector.connection_ref.trim(), pk).await {
            Ok(status) if status.is_success() => Ok(ConnectorTestResult {
                status: "ok".to_string(),
                message: format!("Nango connection exists (HTTP {})", status.as_u16()),
            }),
            Ok(status) => Ok(ConnectorTestResult {
                status: "error".to_string(),
                message: format!("Nango GET /connections/{{id}} returned HTTP {}", status.as_u16()),
            }),
            Err(e) => Ok(ConnectorTestResult {
                status: "error".to_string(),
                message: format!("Nango connection probe failed: {}", e),
            }),
        }
    }
}

pub async fn test_connector_by_engine(
    state: &Arc<AppState>,
    connector: &ConnectorConfig,
) -> anyhow::Result<ConnectorTestResult> {
    match connector.engine.as_str() {
        "nango" => NangoExecutor.test_connection(state, connector).await,
        _ => OminiConnectNativeExecutor.test_connection(state, connector).await,
    }
}

pub fn is_api_key_platform(platform: &str) -> bool {
    platform == "maton" || platform == "qqmail"
}

//! # OminiConnect OAuth2 Vault
//!
//! Centralized OAuth2 token storage and auto-refresh for Chinese SaaS platforms.
//! Supports Feishu (Lark), DingTalk, and WeChat Work.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
//! │ OminiConnect     │─────▶│  oauth_vault     │─────▶│  Platform API   │
//! │ Connectors      │◀─────│  (token vault)   │◀─────│  (OAuth2)       │
//! └─────────────────┘      └──────────────────┘      └─────────────────┘
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod pkce;
pub mod platform;
pub mod platforms;
pub mod token_store;

pub use crate::platform::OAuth2Platform as OAuth2PlatformTrait;
pub use pkce::{code_challenge_s256, random_code_verifier};
pub use platform::OAuth2Platform;
pub use platform::PlatformConfig;
pub use token_store::InMemoryTokenStore;
pub use token_store::SqlxTokenStoreBackend;
pub use token_store::TokenStore;
pub use token_store::TokenStoreBackend;

/// OAuth2 token with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthToken {
    /// Platform identifier (feishu, dingtalk, wechatwork)
    pub platform: String,
    /// User or app identifier
    pub subject: String,
    /// Access token
    pub access_token: String,
    /// Refresh token (if applicable)
    pub refresh_token: Option<String>,
    /// Token type (Bearer)
    pub token_type: String,
    /// Expiry timestamp (Unix epoch seconds)
    pub expires_at: i64,
    /// Scopes granted
    pub scopes: Vec<String>,
}

/// Token refresh request
#[derive(Debug, Clone)]
pub struct TokenRefreshRequest {
    pub platform: String,
    pub refresh_token: String,
}

/// Token refresh result
#[derive(Debug, Clone)]
pub struct TokenRefreshResult {
    pub new_token: OAuthToken,
    pub was_refreshed: bool,
}

/// Token vault manages OAuth2 tokens for all platforms
pub struct OAuthVault {
    store: Arc<TokenStore>,
    platforms: Arc<RwLock<HashMap<String, Arc<dyn OAuth2Platform + Send + Sync>>>>,
}

impl OAuthVault {
    pub fn new(store: Arc<TokenStore>) -> Self {
        Self {
            store,
            platforms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with an in-memory store and no platform registered.
    pub fn new_in_memory() -> Self {
        Self::new(Arc::new(TokenStore::in_memory()))
    }

    /// Register a platform handler (async to allow Arc<Self> usage).
    pub async fn register_platform(&self, platform: Box<dyn OAuth2Platform + Send + Sync>) {
        let name = platform.name().to_string();
        let mut platforms = self.platforms.write().await;
        platforms.insert(name, platform.into());
    }

    /// Get a valid token, refreshing if necessary
    pub async fn get_token(&self, platform: &str, subject: &str) -> Result<String, OAuthError> {
        let token = self
            .store
            .get(platform, subject)
            .await
            .map_err(|e| OAuthError::StorageError(e.to_string()))?
            .ok_or_else(|| OAuthError::TokenNotFound(format!("{}:{}", platform, subject)))?;

        // Check if token is expired
        if Self::is_token_expired(&token) {
            // Try to refresh
            if let Some(refreshed) = self.refresh_token(platform, &token).await? {
                return Ok(refreshed.new_token.access_token);
            }
            return Err(OAuthError::TokenExpired(format!(
                "{}:{}",
                platform, subject
            )));
        }

        Ok(token.access_token)
    }

    /// Store a new token
    pub async fn store_token(&self, token: OAuthToken) -> Result<(), OAuthError> {
        self.store
            .set(&token)
            .await
            .map_err(|e| OAuthError::StorageError(e.to_string()))
    }

    /// Check if token is expired (with 60-second buffer)
    pub fn is_token_expired(token: &OAuthToken) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        token.expires_at - 60 < now
    }

    /// Refresh an expired token
    async fn refresh_token(
        &self,
        platform: &str,
        token: &OAuthToken,
    ) -> Result<Option<TokenRefreshResult>, OAuthError> {
        let platforms = self.platforms.read().await;
        let platform_handler = platforms.get(platform).ok_or_else(|| {
            OAuthError::InvalidConfig(format!("No platform handler registered for {}", platform))
        })?;

        let refresh_token = token.refresh_token.as_ref().ok_or_else(|| {
            OAuthError::TokenExpired(format!("No refresh token for {}", platform))
        })?;

        let new_token = platform_handler
            .refresh_token(refresh_token)
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        // Store the new token
        self.store
            .set(&new_token)
            .await
            .map_err(|e| OAuthError::StorageError(e.to_string()))?;

        Ok(Some(TokenRefreshResult {
            new_token,
            was_refreshed: true,
        }))
    }

    /// Delete a token
    pub async fn delete_token(&self, platform: &str, subject: &str) -> Result<(), OAuthError> {
        self.store
            .delete(platform, subject)
            .await
            .map_err(|e| OAuthError::StorageError(e.to_string()))
    }
}

/// OAuth2 error types
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    #[error("Token not found for platform {0}")]
    TokenNotFound(String),

    #[error("Token expired and cannot be refreshed: {0}")]
    TokenExpired(String),

    #[error("OAuth2 exchange failed: {0}")]
    ExchangeFailed(String),

    #[error("Invalid platform configuration: {0}")]
    InvalidConfig(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Token vault access trait - implemented by OAuthVault for use by connectors
pub trait TokenVaultAccess: Send + Sync {
    fn get_token(
        &self,
        platform: &str,
        subject: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, OAuthError>> + Send + '_>>;
}

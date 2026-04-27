//! Platform trait for OAuth2 operations.

use crate::{OAuthError, OAuthToken};
use async_trait::async_trait;

/// OAuth2 platform handler trait
#[async_trait]
pub trait OAuth2Platform: Send + Sync {
    /// Platform name (feishu, dingtalk, wechatwork)
    fn name(&self) -> &str;

    /// Exchange authorization code for tokens
    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<OAuthToken, OAuthError>;

    /// Refresh an expired token
    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError>;

    /// Get authorization URL
    fn get_auth_url(&self, state: &str) -> String;

    /// Revoke a token
    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError>;
}

/// Platform configuration
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlatformConfig {
    /// Platform name
    pub name: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization endpoint
    pub auth_url: String,
    /// Token endpoint
    pub token_url: String,
    /// Revocation endpoint
    pub revoke_url: Option<String>,
    /// OAuth callback URL
    pub redirect_uri: String,
    /// Default scopes
    pub scopes: Vec<String>,
    /// Agent ID (for WeChat Work QR code login)
    #[serde(default)]
    pub agent_id: String,
}

//! Wraps a native [`OAuth2Platform`] so vault tokens are keyed per portal user + connector.

use async_trait::async_trait;
use omini_connect_oauth_vault::{OAuth2Platform, OAuthError, OAuthToken};

/// Delegates OAuth exchange/refresh/revoke to `inner`, but forces token `platform` to `storage_key`
/// so multiple portal users can each hold a token for the same native platform id.
pub struct VaultNamespacedPlatform {
    storage_key: String,
    inner: Box<dyn OAuth2Platform + Send + Sync>,
}

impl VaultNamespacedPlatform {
    pub fn new(storage_key: String, inner: Box<dyn OAuth2Platform + Send + Sync>) -> Self {
        Self { storage_key, inner }
    }
}

#[async_trait]
impl OAuth2Platform for VaultNamespacedPlatform {
    fn name(&self) -> &str {
        &self.storage_key
    }

    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        let mut t = self.inner.exchange_code(code, redirect_uri).await?;
        t.platform = self.storage_key.clone();
        Ok(t)
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let mut t = self.inner.refresh_token(refresh_token).await?;
        t.platform = self.storage_key.clone();
        Ok(t)
    }

    fn get_auth_url(&self, state: &str) -> String {
        self.inner.get_auth_url(state)
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        self.inner.revoke_token(token).await
    }
}

//! Token storage trait and in-memory implementation.

use crate::OAuthToken;
use async_trait::async_trait;
use std::sync::Arc;

/// Token storage backend trait
#[async_trait]
pub trait TokenStoreBackend: Send + Sync {
    async fn get(&self, platform: &str, subject: &str) -> Result<Option<OAuthToken>, Box<dyn std::error::Error + Send + Sync>>;
    async fn set(&self, token: &OAuthToken) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, platform: &str, subject: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// In-memory token store (for development/testing)
pub struct InMemoryTokenStore {
    tokens: Arc<tokio::sync::RwLock<std::collections::HashMap<String, OAuthToken>>>,
}

impl InMemoryTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}

impl Default for InMemoryTokenStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TokenStoreBackend for InMemoryTokenStore {
    async fn get(&self, platform: &str, subject: &str) -> Result<Option<OAuthToken>, Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("{}:{}", platform, subject);
        let guard = self.tokens.read().await;
        Ok(guard.get(&key).cloned())
    }

    async fn set(&self, token: &OAuthToken) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("{}:{}", token.platform, token.subject);
        let mut guard = self.tokens.write().await;
        guard.insert(key, token.clone());
        Ok(())
    }

    async fn delete(&self, platform: &str, subject: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let key = format!("{}:{}", platform, subject);
        let mut guard = self.tokens.write().await;
        guard.remove(&key);
        Ok(())
    }
}

/// Token store wrapper
pub struct TokenStore {
    backend: Arc<dyn TokenStoreBackend>,
}

impl TokenStore {
    pub fn new(backend: Arc<dyn TokenStoreBackend>) -> Self {
        Self { backend }
    }

    pub fn in_memory() -> Self {
        Self::new(Arc::new(InMemoryTokenStore::new()))
    }

    pub async fn get(&self, platform: &str, subject: &str) -> Result<Option<OAuthToken>, Box<dyn std::error::Error + Send + Sync>> {
        self.backend.get(platform, subject).await
    }

    pub async fn set(&self, token: &OAuthToken) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.backend.set(token).await
    }

    pub async fn delete(&self, platform: &str, subject: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.backend.delete(platform, subject).await
    }
}

impl std::fmt::Debug for TokenStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStore").finish()
    }
}

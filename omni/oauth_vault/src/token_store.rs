//! Token storage trait and in-memory/SQL implementations.

use crate::OAuthToken;
use async_trait::async_trait;
use std::sync::Arc;
use sqlx::FromRow;

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

// ------------------------------------------------------------------------------------------------
// SQL-backed token store
// ------------------------------------------------------------------------------------------------

#[derive(FromRow)]
struct TokenRow {
    platform: String,
    subject: String,
    access_token: String,
    refresh_token: Option<String>,
    token_type: String,
    expires_at: String,
    scopes: String,
}

impl From<TokenRow> for OAuthToken {
    fn from(r: TokenRow) -> Self {
        OAuthToken {
            platform: r.platform,
            subject: r.subject,
            access_token: r.access_token,
            refresh_token: r.refresh_token,
            token_type: r.token_type,
            expires_at: r.expires_at.parse().unwrap_or(0),
            scopes: r.scopes.split_whitespace().map(String::from).collect(),
        }
    }
}

/// SQL-backed token store using sqlx (supports SQLite, PostgreSQL, MySQL via `any` driver)
pub struct SqlxTokenStoreBackend {
    pool: sqlx::AnyPool,
}

impl SqlxTokenStoreBackend {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenStoreBackend for SqlxTokenStoreBackend {
    async fn get(&self, platform: &str, subject: &str) -> Result<Option<OAuthToken>, Box<dyn std::error::Error + Send + Sync>> {
        let row: Option<TokenRow> = sqlx::query_as(
            "SELECT platform, subject, access_token, refresh_token, token_type, expires_at, scopes FROM oauth_tokens WHERE platform = $1 AND subject = $2",
        )
        .bind(platform)
        .bind(subject)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn set(&self, token: &OAuthToken) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let scopes = token.scopes.join(" ");
        sqlx::query(
            r#"INSERT INTO oauth_tokens (platform, subject, access_token, refresh_token, token_type, expires_at, scopes, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, datetime('now'))
               ON CONFLICT(platform, subject) DO UPDATE SET
               access_token = EXCLUDED.access_token,
               refresh_token = EXCLUDED.refresh_token,
               token_type = EXCLUDED.token_type,
               expires_at = EXCLUDED.expires_at,
               scopes = EXCLUDED.scopes"#,
        )
        .bind(&token.platform)
        .bind(&token.subject)
        .bind(&token.access_token)
        .bind(&token.refresh_token)
        .bind(&token.token_type)
        .bind(token.expires_at.to_string())
        .bind(scopes)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, platform: &str, subject: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM oauth_tokens WHERE platform = $1 AND subject = $2")
            .bind(platform)
            .bind(subject)
            .execute(&self.pool)
            .await?;
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

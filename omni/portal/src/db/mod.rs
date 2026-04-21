//! Database persistence layer using sqlx (SQLite + MySQL + PostgreSQL).
//!
//! Configure via `DATABASE_URL` env var:
//! - `sqlite:portal.db` (default if not set)
//! - `mysql://user:pass@localhost/omni_portal` (also works for MariaDB)
//! - `postgres://user:pass@localhost/omni_portal`

use sqlx::any::Any;
use sqlx::pool::PoolOptions;

use crate::auth::models::{ApiKey, Session, User};
use crate::oauth::models::ConnectorConfig;

/// Create a DB pool. Reads `DATABASE_URL` env var, defaults to `sqlite:portal.db`.
pub async fn create_pool() -> anyhow::Result<sqlx::AnyPool> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:portal.db".to_string());
    tracing::info!("Connecting to database: {}", url);

    // Install the any driver with support for sqlite, postgres, and mysql (MariaDB compatible)
    sqlx::any::install_default_drivers();

    let pool = PoolOptions::<Any>::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}

/// Run the initial migration (creates all tables).
pub async fn run_migrations(pool: &sqlx::AnyPool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let migrations = vec![
        // Users table
        r#"CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )"#,
        // Sessions table
        r#"CREATE TABLE IF NOT EXISTS sessions (
            session_id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            created_at TEXT NOT NULL,
            expires_at TEXT NOT NULL
        )"#,
        // API keys table
        r#"CREATE TABLE IF NOT EXISTS api_keys (
            key_hash TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            label TEXT NOT NULL,
            created_at TEXT NOT NULL
        )"#,
        // Connectors table
        r#"CREATE TABLE IF NOT EXISTS connectors (
            platform TEXT PRIMARY KEY,
            client_id TEXT NOT NULL,
            client_secret TEXT NOT NULL,
            redirect_uri TEXT NOT NULL DEFAULT '',
            scopes TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1
        )"#,
        // OAuth tokens table
        r#"CREATE TABLE IF NOT EXISTS oauth_tokens (
            platform TEXT NOT NULL,
            subject TEXT NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT,
            expires_at TEXT,
            created_at TEXT NOT NULL,
            PRIMARY KEY (platform, subject)
        )"#,
    ];

    for migration in migrations {
        sqlx::query(migration).execute(&mut *conn).await?;
    }

    tracing::info!("Database migrations applied");
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// User repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get(&self, username: &str) -> anyhow::Result<Option<User>>;
    async fn insert(&self, user: &User) -> anyhow::Result<()>;
}

pub struct SqlxUserRepo {
    pool: sqlx::AnyPool,
}

impl SqlxUserRepo {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for SqlxUserRepo {
    async fn get(&self, username: &str) -> anyhow::Result<Option<User>> {
        let row: Option<models::UserRow> = sqlx::query_as(
            "SELECT username, password_hash, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn insert(&self, user: &User) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO users (username, password_hash, created_at) VALUES ($1, $2, $3)",
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(user.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Session repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn get(&self, session_id: &str) -> anyhow::Result<Option<Session>>;
    async fn insert(&self, session: &Session) -> anyhow::Result<()>;
    async fn delete(&self, session_id: &str) -> anyhow::Result<()>;
}

pub struct SqlxSessionRepo {
    pool: sqlx::AnyPool,
}

impl SqlxSessionRepo {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionRepository for SqlxSessionRepo {
    async fn get(&self, session_id: &str) -> anyhow::Result<Option<Session>> {
        let row: Option<models::SessionRow> = sqlx::query_as(
            "SELECT session_id, username, created_at, expires_at FROM sessions WHERE session_id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn insert(&self, session: &Session) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO sessions (session_id, username, created_at, expires_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(&session.session_id)
        .bind(&session.username)
        .bind(session.created_at.to_rfc3339())
        .bind(session.expires_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, session_id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM sessions WHERE session_id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// API key repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn get_by_hash(&self, key_hash: &str) -> anyhow::Result<Option<ApiKey>>;
    async fn insert(&self, api_key: &ApiKey) -> anyhow::Result<()>;
    async fn list_all(&self) -> anyhow::Result<Vec<ApiKey>>;
}

pub struct SqlxApiKeyRepo {
    pool: sqlx::AnyPool,
}

impl SqlxApiKeyRepo {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ApiKeyRepository for SqlxApiKeyRepo {
    async fn get_by_hash(&self, key_hash: &str) -> anyhow::Result<Option<ApiKey>> {
        let row: Option<models::ApiKeyRow> = sqlx::query_as(
            "SELECT key_hash, username, label, created_at FROM api_keys WHERE key_hash = $1",
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn insert(&self, api_key: &ApiKey) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO api_keys (key_hash, username, label, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(&api_key.key_hash)
        .bind(&api_key.username)
        .bind(&api_key.label)
        .bind(api_key.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<ApiKey>> {
        let rows: Vec<models::ApiKeyRow> = sqlx::query_as(
            "SELECT key_hash, username, label, created_at FROM api_keys",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

// ------------------------------------------------------------------------------------------------
// Connector repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ConnectorRepository: Send + Sync {
    async fn get(&self, platform: &str) -> anyhow::Result<Option<ConnectorConfig>>;
    async fn list(&self) -> anyhow::Result<Vec<ConnectorConfig>>;
    async fn upsert(&self, config: &ConnectorConfig) -> anyhow::Result<()>;
    async fn delete(&self, platform: &str) -> anyhow::Result<()>;
}

pub struct SqlxConnectorRepo {
    pool: sqlx::AnyPool,
}

impl SqlxConnectorRepo {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConnectorRepository for SqlxConnectorRepo {
    async fn get(&self, platform: &str) -> anyhow::Result<Option<ConnectorConfig>> {
        let row: Option<models::ConnectorRow> = sqlx::query_as(
            "SELECT platform, client_id, client_secret, redirect_uri, scopes, enabled FROM connectors WHERE platform = $1",
        )
        .bind(platform)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn list(&self) -> anyhow::Result<Vec<ConnectorConfig>> {
        let rows: Vec<models::ConnectorRow> = sqlx::query_as(
            "SELECT platform, client_id, client_secret, redirect_uri, scopes, enabled FROM connectors",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn upsert(&self, config: &ConnectorConfig) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO connectors (platform, client_id, client_secret, redirect_uri, scopes, enabled)
               VALUES ($1, $2, $3, $4, $5, $6)
               ON CONFLICT(platform) DO UPDATE SET
               client_id = EXCLUDED.client_id,
               client_secret = EXCLUDED.client_secret,
               redirect_uri = EXCLUDED.redirect_uri,
               scopes = EXCLUDED.scopes,
               enabled = EXCLUDED.enabled"#,
        )
        .bind(&config.platform)
        .bind(&config.client_id)
        .bind(&config.client_secret)
        .bind(&config.redirect_uri)
        .bind(config.scopes.join(" "))
        .bind(if config.enabled { 1 } else { 0 })
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, platform: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM connectors WHERE platform = $1")
            .bind(platform)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Seed
// ------------------------------------------------------------------------------------------------

pub async fn seed_admin_user(user_repo: &dyn UserRepository) {
    use chrono::Utc;

    if let Ok(None) = user_repo.get("admin").await {
        let hash = bcrypt::hash("admin", bcrypt::DEFAULT_COST).unwrap_or_default();
        let user = User {
            username: "admin".to_string(),
            password_hash: hash,
            created_at: Utc::now(),
        };
        if let Err(e) = user_repo.insert(&user).await {
            tracing::error!("Failed to seed admin user: {}", e);
        } else {
            tracing::info!("Seeded default admin user (username=admin, password=admin)");
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Row models
// ------------------------------------------------------------------------------------------------

mod models {
    use chrono::{DateTime, Utc};
    use sqlx::FromRow;

    use crate::auth::models::{ApiKey, Session, User};
    use crate::oauth::models::ConnectorConfig;

    #[derive(FromRow)]
    pub struct UserRow {
        pub username: String,
        pub password_hash: String,
        pub created_at: String,
    }

    impl From<UserRow> for User {
        fn from(r: UserRow) -> Self {
            User {
                username: r.username,
                password_hash: r.password_hash,
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }
    }

    #[derive(FromRow)]
    pub struct SessionRow {
        pub session_id: String,
        pub username: String,
        pub created_at: String,
        pub expires_at: String,
    }

    impl From<SessionRow> for Session {
        fn from(r: SessionRow) -> Self {
            Session {
                session_id: r.session_id,
                username: r.username,
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                expires_at: DateTime::parse_from_rfc3339(&r.expires_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }
    }

    #[derive(FromRow)]
    pub struct ApiKeyRow {
        pub key_hash: String,
        pub username: String,
        pub label: String,
        pub created_at: String,
    }

    impl From<ApiKeyRow> for ApiKey {
        fn from(r: ApiKeyRow) -> Self {
            ApiKey {
                key_hash: r.key_hash,
                username: r.username,
                label: r.label,
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }
    }

    #[derive(FromRow)]
    pub struct ConnectorRow {
        pub platform: String,
        pub client_id: String,
        pub client_secret: String,
        pub redirect_uri: String,
        pub scopes: String,
        pub enabled: i32,
    }

    impl From<ConnectorRow> for ConnectorConfig {
        fn from(r: ConnectorRow) -> Self {
            ConnectorConfig {
                platform: r.platform,
                client_id: r.client_id,
                client_secret: r.client_secret,
                redirect_uri: r.redirect_uri,
                scopes: r.scopes.split_whitespace().map(String::from).collect(),
                enabled: r.enabled != 0,
            }
        }
    }
}

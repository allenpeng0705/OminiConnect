//! Database persistence layer using sqlx (SQLite + MySQL + PostgreSQL).
//!
//! Configure via `DATABASE_URL` env var:
//! - If unset: SQLite at `omini_connect/portal/portal.db` (next to this crate’s `Cargo.toml`)
//! - `mysql://user:pass@localhost/omini_connect_portal` (also works for MariaDB)
//! - `postgres://user:pass@localhost/omini_connect_portal`

use sqlx::any::Any;
use sqlx::pool::PoolOptions;

use crate::auth::models::{ApiKey, Session, User};
use crate::oauth::models::ConnectorConfig;

fn default_sqlite_url() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("portal.db");
    format!("sqlite://{}", path.display())
}

/// Rebuild `connectors` with composite primary key `(owner_username, platform)` and attach legacy rows to `admin`.
async fn migrate_connectors_per_owner(pool: &sqlx::AnyPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS portal_meta (
            k TEXT PRIMARY KEY,
            v TEXT NOT NULL
        )"#,
    )
    .execute(pool)
    .await?;

    let cur: Option<String> = sqlx::query_scalar("SELECT v FROM portal_meta WHERE k = 'schema_version'")
        .fetch_optional(pool)
        .await?;
    if cur.as_deref() == Some("2") {
        return Ok(());
    }

    let mut tx = pool.begin().await?;

    sqlx::query("DROP TABLE IF EXISTS connectors_owner_pk")
        .execute(&mut *tx)
        .await
        .ok();

    sqlx::query(
        r#"CREATE TABLE connectors_owner_pk (
            owner_username TEXT NOT NULL,
            platform TEXT NOT NULL,
            client_id TEXT NOT NULL,
            client_secret TEXT NOT NULL,
            redirect_uri TEXT NOT NULL DEFAULT '',
            scopes TEXT NOT NULL DEFAULT '',
            engine TEXT NOT NULL DEFAULT 'omini_connect_native',
            provider_key TEXT NOT NULL DEFAULT '',
            connection_ref TEXT NOT NULL DEFAULT '',
            agent_id TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1,
            PRIMARY KEY (owner_username, platform)
        )"#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"INSERT INTO connectors_owner_pk (
            owner_username, platform, client_id, client_secret, redirect_uri, scopes, engine, provider_key, connection_ref, agent_id, enabled
        ) SELECT
            'admin',
            platform,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            CASE WHEN TRIM(engine) = '' THEN 'omini_connect_native' ELSE engine END,
            COALESCE(provider_key, ''),
            COALESCE(connection_ref, ''),
            COALESCE(agent_id, ''),
            COALESCE(enabled, 1)
        FROM connectors"#,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query("DROP TABLE connectors")
        .execute(&mut *tx)
        .await?;

    sqlx::query("ALTER TABLE connectors_owner_pk RENAME TO connectors")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM portal_meta WHERE k = 'schema_version'")
        .execute(&mut *tx)
        .await
        .ok();
    sqlx::query("INSERT INTO portal_meta (k, v) VALUES ('schema_version', '2')")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    tracing::info!("Migrated connectors table to per-owner primary key (schema_version=2)");
    Ok(())
}

/// Create a DB pool. Reads `DATABASE_URL` env var, or defaults to SQLite `portal.db` beside this crate.
pub async fn create_pool() -> anyhow::Result<sqlx::AnyPool> {
    let url = std::env::var("DATABASE_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| default_sqlite_url());
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
            created_at TEXT NOT NULL,
            agent_id TEXT
        )"#,
        // Agents table
        r#"CREATE TABLE IF NOT EXISTS agents (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            owner_username TEXT NOT NULL,
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL
        )"#,
        // Connectors table
        r#"CREATE TABLE IF NOT EXISTS connectors (
            platform TEXT PRIMARY KEY,
            client_id TEXT NOT NULL,
            client_secret TEXT NOT NULL,
            redirect_uri TEXT NOT NULL DEFAULT '',
            scopes TEXT NOT NULL DEFAULT '',
            engine TEXT NOT NULL DEFAULT 'omini_connect_native',
            provider_key TEXT NOT NULL DEFAULT '',
            connection_ref TEXT NOT NULL DEFAULT '',
            agent_id TEXT NOT NULL DEFAULT '',
            enabled INTEGER NOT NULL DEFAULT 1
        )"#,
        // OAuth tokens table
        r#"CREATE TABLE IF NOT EXISTS oauth_tokens (
            platform TEXT NOT NULL,
            subject TEXT NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT,
            token_type TEXT NOT NULL DEFAULT 'Bearer',
            expires_at TEXT NOT NULL,
            scopes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            PRIMARY KEY (platform, subject)
        )"#,
    ];

    for migration in migrations {
        sqlx::query(migration).execute(&mut *conn).await?;
    }

    // Backward-compatible column additions for existing databases.
    let connector_alterations = vec![
        "ALTER TABLE connectors ADD COLUMN engine TEXT NOT NULL DEFAULT 'omini_connect_native'",
        "ALTER TABLE connectors ADD COLUMN provider_key TEXT NOT NULL DEFAULT ''",
        "ALTER TABLE connectors ADD COLUMN connection_ref TEXT NOT NULL DEFAULT ''",
        "ALTER TABLE connectors ADD COLUMN agent_id TEXT NOT NULL DEFAULT ''",
    ];
    for statement in connector_alterations {
        if let Err(e) = sqlx::query(statement).execute(&mut *conn).await {
            // Ignore duplicate-column errors across sqlite/postgres/mysql variants.
            tracing::debug!("Skipping connector migration statement '{}': {}", statement, e);
        }
    }

    // Add agent_id column to api_keys (optional column for agent-scoped keys)
    if let Err(e) = sqlx::query("ALTER TABLE api_keys ADD COLUMN agent_id TEXT")
        .execute(&mut *conn)
        .await
    {
        tracing::debug!("Skipping api_keys agent_id migration: {}", e);
    }

    // Add active column to agents
    if let Err(e) = sqlx::query("ALTER TABLE agents ADD COLUMN active INTEGER NOT NULL DEFAULT 1")
        .execute(&mut *conn)
        .await
    {
        tracing::debug!("Skipping agents active migration: {}", e);
    }

    // Rename legacy built-in engine label (pre–OminiConnect naming).
    if let Err(e) = sqlx::query(
        "UPDATE connectors SET engine = 'omini_connect_native' WHERE engine = 'omni_native'",
    )
    .execute(&mut *conn)
    .await
    {
        tracing::debug!("Engine rename migration skipped: {}", e);
    }

    tracing::info!("Database migrations applied");
    drop(conn);
    migrate_connectors_per_owner(pool).await?;
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
    async fn list_by_username(&self, username: &str) -> anyhow::Result<Vec<ApiKey>>;
    async fn delete(&self, key_hash: &str) -> anyhow::Result<()>;
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
            "SELECT key_hash, username, label, created_at, agent_id FROM api_keys WHERE key_hash = $1",
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn insert(&self, api_key: &ApiKey) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO api_keys (key_hash, username, label, created_at, agent_id) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&api_key.key_hash)
        .bind(&api_key.username)
        .bind(&api_key.label)
        .bind(api_key.created_at.to_rfc3339())
        .bind(&api_key.agent_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<ApiKey>> {
        let rows: Vec<models::ApiKeyRow> = sqlx::query_as(
            "SELECT key_hash, username, label, created_at, agent_id FROM api_keys",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_by_username(&self, username: &str) -> anyhow::Result<Vec<ApiKey>> {
        let rows: Vec<models::ApiKeyRow> = sqlx::query_as(
            "SELECT key_hash, username, label, created_at, agent_id FROM api_keys WHERE username = $1",
        )
        .bind(username)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn delete(&self, key_hash: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM api_keys WHERE key_hash = $1")
            .bind(key_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Connector repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait ConnectorRepository: Send + Sync {
    async fn get(&self, owner: &str, platform: &str) -> anyhow::Result<Option<ConnectorConfig>>;
    async fn list(&self, owner: &str) -> anyhow::Result<Vec<ConnectorConfig>>;
    /// All connectors (startup registration / migrations).
    async fn list_all(&self) -> anyhow::Result<Vec<ConnectorConfig>>;
    async fn upsert(&self, owner: &str, config: &ConnectorConfig) -> anyhow::Result<()>;
    async fn delete(&self, owner: &str, platform: &str) -> anyhow::Result<()>;
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
    async fn get(&self, owner: &str, platform: &str) -> anyhow::Result<Option<ConnectorConfig>> {
        let row: Option<models::ConnectorRow> = sqlx::query_as(
            "SELECT owner_username, platform, client_id, client_secret, redirect_uri, scopes, engine, provider_key, connection_ref, agent_id, enabled FROM connectors WHERE owner_username = $1 AND platform = $2",
        )
        .bind(owner)
        .bind(platform)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn list(&self, owner: &str) -> anyhow::Result<Vec<ConnectorConfig>> {
        let rows: Vec<models::ConnectorRow> = sqlx::query_as(
            "SELECT owner_username, platform, client_id, client_secret, redirect_uri, scopes, engine, provider_key, connection_ref, agent_id, enabled FROM connectors WHERE owner_username = $1 ORDER BY platform",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<ConnectorConfig>> {
        let rows: Vec<models::ConnectorRow> = sqlx::query_as(
            "SELECT owner_username, platform, client_id, client_secret, redirect_uri, scopes, engine, provider_key, connection_ref, agent_id, enabled FROM connectors ORDER BY owner_username, platform",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn upsert(&self, owner: &str, config: &ConnectorConfig) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO connectors (owner_username, platform, client_id, client_secret, redirect_uri, scopes, engine, provider_key, connection_ref, agent_id, enabled)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               ON CONFLICT(owner_username, platform) DO UPDATE SET
               client_id = EXCLUDED.client_id,
               client_secret = EXCLUDED.client_secret,
               redirect_uri = EXCLUDED.redirect_uri,
               scopes = EXCLUDED.scopes,
               engine = EXCLUDED.engine,
               provider_key = EXCLUDED.provider_key,
               connection_ref = EXCLUDED.connection_ref,
               agent_id = EXCLUDED.agent_id,
               enabled = EXCLUDED.enabled"#,
        )
        .bind(owner)
        .bind(&config.platform)
        .bind(&config.client_id)
        .bind(&config.client_secret)
        .bind(&config.redirect_uri)
        .bind(config.scopes.join(" "))
        .bind(if config.engine.trim().is_empty() { "omini_connect_native" } else { &config.engine })
        .bind(if config.provider_key.trim().is_empty() { &config.platform } else { &config.provider_key })
        .bind(&config.connection_ref)
        .bind(&config.agent_id)
        .bind(if config.enabled { 1 } else { 0 })
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, owner: &str, platform: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM connectors WHERE owner_username = $1 AND platform = $2")
            .bind(owner)
            .bind(platform)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Agent repository
// ------------------------------------------------------------------------------------------------

#[async_trait::async_trait]
pub trait AgentRepository: Send + Sync {
    async fn get(&self, id: &str) -> anyhow::Result<Option<crate::auth::models::Agent>>;
    async fn insert(&self, agent: &crate::auth::models::Agent) -> anyhow::Result<()>;
    async fn list_by_owner(&self, owner: &str) -> anyhow::Result<Vec<crate::auth::models::Agent>>;
    async fn set_active(&self, id: &str, active: bool) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}

pub struct SqlxAgentRepo {
    pool: sqlx::AnyPool,
}

impl SqlxAgentRepo {
    pub fn new(pool: sqlx::AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AgentRepository for SqlxAgentRepo {
    async fn get(&self, id: &str) -> anyhow::Result<Option<crate::auth::models::Agent>> {
        let row: Option<models::AgentRow> = sqlx::query_as(
            "SELECT id, name, description, owner_username, active, created_at FROM agents WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn insert(&self, agent: &crate::auth::models::Agent) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO agents (id, name, description, owner_username, active, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&agent.id)
        .bind(&agent.name)
        .bind(&agent.description)
        .bind(&agent.owner_username)
        .bind(if agent.active { 1 } else { 0 })
        .bind(agent.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_by_owner(&self, owner: &str) -> anyhow::Result<Vec<crate::auth::models::Agent>> {
        let rows: Vec<models::AgentRow> = sqlx::query_as(
            "SELECT id, name, description, owner_username, active, created_at FROM agents WHERE owner_username = $1 ORDER BY created_at DESC",
        )
        .bind(owner)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn set_active(&self, id: &str, active: bool) -> anyhow::Result<()> {
        sqlx::query("UPDATE agents SET active = $1 WHERE id = $2")
            .bind(if active { 1 } else { 0 })
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM agents WHERE id = $1")
            .bind(id)
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
        pub agent_id: Option<String>,
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
                agent_id: r.agent_id,
            }
        }
    }

    #[derive(FromRow)]
    pub struct AgentRow {
        pub id: String,
        pub name: String,
        pub description: String,
        pub owner_username: String,
        pub active: i32,
        pub created_at: String,
    }

    impl From<AgentRow> for crate::auth::models::Agent {
        fn from(r: AgentRow) -> Self {
            crate::auth::models::Agent {
                id: r.id,
                name: r.name,
                description: r.description,
                owner_username: r.owner_username,
                active: r.active != 0,
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }
        }
    }

    #[derive(FromRow)]
    pub struct ConnectorRow {
        pub owner_username: String,
        pub platform: String,
        pub client_id: String,
        pub client_secret: String,
        pub redirect_uri: String,
        pub scopes: String,
        pub engine: String,
        pub provider_key: String,
        pub connection_ref: String,
        pub agent_id: String,
        pub enabled: i32,
    }

    impl From<ConnectorRow> for ConnectorConfig {
        fn from(r: ConnectorRow) -> Self {
            let platform = r.platform;
            let provider_key = if r.provider_key.is_empty() {
                platform.clone()
            } else {
                r.provider_key
            };
            ConnectorConfig {
                owner_username: r.owner_username,
                platform,
                client_id: r.client_id,
                client_secret: r.client_secret,
                redirect_uri: r.redirect_uri,
                scopes: r.scopes.split_whitespace().map(String::from).collect(),
                engine: if r.engine.is_empty() { "omini_connect_native".to_string() } else { r.engine },
                provider_key,
                connection_ref: r.connection_ref,
                agent_id: r.agent_id,
                enabled: r.enabled != 0,
            }
        }
    }
}

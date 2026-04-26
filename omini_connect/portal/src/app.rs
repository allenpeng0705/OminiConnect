//! Application state shared across all handlers.

use std::sync::Arc;

use sqlx::Any;

use crate::db::{SqlxApiKeyRepo, SqlxConnectorRepo, SqlxSessionRepo, SqlxUserRepo};
use crate::tools::ToolRegistry;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<dyn crate::db::UserRepository>,
    pub sessions: Arc<dyn crate::db::SessionRepository>,
    pub api_keys: Arc<dyn crate::db::ApiKeyRepository>,
    pub connectors: Arc<dyn crate::db::ConnectorRepository>,
    pub oauth_vault: Arc<omini_connect_oauth_vault::OAuthVault>,
    pub tools: Arc<ToolRegistry>,
}

impl AppState {
    pub async fn new(pool: sqlx::pool::Pool<Any>, tools: ToolRegistry) -> Self {
        let user_repo = SqlxUserRepo::new(pool.clone());
        let session_repo = SqlxSessionRepo::new(pool.clone());
        let api_key_repo = SqlxApiKeyRepo::new(pool.clone());
        let connector_repo = SqlxConnectorRepo::new(pool.clone());

        // Create SQLx-backed token store for OAuth token persistence
        let token_store_backend = Arc::new(omini_connect_oauth_vault::SqlxTokenStoreBackend::new(pool.clone()));
        let token_store = Arc::new(omini_connect_oauth_vault::TokenStore::new(token_store_backend));
        let oauth_vault = Arc::new(omini_connect_oauth_vault::OAuthVault::new(token_store));

        Self {
            users: Arc::new(user_repo),
            sessions: Arc::new(session_repo),
            api_keys: Arc::new(api_key_repo),
            connectors: Arc::new(connector_repo),
            oauth_vault,
            tools: Arc::new(tools),
        }
    }
}

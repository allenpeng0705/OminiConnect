//! Application state shared across all handlers.

use std::sync::Arc;

use sqlx::Any;

use crate::db::{SqlxApiKeyRepo, SqlxConnectorRepo, SqlxSessionRepo, SqlxUserRepo};

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<dyn crate::db::UserRepository>,
    pub sessions: Arc<dyn crate::db::SessionRepository>,
    pub api_keys: Arc<dyn crate::db::ApiKeyRepository>,
    pub connectors: Arc<dyn crate::db::ConnectorRepository>,
    pub oauth_vault: Arc<omni_oauth_vault::OAuthVault>,
}

impl AppState {
    pub async fn new(pool: sqlx::pool::Pool<Any>) -> Self {
        let user_repo = SqlxUserRepo::new(pool.clone());
        let session_repo = SqlxSessionRepo::new(pool.clone());
        let api_key_repo = SqlxApiKeyRepo::new(pool.clone());
        let connector_repo = SqlxConnectorRepo::new(pool.clone());

        // Create SQLx-backed token store for OAuth token persistence
        let token_store_backend = Arc::new(omni_oauth_vault::SqlxTokenStoreBackend::new(pool.clone()));
        let token_store = Arc::new(omni_oauth_vault::TokenStore::new(token_store_backend));
        let oauth_vault = Arc::new(omni_oauth_vault::OAuthVault::new(token_store));

        Self {
            users: Arc::new(user_repo),
            sessions: Arc::new(session_repo),
            api_keys: Arc::new(api_key_repo),
            connectors: Arc::new(connector_repo),
            oauth_vault,
        }
    }
}

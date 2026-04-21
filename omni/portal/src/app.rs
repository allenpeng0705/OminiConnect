//! Application state shared across all handlers.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::auth::models::{ApiKey, Session};
use crate::db::UserStore;
use crate::oauth::models::ConnectorConfig;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// User accounts (username → User)
    pub users: Arc<RwLock<UserStore>>,
    /// Active sessions (session_id → Session)
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
    /// API keys (hashed_key → ApiKey)
    pub api_keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    /// Connector configs (platform_name → ConnectorConfig)
    pub connectors: Arc<RwLock<HashMap<String, ConnectorConfig>>>,
    /// OAuth vault — manages tokens for each platform
    pub oauth_vault: Arc<omni_oauth_vault::OAuthVault>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(UserStore::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            api_keys: Arc::new(RwLock::new(HashMap::new())),
            connectors: Arc::new(RwLock::new(HashMap::new())),
            oauth_vault: Arc::new(omni_oauth_vault::OAuthVault::new_in_memory()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

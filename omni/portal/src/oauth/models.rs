//! OAuth data models.

use serde::{Deserialize, Serialize};

/// Configuration for a connector (stored in-portal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub platform: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}

/// Connection status for a connector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub platform: String,
    pub connected: bool,
    pub configured: bool,
    pub expires_at: Option<String>,
}

/// OAuth callback query params (common pattern).
#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

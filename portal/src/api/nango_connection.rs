//! Request/response types shared across API modules.

use serde::Deserialize;

/// POST /api/nango/connections body — create a Nango connection with credentials directly.
#[derive(Debug, Deserialize)]
pub struct CreateNangoConnectionBody {
    pub platform: String,
    pub auth_mode: String,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

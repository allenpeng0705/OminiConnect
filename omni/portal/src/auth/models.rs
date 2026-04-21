//! Auth data models.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A registered user account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    /// bcrypt hash of the password
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

/// An active browser session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// An API key belonging to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Hashed key (prefixed with a random salt portion, stored as `salt:hash`)
    pub key_hash: String,
    pub username: String,
    pub label: String,
    pub created_at: DateTime<Utc>,
}

/// Login request body.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response (session ID set as cookie).
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub username: String,
    pub message: String,
}

/// Generate API key request.
#[derive(Debug, Deserialize)]
pub struct GenerateApiKeyRequest {
    pub label: String,
}

/// API key response — only returned once on creation.
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub key: String, // the raw key, only returned once
    pub label: String,
    pub created_at: DateTime<Utc>,
}

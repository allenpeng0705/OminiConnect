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

/// An API key belonging to a user or an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Hashed key (prefixed with a random salt portion, stored as `salt:hash`)
    pub key_hash: String,
    pub username: String,
    pub label: String,
    pub created_at: DateTime<Utc>,
    /// Set when this key belongs to an agent (NULL for human user keys)
    pub agent_id: Option<String>,
}

/// Login request body.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Signup request body.
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
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

/// An AI agent registered by a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_username: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

/// Request to register a new agent.
#[derive(Debug, Deserialize)]
pub struct RegisterAgentRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Response when an agent is registered — includes the raw API key (shown once).
#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_username: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    /// The raw API key — only returned on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// Agent summary (no sensitive data) for listing.
#[derive(Debug, Serialize)]
pub struct AgentSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Agent> for AgentSummary {
    fn from(a: Agent) -> Self {
        Self {
            id: a.id,
            name: a.name,
            description: a.description,
            active: a.active,
            created_at: a.created_at,
        }
    }
}

impl From<Agent> for AgentResponse {
    fn from(a: Agent) -> Self {
        Self {
            id: a.id,
            name: a.name,
            description: a.description,
            owner_username: a.owner_username,
            active: a.active,
            created_at: a.created_at,
            api_key: None,
        }
    }
}

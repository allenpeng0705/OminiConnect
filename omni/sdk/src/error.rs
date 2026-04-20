//! SDK error types

use thiserror::Error;

/// Result type for SDK operations
pub type Result<T> = std::result::Result<T, SdkError>;

/// SDK error types
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Tool invocation error: {0}")]
    ToolInvocation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Skill error: {0}")]
    Skill(String),
}

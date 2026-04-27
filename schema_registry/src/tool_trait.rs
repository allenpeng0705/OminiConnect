//! Core trait and type definitions for the schema registry.

use serde_json::Value;

/// Platform identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Feishu,
    DingTalk,
    WeChatWork,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Feishu => "feishu",
            Platform::DingTalk => "dingtalk",
            Platform::WeChatWork => "wechatwork",
        }
    }
}

/// Tool category for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Calendar,
    Bitable,
    Messaging,
    Workflow,
    User,
    Customer,
    Department,
    Unknown,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Calendar => "calendar",
            Category::Bitable => "bitable",
            Category::Messaging => "messaging",
            Category::Workflow => "workflow",
            Category::User => "user",
            Category::Customer => "customer",
            Category::Department => "department",
            Category::Unknown => "unknown",
        }
    }
}

/// Result type for schema registry operations
#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("tool not found: {0}")]
    ToolNotFound(String),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("connector error: {0}")]
    ConnectorError(String),
}

/// Unified tool schema representation
#[derive(Debug, Clone)]
pub struct ToolSchema {
    /// Tool name (without platform prefix)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// JSON Schema for input parameters
    pub input_schema: Value,
    /// Source platform
    pub platform: Platform,
    /// Tool category
    pub category: Category,
}

impl ToolSchema {
    /// Get fully qualified name: "{platform}_{name}"
    pub fn fully_qualified_name(&self) -> String {
        format!("{}_{}", self.platform.as_str(), self.name)
    }
}

/// Trait for connector tool registries
pub trait ToolSchemaRegistry: Send + Sync {
    /// Platform identifier
    fn platform(&self) -> Platform;

    /// All tools provided by this connector
    fn all_tools(&self) -> Vec<ToolSchema>;

    /// Get a specific tool by name
    fn get_tool(&self, name: &str) -> Option<ToolSchema> {
        self.all_tools().into_iter().find(|t| t.name == name)
    }
}

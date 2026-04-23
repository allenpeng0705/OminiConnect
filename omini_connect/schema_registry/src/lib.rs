//! # OminiConnect Schema Registry
//!
//! Centralized tool schema registry for OminiConnect MCP connectors.
//! Provides unified tool discovery, filtering, and documentation generation.
//!
//! ## Features
//!
//! - Unified tool schema across all connectors
//! - Filtering by platform, category, and name
//! - MCP tools/list response generation
//! - Documentation generation (Markdown, OpenAPI)
//!
//! ## Usage
//!
//! ```rust
//! use omini_connect_schema_registry::{SchemaRegistry, Platform, Category, ToolFilter};
//!
//! let registry = SchemaRegistry::new();
//! // registry.register(FeishuToolAdapter::new());
//!
//! // List all tools
//! let all_tools = registry.list_tools(None);
//!
//! // Filter by platform
//! let feishu_tools = registry.list_tools_by_platform(Platform::Feishu);
//!
//! // Filter by category
//! let messaging_tools = registry.list_tools_by_category(Category::Messaging);
//!
//! // Generate MCP response
//! let mcp_response = registry.to_mcp_tool_list(None);
//! ```

pub mod tool_trait;
pub mod models;
pub mod registry;
pub mod mcp;
pub mod docs;
pub mod adapters;

pub use tool_trait::{Platform, Category, ToolSchema, ToolSchemaRegistry, SchemaError};
pub use models::ToolFilter;
pub use registry::SchemaRegistry;
pub use mcp::McpToolListResponse;
pub use docs::DocGenerator;

#[cfg(feature = "all")]
impl SchemaRegistry {
    /// Create a registry with all connectors registered
    pub fn with_all_connectors() -> Self {
        let mut registry = Self::new();
        registry.register(adapters::FeishuToolAdapter::new());
        registry.register(adapters::DingTalkToolAdapter::new());
        registry.register(adapters::WeChatWorkToolAdapter::new());
        registry
    }
}

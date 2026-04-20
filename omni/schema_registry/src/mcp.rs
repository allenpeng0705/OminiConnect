//! MCP protocol response generation.

use serde_json::{json, Value};

use super::{SchemaRegistry, ToolFilter};

/// MCP tools/list response builder
pub struct McpToolListResponse {
    registry: SchemaRegistry,
    filter: Option<ToolFilter>,
}

impl McpToolListResponse {
    /// Create a new response builder
    pub fn new(registry: SchemaRegistry) -> Self {
        Self {
            registry,
            filter: None,
        }
    }

    /// Add a filter to the response
    pub fn with_filter(mut self, filter: ToolFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Build the JSON value for MCP tools/list response
    pub fn build(self) -> Value {
        let tools = self.registry.list_tools(self.filter);

        let tool_values: Vec<Value> = tools
            .into_iter()
            .map(|t| {
                json!({
                    "name": t.fully_qualified_name(),
                    "description": t.description,
                    "inputSchema": t.input_schema,
                })
            })
            .collect();

        json!({
            "tools": tool_values
        })
    }
}

impl SchemaRegistry {
    /// Generate MCP tools/list response
    pub fn to_mcp_tool_list(&self, filter: Option<ToolFilter>) -> Value {
        McpToolListResponse::new(self.clone())
            .with_filter(filter.unwrap_or_default())
            .build()
    }
}

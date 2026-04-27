//! Documentation generation for tool schemas.

use serde_json::{json, Value};

use super::{Category, Platform, SchemaRegistry};

/// Documentation generator for tool schemas
pub struct DocGenerator {
    registry: SchemaRegistry,
}

impl DocGenerator {
    /// Create a new documentation generator
    pub fn new(registry: SchemaRegistry) -> Self {
        Self { registry }
    }

    /// Generate markdown documentation for all tools
    pub fn generate_markdown(&self) -> String {
        let mut output = String::from("# OminiConnect Tool Registry\n\n");
        output.push_str(&format!(
            "Total tools: {}\n\n",
            self.registry.total_tool_count()
        ));

        // Group by platform
        for platform in [Platform::Feishu, Platform::DingTalk, Platform::WeChatWork] {
            let tools = self.registry.list_tools_by_platform(platform);
            if tools.is_empty() {
                continue;
            }

            output.push_str(&format!(
                "## {}\n\n",
                match platform {
                    Platform::Feishu => "Feishu (Lark)",
                    Platform::DingTalk => "DingTalk",
                    Platform::WeChatWork => "WeChat Work",
                }
            ));

            // Group by category
            let mut current_category: Option<Category> = None;
            for tool in tools {
                if current_category != Some(tool.category) {
                    current_category = Some(tool.category);
                    output.push_str(&format!("### {}\n\n", self.category_name(&tool.category)));
                }

                output.push_str(&format!(
                    "#### `{}`\n\n{}\n\n**Input Schema:**\n```json\n{}\n```\n\n",
                    tool.fully_qualified_name(),
                    tool.description,
                    serde_json::to_string_pretty(&tool.input_schema).unwrap_or_default()
                ));
            }
            output.push('\n');
        }

        output
    }

    /// Generate OpenAPI-style documentation
    pub fn generate_openapi(&self) -> Value {
        let mut paths = serde_json::Map::new();

        for tool in self.registry.list_tools(None) {
            let method = format!("{}_{}", tool.platform.as_str(), tool.name);
            paths.insert(
                format!("/tools/{}/{}", tool.platform.as_str(), tool.name),
                json!({
                    "post": {
                        "summary": tool.description,
                        "operationId": method,
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": tool.input_schema
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful response"
                            }
                        }
                    }
                }),
            );
        }

        json!({
            "openapi": "3.0.0",
            "info": {
                "title": "OminiConnect Tool Registry",
                "version": "0.1.0",
                "description": "Unified tool schemas for Feishu, DingTalk, and WeChat Work MCP connectors"
            },
            "paths": paths
        })
    }

    /// Generate a summary table
    pub fn generate_summary(&self) -> String {
        let mut output = String::from("# OminiConnect Tool Summary\n\n");

        output.push_str("| Platform | Category | Tool Name | Description |\n");
        output.push_str("|----------|----------|-----------|-------------|\n");

        for tool in self.registry.list_tools(None) {
            output.push_str(&format!(
                "| {} | {} | `{}` | {} |\n",
                self.platform_name(&tool.platform),
                self.category_name(&tool.category),
                tool.name,
                tool.description
            ));
        }

        output
    }

    fn platform_name(&self, platform: &Platform) -> &'static str {
        match platform {
            Platform::Feishu => "Feishu",
            Platform::DingTalk => "DingTalk",
            Platform::WeChatWork => "WeChat Work",
        }
    }

    fn category_name(&self, category: &Category) -> &'static str {
        match category {
            Category::Calendar => "Calendar",
            Category::Bitable => "Bitable",
            Category::Messaging => "Messaging",
            Category::Workflow => "Workflow",
            Category::User => "User",
            Category::Customer => "Customer",
            Category::Department => "Department",
            Category::Unknown => "Other",
        }
    }
}

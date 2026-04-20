//! Data models for the schema registry.

use super::{Category, Platform, ToolSchema};

/// Filter criteria for tool queries
#[derive(Debug, Clone, Default)]
pub struct ToolFilter {
    /// Filter by platform
    pub platform: Option<Platform>,
    /// Filter by category
    pub category: Option<Category>,
    /// Filter by name (case-insensitive contains)
    pub name_contains: Option<String>,
    /// Filter by fully qualified name (exact match)
    pub fully_qualified_name: Option<String>,
}

impl ToolFilter {
    /// Check if this filter matches the given tool
    pub fn matches(&self, tool: &ToolSchema) -> bool {
        if let Some(platform) = &self.platform {
            if &tool.platform != platform {
                return false;
            }
        }
        if let Some(category) = &self.category {
            if &tool.category != category {
                return false;
            }
        }
        if let Some(name) = &self.name_contains {
            if !tool.name.to_lowercase().contains(&name.to_lowercase()) {
                return false;
            }
        }
        if let Some(fqn) = &self.fully_qualified_name {
            if tool.fully_qualified_name() != *fqn {
                return false;
            }
        }
        true
    }
}

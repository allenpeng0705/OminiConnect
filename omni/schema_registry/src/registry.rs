//! Centralized schema registry implementation.

use std::collections::HashMap;
use std::sync::Arc;

use super::{Platform, ToolSchema, ToolSchemaRegistry, Category};
use super::models::ToolFilter;

/// Centralized schema registry for all connectors
pub struct SchemaRegistry {
    adapters: HashMap<Platform, Arc<dyn ToolSchemaRegistry>>,
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Register a connector adapter
    pub fn register<T: ToolSchemaRegistry + 'static>(&mut self, adapter: T) -> &mut Self {
        self.adapters.insert(adapter.platform(), Arc::new(adapter));
        self
    }

    /// List all registered platforms
    pub fn platforms(&self) -> Vec<Platform> {
        self.adapters.keys().cloned().collect()
    }

    /// Get all tools, optionally filtered
    pub fn list_tools(&self, filter: Option<ToolFilter>) -> Vec<ToolSchema> {
        let filter = filter.unwrap_or_default();

        self.adapters
            .values()
            .flat_map(|adapter| adapter.all_tools())
            .filter(|tool| filter.matches(tool))
            .collect()
    }

    /// Get tools for a specific platform
    pub fn list_tools_by_platform(&self, platform: Platform) -> Vec<ToolSchema> {
        self.list_tools(Some(ToolFilter {
            platform: Some(platform),
            ..Default::default()
        }))
    }

    /// Get tools by category across all platforms
    pub fn list_tools_by_category(&self, category: Category) -> Vec<ToolSchema> {
        self.list_tools(Some(ToolFilter {
            category: Some(category),
            ..Default::default()
        }))
    }

    /// Find a specific tool by fully qualified name
    pub fn find_tool(&self, fully_qualified_name: &str) -> Option<ToolSchema> {
        // Parse platform from fqn (e.g., "feishu_calendar_list")
        let parts: Vec<&str> = fully_qualified_name.splitn(2, '_').collect();
        if parts.len() != 2 {
            return None;
        }

        let platform = match parts[0] {
            "feishu" => Platform::Feishu,
            "dingtalk" => Platform::DingTalk,
            "wechatwork" => Platform::WeChatWork,
            _ => return None,
        };

        let tool_name = parts[1];

        self.adapters
            .get(&platform)
            .and_then(|adapter| adapter.get_tool(tool_name))
    }

    /// Get tool count by platform
    pub fn tool_count_by_platform(&self) -> HashMap<Platform, usize> {
        self.adapters
            .iter()
            .map(|(platform, adapter)| (*platform, adapter.all_tools().len()))
            .collect()
    }

    /// Get tool count by category
    pub fn tool_count_by_category(&self) -> HashMap<Category, usize> {
        let mut counts: HashMap<Category, usize> = HashMap::new();
        for adapter in self.adapters.values() {
            for tool in adapter.all_tools() {
                *counts.entry(tool.category).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Total number of registered tools
    pub fn total_tool_count(&self) -> usize {
        self.adapters.values().map(|a| a.all_tools().len()).sum()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SchemaRegistry {
    fn clone(&self) -> Self {
        Self {
            adapters: self.adapters.clone(),
        }
    }
}

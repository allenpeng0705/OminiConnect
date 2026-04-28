//! Tool registry loader.
//!
//! Loads all tools from the YAML registry files and builds
//! the LLM tool list format for testing.

use serde::Deserialize;
use std::collections::HashMap;

/// A tool definition from the registry.
#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub endpoint: String,
    #[serde(rename = "method")]
    pub http_method: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub example_queries: Vec<String>,
    #[serde(default = "default_input_schema")]
    pub input_schema: serde_json::Value,
}

fn default_input_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {},
        "required": []
    })
}

impl Tool {
    /// Richer description helps reduce near-neighbor tool confusion.
    pub fn llm_description(&self) -> String {
        let mut parts = vec![
            format!("{}: {}", self.name, self.description.replace('\n', " ").trim()),
            format!("method={}", self.http_method),
            format!("endpoint={}", self.endpoint),
        ];

        if !self.tags.is_empty() {
            parts.push(format!("tags={}", self.tags.join(",")));
        }
        if !self.example_queries.is_empty() {
            parts.push(format!(
                "examples={}",
                self.example_queries
                    .iter()
                    .take(2)
                    .map(|q| q.replace('\n', " "))
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }

        parts.join(" ; ")
    }
}

/// Load all tools from registry directory.
pub fn load_all_tools(registry_dir: &std::path::Path) -> anyhow::Result<Vec<Tool>> {
    let mut all_tools = Vec::new();

    for entry in std::fs::read_dir(registry_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let content = std::fs::read_to_string(&path)?;
        let tools: Vec<Tool> = serde_yaml::from_str(&content)?;
        all_tools.extend(tools);
    }

    Ok(all_tools)
}

/// Build LLM tool format for passing to LLM API.
pub fn build_llm_tools(tools: &[Tool]) -> Vec<serde_json::Value> {
    tools
        .iter()
        .map(|tool| {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": tool.slug,
                    "description": tool.llm_description(),
                    "parameters": tool.input_schema
                }
            })
        })
        .collect()
}

/// Group tools by provider.
pub fn tools_by_provider(tools: &[Tool]) -> HashMap<String, Vec<&Tool>> {
    let mut map: HashMap<String, Vec<&Tool>> = HashMap::new();
    for tool in tools {
        map.entry(tool.provider.clone()).or_default().push(tool);
    }
    map
}

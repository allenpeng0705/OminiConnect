//! Tool registry for LLM-accessible APIs.
//!
//! Tools are defined in YAML files and loaded at startup. Each tool maps to a
//! provider API endpoint that can be executed via the proxy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// HTTP methods supported by tools.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

/// Input schema for tool parameters (stored as raw JSON Schema).
/// The schema_type is always "object" for tool parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InputSchema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub description: Option<String>,
    /// Parameter definitions.
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    /// Required parameter names.
    #[serde(default)]
    pub required: Vec<String>,
}

/// A single tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Unique slug, e.g. "github_list_repos"
    pub slug: String,
    /// Human-readable name, e.g. "List Repositories"
    pub name: String,
    /// Description of what the tool does (for LLM selection).
    pub description: String,
    /// Provider key / proxy platform, e.g. "github"
    pub provider: String,
    /// API endpoint path, e.g. "/user/repos". May contain {path_params}.
    pub endpoint: String,
    /// HTTP method.
    #[serde(default)]
    pub method: HttpMethod,
    /// JSON Schema for input parameters.
    #[serde(default)]
    pub input_schema: InputSchema,
    /// JSON Schema for response output (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    /// Required OAuth scopes for this tool.
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Tags for categorization / search.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional icon URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

/// A toolkit groups tools by provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toolkit {
    pub slug: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Matches tool.provider / proxy platform.
    pub provider: String,
}

/// Registry containing all loaded tools and toolkits.
#[derive(Debug, Clone)]
pub struct ToolRegistry {
    /// Tools indexed by slug.
    tools_by_slug: HashMap<String, Tool>,
    /// Tools indexed by provider.
    tools_by_provider: HashMap<String, Vec<Tool>>,
    /// All toolkits.
    toolkits: Vec<Toolkit>,
}

impl ToolRegistry {
    /// Create an empty registry (used when loading fails).
    pub fn empty() -> Self {
        Self {
            tools_by_slug: HashMap::new(),
            tools_by_provider: HashMap::new(),
            toolkits: Vec::new(),
        }
    }

    /// Load all YAML files from the given directory.
    pub fn load_from_dir(dir: &Path) -> Result<Self, LoadError> {
        let mut tools_by_slug = HashMap::new();
        let mut tools_by_provider: HashMap<String, Vec<Tool>> = HashMap::new();
        let mut toolkits: Vec<Toolkit> = Vec::new();
        let mut seen_providers: std::collections::HashSet<String> = std::collections::HashSet::new();

        let entries = std::fs::read_dir(dir)
            .map_err(|e| LoadError::ReadDir(dir.display().to_string(), e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }

            let yaml_content = std::fs::read_to_string(&path)
                .map_err(|e| LoadError::ReadFile(path.display().to_string(), e))?;

            let provider_tools: Vec<Tool> = serde_yaml::from_str(&yaml_content)
                .map_err(|e| LoadError::Parse(path.display().to_string(), e))?;

            for tool in provider_tools {
                let slug = tool.slug.clone();
                let provider = tool.provider.clone();

                // Validate slug uniqueness
                if tools_by_slug.contains_key(&slug) {
                    tracing::warn!("Duplicate tool slug '{}', skipping", slug);
                    continue;
                }

                tools_by_slug.insert(slug.clone(), tool.clone());
                tools_by_provider
                    .entry(provider.clone())
                    .or_default()
                    .push(tool);

                // Create toolkit entry if new provider
                if seen_providers.insert(provider.clone()) {
                    toolkits.push(Toolkit {
                        slug: provider.clone(),
                        name: Self::provider_display_name(&provider),
                        logo: Some(format!("/images/template-logos/{}.svg", provider)),
                        provider: provider.clone(),
                    });
                }
            }
        }

        Ok(Self {
            tools_by_slug,
            tools_by_provider,
            toolkits,
        })
    }

    /// Returns all toolkits.
    pub fn toolkits(&self) -> &[Toolkit] {
        &self.toolkits
    }

    /// Returns tools for a specific provider.
    pub fn tools_for_provider(&self, provider: &str) -> Option<&[Tool]> {
        self.tools_by_provider.get(provider).map(|v| v.as_slice())
    }

    /// Returns a tool by slug.
    pub fn tool_by_slug(&self, slug: &str) -> Option<&Tool> {
        self.tools_by_slug.get(slug)
    }

    /// Returns all tool slugs (for search).
    pub fn all_slugs(&self) -> impl Iterator<Item = &str> {
        self.tools_by_slug.keys().map(|s| s.as_str())
    }

    /// Provider display name helper.
    fn provider_display_name(provider: &str) -> String {
        match provider {
            "github" => "GitHub".to_string(),
            "slack" => "Slack".to_string(),
            "notion" => "Notion".to_string(),
            "google" => "Google".to_string(),
            "gitlab" => "GitLab".to_string(),
            "jira" => "Jira".to_string(),
            "salesforce" => "Salesforce".to_string(),
            "hubspot" => "HubSpot".to_string(),
            other => other.replace('-', " ").replace('_', " ")
                .split_whitespace()
                .map(|w| {
                    let mut chars = w.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

/// Errors that can occur loading the tool registry.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("failed to read directory {0}: {1}")]
    ReadDir(String, std::io::Error),
    #[error("failed to read file {0}: {1}")]
    ReadFile(String, std::io::Error),
    #[error("failed to parse {0}: {1}")]
    Parse(String, serde_yaml::Error),
}

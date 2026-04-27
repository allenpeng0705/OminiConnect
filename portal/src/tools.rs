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
    /// Example natural language queries that map to this tool (for LLM selection).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub example_queries: Vec<String>,
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
        let mut seen_providers: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        let entries =
            std::fs::read_dir(dir).map_err(|e| LoadError::ReadDir(dir.display().to_string(), e))?;

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
            other => other
                .replace('-', " ")
                .replace('_', " ")
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

// ─── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tool(slug: &str, provider: &str, scopes: Vec<&str>) -> Tool {
        Tool {
            slug: slug.to_string(),
            name: slug.to_string(),
            description: "test tool".to_string(),
            provider: provider.to_string(),
            endpoint: "/test".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema::default(),
            output_schema: None,
            scopes: scopes.into_iter().map(String::from).collect(),
            tags: vec![],
            icon_url: None,
            example_queries: vec![],
        }
    }

    #[test]
    fn test_tool_registry_empty() {
        let reg = ToolRegistry::empty();
        assert!(reg.toolkits().is_empty());
        assert!(reg.tool_by_slug("anything").is_none());
        assert!(reg.tools_for_provider("github").is_none());
        assert!(reg.all_slugs().next().is_none());
    }

    #[test]
    fn test_http_method_default() {
        assert_eq!(HttpMethod::default(), HttpMethod::GET);
    }

    #[test]
    fn test_http_method_serde() {
        let json = serde_json::to_string(&HttpMethod::GET).unwrap();
        assert_eq!(json, "\"GET\"");
        let json_post = serde_json::to_string(&HttpMethod::POST).unwrap();
        assert_eq!(json_post, "\"POST\"");
    }

    #[test]
    fn test_tool_serde_roundtrip() {
        let tool = make_tool("github_list_repos", "github", vec!["repo"]);
        let json = serde_json::to_string(&tool).unwrap();
        let decoded: Tool = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.slug, "github_list_repos");
        assert_eq!(decoded.provider, "github");
        assert_eq!(decoded.scopes, vec!["repo"]);
    }

    #[test]
    fn test_toolkit_serde() {
        let tk = Toolkit {
            slug: "github".to_string(),
            name: "GitHub".to_string(),
            logo: Some("/images/logo.svg".to_string()),
            provider: "github".to_string(),
        };
        let json = serde_json::to_string(&tk).unwrap();
        let decoded: Toolkit = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.slug, "github");
        assert_eq!(decoded.name, "GitHub");
    }

    #[test]
    fn test_input_schema_default() {
        let schema = InputSchema::default();
        assert!(schema.schema_type.is_none());
        assert!(schema.properties.is_empty());
        assert!(schema.required.is_empty());
    }

    #[test]
    fn test_tool_creation_all_fields() {
        let tool = Tool {
            slug: "github_create_issue".to_string(),
            name: "Create Issue".to_string(),
            description: "Create a GitHub issue".to_string(),
            provider: "github".to_string(),
            endpoint: "/repos/{owner}/{repo}/issues".to_string(),
            method: HttpMethod::POST,
            input_schema: InputSchema {
                schema_type: Some("object".to_string()),
                description: Some("issue parameters".to_string()),
                properties: HashMap::new(),
                required: vec!["title".to_string()],
            },
            output_schema: None,
            scopes: vec!["repo".to_string()],
            tags: vec!["github".to_string(), "issues".to_string()],
            icon_url: None,
            example_queries: vec![
                "create a github issue".to_string(),
                "open an issue".to_string(),
            ],
        };

        assert_eq!(tool.slug, "github_create_issue");
        assert_eq!(tool.method, HttpMethod::POST);
        assert!(tool.input_schema.required.contains(&"title".to_string()));
        assert_eq!(tool.scopes, vec!["repo"]);
    }

    #[test]
    fn test_tool_http_method_variants() {
        for (method_str, method) in [
            ("GET", HttpMethod::GET),
            ("POST", HttpMethod::POST),
            ("PUT", HttpMethod::PUT),
            ("DELETE", HttpMethod::DELETE),
            ("PATCH", HttpMethod::PATCH),
        ] {
            let json = serde_json::to_string(&method).unwrap();
            assert_eq!(json, format!("\"{}\"", method_str));
        }
    }

    #[test]
    fn test_load_error_display() {
        use std::io;
        let err = LoadError::ReadDir(
            "/tmp".to_string(),
            io::Error::new(io::ErrorKind::NotFound, "no such file"),
        );
        assert!(err.to_string().contains("failed to read directory"));
        let err2 = LoadError::ReadFile(
            "/tmp/test.yaml".to_string(),
            io::Error::new(io::ErrorKind::NotFound, "no such file"),
        );
        assert!(err2.to_string().contains("failed to read file"));
    }

    #[test]
    fn test_tool_registry_all_slugs_empty() {
        let reg = ToolRegistry::empty();
        let slugs: Vec<&str> = reg.all_slugs().collect();
        assert!(slugs.is_empty());
    }

    #[test]
    fn test_tool_registry_toolkits_empty() {
        let reg = ToolRegistry::empty();
        assert!(reg.toolkits().is_empty());
    }
}

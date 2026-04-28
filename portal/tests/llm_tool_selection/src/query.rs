//! Query-Tool dataset definition.
//!
//! Each query has an expected tool slug and metadata about the test case.

use serde::Deserialize;

/// A single test query with expected tool.
#[derive(Debug, Clone, Deserialize)]
pub struct QueryCase {
    /// Natural language query from user.
    pub query: String,
    /// Expected tool slug that should be selected.
    pub expected_tool: String,
    /// Which provider this query targets.
    #[serde(default)]
    pub provider: Option<String>,
    /// Category of the query (e.g., "list", "create", "search").
    pub category: String,
    /// Variations of the same query for robustness testing.
    #[serde(default)]
    pub variations: Vec<String>,
    /// Whether this query is intentionally ambiguous (should return candidates).
    #[serde(default)]
    pub ambiguous: bool,
    /// Notes for human reviewers about unclear descriptions.
    #[serde(default)]
    pub description_notes: Option<String>,
}

/// Full query dataset for a provider.
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderQueries {
    pub provider: String,
    pub queries: Vec<QueryCase>,
}

/// Load all query datasets from a directory.
pub fn load_datasets(dir: &std::path::Path) -> anyhow::Result<Vec<ProviderQueries>> {
    let mut datasets = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let content = std::fs::read_to_string(&path)?;
        let dataset: ProviderQueries = serde_yaml::from_str(&content)?;
        datasets.push(dataset);
    }

    Ok(datasets)
}

impl QueryCase {
    /// Generate all variations of this query (original + variations).
    pub fn all_queries(&self) -> Vec<String> {
        let mut queries = vec![self.query.clone()];
        queries.extend(self.variations.clone());
        queries
    }
}

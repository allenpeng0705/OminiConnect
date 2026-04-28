//! Evaluation logic for LLM tool selection.
//!
//! Compares LLM's selected tool against expected and categorizes results.

use serde::Serialize;
use super::query::QueryCase;

/// Result of a single query test.
#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub query: String,
    pub expected_tool: String,
    pub llm_selected_tool: Option<String>,
    pub is_correct: bool,
    pub is_ambiguous_expected: bool,
    pub error_message: Option<String>,
    pub provider: Option<String>,
    pub category: String,
}

/// Category of failure for analysis.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum FailureCategory {
    /// LLM selected wrong tool.
    WrongTool,
    /// LLM returned no tool when one was expected.
    NoToolReturned,
    /// LLM returned ambiguous/candidates when we expected a specific tool.
    UnexpectedAmbiguity,
    /// LLM error (network, parse, etc).
    LlmError,
    /// Tool description was unclear.
    UnclearDescription,
    /// Other.
    Other,
}

impl std::fmt::Display for FailureCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FailureCategory::WrongTool => write!(f, "WrongTool"),
            FailureCategory::NoToolReturned => write!(f, "NoToolReturned"),
            FailureCategory::UnexpectedAmbiguity => write!(f, "UnexpectedAmbiguity"),
            FailureCategory::LlmError => write!(f, "LlmError"),
            FailureCategory::UnclearDescription => write!(f, "UnclearDescription"),
            FailureCategory::Other => write!(f, "Other"),
        }
    }
}

/// A failed test case with context for debugging.
#[derive(Debug, Clone, Serialize)]
pub struct FailedCase {
    pub query: String,
    pub expected_tool: String,
    pub llm_response: String,
    pub available_tools_sample: Vec<String>,
    pub failure_category: String,
    pub notes: Option<String>,
}

/// Evaluate a single query result.
pub fn evaluate_query(
    query: &QueryCase,
    llm_response: &str,
) -> QueryResult {
    // Try to parse LLM response - expect a tool call in JSON format
    // Format: {"tool": "slug_name", "arguments": {...}}

    let selected_tool = extract_tool_from_response(llm_response);

    let is_correct = selected_tool.as_ref().map(|t| t == &query.expected_tool).unwrap_or(false);
    let is_ambiguous_expected = query.ambiguous;

    QueryResult {
        query: query.query.clone(),
        expected_tool: query.expected_tool.clone(),
        llm_selected_tool: selected_tool,
        is_correct,
        is_ambiguous_expected,
        error_message: None,
        provider: query.provider.clone(),
        category: query.category.clone(),
    }
}

/// Extract tool name from LLM JSON response.
fn extract_tool_from_response(response: &str) -> Option<String> {
    // Try JSON format first: {"tool": "slug", ...}
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response) {
        if let Some(tool) = parsed.get("tool").or_else(|| parsed.get("name")) {
            if let Some(s) = tool.as_str() {
                return Some(s.to_string());
            }
        }
    }

    // Try OpenAI tool_calls format
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response) {
        if let Some(choices) = parsed.get("choices").and_then(|c| c.as_array()) {
            for choice in choices {
                if let Some(tool_calls) = choice.get("message").and_then(|m| m.get("tool_calls")) {
                    if let Some(calls) = tool_calls.as_array() {
                        for call in calls {
                            if let Some(function) = call.get("function") {
                                if let Some(name) = function.get("name").and_then(|n| n.as_str()) {
                                    return Some(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback: try to find a slug-like pattern in text
    // e.g., "github_list_repos" or "I should call github_list_repos"
    for word in response.split_whitespace() {
        if word.contains('_') && !word.starts_with('{') {
            return Some(word.trim_matches(|c| c == ',' || c == '.' || c == '"' || c == '}' || c == '(' || c == ')' || c == '\'' || c == '`').to_string());
        }
    }

    None
}

/// Summarize results by category.
pub fn summarize_results(results: &[QueryResult]) -> Summary {
    let total = results.len();
    let correct = results.iter().filter(|r| r.is_correct).count();
    let incorrect: Vec<_> = results.iter().filter(|r| !r.is_correct).collect();

    let mut by_category: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new();
    for r in results {
        let entry = by_category.entry(r.category.clone()).or_insert((0, 0));
        entry.0 += 1;
        if r.is_correct {
            entry.1 += 1;
        }
    }

    let mut by_provider: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new();
    for r in results {
        if let Some(provider) = &r.provider {
            let entry = by_provider.entry(provider.clone()).or_insert((0, 0));
            entry.0 += 1;
            if r.is_correct {
                entry.1 += 1;
            }
        }
    }

    Summary {
        total,
        correct,
        accuracy: if total > 0 { correct as f64 / total as f64 } else { 0.0 },
        by_category,
        by_provider,
        incorrect_sample: incorrect.iter().take(10).map(|r| (*r).clone()).collect(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub total: usize,
    pub correct: usize,
    pub accuracy: f64,
    pub by_category: std::collections::HashMap<String, (usize, usize)>,
    pub by_provider: std::collections::HashMap<String, (usize, usize)>,
    pub incorrect_sample: Vec<QueryResult>,
}

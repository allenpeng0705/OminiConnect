//! Reporter for test results.
//!
//! Records failed cases and unclear descriptions to files
//! for later analysis and improvement.

use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use super::evaluator::{QueryResult, FailureCategory};

/// A failure record for debugging and improvement.
#[derive(Debug, Clone, Serialize)]
pub struct FailureRecord {
    pub query: String,
    pub expected_tool: String,
    pub llm_selected_tool: Option<String>,
    pub failure_type: String,
    pub tool_descriptions: HashMap<String, String>,  // slug -> description
    pub notes: Option<String>,
}

/// Records with unclear tool descriptions that need improvement.
#[derive(Debug, Clone, Serialize)]
pub struct UnclearDescriptionRecord {
    pub tool_slug: String,
    pub provider: String,
    pub current_description: String,
    pub suggested_improvement: Option<String>,
    pub affected_queries: Vec<String>,  // queries that failed due to this
}

pub struct Reporter {
    failures: Vec<FailureRecord>,
    unclear_descriptions: Vec<UnclearDescriptionRecord>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            unclear_descriptions: Vec::new(),
        }
    }

    /// Record a failed case.
    pub fn record_failure(
        &mut self,
        result: &QueryResult,
        tool_descriptions: HashMap<String, String>,
        failure_type: FailureCategory,
        notes: Option<String>,
    ) {
        self.failures.push(FailureRecord {
            query: result.query.clone(),
            expected_tool: result.expected_tool.clone(),
            llm_selected_tool: result.llm_selected_tool.clone(),
            failure_type: failure_type.to_string(),
            tool_descriptions,
            notes,
        });
    }

    /// Record a tool with unclear description.
    pub fn record_unclear_description(&mut self, record: UnclearDescriptionRecord) {
        self.unclear_descriptions.push(record);
    }

    /// Write failures to a JSON file.
    pub fn write_failures(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self.failures)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Write unclear descriptions to a YAML file.
    pub fn write_unclear_descriptions(&self, path: &Path) -> anyhow::Result<()> {
        let yaml = serde_yaml::to_string(&self.unclear_descriptions)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// Print summary to console.
    pub fn print_summary(&self, summary: &super::evaluator::Summary) {
        println!("\n{}", "=".repeat(80));
        println!("LLM TOOL SELECTION TEST SUMMARY");
        println!("{}", "=".repeat(80));
        println!("Total queries: {}", summary.total);
        println!("Correct selections: {}", summary.correct);
        println!("Accuracy: {:.1}%", summary.accuracy * 100.0);
        println!("\nFailed cases recorded: {}", self.failures.len());
        println!("Unclear descriptions recorded: {}", self.unclear_descriptions.len());
        println!("{}", "=".repeat(80));
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}

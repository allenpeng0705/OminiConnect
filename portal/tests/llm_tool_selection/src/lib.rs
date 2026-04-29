//! LLM Tool Selection Test Suite
//!
//! Tests whether an LLM can correctly select tools from a registry
//! based on natural language queries.
//!
//! ## Usage
//!
//! ```bash
//! # Run against full registry with default LLM
//! cargo run -- --registry ./tools/registry --queries ./queries
//!
//! # Run with custom LLM endpoint
//! cargo run -- --registry ./tools/registry --queries ./queries --llm-url http://localhost:4000
//!
//! # Run with specific providers only (comma-separated)
//! cargo run -- --registry ./tools/registry --queries ./queries --providers github,slack,notion
//!
//! # Output results to files
//! cargo run -- --registry ./tools/registry --queries ./queries --output ./test-results
//! ```

pub mod query;
pub mod registry;
pub mod evaluator;
pub mod reporter;

pub use query::QueryCase;
pub use registry::{load_all_tools, build_llm_tools, tools_by_provider};
pub use evaluator::{evaluate_query, summarize_results, Summary, QueryResult};
pub use reporter::Reporter;

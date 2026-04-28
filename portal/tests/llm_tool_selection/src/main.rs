//! LLM Tool Selection Test Runner
//!
//! Tests LLM's ability to select the correct tool from a noisy tool set.
//! The scale determines how many "noise" (incorrect) tools are injected.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use llm_tool_selection_test::{
    query, registry, evaluator,
    load_all_tools,
    evaluate_query, summarize_results,
    QueryCase, QueryResult, Reporter,
};

/// Test scale - controls how many noise (incorrect) tools are injected.
/// The correct tool + noise tools = total tool set presented to LLM.
#[derive(Debug, Clone, Copy, Default)]
pub enum TestScale {
    /// Inject 5-10 noise tools (easy selection)
    Small,
    /// Inject 15-30 noise tools (moderate difficulty)
    #[default]
    Medium,
    /// Inject 50-100 noise tools (challenging)
    Large,
    /// Inject ALL tools as noise (maximum difficulty)
    Full,
}

impl std::str::FromStr for TestScale {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "small" => Ok(TestScale::Small),
            "medium" => Ok(TestScale::Medium),
            "large" => Ok(TestScale::Large),
            "full" => Ok(TestScale::Full),
            _ => Err(format!("Unknown scale: {}", s)),
        }
    }
}

impl TestScale {
    /// Number of noise tools to inject (approximate).
    pub fn noise_count(&self) -> usize {
        match self {
            TestScale::Small => 30,
            TestScale::Medium => 100,
            TestScale::Large => 300,
            TestScale::Full => usize::MAX, // All remaining tools
        }
    }

    /// Seed for reproducibility.
    pub fn seed(&self) -> u64 {
        match self {
            TestScale::Small => 42,
            TestScale::Medium => 123,
            TestScale::Large => 456,
            TestScale::Full => 789,
        }
    }
}

/// CLI Arguments
#[derive(clap::Parser, Debug)]
#[command(name = "llm-tool-test")]
#[command(about = "Test LLM tool selection accuracy with noise injection")]
struct Args {
    /// Path to tools registry directory.
    #[arg(long, default_value = "./tools/registry")]
    registry: PathBuf,

    /// Path to query datasets directory.
    #[arg(long, default_value = "./queries")]
    queries: PathBuf,

    /// Test scale: small (10 noise), medium (30), large (100), full (all).
    #[arg(long, value_parser = clap::value_parser!(TestScale), default_value = "medium")]
    scale: TestScale,

    /// Number of test rounds to run (different noise samples each round).
    #[arg(long, default_value = "3")]
    rounds: usize,

    /// LLM endpoint URL (LiteLLM compatible).
    #[arg(long, default_value = "http://localhost:4000")]
    llm_url: String,

    /// API key for LLM.
    #[arg(long)]
    llm_api_key: Option<String>,

    /// Filter to specific provider only.
    #[arg(long)]
    provider: Option<String>,

    /// Output directory for results.
    #[arg(long)]
    output: Option<PathBuf>,

    /// Model to use.
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,

    /// Run only multi-round tests.
    #[arg(long)]
    multi_round_only: bool,

    /// Delay between requests in milliseconds (to avoid rate limiting).
    #[arg(long, default_value = "0")]
    delay_ms: u64,

    /// Maximum number of queries to run (for quick verification). 0 = unlimited.
    #[arg(long, default_value = "0")]
    max_queries: usize,
}

/// Multi-round test scenario.
#[derive(Debug)]
struct MultiRoundScenario {
    name: String,
    steps: Vec<MultiRoundStep>,
}

#[derive(Debug)]
struct MultiRoundStep {
    query: String,
    expected_tool: String,
    /// Field from previous step's result to extract for next step.
    extract_field: Option<String>,
}

/// Build a noisy tool set: correct tool + random noise tools.
fn build_noisy_tool_set<'a>(
    correct_tool: &'a registry::Tool,
    all_tools: &'a [registry::Tool],
    noise_count: usize,
    rng: &mut impl Rng,
) -> Vec<&'a registry::Tool> {
    let mut tool_set = vec![correct_tool]; // Start with correct tool

    // Collect candidates: all tools except the correct one
    let mut candidates: Vec<_> = all_tools.iter()
        .filter(|t| t.slug != correct_tool.slug)
        .collect();

    // Shuffle and take noise_count
    candidates.shuffle(rng);

    let actual_noise = noise_count.min(candidates.len());
    for tool in candidates.into_iter().take(actual_noise) {
        tool_set.push(tool);
    }

    tool_set
}

/// Run a single query against the LLM with retry logic.
async fn run_query(
    client: &reqwest::Client,
    llm_url: &str,
    api_key: Option<&str>,
    model: &str,
    tools: &[serde_json::Value],
    query: &str,
) -> Result<String> {
    let system_prompt = r#"You are a helpful assistant that selects the appropriate tool to help users.
You have access to tools that can call APIs on behalf of the user.
Select the ONE best tool for the user's request.

When you select a tool, respond with ONLY a JSON object:
{"tool": "tool_slug_name", "arguments": {}}

Do not include any other text in your response."#;

    let messages = serde_json::json!([
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": query}
    ]);

    let body = serde_json::json!({
        "model": model,
        "messages": messages,
        "tools": tools,
        "stream": false
    });

    // Retry logic for connection errors
    let max_retries = 3;
    let mut last_error = None;

    for attempt in 0..max_retries {
        let mut req = client.post(format!("{}/chat/completions", llm_url))
            .header("Content-Type", "application/json")
            .json(&body);

        if let Some(key) = api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        match req.send().await {
            Ok(resp) => {
                // Check for HTTP errors
                if !resp.status().is_success() {
                    let status = resp.status();
                    let body_text = resp.text().await.unwrap_or_default();
                    eprintln!("DEBUG: HTTP {} body: {:?}", status, body_text);
                    // For 503/429, retry with backoff
                    if (status.as_u16() == 503 || status.as_u16() == 429) && attempt < max_retries - 1 {
                        let delay = Duration::from_millis(500 * 2_u64.pow(attempt as u32));
                        tokio::time::sleep(delay).await;
                        last_error = Some(anyhow::anyhow!("HTTP {}: {}", status, body_text));
                        continue;
                    }
                    return Err(anyhow::anyhow!("HTTP error {}: {}", status, body_text));
                }

                let json: serde_json::Value = resp.json().await?;

                // Check for API errors in response
                if let Some(error) = json.get("error") {
                    return Err(anyhow::anyhow!("API error: {:?}", error));
                }

                // Extract response content from message.content or tool_calls
                if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                    if let Some(first) = choices.first() {
                        // Try message.content first
                        if let Some(content) = first.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                            if !content.is_empty() {
                                return Ok(content.to_string());
                            }
                        }
                        // Try tool_calls
                        if let Some(tool_calls) = first.get("message").and_then(|m| m.get("tool_calls")) {
                            if let Some(calls) = tool_calls.as_array() {
                                if let Some(call) = calls.first() {
                                    if let Some(func) = call.get("function") {
                                        if let Some(name) = func.get("name") {
                                            if let Some(name_str) = name.as_str() {
                                                // Return tool call in JSON format
                                                let arguments = func.get("arguments")
                                                    .map(|a| a.to_string())
                                                    .unwrap_or_else(|| "{}".to_string());
                                                return Ok(format!(r#"{{"tool": "{}", "arguments": {}}}"#, name_str, arguments));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                return Ok(String::new());
            }
            Err(e) => {
                last_error = Some(e.into());
                if attempt < max_retries - 1 {
                    let delay = Duration::from_millis(500 * 2_u64.pow(attempt as u32));
                    tokio::time::sleep(delay).await
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Request failed after retries")))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("Loading tools from {}...", args.registry.display());
    let all_tools = load_all_tools(&args.registry)?;
    println!("Loaded {} tools total", all_tools.len());

    // Load query datasets
    println!("Loading queries from {}...", args.queries.display());
    let datasets = query::load_datasets(&args.queries)?;
    println!("Loaded {} provider datasets", datasets.len());

    // Setup HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Filter datasets by provider if specified
    let filtered_datasets: Vec<_> = match &args.provider {
        Some(p) => datasets.iter().filter(|d| d.provider == *p).collect(),
        None => datasets.iter().collect(),
    };

    // Run multiple rounds with different noise samples
    let mut all_results = Vec::new();
    let mut reporter = Reporter::new();
    let mut total_queries = 0;

    for round in 1..=args.rounds {
        println!("\n{}", "=".repeat(80));
        println!("ROUND {} / {} (scale: {:?}, noise: ~{})",
            round, args.rounds, args.scale, args.scale.noise_count());
        println!("{}", "=".repeat(80));

        // Use different seed per round for different noise selection
        let mut rng = rand::rngs::StdRng::seed_from_u64(args.scale.seed() + round as u64);

        let mut round_results = Vec::new();

        for dataset in &filtered_datasets {
            println!("\nProvider: {}", dataset.provider);

            // Get all tools for this provider
            let provider_tools: Vec<_> = all_tools.iter()
                .filter(|t| t.provider == dataset.provider)
                .collect();

            if provider_tools.is_empty() {
                eprintln!("  Warning: no tools found for provider {}", dataset.provider);
                continue;
            }

            for query_case in &dataset.queries {
                // Find the correct tool
                let correct_tool = provider_tools.iter()
                    .find(|t| t.slug == query_case.expected_tool)
                    .copied();

                let correct_tool = match correct_tool {
                    Some(t) => t,
                    None => {
                        eprintln!("  Warning: expected tool {} not found in registry", query_case.expected_tool);
                        continue;
                    }
                };

                // Build noisy tool set
                let noisy_tools = build_noisy_tool_set(
                    correct_tool,
                    &all_tools,
                    args.scale.noise_count(),
                    &mut rng,
                );

                // Test all variations
                for query_text in query_case.all_queries() {
                    // Build LLM tool list from noisy set
                    let llm_tools: Vec<_> = noisy_tools.iter()
                        .map(|t| {
                            serde_json::json!({
                                "type": "function",
                                "function": {
                                    "name": t.slug,
                                    "description": format!("{} - {}", t.name, t.description.replace('\n', " ")),
                                    "parameters": {"type": "object", "properties": {}, "required": []}
                                }
                            })
                        })
                        .collect();

                    print!("  [{} tools] {}... ", llm_tools.len(), &query_text[..query_text.len().min(35)]);

                    match run_query(
                        &client,
                        &args.llm_url,
                        args.llm_api_key.as_deref(),
                        &args.model,
                        &llm_tools,
                        &query_text,
                    ).await {
                        Ok(response) => {
                            let result = evaluate_query(query_case, &response);
                            let noise_tools: Vec<_> = noisy_tools.iter()
                                .filter(|t| t.slug != query_case.expected_tool)
                                .map(|t| t.slug.clone())
                                .collect();

                            if result.is_correct {
                                println!("✓ (noise: {})", noise_tools.len());
                            } else {
                                println!("✗ expected: {}, got: {:?} (noise: {})",
                                    query_case.expected_tool,
                                    result.llm_selected_tool,
                                    noise_tools.len());

                                // Build description map for reporting
                                let desc_map: HashMap<String, String> = noisy_tools.iter()
                                    .map(|t| (t.slug.clone(), format!("{} - {}", t.name, t.description.replace('\n', " "))))
                                    .collect();

                                let failure_type = if result.llm_selected_tool.is_none() {
                                    evaluator::FailureCategory::NoToolReturned
                                } else if query_case.description_notes.is_some() {
                                    evaluator::FailureCategory::UnclearDescription
                                } else {
                                    evaluator::FailureCategory::WrongTool
                                };

                                reporter.record_failure(&result, desc_map, failure_type, None);
                            }

                            round_results.push(result);
                        }
                        Err(e) => {
                            println!("✗ error: {}", e);
                            round_results.push(QueryResult {
                                query: query_text.clone(),
                                expected_tool: query_case.expected_tool.clone(),
                                llm_selected_tool: None,
                                is_correct: false,
                                is_ambiguous_expected: query_case.ambiguous,
                                error_message: Some(e.to_string()),
                                provider: Some(dataset.provider.clone()),
                                category: query_case.category.clone(),
                            });
                        }
                    }

                    // Delay between requests to avoid rate limiting
                    if args.delay_ms > 0 {
                        tokio::time::sleep(Duration::from_millis(args.delay_ms)).await;
                    }

                    // Check max queries limit
                    total_queries += 1;
                    if args.max_queries > 0 && total_queries >= args.max_queries {
                        println!("\nReached max queries limit ({})", args.max_queries);
                        break;
                    }
                }
            }

            if args.max_queries > 0 && total_queries >= args.max_queries {
                break;
            }
        }

        if args.max_queries > 0 && total_queries >= args.max_queries {
            break;
        }

        all_results.extend(round_results);

        // Print round summary
        let round_summary = summarize_results(&all_results);
        println!("\n--- Round {} Summary ---", round);
        println!("  Accuracy: {:.1}% ({}/{})",
            round_summary.accuracy * 100.0,
            round_summary.correct,
            round_summary.total);
    }

    // Multi-round tests
    if !args.multi_round_only {
        println!("\n{}", "=".repeat(80));
        println!("MULTI-ROUND TOOL SELECTION TESTS");
        println!("{}", "=".repeat(80));
        run_multi_round_tests(&client, &args, &all_tools).await?;
    }

    // Final summary
    let summary = summarize_results(&all_results);
    reporter.print_summary(&summary);

    // Write output files
    if let Some(output_dir) = &args.output {
        std::fs::create_dir_all(output_dir)?;
        let timestamp = chrono_lite_timestamp();
        reporter.write_failures(&output_dir.join(format!("failures_{}.json", timestamp)))?;
        reporter.write_unclear_descriptions(&output_dir.join(format!("unclear_{}.yaml", timestamp)))?;

        let summary_json = serde_json::to_string_pretty(&summary)?;
        std::fs::write(output_dir.join(format!("summary_{}.json", timestamp)), summary_json)?;

        println!("\nResults written to {}:", output_dir.display());
        println!("  - failures_{}.json", timestamp);
        println!("  - unclear_{}.yaml", timestamp);
        println!("  - summary_{}.json", timestamp);
    }

    Ok(())
}

/// Simple timestamp for file naming.
fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_secs())
}

/// Run multi-round tool selection tests.
async fn run_multi_round_tests(
    client: &reqwest::Client,
    args: &Args,
    all_tools: &[registry::Tool],
) -> Result<()> {
    let scenarios = vec![
        MultiRoundScenario {
            name: "GitHub: List repos then get repo details".to_string(),
            steps: vec![
                MultiRoundStep {
                    query: "List my GitHub repositories".to_string(),
                    expected_tool: "github_list_repos".to_string(),
                    extract_field: Some("0.full_name".to_string()),
                },
                MultiRoundStep {
                    query: "Get details for that repository".to_string(),
                    expected_tool: "github_get_repo".to_string(),
                    extract_field: None,
                },
            ],
        },
        MultiRoundScenario {
            name: "GitHub: List issues then get specific issue".to_string(),
            steps: vec![
                MultiRoundStep {
                    query: "List open issues in my repo".to_string(),
                    expected_tool: "github_list_issues".to_string(),
                    extract_field: Some("0.number".to_string()),
                },
                MultiRoundStep {
                    query: "Show me issue #123 details".to_string(),
                    expected_tool: "github_get_issue".to_string(),
                    extract_field: None,
                },
            ],
        },
    ];

    for scenario in &scenarios {
        println!("\nScenario: {}", scenario.name);
        let mut conversation = String::new();

        for (step_idx, step) in scenario.steps.iter().enumerate() {
            let query = if step_idx > 0 {
                format!("Previous results: {}\n\nRequest: {}", conversation, step.query)
            } else {
                step.query.clone()
            };

            print!("  Step {}: {}... ", step_idx + 1, &step.query[..step.query.len().min(25)]);

            // Get all github tools with noise
            let github_tools: Vec<_> = all_tools.iter()
                .filter(|t| t.provider == "github")
                .collect();

            let llm_tools: Vec<_> = github_tools.iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.slug,
                            "description": format!("{} - {}", t.name, t.description.replace('\n', " ")),
                            "parameters": {"type": "object", "properties": {}, "required": []}
                        }
                    })
                })
                .collect();

            let response = run_query(
                client,
                &args.llm_url,
                args.llm_api_key.as_deref(),
                &args.model,
                &llm_tools,
                &query,
            ).await?;

            let result = evaluate_query(&QueryCase {
                query: step.query.clone(),
                expected_tool: step.expected_tool.clone(),
                provider: Some("github".to_string()),
                category: "multi_round".to_string(),
                variations: vec![],
                ambiguous: false,
                description_notes: None,
            }, &response);

            if result.is_correct {
                println!("✓");
            } else {
                println!("✗ expected: {}, got: {:?}", step.expected_tool, result.llm_selected_tool);
            }

            if let Some(field) = &step.extract_field {
                conversation.push_str(&format!("{}: {}\n", step.expected_tool, field));
            }
        }
    }

    Ok(())
}

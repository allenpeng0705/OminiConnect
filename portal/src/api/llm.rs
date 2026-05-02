//! LLM Integration — natural language API selection for AI agents.
//!
//! Provides a `/api/llm` endpoint that accepts natural language queries and
//! automatically selects and executes the appropriate tool.
//!
//! Algorithm: Rule-based keyword matching (no external LLM required):
//!   1. Tokenize query (lowercase, remove stop words)
//!   2. Score each tool by keyword/description/platform/action/example_queries matches
//!   3. If top score > threshold → select that tool and extract arguments
//!   4. If ambiguous → return candidates for disambiguation
//!   5. If no match → return helpful error with available tools

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use http_body_util::BodyExt;
use jsonschema::{self, draft7};
use serde::{Deserialize, Serialize};

use crate::api::tools::{auth_user, check_scope_satisfied, tool_error, ScopeSatisfied, substitute_path_params, build_params};
use crate::app::AppState;

/// Stop words to remove from queries for matching.
const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "to", "for", "of", "in", "on", "at", "by", "my", "i", "me", "can", "could",
    "would", "should", "will", "do", "does", "please", "kindly", "just", "now", "and", "or",
    "with", "from",
];

/// Request body for LLM natural language execution.
#[derive(Debug, Deserialize)]
pub struct LlmExecuteRequest {
    /// Natural language query, e.g. "list my github repos sorted by updated"
    pub query: String,
    /// Optional platform hint (e.g. "github"). If omitted, auto-detected.
    #[serde(default)]
    pub platform: Option<String>,
    /// Optional context (currently unused, for future expansion).
    #[serde(default)]
    pub context: Option<serde_json::Value>,
}

/// Response for LLM execution.
#[derive(Debug, Serialize)]
pub struct LlmExecuteResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<CandidateTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tools_hint: Option<String>,
}

/// A candidate tool when query is ambiguous.
#[derive(Debug, Serialize)]
pub struct CandidateTool {
    pub tool: String,
    pub name: String,
    pub match_reason: String,
}

/// Response for GET /api/llm/tools — available tools per platform.
#[derive(Debug, Serialize)]
pub struct LlmToolsResponse {
    pub platforms: HashMap<String, PlatformTools>,
}

#[derive(Debug, Serialize)]
pub struct PlatformTools {
    pub connected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AvailableTool>>,
}

#[derive(Debug, Serialize)]
pub struct AvailableTool {
    pub slug: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub example_queries: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    pub scope_satisfied: String,
}

/// Query params for list available tools.
#[derive(Debug, Deserialize)]
pub struct LlmToolsQuery {
    pub platform: Option<String>,
}

/// Query params for explain.
#[derive(Debug, Deserialize)]
pub struct LlmExplainQuery {
    pub query: String,
    pub tool: String,
}

/// Query params for telemetry.
#[derive(Debug, Deserialize)]
pub struct LlmTelemetryQuery {
    #[serde(default)]
    pub reset: bool,
}

/// POST /api/llm — execute natural language query.
pub async fn execute(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<LlmExecuteRequest>,
) -> Response {
    let (owner, _agent_id, _allowed_tools) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    // Get user's connected platforms and scopes
    let user_connectors = match get_user_connectors(&state, &owner).await {
        Ok(c) => c,
        Err(e) => return e,
    };

    if user_connectors.is_empty() {
        let resp = LlmExecuteResponse {
            ok: false,
            tool: None,
            tool_name: None,
            arguments: None,
            explanation: None,
            result: None,
            error: Some("no_connectors".to_string()),
            message: Some("No platforms connected. Connect one at /dashboard first.".to_string()),
            candidates: None,
            available_tools_hint: None,
        };
        return ok_json(resp);
    }

    // Check if LLM is available (Approach 2)
    let llm_available = state.llm_config.is_available()
        || state.panda_ai_gateway.as_ref().map(|p| p.is_enabled()).unwrap_or(false);

    // If LLM is available, use LLM-based execution
    if llm_available {
        tracing::info!("Using LLM-based execution for query: {}", body.query);
        match llm_execute(&state, &owner, &body.query, &user_connectors).await {
            Ok(resp) => return ok_json(resp),
            Err(e) => return e,
        }
    }

    // Fall back to rule-based execution (Approach 1 style)
    tracing::info!("Using rule-based execution for query: {}", body.query);

    // Normalize query
    let query_lower = body.query.to_lowercase();
    let query_tokens = tokenize(&query_lower);

    // Platform hint or auto-detect
    let target_platform = body
        .platform
        .or_else(|| detect_platform_from_query(&query_lower, &user_connectors));

    // Collect candidate tools (only from connected platforms)
    let platform_filter = target_platform.as_ref();
    let tools: Vec<_> = state
        .tools
        .toolkits()
        .iter()
        .filter(|t| {
            let is_connected = user_connectors.contains_key(&t.provider);
            match platform_filter {
                Some(p) => is_connected && (t.provider == *p || t.slug == *p),
                None => is_connected,
            }
        })
        .flat_map(|t| {
            let granted_scopes = user_connectors
                .get(&t.provider)
                .cloned()
                .unwrap_or_default();
            state
                .tools
                .tools_for_provider(&t.provider)
                .unwrap_or(&[])
                .iter()
                .map(move |tool| (t, tool, granted_scopes.clone()))
        })
        .collect();

    if tools.is_empty() {
        let hint = build_available_hint(&user_connectors, &state);
        let resp = LlmExecuteResponse {
            ok: false,
            tool: None,
            tool_name: None,
            arguments: None,
            explanation: None,
            result: None,
            error: Some("platform_not_connected".to_string()),
            message: Some(format!(
                "Platform '{}' is not connected. Connect it at /dashboard first.",
                target_platform
                    .as_deref()
                    .unwrap_or("(inferred from query)")
            )),
            candidates: None,
            available_tools_hint: Some(hint),
        };
        return ok_json(resp);
    }

    // Score all tools
    let mut scored: Vec<_> = tools
        .iter()
        .map(|(toolkit, tool, granted_scopes)| {
            let score = score_tool(&query_lower, &query_tokens, tool, &toolkit.provider);
            let scope_sat = check_scope_satisfied(&tool.scopes, granted_scopes);
            (toolkit, tool, score, scope_sat)
        })
        .collect();

    // Sort by score descending
    scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let top_score = scored.first().map(|x| x.2).unwrap_or(0.0);
    let top_tools: Vec<_> = scored.iter().take(5).collect();

    // Check if top score is meaningful
    if top_score < 0.1 {
        let hint = build_available_hint(&user_connectors, &state);
        let resp = LlmExecuteResponse {
            ok: false,
            tool: None,
            tool_name: None,
            arguments: None,
            explanation: None,
            result: None,
            error: Some("no_matching_tool".to_string()),
            message: Some("No tool found matching your query.".to_string()),
            candidates: None,
            available_tools_hint: Some(hint),
        };
        return ok_json(resp);
    }

    // Check for ambiguity: is there a close second candidate?
    let ambiguous = if top_tools.len() >= 2 {
        let top = top_tools[0].2;
        let second = top_tools[1].2;
        // Within 20% of top score → ambiguous
        second > 0.0 && (top - second) / top < 0.2
    } else {
        false
    };

    if ambiguous && top_tools.len() >= 2 {
        let candidates: Vec<_> = top_tools
            .iter()
            .take(3)
            .map(|(tk, t, score, _)| CandidateTool {
                tool: t.slug.clone(),
                name: t.name.clone(),
                match_reason: format!("score {:.2}", score),
            })
            .collect();

        let resp = LlmExecuteResponse {
            ok: false,
            tool: None,
            tool_name: None,
            arguments: None,
            explanation: None,
            result: None,
            error: Some("ambiguous".to_string()),
            message: Some(
                "Your query could match multiple tools. Did you mean one of:".to_string(),
            ),
            candidates: Some(candidates),
            available_tools_hint: None,
        };
        return ok_json(resp);
    }

    // Select top tool
    let (_toolkit, selected_tool, _score, scope_sat) = top_tools[0];

    // Check scope satisfaction
    let scope_ok = !matches!(scope_sat, ScopeSatisfied::No);
    let explanation = build_explanation(&body.query, selected_tool, *scope_sat);

    // Extract arguments from query
    let extract_result = crate::argument_extractor::extract_arguments(&body.query, selected_tool);
    let arguments = extract_result.arguments;

    if !scope_ok {
        let resp = LlmExecuteResponse {
            ok: false,
            tool: Some(selected_tool.slug.clone()),
            tool_name: Some(selected_tool.name.clone()),
            arguments: None,
            explanation: Some(explanation),
            result: None,
            error: Some("insufficient_scope".to_string()),
            message: Some(format!(
                "The {} tool requires {} scopes, but your {} connection only has {:?}.",
                selected_tool.name,
                selected_tool.scopes.join(", "),
                selected_tool.provider,
                user_connectors
                    .get(&selected_tool.provider)
                    .cloned()
                    .unwrap_or_default()
            )),
            candidates: None,
            available_tools_hint: None,
        };
        return ok_json(resp);
    }

    // Execute the tool
    let exec_req = crate::api::tools::ExecuteToolRequest {
        tool_slug: selected_tool.slug.clone(),
        platform: selected_tool.provider.clone(),
        arguments: arguments.clone(),
        callback_url: None,
    };

    let exec_resp = crate::api::tools::execute(State(state.clone()), headers, Json(exec_req)).await;

    // Parse the execution response
    let exec_status = exec_resp.status();
    let exec_body = body_from_response_async(exec_resp).await;

    if !exec_status.is_success() {
        let resp = LlmExecuteResponse {
            ok: false,
            tool: Some(selected_tool.slug.clone()),
            tool_name: Some(selected_tool.name.clone()),
            arguments: Some(arguments),
            explanation: Some(explanation),
            result: None,
            error: Some("execution_failed".to_string()),
            message: Some(format!(
                "Tool execution failed with status {}: {}",
                exec_status.as_u16(),
                exec_body
            )),
            candidates: None,
            available_tools_hint: None,
        };
        return ok_json(resp);
    }

    let result_json: serde_json::Value = serde_json::from_str(&exec_body)
        .unwrap_or_else(|_| serde_json::json!({ "raw": exec_body }));

    let resp = LlmExecuteResponse {
        ok: true,
        tool: Some(selected_tool.slug.clone()),
        tool_name: Some(selected_tool.name.clone()),
        arguments: Some(arguments),
        explanation: Some(explanation),
        result: Some(result_json),
        error: None,
        message: None,
        candidates: None,
        available_tools_hint: None,
    };

    ok_json(resp)
}

/// GET /api/llm/tools — return available tools per connected platform.
pub async fn list_tools(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LlmToolsQuery>,
    headers: HeaderMap,
) -> Response {
    let (owner, _agent_id, _allowed_tools) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let user_connectors = match get_user_connectors(&state, &owner).await {
        Ok(c) => c,
        Err(e) => return e,
    };

    let platforms: HashMap<String, PlatformTools> = if let Some(p) = &query.platform {
        let mut map = HashMap::new();
        let connected = user_connectors.contains_key(p);
        let tools = if connected {
            let granted = user_connectors.get(p).cloned().unwrap_or_default();
            let tool_list = state.tools.tools_for_provider(p).unwrap_or(&[]);
            Some(
                tool_list
                    .iter()
                    .map(|t| {
                        let scope_sat = check_scope_satisfied(&t.scopes, &granted);
                        AvailableTool {
                            slug: t.slug.clone(),
                            name: t.name.clone(),
                            description: t.description.clone(),
                            example_queries: t.example_queries.clone(),
                            scopes: t.scopes.clone(),
                            scope_satisfied: match scope_sat {
                                ScopeSatisfied::Yes => "yes".to_string(),
                                ScopeSatisfied::No => "no".to_string(),
                                ScopeSatisfied::Unknown => "unknown".to_string(),
                            },
                        }
                    })
                    .collect(),
            )
        } else {
            None
        };
        map.insert(p.clone(), PlatformTools { connected, tools });
        map
    } else {
        user_connectors
            .keys()
            .map(|platform| {
                let granted = user_connectors.get(platform).cloned().unwrap_or_default();
                let tool_list = state.tools.tools_for_provider(platform).unwrap_or(&[]);
                let tools = Some(
                    tool_list
                        .iter()
                        .map(|t| {
                            let scope_sat = check_scope_satisfied(&t.scopes, &granted);
                            AvailableTool {
                                slug: t.slug.clone(),
                                name: t.name.clone(),
                                description: t.description.clone(),
                                example_queries: t.example_queries.clone(),
                                scopes: t.scopes.clone(),
                                scope_satisfied: match scope_sat {
                                    ScopeSatisfied::Yes => "yes".to_string(),
                                    ScopeSatisfied::No => "no".to_string(),
                                    ScopeSatisfied::Unknown => "unknown".to_string(),
                                },
                            }
                        })
                        .collect(),
                );
                (
                    platform.clone(),
                    PlatformTools {
                        connected: true,
                        tools,
                    },
                )
            })
            .collect()
    };

    ok_json(LlmToolsResponse { platforms })
}

/// POST /api/llm/explain — explain what arguments a tool would use for a query.
pub async fn explain(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<LlmExplainQuery>,
) -> Response {
    let (owner, _agent_id, _allowed_tools) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let user_connectors = match get_user_connectors(&state, &owner).await {
        Ok(c) => c,
        Err(e) => return e,
    };

    let tool = match state.tools.tool_by_slug(&query.tool) {
        Some(t) => t,
        None => {
            let body = serde_json::json!({ "error": "tool_not_found" }).to_string();
            return not_found(body);
        }
    };

    // Check platform is connected
    if !user_connectors.contains_key(&tool.provider) {
        let body = serde_json::json!({
            "error": "platform_not_connected",
            "message": format!("{} is not connected", tool.provider)
        })
        .to_string();
        return not_found(body);
    }

    let granted = user_connectors
        .get(&tool.provider)
        .cloned()
        .unwrap_or_default();
    let scope_sat = check_scope_satisfied(&tool.scopes, &granted);

    let extract_result = crate::argument_extractor::extract_arguments(&query.query, tool);
    let arguments = extract_result.arguments;
    let missing = extract_result.missing_required;

    let explanation = build_explanation(&query.query, tool, scope_sat);

    #[derive(Serialize)]
    struct ExplainResponse<'a> {
        arguments: serde_json::Value,
        missing_params: Vec<String>,
        explanation: &'a str,
        scope_satisfied: String,
    }

    let resp = ExplainResponse {
        arguments,
        missing_params: missing,
        explanation: &explanation,
        scope_satisfied: match scope_sat {
            ScopeSatisfied::Yes => "yes".to_string(),
            ScopeSatisfied::No => "no".to_string(),
            ScopeSatisfied::Unknown => "unknown".to_string(),
        },
    };

    ok_json(resp)
}

/// GET /api/llm/telemetry — show LLM tool-routing telemetry.
pub async fn telemetry(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LlmTelemetryQuery>,
    headers: HeaderMap,
) -> Response {
    let _ = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    let snapshot = {
        let mut guard = state.llm_tool_telemetry.write().await;
        let current = guard.clone();
        if query.reset {
            *guard = crate::telemetry::LlmToolTelemetry::default();
        }
        current
    };

    #[derive(Serialize)]
    struct TelemetryResponse {
        ok: bool,
        reset_applied: bool,
        provider_context_enabled: bool,
        telemetry: crate::telemetry::LlmToolTelemetry,
    }

    ok_json(TelemetryResponse {
        ok: true,
        reset_applied: query.reset,
        provider_context_enabled: state.llm_provider_context_enabled,
        telemetry: snapshot,
    })
}

/// Score how well a tool matches the query (0.0 to 1.0).
fn score_tool(
    query_lower: &str,
    query_tokens: &[&str],
    tool: &crate::tools::Tool,
    provider: &str,
) -> f64 {
    let mut score = 0.0;

    // 1. Platform match (20%)
    if query_lower.contains(provider) {
        score += 0.20;
    }

    // 2. Action verb match (30%)
    // Map query action patterns to HTTP methods / tool patterns
    let action_score = score_action_match(query_lower, tool);
    score += action_score * 0.30;

    // 3. Keyword match in name/description/slug (30%)
    let keyword_score = score_keyword_match(query_tokens, tool);
    score += keyword_score * 0.30;

    // 4. Example queries match (20%)
    let example_score = score_example_queries(query_lower, tool);
    score += example_score * 0.20;

    score
}

fn score_action_match(query_lower: &str, tool: &crate::tools::Tool) -> f64 {
    // Action patterns in queries
    let list_patterns = [
        "list",
        "show",
        "get",
        "fetch",
        "retrieve",
        "all",
        "what do i have",
    ];
    let create_patterns = ["create", "add", "new", "make", "post", "open"];
    let update_patterns = ["update", "edit", "modify", "patch", "put", "change"];
    let delete_patterns = ["delete", "remove", "destroy", "drop", "unlink"];

    let mut max_score: f64 = 0.0;

    for pat in list_patterns {
        if query_lower.contains(pat) && matches!(tool.method, crate::tools::HttpMethod::GET) {
            max_score = max_score.max(1.0);
        }
    }
    for pat in create_patterns {
        if query_lower.contains(pat) && matches!(tool.method, crate::tools::HttpMethod::POST) {
            max_score = max_score.max(1.0);
        }
    }
    for pat in update_patterns {
        if query_lower.contains(pat)
            && matches!(
                tool.method,
                crate::tools::HttpMethod::PUT | crate::tools::HttpMethod::PATCH
            )
        {
            max_score = max_score.max(1.0);
        }
    }
    for pat in delete_patterns {
        if query_lower.contains(pat) && matches!(tool.method, crate::tools::HttpMethod::DELETE) {
            max_score = max_score.max(1.0);
        }
    }

    max_score
}

fn score_keyword_match(tokens: &[&str], tool: &crate::tools::Tool) -> f64 {
    let mut matches = 0;
    let total = tokens.len().max(1);

    let searchable = format!(
        "{} {} {} {} {}",
        tool.slug.replace('_', " "),
        tool.name,
        tool.description.replace('\n', " "),
        tool.tags.join(" "),
        tool.provider
    )
    .to_lowercase();

    // Check if this looks like a Chinese query (no whitespace tokens)
    let is_chinese_query = tokens.iter().any(|t| is_likely_chinese(t));

    for token in tokens {
        if STOP_WORDS.contains(token) {
            continue;
        }
        if token.len() < 2 {
            continue;
        }
        if searchable.contains(*token) {
            matches += 1;
        } else if is_chinese_query {
            // For Chinese: check if any searchable term (especially Chinese name) is a substring of query
            // e.g., token "股东信息" is not in tokens but appears in searchable as tool name
            // Instead, check if searchable Chinese terms appear in the query
            for term in [tool.name.as_str(), &tool.description, tool.provider.as_str()] {
                let term_lower = term.to_lowercase();
                if term_lower.len() >= 3 && query_contains_term(token, &term_lower) {
                    matches += 1;
                    break;
                }
            }
        }
    }

    (matches as f64) / (total as f64)
}

/// Check if the query (represented by the token) contains a specific term from searchable text.
fn query_contains_term(token: &str, term: &str) -> bool {
    // For Chinese: check if the term appears as substring in the original token
    // We use a simpler approach: if the query token is Chinese and contains key Chinese chars from term
    let query_chars: HashSet<char> = token.chars().collect();
    let term_chars: HashSet<char> = term.chars().filter(|c| !c.is_ascii()).collect();
    if term_chars.is_empty() {
        return false;
    }
    // Count how many term chars appear in query
    let overlap: usize = term_chars.intersection(&query_chars).count();
    overlap >= term_chars.len() / 2 && overlap >= 2
}

fn score_example_queries(query: &str, tool: &crate::tools::Tool) -> f64 {
    if tool.example_queries.is_empty() {
        // Fall back to name matching
        let name_words = tool.name.to_lowercase();
        let query_words: HashSet<_> = query.split_whitespace().collect();
        let name_set: HashSet<_> = name_words.split_whitespace().collect();
        let intersection: Vec<_> = query_words.intersection(&name_set).collect();
        if intersection.is_empty() {
            return 0.0;
        }
        return (intersection.len() as f64) / (name_set.len().max(1) as f64);
    }

    // If query looks like Chinese, use placeholder-based pattern matching
    if is_likely_chinese(query) {
        return score_chinese_examples(query, &tool.example_queries);
    }

    let query_lower = query.to_lowercase();
    let mut max_sim: f64 = 0.0;
    for example in &tool.example_queries {
        let example_lower = example.to_lowercase();
        let sim = similarity(&query_lower, &example_lower);
        max_sim = max_sim.max(sim);
    }
    max_sim
}

/// Score Chinese query against example_queries using placeholder pattern matching.
/// Example queries contain placeholders like "某公司" which we substitute with the actual
/// entity from the query to compare structure.
fn score_chinese_examples(query: &str, example_queries: &[String]) -> f64 {
    let placeholders = ["某公司", "某企业", "该公司", "该企业", "某个公司", "某个企业"];
    let query_lower = query.to_lowercase();

    let mut max_score: f64 = 0.0;

    for example in example_queries {
        // Find which placeholder this example uses
        let placeholder = match placeholders.iter().find(|p| example.contains(*p)) {
            Some(p) => *p,
            None => continue,
        };
        let parts: Vec<&str> = example.splitn(2, placeholder).collect();
        if parts.len() != 2 {
            continue;
        }
        let prefix = parts[0];
        let suffix = parts[1];

        // Try full prefix+suffix match
        if !prefix.is_empty()
            && query_lower.starts_with(&prefix.to_lowercase())
            && query_lower.ends_with(&suffix.to_lowercase())
        {
            let prefix_match = prefix.len() as f64 / prefix.len().max(1) as f64;
            let suffix_match = suffix.len() as f64 / suffix.len().max(1) as f64;
            max_score = max_score.max((prefix_match + suffix_match) / 2.0);
            continue;
        }

        // Try suffix-only match (handles queries with different action verbs like 查看/了解)
        if query_lower.ends_with(&suffix.to_lowercase()) && !suffix.is_empty() {
            // Score based on suffix length relative to query length
            let suffix_ratio = suffix.len() as f64 / query_lower.len().max(1) as f64;
            max_score = max_score.max(suffix_ratio.min(1.0));
        }
    }

    max_score
}

/// Simple word overlap similarity.
fn similarity(a: &str, b: &str) -> f64 {
    let a_words: HashSet<_> = a
        .split_whitespace()
        .filter(|w| !STOP_WORDS.contains(w) && w.len() >= 2)
        .collect();
    let b_words: HashSet<_> = b
        .split_whitespace()
        .filter(|w| !STOP_WORDS.contains(w) && w.len() >= 2)
        .collect();

    if a_words.is_empty() || b_words.is_empty() {
        return 0.0;
    }

    let intersection = a_words.intersection(&b_words).count() as f64;
    let union = a_words.union(&b_words).count() as f64;
    intersection / union
}

/// Tokenize query into meaningful words.
fn tokenize(query: &str) -> Vec<&str> {
    query
        .split_whitespace()
        .filter(|w| w.len() >= 2 && !STOP_WORDS.contains(w))
        .collect()
}

/// Detect platform from query keywords.
fn detect_platform_from_query<'a>(
    query_lower: &str,
    connected: &'a HashMap<String, Vec<String>>,
) -> Option<String> {
    let platform_keywords: HashMap<&str, &str> = [
        ("github", "github"),
        ("git", "github"),
        ("slack", "slack"),
        ("notion", "notion"),
        ("jira", "jira"),
        ("salesforce", "salesforce"),
        ("hubspot", "hubspot"),
        ("gitlab", "gitlab"),
        ("stripe", "stripe"),
        ("shopify", "shopify"),
        ("feishu", "feishu"),
        ("lark", "feishu"),
        ("dingtalk", "dingtalk"),
        ("wechatwork", "wechatwork"),
        ("weixin", "wechatwork"),
    ]
    .iter()
    .cloned()
    .collect();

    for (keyword, platform) in platform_keywords {
        if query_lower.contains(keyword) && connected.contains_key(platform) {
            return Some(platform.to_string());
        }
    }

    // Fall back to first connected platform if query has no clear platform
    None
}

/// Extract arguments from query based on tool's input schema.
fn extract_arguments(query: &str, tool: &crate::tools::Tool) -> serde_json::Value {
    let query_lower = query.to_lowercase();

    // Use the new schema-guided extraction
    let result = crate::argument_extractor::extract_arguments(query, tool);
    let mut args = match result.arguments {
        serde_json::Value::Object(m) => m,
        _ => serde_json::Map::new(),
    };

    // Handle Chinese queries using example_queries pattern matching
    if is_likely_chinese(query) {
        if let Some(value) = extract_chinese_entity(query, &tool.example_queries) {
            // Find first string property and set it
            for (param_name, param_spec) in &tool.input_schema.properties {
                if param_spec.get("type").and_then(|t| t.as_str()) == Some("string") {
                    if !args.contains_key(param_name) {
                        args.insert(param_name.clone(), serde_json::Value::String(value.clone()));
                        break;
                    }
                }
            }
        }
    }

    // Handle sort parameter
    if tool.input_schema.properties.contains_key("sort") {
        for sort_val in ["created", "updated", "pushed", "full_name"] {
            if query_lower.contains(sort_val) {
                args.insert(
                    "sort".to_string(),
                    serde_json::Value::String(sort_val.to_string()),
                );
                break;
            }
        }
    }

    // Handle per_page parameter
    if tool.input_schema.properties.contains_key("per_page") {
        if query_lower.contains("all") || query_lower.contains("every") {
            args.insert(
                "per_page".to_string(),
                serde_json::Value::Number(100.into()),
            );
        }
    }

    serde_json::Value::Object(args)
}

/// Detect if a string is likely Chinese (contains significant proportion of CJK characters).
fn is_likely_chinese(s: &str) -> bool {
    let cjk_count = s.chars().filter(|c| {
        let cp = *c as u32;
        (0x4E00..=0x9FFF).contains(&cp) // CJK Unified Ideographs
            || (0x3400..=0x4DBF).contains(&cp) // CJK Extension A
            || (0xF900..=0xFAFF).contains(&cp) // CJK Compatibility
    }).count();
    cjk_count > s.len() / 4
}

/// For Chinese queries, extract entity from query by matching against example query patterns.
/// Example: "查询比亚迪的股东信息" matched against "查询某公司的股东信息" → extracts "比亚迪".
/// Also handles queries whose action verb differs from the example (e.g., "查看小米的主要人员" → "小米").
fn extract_chinese_entity(query: &str, example_queries: &[String]) -> Option<String> {
    // Common placeholder patterns in example queries
    let placeholders = ["某公司", "某企业", "该公司", "该企业", "某个公司", "某个企业"];

    for example in example_queries {
        // Find which placeholder this example uses
        let placeholder = match placeholders.iter().find(|p| example.contains(*p)) {
            Some(p) => *p,
            None => continue,
        };
        let parts: Vec<&str> = example.splitn(2, placeholder).collect();
        if parts.len() != 2 {
            continue;
        }
        let prefix = parts[0]; // e.g., "查询" or "" for patterns starting with placeholder
        let suffix = parts[1]; // e.g., "的股东信息"

        // Must end with suffix
        if !query.ends_with(suffix) {
            continue;
        }

        let entity_candidate: &str;
        if !prefix.is_empty() && query.starts_with(prefix) {
            // Exact prefix+suffix match: straightforward extraction
            entity_candidate = &query[prefix.len()..query.len() - suffix.len()];
        } else if prefix.is_empty() {
            // Example starts with placeholder (no prefix): entity is at start of query
            let end = query.len() - suffix.len();
            entity_candidate = &query[..end];
        } else {
            // Prefix doesn't match: query may have a different action verb
            // e.g., query="查看小米的主要人员", example="查询某公司的主要人员"
            // Strategy: find the entity by matching suffix position and using example prefix length as hint
            // Find position of suffix in query
            if let Some(suffix_pos) = query.find(suffix) {
                // entity is before suffix; use example prefix length as minimum offset
                let min_start = prefix.len();
                // entity starts after any leading verb/action in query (try skipping 1-3 chars)
                let query_prefix_len = suffix_pos; // length of query before suffix
                if query_prefix_len > min_start {
                    // Try to find where the entity starts by looking for natural boundaries
                    // Heuristic: skip 1-3 chars (typical verb length) and take rest
                    let entity = &query[min_start..suffix_pos];
                    let trimmed = entity.trim().trim_end_matches('的');
                    if !trimmed.is_empty() && trimmed.len() >= 2 {
                        return Some(trimmed.to_string());
                    }
                } else if query_prefix_len > 0 {
                    // Entity shorter than example prefix: take what's there
                    let entity = &query[..suffix_pos];
                    let trimmed = entity.trim().trim_end_matches('的');
                    if !trimmed.is_empty() && trimmed.len() >= 2 {
                        return Some(trimmed.to_string());
                    }
                }
            }
            continue;
        }

        let trimmed = entity_candidate.trim().trim_end_matches('的');
        if !trimmed.is_empty() && trimmed.len() >= 2 {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn extract_value_after_word(query: &str, word: &str) -> Option<String> {
    // Look for patterns like "sort by updated" or "in repo myrepo"
    let patterns = [format!("{} ", word), format!(" {} ", word)];

    for pat in &patterns {
        if let Some(idx) = query.find(pat) {
            let after = &query[idx + pat.len()..];
            let end = after.find(' ').unwrap_or(after.len()).min(50);
            let val =
                after[..end].trim_end_matches(|c: char| c.is_ascii_punctuation() || c == '\n');
            if !val.is_empty() && !STOP_WORDS.contains(&val) {
                return Some(val.to_string());
            }
        }
    }

    None
}

fn build_explanation(query: &str, tool: &crate::tools::Tool, scope_sat: ScopeSatisfied) -> String {
    let mut parts = vec![];

    parts.push(format!(
        "Selected {} because your query mentions '{}'.",
        tool.slug,
        query.chars().take(50).collect::<String>()
    ));

    if !tool.scopes.is_empty() {
        match scope_sat {
            ScopeSatisfied::Yes => {
                parts.push("Your connection has the required scopes.".to_string())
            }
            ScopeSatisfied::No => {
                parts.push("Your connection is missing required scopes.".to_string())
            }
            ScopeSatisfied::Unknown => parts.push("Scope status is unknown.".to_string()),
        }
    }

    parts.join(" ")
}

fn build_available_hint(connected: &HashMap<String, Vec<String>>, state: &AppState) -> String {
    let platform_names: Vec<String> = connected
        .keys()
        .map(|p| {
            state
                .tools
                .toolkits()
                .iter()
                .find(|t| t.provider == *p)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| p.clone())
        })
        .collect();

    if platform_names.is_empty() {
        "No platforms connected yet. Connect one at /dashboard.".to_string()
    } else {
        format!(
            "You have {} connected. Available tools include: {}.",
            platform_names.join(", "),
            connected
                .keys()
                .flat_map(|p| state.tools.tools_for_provider(p).unwrap_or(&[]).iter())
                .take(5)
                .map(|t| t.slug.clone())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

async fn get_user_connectors(
    state: &Arc<AppState>,
    owner: &str,
) -> Result<HashMap<String, Vec<String>>, Response> {
    // Returns HashMap<platform, scopes> for enabled connectors (used for scope checking)
    // Note: platform may differ from provider_key (e.g., github vs github-pat)
    let connectors = state.connectors.list_all().await.map_err(|e| {
        let body =
            serde_json::json!({ "error": format!("failed to list connectors: {}", e) }).to_string();
        let mut resp = Response::new(body.into());
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        resp
    })?;

    Ok(connectors
        .into_iter()
        .filter(|c| c.owner_username == owner && c.enabled)
        .map(|c| (c.platform.clone(), c.scopes))
        .collect())
}

async fn body_from_response_async(resp: Response) -> String {
    let mut body = resp.into_body();
    let mut bytes = bytes::BytesMut::new();
    while let Some(item) = body.frame().await {
        if let Ok(frame) = item {
            if let Some(data) = frame.data_ref() {
                bytes.extend_from_slice(data);
            }
        }
    }
    String::from_utf8_lossy(&bytes.freeze()).to_string()
}

fn ok_json<T: Serialize>(value: T) -> Response {
    let body = serde_json::to_string(&value).unwrap_or_default();
    let mut resp = Response::new(body.into());
    *resp.status_mut() = StatusCode::OK;
    resp
}

fn not_found(body: String) -> Response {
    let mut resp = Response::new(body.into());
    *resp.status_mut() = StatusCode::NOT_FOUND;
    resp
}

/// Router for LLM integration.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/llm", post(execute))
        .route("/llm/tools", get(list_tools))
        .route("/llm/explain", post(explain))
        .route("/llm/telemetry", get(telemetry))
}

// ============================================================================
// LLM-based execution (Approach 2) - Multi-step orchestration
// ============================================================================

/// Execute using LLM with multi-step tool orchestration.
///
/// This function:
/// 1. Builds tool descriptions for the LLM
/// 2. Calls the LLM with the user's query
/// 3. Executes selected tools and feeds results back to LLM
/// 4. Repeats until no more tool calls or max rounds reached
/// 5. Returns the final result
async fn llm_execute(
    state: &Arc<AppState>,
    owner: &str,
    query: &str,
    user_connectors: &HashMap<String, Vec<String>>,
) -> Result<LlmExecuteResponse, Response> {
    // Build the list of tools for the LLM and platform->provider mapping
    let (tools, _platform_provider_map) = build_llm_tools(state, owner, user_connectors, query).await?;
    let tools_for_llm = tools; // rename for clarity

    if tools_for_llm.is_empty() {
        return Ok(LlmExecuteResponse {
            ok: false,
            tool: None,
            tool_name: None,
            arguments: None,
            explanation: None,
            result: None,
            error: Some("no_tools_available".to_string()),
            message: Some("No tools available for your connected platforms.".to_string()),
            candidates: None,
            available_tools_hint: None,
        });
    }

    // Build system prompt
    let system_prompt = build_llm_system_prompt(&tools_for_llm);

    // Get max tool rounds from config
    let max_rounds = state.llm_config.max_tool_rounds;

    // Initialize conversation context
    let mut messages = vec![
        crate::llm::Message {
            role: "system".to_string(),
            content: Some(system_prompt),
            tool_call_id: None,
            name: None,
            tool_calls: None,
        },
        crate::llm::Message {
            role: "user".to_string(),
            content: Some(query.to_string()),
            tool_call_id: None,
            name: None,
            tool_calls: None,
        },
    ];

    let mut tool_results: Vec<crate::llm::ToolResult> = Vec::new();
    let mut final_result: Option<serde_json::Value> = None;
    let mut executed_tools: Vec<String> = Vec::new();
    let mut rounds = 0;
    let mut validation_retry_counts: HashMap<String, u8> = HashMap::new();
    let max_validation_retries_per_tool_call: u8 = 2;

    // Tool loop: continue until no more tool calls or max rounds reached
    loop {
        rounds += 1;
        tracing::info!("LLM orchestration round {}/{}", rounds, max_rounds);

        if rounds > max_rounds {
            tracing::warn!("Max tool rounds ({}) reached. Stopping orchestration.", max_rounds);
            break;
        }

        // Determine which LLM client to use
        let llm_response = if let Some(ref panda_ai) = state.panda_ai_gateway {
            if panda_ai.is_enabled() {
                tracing::info!("Using Panda AI Gateway for LLM call");
                let chat_request = crate::llm::ChatRequest {
                    model: state.llm_config.default_model.clone(),
                    messages: messages.clone(),
                    tools: Some(tools_for_llm.clone()),
                    stream: false,
                    temperature: Some(0.0),
                    top_p: Some(1.0),
                };
                panda_ai.chat(chat_request).await.map_err(|e| {
                    tracing::error!("Panda AI Gateway error: {}", e);
                    tool_error(
                        StatusCode::BAD_GATEWAY,
                        &format!("LLM call failed: {}", e),
                    )
                })?
            } else {
                return Err(tool_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Panda AI Gateway not enabled",
                ));
            }
        } else if state.llm_config.is_available() {
            tracing::info!("Using LiteLLM directly for LLM call");
            let chat_request = crate::llm::ChatRequest {
                model: state.llm_config.default_model.clone(),
                messages: messages.clone(),
                tools: Some(tools_for_llm.clone()),
                stream: false,
                temperature: Some(0.0),
                top_p: Some(1.0),
            };
            state.llm_client.chat(chat_request).await.map_err(|e| {
                tracing::error!("LiteLLM error: {}", e);
                tool_error(
                    StatusCode::BAD_GATEWAY,
                    &format!("LLM call failed: {}", e),
                )
            })?
        } else {
            return Err(tool_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "LLM not configured. Set LITELLM_URL or enable Panda AI Gateway.",
            ));
        };

        // Parse tool calls from LLM response (tool_calls are nested inside message in choices)
        let first_choice = llm_response.choices.first();
        let tool_calls = first_choice
            .and_then(|c| c.message.tool_calls.as_ref())
            .cloned()
            .unwrap_or_default();
        tracing::info!("[LLM] tool_calls count: {}, message: {:?}", tool_calls.len(), first_choice.map(|c| &c.message.content));

        if tool_calls.is_empty() {
            // No more tool calls, return the final text response
            let content = first_choice
                .and_then(|c| c.message.content.as_ref())
                .map(|s| s.as_str())
                .unwrap_or_default()
                .to_string();
            final_result = Some(serde_json::json!({
                "response": content,
                "executed_tools": executed_tools,
                "rounds": rounds,
            }));
            break;
        }

        // Execute each tool call and collect results
        let mut new_tool_results = Vec::new();

        for (idx, tool_call) in tool_calls.iter().enumerate() {
            let tool_name = match tool_call.function.name.as_ref() {
                Some(name) => name.clone(),
                None => {
                    tracing::warn!("Tool call {} missing name, skipping", idx);
                    continue;
                }
            };
            {
                let mut telemetry = state.llm_tool_telemetry.write().await;
                telemetry.record_attempt(&tool_name, state.llm_provider_context_enabled);
            }

            let arguments_str = match tool_call.function.arguments.as_ref() {
                Some(args) => args.clone(),
                None => {
                    tracing::warn!("Tool call {} missing arguments, skipping", idx);
                    continue;
                }
            };

            let arguments: serde_json::Value = match serde_json::from_str(&arguments_str) {
                Ok(args) => args,
                Err(e) => {
                    tracing::warn!("Invalid arguments for tool {}: {}", tool_name, e);
                    continue;
                }
            };

            let Some(tool_def) = state.tools.tool_by_slug(&tool_name) else {
                tracing::warn!("Tool '{}' not found during validation", tool_name);
                continue;
            };

            let (validated_args, validation_warnings, missing_required) =
                normalize_arguments_for_schema(arguments, &tool_def.input_schema);
            if !validation_warnings.is_empty() {
                let mut telemetry = state.llm_tool_telemetry.write().await;
                telemetry.record_normalization_warnings(&tool_name, &validation_warnings);
            }

            if !missing_required.is_empty() {
                let retry_key = format!("{}::missing_required", tool_name);
                let retry_count = validation_retry_counts.entry(retry_key.clone()).or_insert(0);
                let should_retry = *retry_count < max_validation_retries_per_tool_call;
                *retry_count += 1;
                {
                    let mut telemetry = state.llm_tool_telemetry.write().await;
                    telemetry.record_missing_required(
                        &tool_name,
                        &missing_required,
                        should_retry,
                        state.llm_provider_context_enabled,
                    );
                }
                let missing_details = build_missing_param_details(&tool_def.input_schema, &missing_required);
                let clarification_question = build_clarification_question(&tool_name, &missing_details);
                let call_id = tool_call.id.clone().unwrap_or_else(|| format!("call-{}", idx));
                let result = serde_json::json!({
                    "error": "missing_required_arguments",
                    "tool": tool_name,
                    "missing_required": missing_required,
                    "missing_param_details": missing_details,
                    "clarification_question": clarification_question,
                    "retryable": should_retry,
                    "hint": if should_retry {
                        "Fix arguments and call the same tool again with all required parameters."
                    } else {
                        "Required parameters are still missing after retries. Ask user a concise clarification question."
                    }
                });
                executed_tools.push(tool_name.clone());
                new_tool_results.push(crate::llm::ToolResult {
                    call_id,
                    tool_name,
                    result,
                });
                continue;
            }

            let schema_errors = validate_arguments_with_jsonschema(&validated_args, &tool_def.input_schema);
            if !schema_errors.is_empty() {
                let retry_key = format!("{}::schema", tool_name);
                let retry_count = validation_retry_counts.entry(retry_key.clone()).or_insert(0);
                let should_retry = *retry_count < max_validation_retries_per_tool_call;
                *retry_count += 1;
                {
                    let mut telemetry = state.llm_tool_telemetry.write().await;
                    telemetry.record_schema_failure(
                        &tool_name,
                        should_retry,
                        state.llm_provider_context_enabled,
                    );
                }
                let schema_repair_hints = build_schema_repair_hints(&tool_def.input_schema, &schema_errors);
                let call_id = tool_call.id.clone().unwrap_or_else(|| format!("call-{}", idx));
                let result = serde_json::json!({
                    "error": "schema_validation_failed",
                    "tool": tool_name,
                    "retryable": should_retry,
                    "errors": schema_errors,
                    "repair_hints": schema_repair_hints,
                    "hint": if should_retry {
                        "Fix arguments to match the tool JSON schema and retry this tool."
                    } else {
                        "Schema validation keeps failing. Ask user for missing or corrected values."
                    }
                });
                new_tool_results.push(crate::llm::ToolResult {
                    call_id,
                    tool_name,
                    result,
                });
                continue;
            }

            // Execute the tool
            tracing::info!("Executing tool: {} with args: {:?}", tool_name, validated_args);
            let tool_result = execute_tool_for_llm(
                state,
                owner,
                &tool_name,
                validated_args,
                user_connectors,
            ).await?;
            {
                let mut telemetry = state.llm_tool_telemetry.write().await;
                telemetry.record_execution(&tool_name, state.llm_provider_context_enabled);
            }

            let tool_result = if validation_warnings.is_empty() {
                tool_result
            } else {
                serde_json::json!({
                    "result": tool_result,
                    "warnings": validation_warnings,
                })
            };

            let call_id = tool_call.id.clone().unwrap_or_else(|| format!("call-{}", idx));
            executed_tools.push(tool_name.clone());
            new_tool_results.push(crate::llm::ToolResult {
                call_id,
                tool_name,
                result: tool_result,
            });
        }

        // Add the assistant's tool_calls message to conversation history
        let tool_calls_for_msg: Option<Vec<crate::llm::MessageToolCall>> = first_choice
            .as_ref()
            .and_then(|c| c.message.tool_calls.as_ref())
            .map(|tcs| {
                tcs.iter()
                    .filter_map(|tc| {
                        let id = tc.id.clone()?;
                        let name = tc.function.name.clone()?;
                        let args = tc.function.arguments.clone()?;
                        Some(crate::llm::MessageToolCall::new(id, name, args))
                    })
                    .collect()
            });

        let assistant_msg = crate::llm::Message {
            role: "assistant".to_string(),
            content: first_choice.as_ref().and_then(|c| c.message.content.clone()),
            tool_call_id: None,
            name: None,
            tool_calls: tool_calls_for_msg,
        };
        messages.push(assistant_msg);

        // Add tool results to messages and continue loop
        for tool_result in new_tool_results {
            let call_id = tool_result.call_id.clone();
            let tool_name = tool_result.tool_name.clone();
            let result_str = serde_json::to_string(&tool_result.result).unwrap_or_default();
            tracing::debug!("Tool result call_id={} tool_name={} result={}", call_id, tool_name, result_str);
            messages.push(crate::llm::Message {
                role: "tool".to_string(),
                content: Some(format!(
                    "{{\"tool\": \"{}\", \"result\": {}}}",
                    tool_name, result_str
                )),
                tool_call_id: Some(call_id),
                name: None,
                tool_calls: None,
            });
            tool_results.push(tool_result);
        }
    }

    Ok(LlmExecuteResponse {
        ok: true,
        tool: None,
        tool_name: None,
        arguments: None,
        explanation: Some(format!("Executed {} tool(s) in {} round(s)", executed_tools.len(), rounds)),
        result: final_result,
        error: None,
        message: None,
        candidates: None,
        available_tools_hint: None,
    })
}

fn normalize_arguments_for_schema(
    arguments: serde_json::Value,
    schema: &crate::tools::InputSchema,
) -> (serde_json::Value, Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut normalized = serde_json::Map::new();
    let mut missing_required = Vec::new();

    let raw_obj = arguments.as_object().cloned().unwrap_or_default();

    for required in &schema.required {
        if !raw_obj.contains_key(required) {
            missing_required.push(required.clone());
        }
    }

    for (name, value) in raw_obj {
        let Some(spec) = schema.properties.get(&name) else {
            warnings.push(format!("ignored_unknown_parameter: {}", name));
            continue;
        };
        let coerced = coerce_value_to_schema_type(&name, value, spec, &mut warnings);
        normalized.insert(name, coerced);
    }

    (serde_json::Value::Object(normalized), warnings, missing_required)
}

fn coerce_value_to_schema_type(
    field: &str,
    value: serde_json::Value,
    spec: &serde_json::Value,
    warnings: &mut Vec<String>,
) -> serde_json::Value {
    let Some(expected_type) = spec.get("type").and_then(|t| t.as_str()) else {
        return value;
    };

    if let Some(enum_values) = spec.get("enum").and_then(|e| e.as_array()) {
        if enum_values.iter().any(|v| v == &value) {
            return value;
        }
        if let Some(s) = value.as_str() {
            if let Some(found) = enum_values.iter().find(|v| v.as_str() == Some(s)) {
                return found.clone();
            }
        }
    }

    match expected_type {
        "string" => {
            if let Some(s) = value.as_str() {
                serde_json::Value::String(s.to_string())
            } else {
                warnings.push(format!("coerced_to_string: {}", field));
                serde_json::Value::String(value.to_string())
            }
        }
        "integer" => {
            if let Some(i) = value.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(s) = value.as_str() {
                match s.parse::<i64>() {
                    Ok(i) => {
                        warnings.push(format!("coerced_to_integer: {}", field));
                        serde_json::Value::Number(i.into())
                    }
                    Err(_) => {
                        warnings.push(format!("invalid_integer_kept: {}", field));
                        value
                    }
                }
            } else {
                value
            }
        }
        "number" => {
            if value.is_number() {
                value
            } else if let Some(s) = value.as_str() {
                match s.parse::<f64>() {
                    Ok(n) => {
                        warnings.push(format!("coerced_to_number: {}", field));
                        serde_json::json!(n)
                    }
                    Err(_) => {
                        warnings.push(format!("invalid_number_kept: {}", field));
                        value
                    }
                }
            } else {
                value
            }
        }
        "boolean" => {
            if value.is_boolean() {
                value
            } else if let Some(s) = value.as_str() {
                let s = s.to_lowercase();
                match s.as_str() {
                    "true" | "1" | "yes" => {
                        warnings.push(format!("coerced_to_boolean: {}", field));
                        serde_json::Value::Bool(true)
                    }
                    "false" | "0" | "no" => {
                        warnings.push(format!("coerced_to_boolean: {}", field));
                        serde_json::Value::Bool(false)
                    }
                    _ => value,
                }
            } else {
                value
            }
        }
        "array" => {
            if value.is_array() {
                value
            } else {
                warnings.push(format!("wrapped_in_array: {}", field));
                serde_json::Value::Array(vec![value])
            }
        }
        "object" => {
            if value.is_object() {
                value
            } else {
                warnings.push(format!("invalid_object_kept: {}", field));
                value
            }
        }
        _ => value,
    }
}

fn validate_arguments_with_jsonschema(
    arguments: &serde_json::Value,
    schema: &crate::tools::InputSchema,
) -> Vec<String> {
    let schema_json = serde_json::json!({
        "type": schema.schema_type.clone().unwrap_or_else(|| "object".to_string()),
        "description": schema.description.clone().unwrap_or_default(),
        "properties": schema.properties,
        "required": schema.required,
        // We reject extra keys up front in normalize_arguments_for_schema.
        "additionalProperties": false
    });

    let compiled = match draft7::new(&schema_json) {
        Ok(c) => c,
        Err(e) => {
            return vec![format!("invalid_tool_schema: {}", e)];
        }
    };

    let mut errors = Vec::new();
    for err in compiled.iter_errors(arguments) {
        errors.push(err.to_string());
    }
    errors
}

/// Execute a single tool for LLM orchestration.
async fn execute_tool_for_llm(
    state: &Arc<AppState>,
    owner: &str,
    tool_name: &str,
    arguments: serde_json::Value,
    user_connectors: &HashMap<String, Vec<String>>,
) -> Result<serde_json::Value, Response> {
    // Find the tool definition
    let tool_def = state
        .tools
        .tool_by_slug(tool_name)
        .ok_or_else(|| tool_error(StatusCode::NOT_FOUND, &format!("Tool '{}' not found", tool_name)))?;

    // Find the platform (key in user_connectors) that corresponds to this tool's provider
    // tool_def.provider could be "github-pat" but connector might be stored under "github"
    let platform = user_connectors
        .keys()
        .find(|p| {
            // Check if tools_for_provider would return this tool for this platform
            let tools = state.tools.tools_for_provider(p);
            tools.map(|t| t.iter().any(|tool| tool.slug == tool_name)).unwrap_or(false)
        })
        .ok_or_else(|| tool_error(
            StatusCode::BAD_REQUEST,
            &format!("No connected platform provides tool '{}'", tool_name),
        ))?;

    // Check if platform is connected (it should be since we found it above)
    if !user_connectors.contains_key(platform) {
        return Err(tool_error(
            StatusCode::BAD_REQUEST,
            &format!(
                "Platform '{}' is not connected. Connect it at /dashboard first.",
                platform
            ),
        ));
    }

    // Check scope satisfaction
    let granted_scopes = user_connectors.get(platform).cloned().unwrap_or_default();
    let scope_sat = check_scope_satisfied(&tool_def.scopes, &granted_scopes);
    if scope_sat == ScopeSatisfied::No {
        return Ok(serde_json::json!({
            "error": "insufficient_scope",
            "message": format!(
                "The {} tool requires {} scopes, but your {} connection only has {:?}.",
                tool_def.name,
                tool_def.scopes.join(", "),
                platform,
                granted_scopes
            )
        }));
    }

    // Get the connector for this platform (not tool_def.provider which might be different)
    let connector = state.connectors.get(owner, platform).await.map_err(|e| {
        tracing::error!("DB error getting connector: {}", e);
        tool_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to get connector")
    })?.ok_or_else(|| tool_error(
        StatusCode::NOT_FOUND,
        &format!("Connector for platform '{}' not found", platform)
    ))?;

    // Execute the tool via the proxy directly
    let result = execute_tool_direct(state, &connector, tool_def, arguments).await?;

    Ok(result)
}

/// Execute a tool directly using connector credentials (no auth required).
async fn execute_tool_direct(
    state: &Arc<AppState>,
    connector: &crate::oauth::models::ConnectorConfig,
    tool_def: &crate::tools::Tool,
    arguments: serde_json::Value,
) -> Result<serde_json::Value, Response> {
    use crate::api::proxy::get_platform_base_url;

    let base_url = get_platform_base_url(&tool_def.provider);
    if base_url.is_empty() {
        return Err(tool_error(StatusCode::BAD_REQUEST, "Unknown platform"));
    }

    // Build the path by substituting path parameters
    let path = substitute_path_params(&tool_def.endpoint, &arguments);

    // Build query and body from arguments
    let (query_string, body_json) = build_params(&tool_def.method, &arguments, &tool_def.endpoint);

    // Determine auth approach based on engine
    if connector.engine == "nango" {
        // Use Nango's proxy with provider_key and connection_ref
        let Some((nango_base, nango_secret)) = crate::nango::nango_credentials() else {
            return Err(tool_error(StatusCode::SERVICE_UNAVAILABLE, "NANGO_BASE_URL and NANGO_SECRET_KEY must be set"));
        };
        let pk = connector.provider_key.trim();
        let cref = connector.connection_ref.trim();
        if pk.is_empty() || cref.is_empty() {
            return Err(tool_error(StatusCode::UNAUTHORIZED, "nango connector missing provider_key or connection_ref"));
        }

        let method = match tool_def.method {
            crate::tools::HttpMethod::GET => reqwest::Method::GET,
            crate::tools::HttpMethod::POST => reqwest::Method::POST,
            crate::tools::HttpMethod::PUT => reqwest::Method::PUT,
            crate::tools::HttpMethod::DELETE => reqwest::Method::DELETE,
            crate::tools::HttpMethod::PATCH => reqwest::Method::PATCH,
        };

        let path_with_query = if let Some(q) = query_string.as_ref() {
            format!("{}{}", path, q)
        } else {
            path.clone()
        };

        let body_bytes = body_json
            .map(|b| bytes::Bytes::from(b))
            .unwrap_or_default();

        match crate::nango::forward_proxy(
            &nango_base,
            &nango_secret,
            pk,
            cref,
            method,
            &path_with_query,
            body_bytes,
            Some("application/json"),
            None,
        )
        .await
        {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                if !status.is_success() {
                    return Ok(serde_json::json!({
                        "error": "api_error",
                        "status": status.as_u16(),
                        "message": body
                    }));
                }
                let result = serde_json::from_str(&body).unwrap_or_else(|_| {
                    serde_json::json!({ "raw": body })
                });
                return Ok(result);
            }
            Err(e) => {
                tracing::error!("Nango proxy error: {}", e);
                return Err(tool_error(StatusCode::BAD_GATEWAY, &format!("Nango proxy failed: {}", e)));
            }
        }
    }

    // For non-nango engines (PAT-based like github), use direct API call with Bearer token
    let base_url = get_platform_base_url(&tool_def.provider);
    if base_url.is_empty() {
        return Err(tool_error(StatusCode::BAD_REQUEST, "Unknown platform"));
    }

    // Build the full URL
    let full_url = format!("{}{}{}",
        base_url,
        path,
        query_string.as_ref().map(|s| s.as_str()).unwrap_or("")
    );

    // Use client_secret as Bearer token (PAT for GitHub), or X-Api-Key for NPS
    let token = if !connector.client_secret.is_empty() {
        connector.client_secret.clone()
    } else {
        connector.client_id.clone()
    };
    let is_nps = tool_def.provider == "mcp-nationalparks";
    let auth_headers = if is_nps {
        reqwest::header::HeaderMap::from_iter([
            ("X-Api-Key".parse().unwrap(), token.parse().unwrap()),
        ])
    } else {
        reqwest::header::HeaderMap::from_iter([
            (reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap()),
        ])
    };

    // Make the HTTP request
    let client = reqwest::Client::new();
    let method = match tool_def.method {
        crate::tools::HttpMethod::GET => reqwest::Method::GET,
        crate::tools::HttpMethod::POST => reqwest::Method::POST,
        crate::tools::HttpMethod::PUT => reqwest::Method::PUT,
        crate::tools::HttpMethod::DELETE => reqwest::Method::DELETE,
        crate::tools::HttpMethod::PATCH => reqwest::Method::PATCH,
    };

    let mut request = client.request(method, &full_url)
        .headers(auth_headers)
        .header(reqwest::header::CONTENT_TYPE, "application/json");

    if let Some(body) = body_json {
        request = request.body(body);
    }

    let resp = request.send().await.map_err(|e| {
        tracing::error!("HTTP request failed: {}", e);
        tool_error(StatusCode::BAD_GATEWAY, &format!("Request failed: {}", e))
    })?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Ok(serde_json::json!({
            "error": "api_error",
            "status": status.as_u16(),
            "message": body
        }));
    }

    // Try to parse as JSON, fall back to raw string
    let result = serde_json::from_str(&body).unwrap_or_else(|_| {
        serde_json::json!({ "raw": body })
    });

    Ok(result)
}

/// Build the list of tools in LiteLLM format for the LLM.
/// Also returns a map of platform -> (provider_key, scopes) for tool execution.
async fn build_llm_tools(
    state: &Arc<AppState>,
    owner: &str,
    user_connectors: &HashMap<String, Vec<String>>,
    query: &str,
) -> Result<(Vec<crate::llm::LLMTool>, HashMap<String, (String, Vec<String>)>), Response> {
    // First get all connectors with their provider_key
    let all_connectors = state.connectors.list_all().await.map_err(|e| {
        let body = serde_json::json!({ "error": format!("failed to list connectors: {}", e) }).to_string();
        let mut resp = Response::new(body.into());
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        resp
    })?;

    // Build platform -> (provider_key, scopes) mapping for enabled connectors
    let mut platform_provider_map: HashMap<String, (String, Vec<String>)> = HashMap::new();
    for c in all_connectors.iter().filter(|c| c.owner_username == owner && c.enabled) {
        platform_provider_map.insert(c.platform.clone(), (c.provider_key.clone(), c.scopes.clone()));
    }

    let mut tools = Vec::new();
    let query_lower = query.to_lowercase();

    // Gather provider-level context first for intent pre-ranking.
    let mut provider_contexts: HashMap<String, ProviderContext> = HashMap::new();
    for (platform, _granted_scopes) in user_connectors {
        let provider_key = if let Some((pk, _)) = platform_provider_map.get(platform) {
            pk.as_str()
        } else {
            platform
        };
        let provider_tools = state
            .tools
            .tools_for_provider(provider_key)
            .or_else(|| state.tools.tools_for_provider(platform))
            .unwrap_or(&[]);
        if provider_tools.is_empty() {
            continue;
        }
        let ctx = build_provider_context(state, platform, provider_tools);
        provider_contexts.insert(platform.clone(), ctx);
    }

    let selected_platforms = if state.llm_provider_context_enabled {
        select_provider_candidates(&query_lower, &provider_contexts)
    } else {
        user_connectors.keys().cloned().collect()
    };

    for (platform, granted_scopes) in user_connectors {
        if !selected_platforms.contains(platform) {
            continue;
        }
        // Get the provider_key from the platform_provider_map
        // For github-pat, provider_key might be "github-pat__github-pat-6hcb4w" but tools are stored under "github"
        // We need to find which provider has tools for this platform
        let provider_key = if let Some((pk, _)) = platform_provider_map.get(platform) {
            pk.as_str()
        } else {
            platform
        };

        // Try tools_for_provider with provider_key first, then fallback to platform name
        // This handles cases like "github-pat__github-pat-6hcb4w" vs "github"
        let provider_tools = state.tools.tools_for_provider(provider_key)
            .or_else(|| state.tools.tools_for_provider(platform))
            .unwrap_or(&[]);
        let granted: std::collections::HashSet<_> = granted_scopes.iter().collect();

        for tool in provider_tools {
            // Check if all required scopes are satisfied
            let all_scopes_satisfied = tool.scopes.is_empty()
                || tool.scopes.iter().all(|s| granted.contains(s));

            if !all_scopes_satisfied {
                continue; // Skip tools with unsatisfied scopes
            }

            let llm_tool = crate::llm::LLMTool {
                tool_type: "function".to_string(),
                function: crate::llm::LLMFunction {
                    name: tool.slug.clone(),
                    description: llm_tool_description(
                        tool,
                        provider_contexts.get(platform),
                    ),
                    parameters: serde_json::to_value(&tool.input_schema).unwrap_or_else(|_| {
                        serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })
                    }),
                },
            };
            tools.push(llm_tool);
        }
    }

    Ok((tools, platform_provider_map))
}

#[derive(Debug, Clone)]
struct ProviderContext {
    provider_key: String,
    provider_name: String,
    provider_summary: String,
    categories: Vec<String>,
}

fn build_provider_context(
    state: &Arc<AppState>,
    platform: &str,
    provider_tools: &[crate::tools::Tool],
) -> ProviderContext {
    let provider_name = state
        .tools
        .toolkits()
        .iter()
        .find(|t| t.provider == platform)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| platform.to_string());

    let mut tag_count: HashMap<String, usize> = HashMap::new();
    for tool in provider_tools {
        for tag in &tool.tags {
            let t = tag.to_lowercase();
            *tag_count.entry(t).or_insert(0) += 1;
        }
    }
    let mut categories: Vec<(String, usize)> = tag_count.into_iter().collect();
    categories.sort_by(|a, b| b.1.cmp(&a.1));
    let categories = categories.into_iter().map(|(t, _)| t).take(3).collect::<Vec<_>>();

    let provider_summary = if categories.is_empty() {
        format!("APIs for {} workflows", provider_name)
    } else {
        format!("APIs for {} workflows", categories.join(", "))
    };

    ProviderContext {
        provider_key: platform.to_string(),
        provider_name,
        provider_summary,
        categories,
    }
}

fn score_provider_intent(query_lower: &str, ctx: &ProviderContext) -> f64 {
    let mut score = 0.0;
    if query_lower.contains(&ctx.provider_key.to_lowercase())
        || query_lower.contains(&ctx.provider_name.to_lowercase())
    {
        score += 1.0;
    }
    for c in &ctx.categories {
        if query_lower.contains(c) {
            score += 0.4;
        }
    }
    score
}

fn select_provider_candidates(
    query_lower: &str,
    contexts: &HashMap<String, ProviderContext>,
) -> std::collections::HashSet<String> {
    let mut scored: Vec<(String, f64)> = contexts
        .iter()
        .map(|(platform, ctx)| (platform.clone(), score_provider_intent(query_lower, ctx)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let explicit: Vec<String> = scored
        .iter()
        .filter(|(_, s)| *s >= 1.0)
        .map(|(p, _)| p.clone())
        .collect();
    if !explicit.is_empty() {
        return explicit.into_iter().collect();
    }

    let top_n = 4usize;
    scored
        .into_iter()
        .take(top_n)
        .map(|(p, _)| p)
        .collect()
}

fn llm_tool_description(tool: &crate::tools::Tool, provider_ctx: Option<&ProviderContext>) -> String {
    let mut parts = vec![
        format!("{}: {}", tool.name, tool.description.replace('\n', " ").trim()),
        format!("method={:?}", tool.method),
        format!("endpoint={}", tool.endpoint),
    ];
    if let Some(ctx) = provider_ctx {
        parts.push(format!("provider={}", ctx.provider_name));
        parts.push(format!("provider_summary={}", ctx.provider_summary));
        if !ctx.categories.is_empty() {
            parts.push(format!("provider_categories={}", ctx.categories.join(",")));
        }
    }

    let required = schema_required_params(&tool.input_schema);
    if !required.is_empty() {
        parts.push(format!("required_params={}", required.join(",")));
    }

    let optional = schema_optional_params(&tool.input_schema);
    if !optional.is_empty() {
        parts.push(format!("optional_params={}", optional.join(",")));
    }

    if !tool.tags.is_empty() {
        parts.push(format!("tags={}", tool.tags.join(",")));
    }

    if !tool.example_queries.is_empty() {
        parts.push(format!(
            "examples={}",
            tool.example_queries
                .iter()
                .take(2)
                .map(|q| q.replace('\n', " "))
                .collect::<Vec<_>>()
                .join(" | ")
        ));
    }

    parts.join(" ; ")
}

fn schema_required_params(schema: &crate::tools::InputSchema) -> Vec<String> {
    schema.required.clone()
}

fn schema_optional_params(schema: &crate::tools::InputSchema) -> Vec<String> {
    schema
        .properties
        .keys()
        .filter(|k| !schema.required.contains(*k))
        .cloned()
        .collect()
}

fn build_missing_param_details(
    schema: &crate::tools::InputSchema,
    missing_required: &[String],
) -> Vec<serde_json::Value> {
    missing_required
        .iter()
        .map(|name| {
            let spec = schema.properties.get(name);
            let param_type = spec
                .and_then(|s| s.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("unknown");
            let description = spec
                .and_then(|s| s.get("description"))
                .and_then(|d| d.as_str())
                .unwrap_or("No description available");
            let enum_values = spec
                .and_then(|s| s.get("enum"))
                .and_then(|e| e.as_array())
                .map(|arr| {
                    arr.iter()
                        .take(5)
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            serde_json::json!({
                "name": name,
                "type": param_type,
                "description": description,
                "enum_values": enum_values
            })
        })
        .collect()
}

fn build_clarification_question(tool_name: &str, missing_details: &[serde_json::Value]) -> String {
    if missing_details.is_empty() {
        return format!("I need more information to call {}. Could you provide the required fields?", tool_name);
    }
    let fields = missing_details
        .iter()
        .filter_map(|d| d.get("name").and_then(|n| n.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    format!("To run {}, please provide: {}.", tool_name, fields)
}

fn build_schema_repair_hints(
    schema: &crate::tools::InputSchema,
    schema_errors: &[String],
) -> Vec<String> {
    let mut hints = Vec::new();
    for (name, spec) in &schema.properties {
        let t = spec.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        if schema_errors.iter().any(|e| e.contains(name)) {
            let mut hint = format!("Parameter '{}' should be type '{}'", name, t);
            if let Some(enum_values) = spec.get("enum").and_then(|e| e.as_array()) {
                let values = enum_values.iter().take(5).map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
                hint.push_str(&format!(" and one of [{}]", values));
            }
            hints.push(hint);
        }
    }
    if hints.is_empty() {
        hints.push("Arguments must strictly match the JSON schema (types, required, enum constraints).".to_string());
    }
    hints
}

/// Build system prompt for the LLM.
fn build_llm_system_prompt(tools_for_llm: &[crate::llm::LLMTool]) -> String {
    let few_shot = build_dynamic_few_shot_examples(tools_for_llm);
    let mut prompt = r#"You are a precise API tool router and argument extractor.

You have access to tools that can call APIs on behalf of the user. Each tool has:
- A name (e.g., 'github_list_repos')
- A description of what it does
- An input schema specifying required and optional parameters

When the user asks you to do something:
1. Understand what they want to accomplish
2. Select the single best tool
3. Extract only arguments supported by that tool schema
4. Provide correctly typed JSON arguments

IMPORTANT - Argument Extraction:
- For owner/org parameters ("Tesla's repos"), the entity BEFORE "'s" is usually the owner value.
- For slash formats ("openai/gpt-4"), the part BEFORE "/" is typically owner/org, AFTER "/" is typically repo/name.
- Match entities against parameter DESCRIPTIONS to disambiguate: e.g., "Tesla" matches a param with "owner" or "organization" in its description.
- For enum parameters ("sorted by updated"), match the exact enum value from the schema.
- Never fabricate parameter values; if no entity found for a required param, leave it out and explain.
- Map natural language to schema EXACTLY: "repos" -> "repository", "my" possessive -> owner.

IMPORTANT:
- Never invent parameter names not in the schema.
- Always include all required parameters if user gave enough information.
- Respect types exactly: integer/number/boolean/object/array/string.
- Prefer enum values when provided.
- If a required parameter is missing from user input, ask a concise clarification question.
- Do not fabricate IDs, emails, timestamps, or URLs.
- If multiple tools could work, choose the most specific one.
- If tool validation returns missing_required_arguments or schema_validation_failed, correct arguments and retry.
- If retryable=false, ask the user a concise clarification question using missing_param_details.

Tool-call output format:
- Use tool calls with valid JSON arguments.
- Arguments must be a JSON object (not a string).
"#
    .to_string();
    prompt.push_str("\nFew-shot examples:\n");
    prompt.push_str(&few_shot);
    prompt
}

fn build_dynamic_few_shot_examples(tools_for_llm: &[crate::llm::LLMTool]) -> String {
    let mut examples = Vec::new();
    for tool in tools_for_llm.iter().take(3) {
        let name = &tool.function.name;
        let params = tool
            .function
            .parameters
            .get("properties")
            .and_then(|p| p.as_object())
            .map(|o| o.keys().take(2).cloned().collect::<Vec<_>>())
            .unwrap_or_default();

        let args = if params.is_empty() {
            "{}".to_string()
        } else {
            let pairs = params
                .iter()
                .map(|p| format!(r#""{}":"<value>""#, p))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", pairs)
        };

        examples.push(format!(
            "- User intent: use {} ; Tool call: {{\"name\":\"{}\",\"arguments\":{}}}",
            tool.function.description, name, args
        ));
    }

    if examples.is_empty() {
        "- User: \"List records\" ; Tool call: {\"name\":\"tool_name\",\"arguments\":{}}".to_string()
    } else {
        examples.join("\n")
    }
}

// ============================================================================
// Unit tests for rule-based matching (Approach 1)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that tokenize correctly splits queries.
    #[test]
    fn test_tokenize() {
        let query = "list my github repositories sorted by updated";
        let tokens = tokenize(query);

        // Should remove stop words and short tokens
        assert!(!tokens.contains(&"my"));
        assert!(!tokens.contains(&"i"));
        assert!(!tokens.contains(&"a"));
        assert!(tokens.contains(&"list"));
        assert!(tokens.contains(&"github"));
        assert!(tokens.contains(&"repositories"));
    }

    /// Test that score_tool rewards platform mentions.
    #[test]
    fn test_score_tool_platform_match() {
        use crate::tools::{Tool, HttpMethod, InputSchema};

        let tool = Tool {
            slug: "github_list_repos".to_string(),
            name: "List Repositories".to_string(),
            description: "List GitHub repos".to_string(),
            provider: "github".to_string(),
            endpoint: "/user/repos".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema::default(),
            output_schema: None,
            scopes: vec![],
            tags: vec!["repos".to_string()],
            icon_url: None,
            example_queries: vec![],
        };

        let query_lower = "list my github repositories";
        let query_tokens = tokenize(query_lower);

        let score = score_tool(query_lower, &query_tokens, &tool, "github");

        // Platform match should contribute 20%
        assert!(score >= 0.2, "score should include platform match bonus");
    }

    /// Test that score_tool rewards action verb matches.
    #[test]
    fn test_score_tool_action_match() {
        use crate::tools::{Tool, HttpMethod, InputSchema};

        let tool = Tool {
            slug: "github_list_repos".to_string(),
            name: "List Repositories".to_string(),
            description: "List GitHub repos".to_string(),
            provider: "github".to_string(),
            endpoint: "/user/repos".to_string(),
            method: HttpMethod::GET, // GET method
            input_schema: InputSchema::default(),
            output_schema: None,
            scopes: vec![],
            tags: vec![],
            icon_url: None,
            example_queries: vec![],
        };

        // Query with "list" should match GET method tools
        let query_lower = "list repositories";
        let query_tokens = tokenize(query_lower);

        let score = score_tool(query_lower, &query_tokens, &tool, "github");
        assert!(score > 0.0, "score should be positive for action match");
    }

    /// Test that detect_platform_from_query finds github.
    #[test]
    fn test_detect_platform_github() {
        let connected: std::collections::HashMap<String, Vec<String>> =
            [("github".to_string(), vec!["repo".to_string()])]
            .into_iter()
            .collect();

        let query = "list my github repositories";
        let platform = detect_platform_from_query(query, &connected);

        assert_eq!(platform, Some("github".to_string()));
    }

    /// Test that detect_platform_from_query returns None for unconnected platforms.
    #[test]
    fn test_detect_platform_not_connected() {
        let connected: std::collections::HashMap<String, Vec<String>> =
            [("slack".to_string(), vec!["channels:read".to_string()])]
            .into_iter()
            .collect();

        let query = "list my github repositories";
        let platform = detect_platform_from_query(query, &connected);

        assert_eq!(platform, None, "should not detect github if not connected");
    }

    /// Test extract_arguments extracts sort parameter.
    #[test]
    fn test_extract_arguments_sort() {
        use crate::tools::{Tool, HttpMethod, InputSchema};

        let tool = Tool {
            slug: "github_list_repos".to_string(),
            name: "List Repositories".to_string(),
            description: "List GitHub repos".to_string(),
            provider: "github".to_string(),
            endpoint: "/user/repos".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema {
                schema_type: Some("object".to_string()),
                description: Some("".to_string()),
                properties: [(
                    "sort".to_string(),
                    serde_json::json!({
                        "type": "string",
                        "description": "Sort order",
                        "enum": ["created", "updated"]
                    }),
                )]
                .into_iter()
                .collect(),
                required: vec![],
            },
            output_schema: None,
            scopes: vec![],
            tags: vec![],
            icon_url: None,
            example_queries: vec![],
        };

        let query = "list repositories sorted by updated";
        let args = extract_arguments(query, &tool);

        assert_eq!(args.get("sort").and_then(|v| v.as_str()), Some("updated"));
    }

    /// Test extract_arguments handles per_page for "all" keyword.
    #[test]
    fn test_extract_arguments_per_page_all() {
        use crate::tools::{Tool, HttpMethod, InputSchema};

        let tool = Tool {
            slug: "github_list_repos".to_string(),
            name: "List Repositories".to_string(),
            description: "List GitHub repos".to_string(),
            provider: "github".to_string(),
            endpoint: "/user/repos".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema {
                schema_type: Some("object".to_string()),
                description: Some("".to_string()),
                properties: [(
                    "per_page".to_string(),
                    serde_json::json!({
                        "type": "integer",
                        "description": "Results per page"
                    }),
                )]
                .into_iter()
                .collect(),
                required: vec![],
            },
            output_schema: None,
            scopes: vec![],
            tags: vec![],
            icon_url: None,
            example_queries: vec![],
        };

        let query = "list all repositories";
        let args = extract_arguments(query, &tool);

        assert_eq!(
            args.get("per_page").and_then(|v| v.as_u64()),
            Some(100),
            "per_page should be 100 when query contains 'all'"
        );
    }

    /// Test similarity function.
    #[test]
    fn test_similarity() {
        let a = "list my github repositories";
        let b = "show my github repos";

        let sim = similarity(a, b);
        assert!(
            sim > 0.1,
            "similar queries should have positive similarity, got {}",
            sim
        );
        assert!(sim < 1.0, "different queries should not have full similarity");
    }

    /// Test that similarity returns 0 for empty input.
    #[test]
    fn test_similarity_empty() {
        let sim = similarity("", "list github repos");
        assert_eq!(sim, 0.0);

        let sim2 = similarity("list github repos", "");
        assert_eq!(sim2, 0.0);
    }
}

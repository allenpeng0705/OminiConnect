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
use serde::{Deserialize, Serialize};

use crate::api::tools::{auth_user, check_scope_satisfied, ScopeSatisfied};
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
    let arguments = extract_arguments(&body.query, selected_tool);

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

    let arguments = extract_arguments(&query.query, tool);
    let missing: Vec<String> = tool
        .input_schema
        .required
        .iter()
        .filter(|r| {
            arguments
                .as_object()
                .map(|o| !o.contains_key(*r))
                .unwrap_or(true)
        })
        .cloned()
        .collect();

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

    for token in tokens {
        if STOP_WORDS.contains(token) {
            continue;
        }
        if token.len() < 2 {
            continue;
        }
        if searchable.contains(*token) {
            matches += 1;
        }
    }

    (matches as f64) / (total as f64)
}

fn score_example_queries(query_lower: &str, tool: &crate::tools::Tool) -> f64 {
    if tool.example_queries.is_empty() {
        // Fall back to name matching
        let name_words = tool.name.to_lowercase();
        let query_words: HashSet<_> = query_lower.split_whitespace().collect();
        let name_set: HashSet<_> = name_words.split_whitespace().collect();
        let intersection: Vec<_> = query_words.intersection(&name_set).collect();
        if intersection.is_empty() {
            return 0.0;
        }
        return (intersection.len() as f64) / (name_set.len().max(1) as f64);
    }

    let mut max_sim: f64 = 0.0;
    for example in &tool.example_queries {
        let example_lower = example.to_lowercase();
        let sim = similarity(query_lower, &example_lower);
        max_sim = max_sim.max(sim);
    }
    max_sim
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
    let mut args = serde_json::Map::new();
    let query_lower = query.to_lowercase();

    // Try to extract common parameters from query
    for (param_name, param_spec) in &tool.input_schema.properties {
        let param_lower = param_name.to_lowercase();

        // Check if query mentions this parameter
        if query_lower.contains(&param_lower) {
            // Try to extract value after the parameter name
            if let Some(value) = extract_value_after_word(&query_lower, &param_lower) {
                args.insert(param_name.clone(), serde_json::Value::String(value));
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
}

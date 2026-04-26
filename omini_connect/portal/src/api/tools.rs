//! Tool registry API endpoints.

use std::{collections::HashSet, sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use http_body_util::BodyExt;
use regex::Regex;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::app::AppState;
use crate::oauth::models::ConnectorConfig;
use crate::tools::{HttpMethod, Tool};

impl HttpMethod {
    fn as_reqwest_method(&self) -> reqwest::Method {
        match self {
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
        }
    }
}

/// Query params for listing tools.
#[derive(Debug, Deserialize)]
pub struct ListToolsQuery {
    /// Optional platform/provider filter.
    pub platform: Option<String>,
}

/// Query params for searching tools.
#[derive(Debug, Deserialize)]
pub struct SearchToolsQuery {
    /// Search query (matches name, description, tags, slug).
    pub q: String,
    /// Optional platform/provider filter.
    pub platform: Option<String>,
    /// Optional: only return tools with these scopes satisfied ("yes" | "no" | "any").
    pub filter_scope: Option<String>,
}

/// Response: list of toolkits with their tools.
#[derive(Debug, Serialize)]
pub struct ListToolsResponse {
    pub toolkits: Vec<ToolkitWithTools>,
}

#[derive(Debug, Serialize)]
pub struct ToolkitWithTools {
    pub slug: String,
    pub name: String,
    pub logo: Option<String>,
    pub provider: String,
    pub tools: Vec<ToolSummary>,
}

/// Whether the tool's required scopes are satisfied by the connector.
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScopeSatisfied {
    Yes,
    No,
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct ToolSummary {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub method: String,
    pub endpoint: String,
    /// Required scopes for this tool.
    pub scopes: Vec<String>,
    /// Whether the connector has all required scopes.
    pub scope_satisfied: ScopeSatisfied,
    pub tags: Vec<String>,
}

/// List tools for the authenticated user's connectors.
pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListToolsQuery>,
    headers: HeaderMap,
) -> Result<Json<ListToolsResponse>, Response> {
    let (owner, _agent_id) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let platform = query.platform;

    let toolkits = if let Some(p) = platform {
        // Single platform requested - get connector scopes
        let connector_scopes = get_connector_scopes(&state, &owner, &p).await;
        let tools = state.tools.tools_for_provider(&p).unwrap_or(&[]);
        let toolkit = get_or_create_toolkit(&state, &p);
        vec![build_toolkit_response(&toolkit, tools, &connector_scopes)]
    } else {
        // All platforms - find which ones the user has connectors for
        let connectors = state.connectors.list_all().await
            .map_err(|e| tool_error(StatusCode::INTERNAL_SERVER_ERROR, &*format!("failed to list connectors: {}", e)))?;

        // Build map of platform -> granted scopes for user's connectors
        let scopes_by_platform: std::collections::HashMap<_, _> = connectors
            .into_iter()
            .filter(|c| c.owner_username == owner && c.enabled)
            .map(|c| (c.platform.clone(), c.scopes))
            .collect();

        let platforms: std::collections::HashSet<_> = scopes_by_platform.keys().cloned().collect();

        state.tools.toolkits()
            .iter()
            .filter(|t| platforms.contains(&t.provider) || platforms.contains(&t.slug))
            .map(|t| {
                let tools = state.tools.tools_for_provider(&t.provider).unwrap_or(&[]);
                let granted_scopes = scopes_by_platform.get(&t.provider).cloned().unwrap_or_default();
                build_toolkit_response(t, tools, &granted_scopes)
            })
            .collect()
    };

    Ok(Json(ListToolsResponse { toolkits }))
}

/// Response for search results (flat list of matching tools).
#[derive(Debug, Serialize)]
pub struct SearchToolsResponse {
    pub tools: Vec<ToolSummary>,
    pub query: String,
}

/// Search tools by name, description, tags, or slug.
pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchToolsQuery>,
    headers: HeaderMap,
) -> Result<Json<SearchToolsResponse>, Response> {
    let (owner, _agent_id) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    let q = query.q.to_lowercase();
    let platform_filter = query.platform.as_ref();
    let filter_scope = query.filter_scope.as_deref();

    // Get user's connectors and their scopes
    let connectors = state.connectors.list_all().await
        .map_err(|e| tool_error(StatusCode::INTERNAL_SERVER_ERROR, &*format!("failed to list connectors: {}", e)))?;

    let user_connectors: std::collections::HashMap<_, _> = connectors
        .into_iter()
        .filter(|c| c.owner_username == owner && c.enabled)
        .map(|c| (c.platform.clone(), c.scopes))
        .collect();

    let all_tools: Vec<_> = state.tools.toolkits()
        .iter()
        .flat_map(|t| {
            let toolkit = t.clone();
            state.tools.tools_for_provider(&t.provider)
                .unwrap_or(&[])
                .iter()
                .map(move |tool| (toolkit.clone(), tool))
        })
        .collect();

    let matching_tools: Vec<ToolSummary> = all_tools
        .into_iter()
        .filter(|(toolkit, tool)| {
            // Platform filter
            if let Some(p) = platform_filter {
                if &toolkit.provider != p && &toolkit.slug != p {
                    return false;
                }
            }

            // Search filter - match against name, description, tags, slug
            let search_text = format!(
                "{} {} {} {}",
                tool.slug,
                tool.name,
                tool.description.replace('\n', " "),
                tool.tags.join(" ")
            ).to_lowercase();

            if !search_text.contains(&q) {
                return false;
            }

            // Scope filter
            if let Some(filter) = filter_scope {
                let granted = user_connectors.get(&tool.provider).cloned().unwrap_or_default();
                let scope_sat = check_scope_satisfied(&tool.scopes, &granted);

                match (filter, scope_sat) {
                    ("yes", ScopeSatisfied::Yes) => true,
                    ("no", ScopeSatisfied::No) => true,
                    ("any", _) => true,
                    ("yes", _) | ("no", _) | (_, ScopeSatisfied::Unknown) => false,
                    _ => true,
                }
            } else {
                true
            }
        })
        .map(|(_toolkit, tool)| {
            let granted = user_connectors.get(&tool.provider).cloned().unwrap_or_default();
            let scope_sat = check_scope_satisfied(&tool.scopes, &granted);
            ToolSummary {
                slug: tool.slug.clone(),
                name: tool.name.clone(),
                description: tool.description.clone(),
                method: format!("{:?}", tool.method),
                endpoint: tool.endpoint.clone(),
                scopes: tool.scopes.clone(),
                scope_satisfied: scope_sat,
                tags: tool.tags.clone(),
            }
        })
        .collect();

    Ok(Json(SearchToolsResponse {
        tools: matching_tools,
        query: query.q.clone(),
    }))
}

/// Get the connector's granted scopes for a platform.
async fn get_connector_scopes(
    state: &Arc<AppState>,
    owner: &str,
    platform: &str,
) -> Vec<String> {
    match state.connectors.get(owner, platform).await {
        Ok(Some(c)) if c.enabled => c.scopes,
        _ => Vec::new(),
    }
}

/// Get toolkit from registry or create a default one.
fn get_or_create_toolkit(state: &Arc<AppState>, provider: &str) -> crate::tools::Toolkit {
    state.tools.toolkits()
        .iter()
        .find(|t| t.provider == provider)
        .cloned()
        .unwrap_or_else(|| crate::tools::Toolkit {
            slug: provider.to_string(),
            name: provider.to_string(),
            logo: Some(format!("/images/template-logos/{}.svg", provider)),
            provider: provider.to_string(),
        })
}

/// Check if all required scopes are satisfied by granted scopes.
pub fn check_scope_satisfied(required: &[String], granted: &[String]) -> ScopeSatisfied {
    if required.is_empty() {
        return ScopeSatisfied::Yes;
    }
    if granted.is_empty() {
        return ScopeSatisfied::Unknown;
    }
    let granted_set: std::collections::HashSet<_> = granted.iter().collect();
    if required.iter().all(|r| granted_set.contains(r)) {
        ScopeSatisfied::Yes
    } else {
        ScopeSatisfied::No
    }
}

fn build_toolkit_response(
    toolkit: &crate::tools::Toolkit,
    tools: &[Tool],
    granted_scopes: &[String],
) -> ToolkitWithTools {
    ToolkitWithTools {
        slug: toolkit.slug.clone(),
        name: toolkit.name.clone(),
        logo: toolkit.logo.clone(),
        provider: toolkit.provider.clone(),
        tools: tools.iter().map(|t| {
            let scope_satisfied = check_scope_satisfied(&t.scopes, granted_scopes);
            ToolSummary {
                slug: t.slug.clone(),
                name: t.name.clone(),
                description: t.description.clone(),
                method: format!("{:?}", t.method),
                endpoint: t.endpoint.clone(),
                scopes: t.scopes.clone(),
                scope_satisfied,
                tags: t.tags.clone(),
            }
        }).collect(),
    }
}

/// Request body for tool execution.
#[derive(Debug, Deserialize)]
pub struct ExecuteToolRequest {
    pub tool_slug: String,
    pub platform: String,
    pub arguments: serde_json::Value,
    /// Optional callback URL for async result delivery.
    /// When provided, the call returns 202 Accepted with a call_id immediately,
    /// and the result is POSTed to the callback URL once ready.
    #[serde(default)]
    pub callback_url: Option<String>,
}

/// Execute a tool.
pub async fn execute(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ExecuteToolRequest>,
) -> Response {
    let (owner, agent_id) = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    // Look up tool
    let tool = match state.tools.tool_by_slug(&body.tool_slug) {
        Some(t) => t,
        None => {
            return tool_error(StatusCode::NOT_FOUND, "tool not found");
        }
    };

    // Get connector for (owner, platform)
    let connector = match state.connectors.get(&owner, &body.platform).await {
        Ok(Some(c)) if c.enabled => c,
        Ok(Some(_)) => return tool_error(StatusCode::FORBIDDEN, "connector is disabled"),
        Ok(None) => return tool_error(StatusCode::NOT_FOUND, "platform not configured"),
        Err(_) => return tool_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to get connector"),
    };

    // Build the native path by substituting path parameters
    let native_path = substitute_path_params(&tool.endpoint, &body.arguments);

    // Build query string from arguments
    let (query_string, body_json) = build_params(&tool.method, &body.arguments, &tool.endpoint);

    // If callback_url is provided, return 202 immediately and fire-and-forget
    if let Some(callback_url) = &body.callback_url {
        let call_id = uuid::Uuid::new_v4().to_string();
        let owner_clone = owner.clone();
        let agent_id_str = agent_id.clone();
        let agent_id_for_record = agent_id_str.clone().unwrap_or_else(|| "".to_string());
        let exec_record_for_callback = crate::db::ToolExecution {
            id: call_id.clone(),
            agent_id: agent_id_for_record.clone(),
            owner_username: owner_clone.clone(),
            tool_slug: body.tool_slug.clone(),
            platform: body.platform.clone(),
            arguments: body.arguments.clone(),
            result: "".to_string(),
            status: "pending".to_string(),
            duration_ms: 0,
            created_at: chrono::Utc::now(),
        };
        // Record as pending
        if let Err(e) = state.tool_executions.insert(&exec_record_for_callback).await {
            tracing::warn!("Failed to record pending tool execution: {}", e);
        }

        // Spawn async task to execute and post to callback
        let state_clone = Arc::clone(&state);
        let tool_clone = tool.clone();
        let connector_clone = connector.clone();
        let native_path_clone = native_path.clone();
        let query_string_clone = query_string.clone();
        let body_json_clone = body_json.clone();
        let callback_url_clone = callback_url.clone();
        let call_id_clone = call_id.clone();
        let agent_id_for_async = agent_id_str.clone().unwrap_or_else(|| "".to_string());

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            let exec_result = if connector_clone.engine == "nango" {
                execute_nango(&state_clone, &connector_clone, &tool_clone.method, &native_path_clone, query_string_clone, body_json_clone).await
            } else if connector_clone.platform == "maton" || connector_clone.platform == "qqmail" || connector_clone.platform == "github" {
                execute_api_key(&connector_clone, &tool_clone.method, &native_path_clone, query_string_clone, body_json_clone).await
            } else {
                execute_oauth_vault(&state_clone, &owner_clone, &connector_clone, &tool_clone.method, &native_path_clone, query_string_clone, body_json_clone).await
            };

            let duration_ms = start.elapsed().as_millis() as i64;
            let status_code = exec_result.status();
            let status = if status_code.is_success() { "success" } else { "error" };

            // Extract body
            let body_bytes = {
                use axum::body::HttpBody;
                let mut body = exec_result.into_body();
                let mut bytes = bytes::BytesMut::new();
                while let Some(item) = body.frame().await {
                    if let Ok(frame) = item {
                        if let Some(data) = frame.data_ref() {
                            bytes.extend_from_slice(data);
                        }
                    }
                }
                bytes.freeze()
            };
            let result_text = String::from_utf8_lossy(&body_bytes).to_string();

            // Update audit record with result
            // We update by inserting a new record with same id (upsert behavior is ok for this simple case)
            let updated_record = crate::db::ToolExecution {
                id: call_id_clone.clone(),
                agent_id: agent_id_for_async.clone(),
                owner_username: owner_clone,
                tool_slug: tool_clone.slug.clone(),
                platform: tool_clone.provider.clone(),
                arguments: serde_json::Value::Object(Default::default()),
                result: result_text.clone(),
                status: status.to_string(),
                duration_ms,
                created_at: chrono::Utc::now(),
            };
            let _ = state_clone.tool_executions.insert(&updated_record).await;

            // POST result to callback_url
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "call_id": call_id_clone,
                "tool_slug": tool_clone.slug,
                "platform": tool_clone.provider,
                "status": status,
                "result": result_text,
                "duration_ms": duration_ms,
            });
            match client.post(&callback_url_clone).json(&payload).send().await {
                Ok(resp) => tracing::info!("Callback posted to {}: {}", callback_url_clone, resp.status()),
                Err(e) => tracing::warn!("Callback to {} failed: {}", callback_url_clone, e),
            }
        });

        // Return 202 Accepted immediately
        let body = serde_json::json!({ "call_id": call_id, "status": "pending" }).to_string();
        let mut resp = axum::response::Response::new(body.into());
        *resp.status_mut() = StatusCode::ACCEPTED;
        return resp;
    }

    // Inline execution (no callback)
    let start = std::time::Instant::now();
    let exec_result: Response;

    // Forward based on connector type (similar to proxy.rs)
    if connector.engine == "nango" {
        exec_result = execute_nango(&state, &connector, &tool.method, &native_path, query_string, body_json).await;
    } else if connector.platform == "maton" || connector.platform == "qqmail" || connector.platform == "github" {
        exec_result = execute_api_key(&connector, &tool.method, &native_path, query_string, body_json).await;
    } else {
        exec_result = execute_oauth_vault(&state, &owner, &connector, &tool.method, &native_path, query_string, body_json).await;
    };

    let duration_ms = start.elapsed().as_millis() as i64;
    let status_code = exec_result.status();
    let status = if status_code.is_success() { "success" } else { "error" };

    // Extract body bytes for audit log (body is consumed from Response)
    let body_bytes = {
        use axum::body::HttpBody;
        let mut body = exec_result.into_body();
        let mut bytes = bytes::BytesMut::new();
        while let Some(item) = body.frame().await {
            if let Ok(frame) = item {
                if let Some(data) = frame.data_ref() {
                    bytes.extend_from_slice(data);
                }
            }
        }
        bytes.freeze()
    };
    let result_text = String::from_utf8_lossy(&body_bytes).to_string();

    // Record to audit log
    let exec_record = crate::db::ToolExecution {
        id: uuid::Uuid::new_v4().to_string(),
        agent_id: agent_id.unwrap_or_else(|| "".to_string()),
        owner_username: owner,
        tool_slug: body.tool_slug.clone(),
        platform: body.platform.clone(),
        arguments: body.arguments,
        result: result_text.clone(),
        status: status.to_string(),
        duration_ms,
        created_at: chrono::Utc::now(),
    };
    if let Err(e) = state.tool_executions.insert(&exec_record).await {
        tracing::warn!("Failed to record tool execution audit: {}", e);
    }

    let mut resp = axum::response::Response::new(result_text.into());
    *resp.status_mut() = status_code;
    resp
}

/// Substitutes {path_param} placeholders in the endpoint with values from arguments.
fn substitute_path_params(endpoint: &str, arguments: &serde_json::Value) -> String {
    let mut result = endpoint.to_string();
    if let Some(obj) = arguments.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{}}}", key);
            if let Some(val_str) = value.as_str() {
                result = result.replace(&placeholder, val_str);
            } else {
                result = result.replace(&placeholder, &value.to_string());
            }
        }
    }
    result
}

/// Builds query string and JSON body from arguments.
fn build_params(
    method: &HttpMethod,
    arguments: &serde_json::Value,
    endpoint: &str,
) -> (Option<String>, Option<String>) {
    let obj = match arguments.as_object() {
        Some(o) => o,
        None => return (None, None),
    };

    // Find which args are path params (already substituted)
    let path_params: HashSet<String> = Regex::new(r"\{([^}]+)\}")
        .unwrap()
        .captures_iter(endpoint)
        .map(|c| c[1].to_string())
        .collect();

    let mut query_pairs: Vec<(String, String)> = Vec::new();
    let mut body_props: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

    for (key, value) in obj {
        if path_params.contains(key.as_str()) {
            continue; // Already substituted in path
        }

        if matches!(method, HttpMethod::GET) {
            // GET params go to query string
            if let Some(s) = value.as_str() {
                query_pairs.push((key.clone(), s.to_string()));
            } else {
                query_pairs.push((key.clone(), value.to_string()));
            }
        } else {
            // POST/PUT/PATCH params go to body
            body_props.insert(key.clone(), value.clone());
        }
    }

    let query_string = if query_pairs.is_empty() {
        None
    } else {
        Some(query_pairs
            .iter()
            .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
            .collect::<Vec<_>>()
            .join("&"))
    };

    let body_json = if body_props.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&body_props).unwrap_or_default())
    };

    (query_string, body_json)
}

/// Execute via Nango.
async fn execute_nango(
    _state: &Arc<AppState>,
    connector: &ConnectorConfig,
    method: &HttpMethod,
    native_path: &str,
    query_string: Option<String>,
    body_json: Option<String>,
) -> Response {
    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return tool_error(StatusCode::SERVICE_UNAVAILABLE, "Nango not configured");
    };

    let pk = connector.provider_key.trim();
    let cref = connector.connection_ref.trim();
    if pk.is_empty() || cref.is_empty() {
        return tool_error(StatusCode::UNAUTHORIZED, "Nango connector not fully configured");
    }

    // Build full path with query string
    let full_path = match (&query_string, native_path.contains('?')) {
        (Some(q), false) => format!("{}?{}", native_path, q),
        (Some(q), true) => format!("{}&{}", native_path, q),
        (None, _) => native_path.to_string(),
    };

    let body = body_json.clone().map(|b| b.into()).unwrap_or_default();
    let reqwest_method = method.as_reqwest_method();
    let body_content_type = body_json.as_ref().map(|s| s.as_str());

    match crate::nango::forward_proxy(
        &base,
        &secret,
        pk,
        cref,
        reqwest_method,
        &full_path,
        body,
        body_content_type,
        None,
    )
    .await
    {
        Ok(resp) => map_reqwest_to_axum(resp).await,
        Err(e) => {
            tracing::error!("Nango tool execution error: {}", e);
            tool_error(StatusCode::BAD_GATEWAY, "Nango proxy request failed")
        }
    }
}

/// Execute for API-key / PAT platforms (maton, qqmail, github with native engine — same pattern as `api/proxy.rs`).
async fn execute_api_key(
    connector: &ConnectorConfig,
    method: &HttpMethod,
    native_path: &str,
    query_string: Option<String>,
    body_json: Option<String>,
) -> Response {
    let access_token = match connector.platform.as_str() {
        "maton" | "github" => {
            let t = connector.client_secret.trim();
            if t.is_empty() {
                connector.client_id.trim().to_string()
            } else {
                connector.client_secret.clone()
            }
        }
        _ => connector.client_secret.clone(),
    };
    if access_token.is_empty() {
        return tool_error(StatusCode::UNAUTHORIZED, "API key or PAT not configured");
    }

    let base_url = match get_platform_base_url(&connector.platform) {
        Some(url) => url,
        None => return tool_error(StatusCode::BAD_GATEWAY, "unsupported platform for direct API access"),
    };
    let full_url = match &query_string {
        Some(q) => format!("{}/{}?{}", base_url, native_path, q),
        None => format!("{}/{}", base_url, native_path),
    };

    let client = match reqwest::Client::builder().timeout(Duration::from_secs(60)).build() {
        Ok(c) => c,
        Err(_) => return tool_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to create HTTP client"),
    };

    let mut req_builder = client.request(method.as_reqwest_method(), &full_url);
    req_builder = req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
    req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");

    if let Some(body) = body_json {
        req_builder = req_builder.header("Content-Type", "application/json").body(reqwest::Body::from(body));
    }

    match req_builder.send().await {
        Ok(resp) => map_reqwest_to_axum(resp).await,
        Err(e) => {
            tracing::error!("API-key tool execution error: {}", e);
            tool_error(StatusCode::BAD_GATEWAY, "upstream request failed")
        }
    }
}

/// Execute for OAuth vault platforms.
async fn execute_oauth_vault(
    state: &Arc<AppState>,
    owner: &str,
    connector: &ConnectorConfig,
    method: &HttpMethod,
    native_path: &str,
    query_string: Option<String>,
    body_json: Option<String>,
) -> Response {
    let vk = crate::connector_scope::oauth_vault_platform_key(owner, &connector.platform);
    let access_token = match state.oauth_vault.get_token(&vk, "user").await {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("No token for tool execution: {}", e);
            return tool_error(StatusCode::UNAUTHORIZED, "no token available");
        }
    };

    let base_url = match get_platform_base_url(&connector.platform) {
        Some(url) => url,
        None => return tool_error(StatusCode::BAD_GATEWAY, "unsupported platform for direct API access"),
    };
    let full_url = match &query_string {
        Some(q) => format!("{}/{}?{}", base_url, native_path, q),
        None => format!("{}/{}", base_url, native_path),
    };

    let client = match reqwest::Client::builder().timeout(Duration::from_secs(60)).build() {
        Ok(c) => c,
        Err(_) => return tool_error(StatusCode::INTERNAL_SERVER_ERROR, "failed to create HTTP client"),
    };

    let mut req_builder = client.request(method.as_reqwest_method(), &full_url);
    req_builder = req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
    req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");

    if let Some(body) = body_json {
        req_builder = req_builder.header("Content-Type", "application/json").body(reqwest::Body::from(body));
    }

    match req_builder.send().await {
        Ok(resp) => map_reqwest_to_axum(resp).await,
        Err(e) => {
            tracing::error!("OAuth vault tool execution error: {}", e);
            tool_error(StatusCode::BAD_GATEWAY, "upstream request failed")
        }
    }
}

/// Authenticate via Bearer token and return (owner_username, agent_id).
async fn auth_user(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<(String, Option<String>), Response> {
    let api_key = match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        Some(v) => v.strip_prefix("Bearer ").unwrap_or(v),
        None => {
            return Err(tool_error(StatusCode::UNAUTHORIZED, "missing authorization header"));
        }
    };

    let api_keys = state.api_keys.list_all().await
        .map_err(|e| tool_error(StatusCode::INTERNAL_SERVER_ERROR, &*format!("failed to list API keys: {}", e)))?;

    for ak in api_keys {
        if bcrypt::verify(api_key, &ak.key_hash).ok() == Some(true) {
            return Ok((ak.username, ak.agent_id));
        }
    }

    Err(tool_error(StatusCode::UNAUTHORIZED, "invalid API key"))
}

async fn map_reqwest_to_axum(resp: reqwest::Response) -> Response {
    let status = resp.status();
    let body = resp.bytes().await.unwrap_or_default();
    let code = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let mut response = Response::new(body.into());
    *response.status_mut() = code;
    response
}

fn tool_error(status: StatusCode, message: &str) -> Response {
    let body = serde_json::json!({ "error": message }).to_string();
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response
}

fn get_platform_base_url(platform: &str) -> Option<&'static str> {
    Some(match platform {
        "github" => "https://api.github.com",
        "feishu" => "https://open.feishu.cn/open-apis",
        "dingtalk" => "https://api.dingtalk.com",
        "wechatwork" => "https://qyapi.weixin.qq.com",
        "linkedin" => "https://api.linkedin.com",
        "facebook" => "https://graph.facebook.com/v21.0",
        "x" => "https://api.x.com/2",
        "maton" => "https://api.maton.ai",
        "qqmail" => "https://api.exmail.qq.com",
        "slack" => "https://slack.com/api",
        "notion" => "https://api.notion.com/v1",
        "hubspot" => "https://api.hubapi.com",
        "salesforce" => "https://login.salesforce.com/services/oauth2",
        "gitlab" => "https://gitlab.com/api/v4",
        "jira" => "https://api.atlassian.com",
        "confluence" => "https://api.atlassian.com",
        "google" => "https://www.googleapis.com",
        "zoom" => "https://api.zoom.us/v2",
        "stripe" => "https://api.stripe.com/v1",
        "shopify" => "https://{shop}.myshopify.com/admin/api",
        _ => return None,
    })
}

/// Router for tools API.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tools", get(list))
        .route("/tools/search", get(search))
        .route("/tools/execute", post(execute))
}

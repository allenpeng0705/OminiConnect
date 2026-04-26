//! MCP (Model Context Protocol) server endpoint for AI agent integration.
//!
//! MCP is a standardized protocol for AI models to discover and call tools.
//! This implementation provides an MCP-compatible `/mcp` endpoint.
//!
//! MCP uses JSON-RPC 2.0 with these methods:
//! - `tools/list` - Returns available tools
//! - `tools/call` - Executes a tool
//!
//! The endpoint accepts POST requests with JSON-RPC body and returns SSE stream.

use std::{sync::Arc, collections::HashMap};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::api::tools::{ScopeSatisfied, check_scope_satisfied};

/// MCP JSON-RPC request.
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    #[serde(default)]
    params: serde_json::Value,
}

/// MCP JSON-RPC response.
#[derive(Debug, Serialize)]
pub struct McpResponse {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

#[derive(Debug, Serialize)]
pub struct McpError {
    code: i32,
    message: String,
}

impl McpResponse {
    fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: serde_json::Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError { code, message }),
        }
    }
}

/// MCP tool representation.
#[derive(Debug, Serialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_satisfied: Option<String>,
}

impl From<&crate::tools::Tool> for McpTool {
    fn from(tool: &crate::tools::Tool) -> Self {
        let properties: HashMap<String, serde_json::Value> = tool.input_schema.properties
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let input_schema = serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": tool.input_schema.required,
        });

        Self {
            name: tool.slug.clone(),
            description: tool.description.clone(),
            input_schema,
            scope_satisfied: None,
        }
    }
}

/// Handle MCP JSON-RPC request.
pub async fn handle_mcp(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<McpRequest>,
) -> Response {
    // Auth check
    let owner = match auth_user(&state, &headers).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    match request.method.as_str() {
        "tools/list" => handle_tools_list(state, owner, request.id).await,
        "tools/call" => handle_tools_call(state, owner, request.id, request.params).await,
        _ => {
            let response = McpResponse::error(
                request.id,
                -32601,
                format!("Method not found: {}", request.method),
            );
            let body = serde_json::to_string(&response).unwrap_or_default();
            let mut resp = Response::new(body.into());
            *resp.status_mut() = StatusCode::OK;
            resp
        }
    }
}

/// Get user's connectors and their granted scopes.
async fn get_user_connectors(
    state: &Arc<AppState>,
    owner: &str,
) -> Result<HashMap<String, Vec<String>>, Response> {
    let connectors = state.connectors.list_all().await
        .map_err(|e| {
            let response = McpResponse::error(
                serde_json::Value::Null,
                -32603,
                format!("Internal error: {}", e),
            );
            let body = serde_json::to_string(&response).unwrap_or_default();
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

/// Handle tools/list method.
async fn handle_tools_list(
    state: Arc<AppState>,
    owner: String,
    id: serde_json::Value,
) -> Response {
    let user_connectors = match get_user_connectors(&state, &owner).await {
        Ok(c) => c,
        Err(e) => return e,
    };

    let tools: Vec<McpTool> = state.tools.toolkits()
        .iter()
        .filter(|t| user_connectors.contains_key(&t.provider))
        .flat_map(|t| {
            let granted_scopes = user_connectors.get(&t.provider).cloned().unwrap_or_default();
            state.tools.tools_for_provider(&t.provider)
                .unwrap_or(&[])
                .iter()
                .map(move |tool| {
                    let scope_sat = crate::api::tools::check_scope_satisfied(&tool.scopes, &granted_scopes);
                    let mut mcpt = McpTool::from(tool);
                    mcpt.scope_satisfied = Some(match scope_sat {
                        crate::api::tools::ScopeSatisfied::Yes => "yes".to_string(),
                        crate::api::tools::ScopeSatisfied::No => "no".to_string(),
                        crate::api::tools::ScopeSatisfied::Unknown => "unknown".to_string(),
                    });
                    mcpt
                })
        })
        .collect();

    let result = serde_json::json!({
        "tools": tools
    });

    let response = McpResponse::success(id, result);
    let body = serde_json::to_string(&response).unwrap_or_default();
    let mut resp = Response::new(body.into());
    *resp.status_mut() = StatusCode::OK;
    resp
}

/// Handle tools/call method.
async fn handle_tools_call(
    state: Arc<AppState>,
    owner: String,
    id: serde_json::Value,
    params: serde_json::Value,
) -> Response {
    let params_obj = match params.as_object() {
        Some(o) => o,
        None => {
            let response = McpResponse::error(id, -32602, "Invalid params: expected object".to_string());
            let body = serde_json::to_string(&response).unwrap_or_default();
            let mut resp = Response::new(body.into());
            *resp.status_mut() = StatusCode::OK;
            return resp;
        }
    };

    let tool_name = match params_obj.get("name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => {
            let response = McpResponse::error(id, -32602, "Missing tool name".to_string());
            let body = serde_json::to_string(&response).unwrap_or_default();
            let mut resp = Response::new(body.into());
            *resp.status_mut() = StatusCode::OK;
            return resp;
        }
    };

    let arguments = params_obj.get("arguments")
        .and_then(|v| v.as_object())
        .map(|o| serde_json::Value::Object(o.clone()))
        .unwrap_or(serde_json::Value::Object(Default::default()));

    // Look up tool
    let tool = match state.tools.tool_by_slug(tool_name) {
        Some(t) => t,
        None => {
            let response = McpResponse::error(id, -32602, format!("Tool not found: {}", tool_name));
            let body = serde_json::to_string(&response).unwrap_or_default();
            let mut resp = Response::new(body.into());
            *resp.status_mut() = StatusCode::OK;
            return resp;
        }
    };

    // Get connector
    let user_connectors = match get_user_connectors(&state, &owner).await {
        Ok(c) => c,
        Err(e) => return e,
    };

    let connector_scopes = match user_connectors.get(&tool.provider) {
        Some(s) => s.clone(),
        None => {
            let response = McpResponse::error(id, -32602, format!("No connector for platform: {}", tool.provider));
            let body = serde_json::to_string(&response).unwrap_or_default();
            let mut resp = Response::new(body.into());
            *resp.status_mut() = StatusCode::OK;
            return resp;
        }
    };

    // Check scope satisfaction
    let scope_sat = check_scope_satisfied(&tool.scopes, &connector_scopes);
    if matches!(scope_sat, ScopeSatisfied::No) {
        let response = McpResponse::error(id, -32602, "Insufficient scopes for this tool".to_string());
        let body = serde_json::to_string(&response).unwrap_or_default();
        let mut resp = Response::new(body.into());
        *resp.status_mut() = StatusCode::OK;
        return resp;
    }

    // Execute via tools::execute
    let execute_request = crate::api::tools::ExecuteToolRequest {
        tool_slug: tool_name.to_string(),
        platform: tool.provider.clone(),
        arguments,
    };

    // Use tools execute handler
    let resp = crate::api::tools::execute(
        State(state),
        headers_from_owner(&owner),
        Json(execute_request),
    ).await;

    resp
}

/// Create a minimal header map for internal calls (since we bypass auth).
fn headers_from_owner(_owner: &str) -> HeaderMap {
    HeaderMap::new()
}

/// MCP SSE stream endpoint for tools/listen (optional long-polling).
pub async fn handle_mcp_sse(
    State(_state): State<Arc<AppState>>,
    _headers: HeaderMap,
) -> Response {
    // For now, return a simple message that MCP is available
    let body = "MCP endpoint available. Use POST with JSON-RPC.";
    let mut resp = Response::new(body.into());
    *resp.status_mut() = StatusCode::OK;
    resp
}

/// Authenticate via Bearer token and return owner username.
async fn auth_user(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<String, Response> {
    use reqwest::header::AUTHORIZATION;

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
            return Ok(ak.username);
        }
    }

    Err(tool_error(StatusCode::UNAUTHORIZED, "invalid API key"))
}

fn tool_error(status: StatusCode, message: &str) -> Response {
    let body = serde_json::json!({ "error": message }).to_string();
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response
}

/// Router for MCP endpoint.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/mcp", post(handle_mcp))
        .route("/mcp/sse", get(handle_mcp_sse))
}

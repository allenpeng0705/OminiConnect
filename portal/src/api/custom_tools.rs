//! Custom tool registration API — allows users to register their own tools.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::db::CustomTool;
use crate::tools::{HttpMethod, InputSchema, Tool};

/// Request to create or update a custom tool.
#[derive(Debug, Deserialize)]
pub struct CreateCustomToolRequest {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub endpoint: String,
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default)]
    pub input_schema: InputSchema,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_method() -> String {
    "GET".to_string()
}

/// Response for a custom tool.
#[derive(Debug, Serialize)]
pub struct CustomToolResponse {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub endpoint: String,
    pub method: String,
    pub input_schema: InputSchema,
    pub scopes: Vec<String>,
    pub tags: Vec<String>,
}

impl From<CustomTool> for CustomToolResponse {
    fn from(ct: CustomTool) -> Self {
        Self {
            slug: ct.slug,
            name: ct.name,
            description: ct.description,
            provider: ct.provider,
            endpoint: ct.endpoint,
            method: format!("{:?}", ct.method),
            input_schema: ct.input_schema,
            scopes: ct.scopes,
            tags: ct.tags,
        }
    }
}

/// Create the custom tools router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/custom-tools", get(list_custom_tools))
        .route("/custom-tools", post(create_custom_tool))
        .route("/custom-tools/:slug", delete(delete_custom_tool))
}

/// GET /api/custom-tools — list user's custom tools
pub async fn list_custom_tools(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    match state.custom_tools.list_by_owner(&auth.username).await {
        Ok(tools) => {
            let responses: Vec<CustomToolResponse> = tools.into_iter().map(|t| t.into()).collect();
            (StatusCode::OK, Json(serde_json::json!({ "tools": responses }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list custom tools: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to list custom tools"
            }))).into_response()
        }
    }
}

/// POST /api/custom-tools — create or update a custom tool
pub async fn create_custom_tool(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateCustomToolRequest>,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    // Validate method
    let method = match req.method.to_uppercase().as_str() {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        "PATCH" => HttpMethod::PATCH,
        _ => {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "invalid method, must be GET, POST, PUT, DELETE, or PATCH"
            }))).into_response();
        }
    };

    let tool = CustomTool {
        slug: req.slug.clone(),
        owner_username: auth.username,
        name: req.name,
        description: req.description,
        provider: req.provider,
        endpoint: req.endpoint,
        method,
        input_schema: req.input_schema,
        scopes: req.scopes,
        tags: req.tags,
        created_at: chrono::Utc::now(),
    };

    match state.custom_tools.upsert(&tool).await {
        Ok(_) => {
            let response: CustomToolResponse = tool.into();
            (StatusCode::CREATED, Json(serde_json::json!({ "tool": response }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create custom tool: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to create custom tool"
            }))).into_response()
        }
    }
}

/// DELETE /api/custom-tools/:slug — delete a custom tool
pub async fn delete_custom_tool(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    match state.custom_tools.delete(&slug, &auth.username).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to delete custom tool: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to delete custom tool"
            }))).into_response()
        }
    }
}

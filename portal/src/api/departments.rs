//! Department scopes API — manage department-based scope mappings.

use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::auth::middleware::try_auth;

/// A department scope mapping.
#[derive(Debug, Clone, Serialize)]
pub struct DepartmentScope {
    pub department: String,
    pub scopes: Vec<String>,
    pub description: String,
}

/// Request to create or update a department scope mapping.
#[derive(Debug, Deserialize)]
pub struct UpsertDepartmentScopeRequest {
    pub department: String,
    pub scopes: Vec<String>,
    pub description: Option<String>,
}

/// Create the department scopes router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/departments", get(list_department_scopes))
        .route("/admin/departments", post(upsert_department_scope))
        .route("/admin/departments/:department", delete(delete_department_scope))
}

/// GET /api/admin/departments — list all department scope mappings (admin only)
pub async fn list_department_scopes(
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

    // Check if user is admin
    if auth.username != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "admin only"
        }))).into_response();
    }

    match state.department_scopes.list_all().await {
        Ok(scopes) => {
            (StatusCode::OK, Json(serde_json::json!({ "departments": scopes }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list department scopes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to list department scopes"
            }))).into_response()
        }
    }
}

/// POST /api/admin/departments — create or update a department scope mapping
pub async fn upsert_department_scope(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<UpsertDepartmentScopeRequest>,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    // Check if user is admin
    if auth.username != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "admin only"
        }))).into_response();
    }

    let dept_scope = crate::db::DepartmentScope {
        department: req.department.clone(),
        scopes: req.scopes,
        description: req.description.unwrap_or_default(),
    };

    match state.department_scopes.upsert(&dept_scope).await {
        Ok(_) => {
            (StatusCode::CREATED, Json(serde_json::json!({ "ok": true }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to upsert department scope: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to upsert department scope"
            }))).into_response()
        }
    }
}

/// DELETE /api/admin/departments/:department — delete a department scope mapping
pub async fn delete_department_scope(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Path(department): axum::extract::Path<String>,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    // Check if user is admin
    if auth.username != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "admin only"
        }))).into_response();
    }

    match state.department_scopes.delete(&department).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to delete department scope: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to delete department scope"
            }))).into_response()
        }
    }
}

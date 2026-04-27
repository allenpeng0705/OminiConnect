//! Audit log API — view tool execution history.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::auth::middleware::try_auth;

/// Query parameters for listing audit logs.
#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    /// Filter by agent ID (optional).
    pub agent_id: Option<String>,
    /// Filter by platform (optional).
    pub platform: Option<String>,
    /// Limit number of results (default 50, max 200).
    pub limit: Option<usize>,
}

/// A single audit log entry.
#[derive(Debug, Serialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub agent_id: String,
    pub tool_slug: String,
    pub platform: String,
    pub arguments: serde_json::Value,
    pub result: String,
    pub status: String,
    pub duration_ms: i64,
    pub created_at: String,
}

/// Response containing audit log entries.
#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub logs: Vec<AuditLogEntry>,
}

/// Create the audit log router
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/audit/logs", get(list_audit_logs))
}

/// GET /api/audit/logs — list tool execution audit logs
pub async fn list_audit_logs(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuditLogQuery>,
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

    let limit = query.limit.unwrap_or(50).min(200);

    let logs = if let Some(agent_id) = &query.agent_id {
        // Filter by agent
        state.tool_executions.list_by_agent(agent_id, limit).await
    } else {
        // List by owner (all agents)
        state.tool_executions.list_by_owner(&auth.username, limit).await
    };

    match logs {
        Ok(execs) => {
            // Filter by platform if specified
            let filtered: Vec<AuditLogEntry> = execs
                .into_iter()
                .filter(|e| {
                    if let Some(ref platform) = query.platform {
                        e.platform == *platform
                    } else {
                        true
                    }
                })
                .map(|e| AuditLogEntry {
                    id: e.id,
                    agent_id: e.agent_id,
                    tool_slug: e.tool_slug,
                    platform: e.platform,
                    arguments: e.arguments,
                    result: e.result,
                    status: e.status,
                    duration_ms: e.duration_ms,
                    created_at: e.created_at.to_rfc3339(),
                })
                .collect();

            (StatusCode::OK, Json(AuditLogResponse { logs: filtered })).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list audit logs: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to list audit logs"
            }))).into_response()
        }
    }
}

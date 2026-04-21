//! Connector management API.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};

use crate::app::AppState;
use crate::oauth::models::ConnectorConfig;

/// GET /api/connectors
pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let connectors = state.connectors.read().await;
    let list: Vec<serde_json::Value> = connectors
        .values()
        .map(|c| {
            serde_json::json!({
                "platform": c.platform,
                "client_id": c.client_id,
                "redirect_uri": c.redirect_uri,
                "scopes": c.scopes,
                "enabled": c.enabled,
                "connected": false, // TODO: check oauth_vault for actual token
            })
        })
        .collect();
    Json(list)
}

/// POST /api/connectors
pub async fn upsert(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ConnectorConfig>,
) -> impl IntoResponse {
    let platform = payload.platform.clone();
    state.connectors.write().await.insert(platform.clone(), payload);
    tracing::info!("Connector {} saved", platform);
    (StatusCode::CREATED, Json(serde_json::json!({ "message": "saved" })))
}

/// DELETE /api/connectors/:platform
pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let removed = state.connectors.write().await.remove(&platform);
    if removed.is_some() {
        let _ = state.oauth_vault.delete_token(&platform, "user").await;
        tracing::info!("Connector {} deleted", platform);
        (StatusCode::OK, Json(serde_json::json!({ "message": "deleted" })))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not found" })))
    }
}

/// GET /api/connectors/:platform/status
pub async fn status(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let connectors = state.connectors.read().await;
    let configured = connectors.contains_key(&platform);
    drop(connectors);

    let connected = state.oauth_vault.get_token(&platform, "user").await.is_ok();

    Json(serde_json::json!({
        "platform": platform,
        "configured": configured,
        "connected": connected,
    }))
}

/// POST /api/connectors/:platform/test
pub async fn test(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    // Check connector is configured first
    {
        let connectors = state.connectors.read().await;
        if !connectors.contains_key(&platform) {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not configured" })));
        }
    }

    match state.oauth_vault.get_token(&platform, "user").await {
        Ok(token) => {
            tracing::info!("Token test OK for {}: {}", platform, &token[..token.len().min(10)]);
            (StatusCode::OK, Json(serde_json::json!({
                "platform": platform,
                "status": "ok",
                "message": "Token retrieved successfully",
            })))
        }
        Err(e) => {
            tracing::warn!("Token test FAILED for {}: {}", platform, e);
            (StatusCode::OK, Json(serde_json::json!({
                "platform": platform,
                "status": "error",
                "message": e.to_string(),
            })))
        }
    }
}

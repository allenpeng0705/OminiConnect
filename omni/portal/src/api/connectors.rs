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
    let connectors = state.connectors.list().await
        .map_err(|e| {
            tracing::error!("DB error listing connectors: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" })))
        });

    match connectors {
        Ok(list) => {
            let json: Vec<serde_json::Value> = list
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "platform": c.platform,
                        "client_id": c.client_id,
                        "has_client_secret": !c.client_secret.is_empty(),
                        "redirect_uri": c.redirect_uri,
                        "scopes": c.scopes,
                        "enabled": c.enabled,
                        "connected": false,
                    })
                })
                .collect();
            Json(json).into_response()
        }
        Err(r) => r.into_response(),
    }
}

/// POST /api/connectors
pub async fn upsert(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<ConnectorConfig>,
) -> impl IntoResponse {
    // Preserve existing client_secret if not provided (empty string means unchanged)
    if payload.client_secret.is_empty() {
        if let Ok(Some(existing)) = state.connectors.get(&payload.platform).await {
            payload.client_secret = existing.client_secret;
        }
    }

    if let Err(e) = state.connectors.upsert(&payload).await {
        tracing::error!("DB error upserting connector: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
    }
    tracing::info!("Connector {} saved", payload.platform);
    (StatusCode::CREATED, Json(serde_json::json!({ "message": "saved" }))).into_response()
}

/// DELETE /api/connectors/:platform
pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    if let Err(e) = state.connectors.delete(&platform).await {
        tracing::error!("DB error deleting connector: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
    }
    let _ = state.oauth_vault.delete_token(&platform, "user").await;
    tracing::info!("Connector {} deleted", platform);
    (StatusCode::OK, Json(serde_json::json!({ "message": "deleted" }))).into_response()
}

/// GET /api/connectors/:platform/status
pub async fn status(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let configured = state.connectors.get(&platform).await
        .map(|opt| opt.is_some())
        .unwrap_or(false);

    let client_id = state.connectors.get(&platform).await
        .ok()
        .flatten()
        .map(|c| c.client_id);

    let connected = if platform == "maton" || platform == "qqmail" {
        client_id.as_ref().map(|k| !k.is_empty()).unwrap_or(false)
    } else {
        state.oauth_vault.get_token(&platform, "user").await.is_ok()
    };

    Json(serde_json::json!({
        "platform": platform,
        "configured": configured,
        "connected": connected,
    })).into_response()
}

/// POST /api/connectors/:platform/test
pub async fn test(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let connector = match state.connectors.get(&platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not configured" }))).into_response();
        }
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
        }
    };

    if platform == "maton" || platform == "qqmail" {
        if !connector.client_id.is_empty() {
            (StatusCode::OK, Json(serde_json::json!({
                "platform": platform,
                "status": "ok",
                "message": "API key configured",
            }))).into_response()
        } else {
            (StatusCode::OK, Json(serde_json::json!({
                "platform": platform,
                "status": "error",
                "message": "API key not set",
            }))).into_response()
        }
    } else {
        match state.oauth_vault.get_token(&platform, "user").await {
            Ok(token) => {
                tracing::info!("Token test OK for {}: {}", platform, &token[..token.len().min(10)]);
                (StatusCode::OK, Json(serde_json::json!({
                    "platform": platform,
                    "status": "ok",
                    "message": "Token retrieved successfully",
                }))).into_response()
            }
            Err(e) => {
                tracing::warn!("Token test FAILED for {}: {}", platform, e);
                (StatusCode::OK, Json(serde_json::json!({
                    "platform": platform,
                    "status": "error",
                    "message": e.to_string(),
                }))).into_response()
            }
        }
    }
}

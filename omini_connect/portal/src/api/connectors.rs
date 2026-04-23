//! Connector management API.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};

use crate::app::AppState;
use crate::connector_engine::{is_api_key_platform, test_connector_by_engine};
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
                        "engine": c.engine,
                        "provider_key": c.provider_key,
                        "connection_ref": c.connection_ref,
                        "agent_id": c.agent_id,
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
    let existing = state.connectors.get(&payload.platform).await.ok().flatten();

    // Preserve existing client_secret if not provided (empty string means unchanged)
    if payload.client_secret.is_empty() {
        if let Some(existing) = existing.as_ref() {
            payload.client_secret = existing.client_secret.clone();
        }
    }

    // Keep previous engine/provider/connection_ref when omitted in partial updates.
    if payload.engine.trim().is_empty() {
        payload.engine = existing
            .as_ref()
            .map(|c| c.engine.clone())
            .unwrap_or_else(|| "omini_connect_native".to_string());
    }
    if payload.provider_key.trim().is_empty() {
        payload.provider_key = existing
            .as_ref()
            .map(|c| c.provider_key.clone())
            .unwrap_or_else(|| payload.platform.clone());
    }
    if payload.connection_ref.trim().is_empty() {
        payload.connection_ref = existing
            .as_ref()
            .map(|c| c.connection_ref.clone())
            .unwrap_or_default();
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
    let connector = state.connectors.get(&platform).await.ok().flatten();
    let configured = connector.is_some();

    let connected = if let Some(c) = connector.as_ref() {
        match c.engine.as_str() {
            "nango" => !c.connection_ref.trim().is_empty(),
            _ if is_api_key_platform(&platform) => !c.client_id.is_empty(),
            _ => state.oauth_vault.get_token(&platform, "user").await.is_ok(),
        }
    } else {
        false
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

    match test_connector_by_engine(&state, &connector).await {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!({
            "platform": platform,
            "engine": connector.engine,
            "status": result.status,
            "message": result.message,
        }))).into_response(),
        Err(e) => {
            tracing::error!("Connector test failed for {}: {}", platform, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "connector test failed",
            }))).into_response()
        }
    }
}

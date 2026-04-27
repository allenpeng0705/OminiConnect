//! Token delivery endpoint — returns fresh OAuth tokens for direct API calls.
//!
//! GET /api/token/{platform} — get a fresh token for the authenticated user.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use serde_json::json;

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::connector_scope::oauth_vault_platform_key;

/// Create the token router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/token", axum::routing::get(list_tokens))
        .route("/token/:platform", axum::routing::get(get_token))
}

/// GET /api/token/{platform}
/// Returns a fresh access token for the authenticated user.
/// The token is auto-refreshed if expired.
pub async fn get_token(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Validate via session or API key
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "unauthorized",
                "message": "Invalid or missing authentication"
            }))).into_response();
        }
    };

    // Get connector for this user and platform
    let connector = match state.connectors.get(&auth.username, &platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "connector_not_found",
                "message": format!("{} is not connected", platform)
            }))).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to get connector for token: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "internal_error",
                "message": "Failed to get connector"
            }))).into_response();
        }
    };

    if !connector.enabled {
        return (StatusCode::FORBIDDEN, Json(json!({
            "error": "connector_disabled",
            "message": "Connector is disabled"
        }))).into_response();
    }

    // Get the vault key for this platform and user
    let platform_key = oauth_vault_platform_key(&auth.username, &platform);

    // Get token from oauth_vault - requires (platform, subject)
    let token_result = state.oauth_vault.get_token(&platform_key, &auth.username).await;

    match token_result {
        Ok(token) => {
            // Token found
            (StatusCode::OK, Json(json!({
                "platform": platform,
                "access_token": token,
            }))).into_response()
        }
        Err(_e) => {
            // Check if this is a static token platform (maton, qqmail)
            if platform == "maton" || platform == "qqmail" {
                // For static token platforms, client_id holds the API key
                let static_token = connector.client_id.clone();
                if !static_token.is_empty() {
                    return (StatusCode::OK, Json(json!({
                        "platform": platform,
                        "access_token": static_token,
                    }))).into_response();
                }
            }
            (StatusCode::NOT_FOUND, Json(json!({
                "error": "token_not_found",
                "message": "No token available for this platform. Please re-authenticate."
            }))).into_response()
        }
    }
}

/// GET /api/token — list all available tokens for the authenticated user
pub async fn list_tokens(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Validate via session or API key
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({
                "error": "unauthorized",
                "message": "Invalid or missing authentication"
            }))).into_response();
        }
    };

    // Get all connectors for this user
    let connectors = match state.connectors.list(&auth.username).await {
        Ok(conns) => conns,
        Err(e) => {
            tracing::error!("Failed to list connectors: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "internal_error",
                "message": "Failed to list connectors"
            }))).into_response();
        }
    };

    let mut tokens = Vec::new();
    for connector in connectors {
        let platform_key = oauth_vault_platform_key(&auth.username, &connector.platform);

        let has_token = state.oauth_vault.get_token(&platform_key, &auth.username).await.is_ok();
        let is_static = connector.platform == "maton" || connector.platform == "qqmail";

        tokens.push(json!({
            "platform": connector.platform,
            "enabled": connector.enabled,
            "has_token": has_token || is_static,
            "is_static": is_static,
        }));
    }

    (StatusCode::OK, Json(json!({
        "tokens": tokens
    }))).into_response()
}

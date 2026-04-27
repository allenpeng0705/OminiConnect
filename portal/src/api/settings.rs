//! User settings API — data residency and other preferences.

use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::auth::middleware::try_auth;

/// Request to update user settings.
#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub data_residency: Option<String>,
}

/// Current user settings response.
#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub data_residency: Option<String>,
}

/// Create the settings router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/settings", axum::routing::get(get_settings))
        .route("/settings", axum::routing::patch(update_settings))
}

/// GET /api/settings — get current user settings
pub async fn get_settings(
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

    match state.users.get(&auth.username).await {
        Ok(Some(user)) => {
            let data_residency = user.data_residency.map(|r| match r {
                crate::auth::models::DataResidency::Us => "us".to_string(),
                crate::auth::models::DataResidency::Eu => "eu".to_string(),
                crate::auth::models::DataResidency::Cn => "cn".to_string(),
            });
            (StatusCode::OK, Json(SettingsResponse { data_residency })).into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "user not found"}))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get user settings: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to get settings"}))).into_response()
        }
    }
}

/// PATCH /api/settings — update user settings
pub async fn update_settings(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<UpdateSettingsRequest>,
) -> impl IntoResponse {
    let auth = match try_auth(&state, &headers).await {
        Some(a) => a,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "unauthorized"
            }))).into_response();
        }
    };

    // Validate data_residency if provided
    if let Some(ref residency) = req.data_residency {
        match residency.as_str() {
            "us" | "eu" | "cn" | "" => {}
            _ => {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "data_residency must be one of: us, eu, cn, or empty to clear"
                }))).into_response();
            }
        }
    }

    // Update data_residency
    if let Some(residency) = &req.data_residency {
        if let Err(e) = state.users.update_data_residency(&auth.username, residency).await {
            tracing::error!("Failed to update data_residency: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "failed to update settings"
            }))).into_response();
        }
    }

    // Return updated settings
    match state.users.get(&auth.username).await {
        Ok(Some(user)) => {
            let data_residency = user.data_residency.map(|r| match r {
                crate::auth::models::DataResidency::Us => "us".to_string(),
                crate::auth::models::DataResidency::Eu => "eu".to_string(),
                crate::auth::models::DataResidency::Cn => "cn".to_string(),
            });
            (StatusCode::OK, Json(SettingsResponse { data_residency })).into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "user not found"}))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get user settings: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to get settings"}))).into_response()
        }
    }
}

//! Authentication HTTP handlers.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use http::{header, StatusCode};
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::auth::models::{
    ApiKey, ApiKeyResponse, GenerateApiKeyRequest, LoginRequest,
};

/// POST /auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Response {
    let user = match state.users.get(&req.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response();
        }
        Err(e) => {
            tracing::error!("DB error looking up user: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    match bcrypt::verify(&req.password, &user.password_hash) {
        Ok(true) => {}
        _ => {
            return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response();
        }
    }

    let session_id = Uuid::new_v4().to_string();
    let session = crate::auth::models::Session {
        session_id: session_id.clone(),
        username: req.username.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };

    if let Err(e) = state.sessions.insert(&session).await {
        tracing::error!("DB error inserting session: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    let mut resp = Redirect::to("/").into_response();
    let cookie = format!(
        "session={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Lax",
        session_id
    );
    if let Ok(h) = http::HeaderValue::from_str(&cookie) {
        resp.headers_mut().insert(header::SET_COOKIE, h);
    }

    tracing::info!("User {} logged in", req.username);
    resp
}

/// POST /auth/logout
pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
) -> Response {
    let session_id = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .find_map(|pair| {
                    let (k, v) = pair.trim().split_once('=')?;
                    if k == "session" { Some(v) } else { None }
                })
        });

    if let Some(sid) = session_id {
        if let Ok(Some(session)) = state.sessions.get(sid).await {
            let username = session.username.clone();
            if state.sessions.delete(sid).await.is_ok() {
                tracing::info!("User {} logged out", username);
            }
        }
    }
    Redirect::to("/auth/login").into_response()
}

/// POST /auth/apikey — generate a new API key for the current user
pub async fn generate_api_key(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    Json(req): Json<GenerateApiKeyRequest>,
) -> impl IntoResponse {
    let username = match try_auth(&state, &headers).await {
        Some(auth) => auth.username,
        None => {
            return (StatusCode::UNAUTHORIZED, "Not authenticated").into_response();
        }
    };

    let raw_key = format!("{}:{}", username, Uuid::new_v4());
    let hash = bcrypt::hash(&raw_key, bcrypt::DEFAULT_COST).unwrap_or_default();

    let api_key = ApiKey {
        key_hash: hash,
        username: username.clone(),
        label: req.label.clone(),
        created_at: chrono::Utc::now(),
    };

    if let Err(e) = state.api_keys.insert(&api_key).await {
        tracing::error!("DB error inserting API key: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    tracing::info!("API key generated for user {}", username);

    let resp = ApiKeyResponse {
        key: raw_key,
        label: req.label,
        created_at: api_key.created_at,
    };

    (StatusCode::CREATED, axum::Json(resp)).into_response()
}

/// GET /auth/me — returns current user info
pub async fn me(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
) -> Response {
    match try_auth(&state, &headers).await {
        Some(auth) => axum::Json(serde_json::json!({ "username": auth.username })).into_response(),
        None => (StatusCode::UNAUTHORIZED, "Not authenticated").into_response(),
    }
}

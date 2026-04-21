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
    ApiKeyResponse, GenerateApiKeyRequest, LoginRequest,
};

/// POST /auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Response {
    let users = state.users.read().await;
    let user = match users.get(&req.username) {
        Some(u) => u,
        None => {
            return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response();
        }
    };

    match bcrypt::verify(&req.password, &user.password_hash) {
        Ok(true) => {}
        _ => {
            return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response();
        }
    }
    drop(users);

    let session_id = Uuid::new_v4().to_string();
    let session = crate::auth::models::Session {
        session_id: session_id.clone(),
        username: req.username.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };

    state.sessions.write().await.insert(session_id.clone(), session);

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
        if let Some(username) = state.sessions.read().await.get(sid).map(|s| s.username.clone()) {
            state.sessions.write().await.remove(sid);
            tracing::info!("User {} logged out", username);
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

    let api_key = ApiKeyResponse {
        key: raw_key.clone(),
        label: req.label,
        created_at: chrono::Utc::now(),
    };

    state.api_keys.write().await.insert(raw_key.clone(), crate::auth::models::ApiKey {
        key_hash: hash,
        username: username.clone(),
        label: api_key.label.clone(),
        created_at: api_key.created_at,
    });

    tracing::info!("API key generated for user {}", username);

    (StatusCode::CREATED, axum::Json(api_key)).into_response()
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

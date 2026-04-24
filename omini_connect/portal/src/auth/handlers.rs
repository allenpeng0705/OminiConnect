//! Authentication HTTP handlers.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use http::{header, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::auth::models::{
    ApiKey, ApiKeyResponse, GenerateApiKeyRequest, LoginRequest,
};

#[derive(Debug, Deserialize)]
pub struct OidcCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OidcProviderMetadata {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: Option<String>,
}

fn auth_mode() -> String {
    std::env::var("AUTH_MODE")
        .ok()
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "local".to_string())
}

fn oidc_env() -> Option<(String, String, String, String)> {
    let issuer = std::env::var("OIDC_ISSUER_URL").ok()?.trim().trim_end_matches('/').to_string();
    let client_id = std::env::var("OIDC_CLIENT_ID").ok()?.trim().to_string();
    let client_secret = std::env::var("OIDC_CLIENT_SECRET").ok()?.trim().to_string();
    let redirect_uri = std::env::var("OIDC_REDIRECT_URI").ok()?.trim().to_string();
    if issuer.is_empty() || client_id.is_empty() || client_secret.is_empty() || redirect_uri.is_empty() {
        return None;
    }
    Some((issuer, client_id, client_secret, redirect_uri))
}

async fn fetch_oidc_metadata(issuer: &str) -> anyhow::Result<OidcProviderMetadata> {
    let url = format!("{issuer}/.well-known/openid-configuration");
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("OIDC discovery failed {status}: {text}");
    }
    Ok(resp.json::<OidcProviderMetadata>().await?)
}

async fn ensure_user_exists(state: &Arc<AppState>, username: &str) -> anyhow::Result<()> {
    if state.users.get(username).await?.is_none() {
        let user = crate::auth::models::User {
            username: username.to_string(),
            // Local password is unused for OIDC-provisioned users.
            password_hash: "!oidc-managed!".to_string(),
            created_at: chrono::Utc::now(),
        };
        state.users.insert(&user).await?;
    }
    Ok(())
}

fn build_session_response(session_id: &str, redirect_to: &str) -> Response {
    let mut resp = Redirect::to(redirect_to).into_response();
    let cookie = format!(
        "session={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Lax; Secure",
        session_id
    );
    if let Ok(h) = http::HeaderValue::from_str(&cookie) {
        resp.headers_mut().insert(header::SET_COOKIE, h);
    }
    resp
}

/// GET /auth/config
pub async fn config() -> impl IntoResponse {
    let mode = auth_mode();
    let oidc_ready = oidc_env().is_some();
    let login_path = if mode == "oidc" && oidc_ready {
        "/auth/oidc/login"
    } else {
        "/auth/login"
    };

    Json(serde_json::json!({
        "mode": mode,
        "oidc_ready": oidc_ready,
        "login_path": login_path,
    }))
}

/// POST /auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Response {
    if auth_mode() == "oidc" {
        return (
            StatusCode::BAD_REQUEST,
            "Local password login is disabled. Use OIDC sign-in.",
        )
            .into_response();
    }

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

    tracing::info!("User {} logged in", req.username);
    build_session_response(&session_id, "/")
}

/// GET /auth/oidc/login
pub async fn oidc_login() -> Response {
    if auth_mode() != "oidc" {
        return (StatusCode::BAD_REQUEST, "OIDC mode is not enabled").into_response();
    }
    let Some((issuer, client_id, _client_secret, redirect_uri)) = oidc_env() else {
        return (StatusCode::SERVICE_UNAVAILABLE, "OIDC is not configured").into_response();
    };

    let meta = match fetch_oidc_metadata(&issuer).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("OIDC metadata fetch failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "OIDC discovery failed").into_response();
        }
    };

    let state = Uuid::new_v4().to_string();
    let auth_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
        meta.authorization_endpoint,
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode("openid profile email"),
        urlencoding::encode(&state)
    );
    let mut resp = Redirect::to(&auth_url).into_response();
    let state_cookie = format!(
        "oidc_state={}; HttpOnly; Path=/auth/oidc; Max-Age=600; SameSite=Lax; Secure",
        state
    );
    if let Ok(h) = http::HeaderValue::from_str(&state_cookie) {
        resp.headers_mut().append(header::SET_COOKIE, h);
    }
    resp
}

/// GET /auth/oidc/callback
pub async fn oidc_callback(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    Query(q): Query<OidcCallbackQuery>,
) -> Response {
    if auth_mode() != "oidc" {
        return (StatusCode::BAD_REQUEST, "OIDC mode is not enabled").into_response();
    }
    if let Some(err) = q.error.as_deref() {
        let detail = q.error_description.as_deref().unwrap_or("");
        tracing::warn!("OIDC callback error: {} {}", err, detail);
        return Redirect::to("/auth/login").into_response();
    }
    let Some(code) = q.code.as_deref() else {
        return (StatusCode::BAD_REQUEST, "Missing OIDC code").into_response();
    };
    let Some(state_param) = q.state.as_deref() else {
        return (StatusCode::BAD_REQUEST, "Missing OIDC state").into_response();
    };

    let cookie_state = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|pair| {
                    let (k, v) = pair.trim().split_once('=')?;
                    if k == "oidc_state" { Some(v) } else { None }
                })
        });
    if cookie_state != Some(state_param) {
        return (StatusCode::UNAUTHORIZED, "OIDC state mismatch").into_response();
    }

    let Some((issuer, client_id, client_secret, redirect_uri)) = oidc_env() else {
        return (StatusCode::SERVICE_UNAVAILABLE, "OIDC is not configured").into_response();
    };
    let meta = match fetch_oidc_metadata(&issuer).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("OIDC metadata fetch failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "OIDC discovery failed").into_response();
        }
    };

    let client = match reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("OIDC client init failed: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "OIDC client init failed").into_response();
        }
    };

    let token_resp = client
        .post(&meta.token_endpoint)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ])
        .send()
        .await;
    let token_json: Value = match token_resp {
        Ok(resp) if resp.status().is_success() => match resp.json().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("OIDC token JSON parse failed: {}", e);
                return (StatusCode::BAD_GATEWAY, "OIDC token parse failed").into_response();
            }
        },
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            tracing::error!("OIDC token exchange failed {}: {}", status, text);
            return (StatusCode::BAD_GATEWAY, "OIDC token exchange failed").into_response();
        }
        Err(e) => {
            tracing::error!("OIDC token exchange request failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "OIDC token exchange failed").into_response();
        }
    };

    let access_token = token_json.get("access_token").and_then(|v| v.as_str()).unwrap_or_default();
    if access_token.is_empty() {
        return (StatusCode::BAD_GATEWAY, "OIDC access token missing").into_response();
    }

    let Some(userinfo_endpoint) = meta.userinfo_endpoint.as_deref() else {
        return (StatusCode::BAD_GATEWAY, "OIDC userinfo endpoint is missing").into_response();
    };
    let profile_resp = client
        .get(userinfo_endpoint)
        .bearer_auth(access_token)
        .send()
        .await;
    let profile_json: Value = match profile_resp {
        Ok(resp) if resp.status().is_success() => match resp.json().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("OIDC userinfo JSON parse failed: {}", e);
                return (StatusCode::BAD_GATEWAY, "OIDC user profile parse failed").into_response();
            }
        },
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            tracing::error!("OIDC userinfo failed {}: {}", status, text);
            return (StatusCode::BAD_GATEWAY, "OIDC user profile fetch failed").into_response();
        }
        Err(e) => {
            tracing::error!("OIDC userinfo request failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "OIDC user profile fetch failed").into_response();
        }
    };

    let username = profile_json
        .get("preferred_username")
        .and_then(|v| v.as_str())
        .or_else(|| profile_json.get("email").and_then(|v| v.as_str()))
        .or_else(|| profile_json.get("sub").and_then(|v| v.as_str()))
        .unwrap_or("")
        .trim()
        .to_string();
    if username.is_empty() {
        return (StatusCode::BAD_GATEWAY, "OIDC profile missing username/email/sub").into_response();
    }

    if let Err(e) = ensure_user_exists(&state, &username).await {
        tracing::error!("Failed to provision OIDC user {}: {}", username, e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to provision user").into_response();
    }

    let session_id = Uuid::new_v4().to_string();
    let session = crate::auth::models::Session {
        session_id: session_id.clone(),
        username: username.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };
    if let Err(e) = state.sessions.insert(&session).await {
        tracing::error!("DB error inserting OIDC session: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    tracing::info!("OIDC user {} logged in", username);
    build_session_response(&session_id, "/")
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

/// GET /auth/apikey — list all API keys for current user
pub async fn list_api_keys(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
) -> Response {
    let username = match try_auth(&state, &headers).await {
        Some(auth) => auth.username,
        None => {
            return (StatusCode::UNAUTHORIZED, "Not authenticated").into_response();
        }
    };

    match state.api_keys.list_by_username(&username).await {
        Ok(keys) => {
            let resp: Vec<_> = keys.into_iter().map(|k| {
                serde_json::json!({
                    "key_hash": k.key_hash,
                    "username": k.username,
                    "label": k.label,
                    "created_at": k.created_at,
                })
            }).collect();
            axum::Json(resp).into_response()
        }
        Err(e) => {
            tracing::error!("DB error listing API keys: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
        }
    }
}

/// DELETE /auth/apikey/:key_hash — delete an API key
pub async fn delete_api_key(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    axum::extract::Path(key_hash): axum::extract::Path<String>,
) -> Response {
    let username = match try_auth(&state, &headers).await {
        Some(auth) => auth.username,
        None => {
            return (StatusCode::UNAUTHORIZED, "Not authenticated").into_response();
        }
    };

    // Verify the key belongs to this user
    match state.api_keys.get_by_hash(&key_hash).await {
        Ok(Some(key)) if key.username == username => {
            if let Err(e) = state.api_keys.delete(&key_hash).await {
                tracing::error!("DB error deleting API key: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
            }
            tracing::info!("API key deleted for user {}", username);
            (StatusCode::NO_CONTENT).into_response()
        }
        Ok(Some(_)) => {
            (StatusCode::FORBIDDEN, "Cannot delete another user's key").into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, "API key not found").into_response()
        }
        Err(e) => {
            tracing::error!("DB error looking up API key: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
        }
    }
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

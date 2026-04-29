//! Authentication HTTP handlers.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use http::{header, HeaderMap, StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::auth::models::{
    ApiKey, ApiKeyResponse, GenerateApiKeyRequest, LoginRequest, SignupRequest,
};

fn nango_http_client() -> anyhow::Result<reqwest::Client> {
    // Avoid inheriting host proxy settings for localhost bridge calls.
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .no_proxy()
        .build()
        .map_err(Into::into)
}

fn nango_auth_bridge_base_url() -> Option<String> {
    std::env::var("NANGO_BASE_URL")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
}

fn google_auth_configured() -> bool {
    let managed = std::env::var("FLAG_MANAGED_AUTH_ENABLED")
        .ok()
        .or_else(|| std::env::var("FLAG_HAS_MANAGED_AUTH").ok())
        .map(|s| {
            matches!(
                s.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false);
    let workos_key = std::env::var("WORKOS_API_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let workos_client = std::env::var("WORKOS_CLIENT_ID")
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    managed
        && !workos_key.is_empty()
        && !workos_client.is_empty()
        && nango_auth_bridge_base_url().is_some()
}

fn cookie_bridge_path() -> String {
    std::env::var("NANGO_BRIDGE_COOKIE_PATH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/".to_string())
}

fn cookie_bridge_same_site() -> String {
    std::env::var("NANGO_BRIDGE_COOKIE_SAMESITE")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Lax".to_string())
}

fn cookie_bridge_secure() -> bool {
    // If explicitly set, use that value.
    if let Ok(v) = std::env::var("NANGO_BRIDGE_COOKIE_SECURE") {
        return matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        );
    }
    // Default: auto-detect from PORTAL_BASE_URL scheme (HTTPS → Secure, HTTP → no Secure).
    let base =
        std::env::var("PORTAL_BASE_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    base.trim().starts_with("https://")
}

fn cookie_bridge_domain() -> Option<String> {
    std::env::var("NANGO_BRIDGE_COOKIE_DOMAIN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn normalize_set_cookie(line: &str) -> String {
    let mut parts = line
        .split(';')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return line.to_string();
    }

    let mut has_path = false;
    let mut has_same_site = false;
    let mut has_secure = false;
    let mut has_http_only = false;
    let mut domain_idx: Option<usize> = None;

    for (i, p) in parts.iter().enumerate().skip(1) {
        let lower = p.to_ascii_lowercase();
        if lower.starts_with("path=") {
            has_path = true;
        } else if lower.starts_with("samesite=") {
            has_same_site = true;
        } else if lower == "secure" {
            has_secure = true;
        } else if lower == "httponly" {
            has_http_only = true;
        } else if lower.starts_with("domain=") {
            domain_idx = Some(i);
        }
    }

    if has_path {
        for p in parts.iter_mut().skip(1) {
            if p.to_ascii_lowercase().starts_with("path=") {
                *p = format!("Path={}", cookie_bridge_path());
            }
        }
    } else {
        parts.push(format!("Path={}", cookie_bridge_path()));
    }

    if has_same_site {
        for p in parts.iter_mut().skip(1) {
            if p.to_ascii_lowercase().starts_with("samesite=") {
                *p = format!("SameSite={}", cookie_bridge_same_site());
            }
        }
    } else {
        parts.push(format!("SameSite={}", cookie_bridge_same_site()));
    }

    if cookie_bridge_secure() && !has_secure {
        parts.push("Secure".to_string());
    }
    if !has_http_only {
        parts.push("HttpOnly".to_string());
    }

    match (cookie_bridge_domain(), domain_idx) {
        (Some(domain), Some(i)) => parts[i] = format!("Domain={domain}"),
        (Some(domain), None) => parts.push(format!("Domain={domain}")),
        (None, Some(i)) => {
            parts.remove(i);
        }
        (None, None) => {}
    }

    parts.join("; ")
}

fn format_cookie(name: &str, value: &str, max_age: i64, http_only: bool) -> String {
    let mut parts = vec![
        format!("{name}={value}"),
        format!("Path={}", cookie_bridge_path()),
        format!("Max-Age={max_age}"),
        format!("SameSite={}", cookie_bridge_same_site()),
    ];
    if cookie_bridge_secure() {
        parts.push("Secure".to_string());
    }
    if http_only {
        parts.push("HttpOnly".to_string());
    }
    if let Some(domain) = cookie_bridge_domain() {
        parts.push(format!("Domain={domain}"));
    }
    parts.join("; ")
}

fn build_session_response_with_bridge(
    session_id: &str,
    redirect_to: &str,
    extra_set_cookies: &[String],
) -> Response {
    let mut resp = Redirect::to(redirect_to).into_response();
    let cookie = format_cookie("session", session_id, 86400, true);
    if let Ok(h) = http::HeaderValue::from_str(&cookie) {
        resp.headers_mut().append(header::SET_COOKIE, h);
    }
    for c in extra_set_cookies {
        if let Ok(h) = http::HeaderValue::from_str(c) {
            resp.headers_mut().append(header::SET_COOKIE, h);
        }
    }
    resp
}

fn extract_set_cookie_headers(headers: &HeaderMap) -> Vec<String> {
    headers
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok().map(normalize_set_cookie))
        .collect()
}

async fn nango_signup(
    base_url: &str,
    name: &str,
    email: &str,
    password: &str,
) -> anyhow::Result<Vec<String>> {
    let client = nango_http_client()?;
    let url = format!("{}/api/v1/account/signup", base_url.trim_end_matches('/'));
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": name,
            "email": email,
            "password": password
        }))
        .send()
        .await?;
    if resp.status().is_success() {
        return Ok(extract_set_cookie_headers(resp.headers()));
    }
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    // Pass through 400-level errors as structured errors so the handler can respond properly
    if !status.is_server_error() {
        anyhow::bail!("{}|{}|{}", status.as_u16(), text, "");
    }
    anyhow::bail!("Nango signup failed {status}: {text}");
}

async fn nango_signin(base_url: &str, email: &str, password: &str) -> anyhow::Result<Vec<String>> {
    let client = nango_http_client()?;
    let url = format!("{}/api/v1/account/signin", base_url.trim_end_matches('/'));
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;
    if resp.status().is_success() {
        return Ok(extract_set_cookie_headers(resp.headers()));
    }
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if !status.is_server_error() {
        anyhow::bail!("{}|{}|{}", status.as_u16(), text, "");
    }
    anyhow::bail!("Nango signin failed {status}: {text}");
}

async fn nango_logout(
    base_url: &str,
    incoming_cookie_header: Option<&str>,
) -> anyhow::Result<Vec<String>> {
    let client = nango_http_client()?;
    let url = format!("{}/api/v1/account/logout", base_url.trim_end_matches('/'));
    let mut req = client.post(url);
    if let Some(cookies) = incoming_cookie_header.filter(|v| !v.trim().is_empty()) {
        req = req.header(header::COOKIE, cookies);
    }
    let resp = req.send().await?;
    if resp.status().is_success() {
        return Ok(extract_set_cookie_headers(resp.headers()));
    }
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("Nango logout failed {status}: {text}");
}

#[derive(Debug, Deserialize)]
struct ManagedSignupUrlData {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ManagedSignupUrlResponse {
    data: ManagedSignupUrlData,
}

async fn nango_google_auth_url(base_url: &str) -> anyhow::Result<String> {
    let client = nango_http_client()?;
    let url = format!(
        "{}/api/v1/account/managed/signup",
        base_url.trim_end_matches('/')
    );
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "provider": "GoogleOAuth"
        }))
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango managed signup failed {status}: {text}");
    }
    let body = resp.text().await.unwrap_or_default();
    let parsed: ManagedSignupUrlResponse = serde_json::from_str(&body).map_err(|e| {
        anyhow::anyhow!("Nango managed signup JSON parse failed: {e}. Body: {body}")
    })?;
    if parsed.data.url.trim().is_empty() {
        anyhow::bail!("Nango managed signup returned empty URL");
    }
    Ok(parsed.data.url)
}

/// GET /auth/google — start Google OAuth sign-in via Nango managed auth.
pub async fn google_start() -> Response {
    if !google_auth_configured() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Google sign-in is not configured. Set FLAG_MANAGED_AUTH_ENABLED=true and WorkOS credentials.",
        )
            .into_response();
    }
    let Some(base) = nango_auth_bridge_base_url() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "NANGO_BASE_URL is required",
        )
            .into_response();
    };
    match nango_google_auth_url(&base).await {
        Ok(url) => Redirect::to(&url).into_response(),
        Err(e) => {
            tracing::error!("Google sign-in start failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                format!("Google sign-in is not available: {e}"),
            )
                .into_response()
        }
    }
}

/// GET /auth/capabilities
pub async fn capabilities() -> impl IntoResponse {
    Json(serde_json::json!({
        "google_login_enabled": google_auth_configured(),
    }))
}

async fn ensure_local_user(state: &Arc<AppState>, email: &str) -> anyhow::Result<()> {
    if state.users.get(email).await?.is_none() {
        let user = crate::auth::models::User {
            username: email.to_string(),
            password_hash: "!managed-by-nango!".to_string(),
            created_at: chrono::Utc::now(),
            data_residency: None,
            department: None,
        };
        state.users.insert(&user).await?;
    }
    Ok(())
}

/// POST /auth/login
pub async fn login(State(state): State<Arc<AppState>>, Json(req): Json<LoginRequest>) -> Response {
    let email = req.email.trim().to_ascii_lowercase();
    if email.is_empty() || req.password.is_empty() {
        return (StatusCode::BAD_REQUEST, "Email and password are required").into_response();
    }
    let Some(base) = nango_auth_bridge_base_url() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "NANGO_BASE_URL is required",
        )
            .into_response();
    };
    let nango_set_cookies = match nango_signin(&base, &email, &req.password).await {
        Ok(cookies) => cookies,
        Err(e) => {
            let err_str = e.to_string();
            let parts: Vec<&str> = err_str.splitn(3, '|').collect();
            if parts.len() >= 2 {
                if let Ok(status_code) = parts[0].parse::<u16>() {
                    if status_code >= 400 && status_code < 500 {
                        let body = parts[1].to_string();
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            if let Some(msg) = json.get("error")
                                .and_then(|e| e.get("message"))
                                .and_then(|m| m.as_str()) {
                                return (StatusCode::from_u16(status_code).unwrap_or(StatusCode::UNAUTHORIZED), msg.to_string()).into_response();
                            }
                        }
                        return (StatusCode::from_u16(status_code).unwrap_or(StatusCode::UNAUTHORIZED), body).into_response();
                    }
                }
            }
            return (
                StatusCode::UNAUTHORIZED,
                format!("Sign-in failed: {e}"),
            )
                .into_response()
        }
    };
    if let Err(e) = ensure_local_user(&state, &email).await {
        tracing::error!("Failed to mirror Nango user {} locally: {}", email, e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to mirror user").into_response();
    }

    let session_id = Uuid::new_v4().to_string();
    let session = crate::auth::models::Session {
        session_id: session_id.clone(),
        username: email.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };
    if let Err(e) = state.sessions.insert(&session).await {
        tracing::error!("DB error inserting session: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    tracing::info!("User {} logged in via Nango bridge", email);
    build_session_response_with_bridge(&session_id, "/", &nango_set_cookies)
}

/// POST /auth/signup
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupRequest>,
) -> Response {
    let email = req.email.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return (StatusCode::BAD_REQUEST, "Invalid email").into_response();
    }
    if req.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters",
        )
            .into_response();
    }
    let display_name = req.name.unwrap_or_else(|| email.clone()).trim().to_string();
    let Some(base) = nango_auth_bridge_base_url() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "NANGO_BASE_URL is required",
        )
            .into_response();
    };

    let nango_set_cookies = match nango_signup(&base, &display_name, &email, &req.password).await {
        Ok(cookies) => cookies,
        Err(e) => {
            let err_str = e.to_string();
            // Structured error format: "status|body|hint"
            let parts: Vec<&str> = err_str.splitn(3, '|').collect();
            if parts.len() >= 2 {
                if let Ok(status_code) = parts[0].parse::<u16>() {
                    if status_code >= 400 && status_code < 500 {
                        // Parse error message from Nango response
                        let body = parts[1].to_string();
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            if let Some(msg) = json.get("error")
                                .and_then(|e| e.get("message"))
                                .and_then(|m| m.as_str()) {
                                return (StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_REQUEST), msg.to_string()).into_response();
                            }
                        }
                        return (StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_REQUEST), body).into_response();
                    }
                }
            }
            tracing::error!("Nango signup bridge failed for {}: {}", email, err_str);
            return (StatusCode::BAD_GATEWAY, format!("Signup failed: {}", e)).into_response();
        }
    };
    if let Err(e) = ensure_local_user(&state, &email).await {
        tracing::error!("DB error creating local mirror user {}: {}", email, e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    let session_id = Uuid::new_v4().to_string();
    let session = crate::auth::models::Session {
        session_id: session_id.clone(),
        username: email.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };
    if let Err(e) = state.sessions.insert(&session).await {
        tracing::error!("DB error inserting signup session for {}: {}", email, e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
    }

    tracing::info!("User {} signed up via Nango bridge", email);
    build_session_response_with_bridge(&session_id, "/", &nango_set_cookies)
}

/// POST /auth/logout
pub async fn logout(State(state): State<Arc<AppState>>, headers: http::HeaderMap) -> Response {
    let session_id = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';').find_map(|pair| {
                let (k, v) = pair.trim().split_once('=')?;
                if k == "session" {
                    Some(v)
                } else {
                    None
                }
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
    let incoming_cookie_header = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let mut nango_set_cookies: Vec<String> = Vec::new();
    if let Some(base) = nango_auth_bridge_base_url() {
        match nango_logout(&base, incoming_cookie_header.as_deref()).await {
            Ok(cookies) => {
                nango_set_cookies = cookies;
            }
            Err(e) => {
                tracing::warn!("Nango logout bridge failed: {}", e);
            }
        }
    }
    let mut resp = Redirect::to("/auth/login").into_response();
    // Clear local session cookie explicitly.
    if let Ok(h) = http::HeaderValue::from_str(&format_cookie("session", "", 0, true)) {
        resp.headers_mut().append(header::SET_COOKIE, h);
    }
    for c in nango_set_cookies {
        if let Ok(h) = http::HeaderValue::from_str(&c) {
            resp.headers_mut().append(header::SET_COOKIE, h);
        }
    }
    resp
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

    let raw_key = Uuid::new_v4().to_string();
    let hash = bcrypt::hash(&raw_key, bcrypt::DEFAULT_COST).unwrap_or_default();

    let api_key = ApiKey {
        key_hash: hash,
        username: username.clone(),
        label: req.label.clone(),
        created_at: chrono::Utc::now(),
        agent_id: None,
        allowed_tools: req.allowed_tools.clone(),
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
            let resp: Vec<_> = keys
                .into_iter()
                .map(|k| {
                    serde_json::json!({
                        "key_hash": k.key_hash,
                        "username": k.username,
                        "label": k.label,
                        "created_at": k.created_at,
                    })
                })
                .collect();
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
        Ok(Some(_)) => (StatusCode::FORBIDDEN, "Cannot delete another user's key").into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "API key not found").into_response(),
        Err(e) => {
            tracing::error!("DB error looking up API key: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
        }
    }
}

/// GET /auth/me — returns current user info
pub async fn me(State(state): State<Arc<AppState>>, headers: http::HeaderMap) -> Response {
    match try_auth(&state, &headers).await {
        Some(auth) => axum::Json(serde_json::json!({ "username": auth.username })).into_response(),
        None => (StatusCode::UNAUTHORIZED, "Not authenticated").into_response(),
    }
}

//! Auth middleware — validates session cookie or API key header.

use std::sync::Arc;

use http::{header::COOKIE, HeaderMap};

use crate::app::AppState;

/// Authenticated user extracted from request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub username: String,
    pub session_id: Option<String>,
}

/// Try to extract auth from request. Returns Some(AuthUser) if valid.
pub async fn try_auth(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Option<AuthUser> {
    // 1. Try session cookie
    if let Some(auth) = extract_session(state, headers).await {
        return Some(auth);
    }

    // 2. Try API key header
    extract_api_key(state, headers).await
}

async fn extract_session(state: &Arc<AppState>, headers: &HeaderMap) -> Option<AuthUser> {
    let cookie_header = headers.get(COOKIE)?.to_str().ok()?;
    let session_id = cookie_header
        .split(';')
        .find_map(|pair| {
            let (k, v) = pair.trim().split_once('=')?;
            if k == "session" { Some(v) } else { None }
        })?;

    let sessions = state.sessions.read().await;
    let session = sessions.get(session_id)?;

    if session.expires_at < chrono::Utc::now() {
        return None;
    }

    Some(AuthUser {
        username: session.username.clone(),
        session_id: Some(session.session_id.clone()),
    })
}

async fn extract_api_key(state: &Arc<AppState>, headers: &HeaderMap) -> Option<AuthUser> {
    let auth_header = headers.get(http::header::AUTHORIZATION)?.to_str().ok()?;
    let raw_key = auth_header.strip_prefix("Bearer ")?;

    let api_keys = state.api_keys.read().await;
    for (_stored_key, ak) in api_keys.iter() {
        if bcrypt::verify(raw_key, &ak.key_hash).ok() == Some(true) {
            return Some(AuthUser {
                username: ak.username.clone(),
                session_id: None,
            });
        }
    }

    None
}

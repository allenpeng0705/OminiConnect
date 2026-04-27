//! Auth middleware — validates session cookie or API key header.

use std::sync::Arc;

use http::{header::COOKIE, HeaderMap};

use crate::app::AppState;

/// Authenticated user extracted from request.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub username: String,
}

/// Try to extract auth from request. Returns Some(AuthUser) if valid.
pub async fn try_auth(state: &Arc<AppState>, headers: &HeaderMap) -> Option<AuthUser> {
    // 1. Try session cookie
    if let Some(auth) = extract_session(state, headers).await {
        return Some(auth);
    }

    // 2. Try API key header
    extract_api_key(state, headers).await
}

async fn extract_session(state: &Arc<AppState>, headers: &HeaderMap) -> Option<AuthUser> {
    let cookie_header = headers.get(COOKIE)?.to_str().ok()?;
    let session_id = cookie_header.split(';').find_map(|pair| {
        let (k, v) = pair.trim().split_once('=')?;
        if k == "session" {
            Some(v)
        } else {
            None
        }
    })?;

    let session = match state.sessions.get(session_id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            tracing::debug!("Session not found: {}", session_id);
            return None;
        }
        Err(e) => {
            tracing::error!("DB error looking up session: {}", e);
            return None;
        }
    };

    if session.expires_at < chrono::Utc::now() {
        tracing::debug!(
            "Session expired: {} (expires_at={})",
            session_id,
            session.expires_at
        );
        return None;
    }

    Some(AuthUser {
        username: session.username.clone(),
    })
}

async fn extract_api_key(state: &Arc<AppState>, headers: &HeaderMap) -> Option<AuthUser> {
    let auth_header = headers.get(http::header::AUTHORIZATION)?.to_str().ok()?;
    let raw_key = auth_header.strip_prefix("Bearer ")?;

    // Look up API key by iterating and checking bcrypt hash (same as original in-memory design)
    let api_keys = state.api_keys.list_all().await.ok()?;

    for ak in api_keys {
        if bcrypt::verify(raw_key, &ak.key_hash).ok() == Some(true) {
            return Some(AuthUser {
                username: ak.username.clone(),
            });
        }
    }

    None
}

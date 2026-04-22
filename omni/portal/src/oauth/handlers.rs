//! OAuth HTTP handlers — initiate and handle OAuth callbacks.

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
    http::StatusCode,
};
use uuid::Uuid;

use omni_oauth_vault::platform::{OAuth2Platform, PlatformConfig};

use crate::app::AppState;
use crate::auth::models::Session;
use crate::oauth::models::OAuthCallbackQuery;

/// Supported platforms.
const SUPPORTED_PLATFORMS: &[&str] = &["feishu", "dingtalk", "wechatwork", "linkedin", "facebook", "x"];

/// Base URL for the portal's OAuth callback.
fn portal_base_url() -> String {
    std::env::var("PORTAL_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8090".to_string())
}

/// GET /auth/{platform} — initiate OAuth flow for a platform.
pub async fn oauth_init(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> Response {
    if !SUPPORTED_PLATFORMS.contains(&platform.as_str()) {
        return (StatusCode::NOT_FOUND, format!("Unknown platform: {platform}")).into_response();
    }

    let config = match state.connectors.get(&platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, "Connector not configured yet").into_response();
        }
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    let redirect_uri = format!("{}/oauth/{}/callback", portal_base_url(), platform);

    let platform_config = PlatformConfig {
        name: platform.clone(),
        client_id: config.client_id.clone(),
        client_secret: config.client_secret.clone(),
        auth_url: String::new(),
        token_url: String::new(),
        revoke_url: None,
        redirect_uri: redirect_uri.clone(),
        scopes: config.scopes.clone(),
    };

    let handler: Box<dyn OAuth2Platform + Send + Sync> = match platform.as_str() {
        "feishu" => Box::new(omni_oauth_vault::platforms::FeishuPlatform::new(platform_config)),
        "dingtalk" => Box::new(omni_oauth_vault::platforms::DingTalkPlatform::new(platform_config)),
        "wechatwork" => Box::new(omni_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config)),
        "linkedin" => Box::new(omni_oauth_vault::platforms::LinkedInPlatform::new(platform_config)),
        "facebook" => Box::new(omni_oauth_vault::platforms::FacebookPlatform::new(platform_config)),
        "x" => Box::new(omni_oauth_vault::platforms::XPlatform::new(platform_config)),
        _ => unreachable!(),
    };

    let state_param = Uuid::new_v4().to_string();
    let auth_url = handler.get_auth_url(&state_param);

    // Store state for CSRF validation
    let oauth_session = Session {
        session_id: format!("oauth_state:{}", state_param),
        username: platform.clone(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::minutes(10),
    };
    if let Err(e) = state.sessions.insert(&oauth_session).await {
        tracing::error!("Failed to store OAuth state: {}", e);
    }

    tracing::info!("Initiating OAuth for {} -> redirect to {}", platform, auth_url);
    Redirect::to(&auth_url).into_response()
}

/// GET /oauth/{platform}/callback — handle OAuth callback from platform.
pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Response {
    if let Some(err) = &query.error {
        tracing::warn!("OAuth error for {}: {} ({})", platform, err, query.error_description.as_deref().unwrap_or(""));
        return (StatusCode::BAD_REQUEST, format!("OAuth error: {err}")).into_response();
    }

    let code = match &query.code {
        Some(c) => c,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing authorization code").into_response();
        }
    };

    let redirect_uri = format!("{}/oauth/{}/callback", portal_base_url(), platform);

    let config = match state.connectors.get(&platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, "Connector not configured").into_response();
        }
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
        }
    };

    let platform_config = PlatformConfig {
        name: platform.clone(),
        client_id: config.client_id.clone(),
        client_secret: config.client_secret.clone(),
        auth_url: String::new(),
        token_url: String::new(),
        revoke_url: None,
        redirect_uri: redirect_uri.clone(),
        scopes: config.scopes.clone(),
    };

    let handler: Box<dyn OAuth2Platform + Send + Sync> = match platform.as_str() {
        "feishu" => Box::new(omni_oauth_vault::platforms::FeishuPlatform::new(platform_config)),
        "dingtalk" => Box::new(omni_oauth_vault::platforms::DingTalkPlatform::new(platform_config)),
        "wechatwork" => Box::new(omni_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config)),
        "linkedin" => Box::new(omni_oauth_vault::platforms::LinkedInPlatform::new(platform_config)),
        "facebook" => Box::new(omni_oauth_vault::platforms::FacebookPlatform::new(platform_config)),
        "x" => Box::new(omni_oauth_vault::platforms::XPlatform::new(platform_config)),
        _ => return (StatusCode::NOT_FOUND, "Unknown platform").into_response(),
    };

    match handler.exchange_code(code, &redirect_uri).await {
        Ok(token) => {
            if let Err(e) = state.oauth_vault.store_token(token).await {
                tracing::error!("Failed to store token for {}: {}", platform, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to store token").into_response();
            }
            tracing::info!("OAuth succeeded for {}", platform);
            Redirect::to("/").into_response()
        }
        Err(e) => {
            tracing::error!("Token exchange failed for {}: {}", platform, e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Token exchange failed: {e}")).into_response()
        }
    }
}

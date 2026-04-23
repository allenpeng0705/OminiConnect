//! OAuth HTTP handlers — initiate and handle OAuth callbacks.

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
    http::StatusCode,
};
use uuid::Uuid;

use omini_connect_oauth_vault::platform::{OAuth2Platform, PlatformConfig};

use crate::app::AppState;
use crate::auth::models::Session;
use crate::oauth::models::OAuthCallbackQuery;

/// Built-in OminiConnect OAuth vault platforms (`engine=omini_connect_native` only).
const NATIVE_OAUTH_PLATFORMS: &[&str] = &["feishu", "dingtalk", "wechatwork", "linkedin", "facebook", "x"];

/// Base URL for the portal's OAuth callback.
fn portal_base_url() -> String {
    std::env::var("PORTAL_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string())
}

/// GET /auth/{platform} — initiate OAuth flow for a platform.
pub async fn oauth_init(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
) -> Response {
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

    if config.engine == "nango" {
        let Some((base, secret)) = crate::nango::nango_credentials() else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Set NANGO_BASE_URL and NANGO_SECRET_KEY to use engine=nango",
            )
                .into_response();
        };
        let integration_key = config.provider_key.trim();
        if integration_key.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                "provider_key must be set to your Nango integration unique key when engine=nango",
            )
                .into_response();
        }
        let end_user_id = crate::nango::end_user_id_for_connector(&platform);
        let scopes_opt = if config.scopes.is_empty() {
            None
        } else {
            Some((integration_key, config.scopes.as_slice()))
        };
        let body = crate::nango::connect_session_body(&end_user_id, integration_key, scopes_opt);
        return match crate::nango::create_connect_session(&base, &secret, &body).await {
            Ok(connect_link) => {
                tracing::info!("Nango Connect session for {} → redirect", platform);
                Redirect::to(&connect_link).into_response()
            }
            Err(e) => {
                tracing::error!("Nango connect/sessions failed: {}", e);
                (StatusCode::BAD_GATEWAY, format!("Nango Connect session failed: {e}")).into_response()
            }
        };
    }

    if !NATIVE_OAUTH_PLATFORMS.contains(&platform.as_str()) {
        return (
            StatusCode::NOT_FOUND,
            format!(
                "Unknown native OAuth connector `{platform}`: use engine `nango` with a Nango integration key, or a built-in native platform."
            ),
        )
            .into_response();
    }

    // Use the redirect_uri stored in the connector config (set by user in the UI)
    let redirect_uri = if config.redirect_uri.is_empty() {
        format!("{}/oauth/{}/callback", portal_base_url(), platform)
    } else {
        config.redirect_uri.clone()
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
        agent_id: config.agent_id.clone(),
    };

    let state_param = Uuid::new_v4().to_string();

    // X requires PKCE (RFC 7636): store verifier keyed by OAuth `state`, send challenge on authorize.
    let auth_url = if platform == "x" {
        let verifier = omini_connect_oauth_vault::random_code_verifier();
        let challenge = omini_connect_oauth_vault::code_challenge_s256(&verifier);
        let x = omini_connect_oauth_vault::platforms::XPlatform::new(platform_config.clone());
        let pkce_session = Session {
            session_id: format!("oauth_pkce:{}", state_param),
            username: verifier,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(10),
        };
        if let Err(e) = state.sessions.insert(&pkce_session).await {
            tracing::error!("Failed to store OAuth PKCE verifier: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to prepare OAuth session").into_response();
        }
        x.authorize_url(&state_param, &challenge)
    } else {
        let handler: Box<dyn OAuth2Platform + Send + Sync> = match platform.as_str() {
            "feishu" => Box::new(omini_connect_oauth_vault::platforms::FeishuPlatform::new(platform_config)),
            "dingtalk" => Box::new(omini_connect_oauth_vault::platforms::DingTalkPlatform::new(platform_config)),
            "wechatwork" => Box::new(omini_connect_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config)),
            "linkedin" => Box::new(omini_connect_oauth_vault::platforms::LinkedInPlatform::new(platform_config)),
            "facebook" => Box::new(omini_connect_oauth_vault::platforms::FacebookPlatform::new(platform_config)),
            _ => unreachable!(),
        };
        handler.get_auth_url(&state_param)
    };

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
    if platform == "wechatwork" {
        tracing::debug!("WeChat Work OAuth URL: {}", auth_url);
    }
    Redirect::to(&auth_url).into_response()
}

/// GET /oauth/{platform}/callback — handle OAuth callback from platform.
pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Response {
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

    if config.engine == "nango" {
        return (
            StatusCode::BAD_REQUEST,
            format!(
                "This connector uses Nango OAuth. After finishing in the Nango Connect UI, open /oauth/{platform}/nango-finalize to store the connection."
            ),
        )
            .into_response();
    }

    if !NATIVE_OAUTH_PLATFORMS.contains(&platform.as_str()) {
        return (StatusCode::NOT_FOUND, "Unknown native OAuth platform").into_response();
    }

    if let Some(err) = &query.error {
        let err_desc = query.error_description.as_deref().unwrap_or("");
        tracing::warn!("OAuth error for {}: {} ({})", platform, err, err_desc);
        return (StatusCode::BAD_REQUEST, format!("OAuth error: {err} - {err_desc}")).into_response();
    }

    let code = match &query.code {
        Some(c) => c,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing authorization code").into_response();
        }
    };

    // Use the redirect_uri stored in the connector config
    let redirect_uri = if config.redirect_uri.is_empty() {
        format!("{}/oauth/{}/callback", portal_base_url(), platform)
    } else {
        config.redirect_uri.clone()
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
        agent_id: config.agent_id.clone(),
    };

    let token_result = if platform == "x" {
        let state_val = match &query.state {
            Some(s) if !s.is_empty() => s.as_str(),
            _ => {
                return (StatusCode::BAD_REQUEST, "Missing OAuth state (required for X)").into_response();
            }
        };

        let pkce_sid = format!("oauth_pkce:{state_val}");
        let verifier = match state.sessions.get(&pkce_sid).await {
            Ok(Some(s)) => s.username,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    "Missing or expired PKCE session — go back and start Connect OAuth again",
                )
                    .into_response();
            }
        };

        let x = omini_connect_oauth_vault::platforms::XPlatform::new(platform_config);
        x.exchange_authorization_code(code, &redirect_uri, &verifier).await
    } else {
        let handler: Box<dyn OAuth2Platform + Send + Sync> = match platform.as_str() {
            "feishu" => Box::new(omini_connect_oauth_vault::platforms::FeishuPlatform::new(platform_config)),
            "dingtalk" => Box::new(omini_connect_oauth_vault::platforms::DingTalkPlatform::new(platform_config)),
            "wechatwork" => Box::new(omini_connect_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config)),
            "linkedin" => Box::new(omini_connect_oauth_vault::platforms::LinkedInPlatform::new(platform_config)),
            "facebook" => Box::new(omini_connect_oauth_vault::platforms::FacebookPlatform::new(platform_config)),
            _ => return (StatusCode::NOT_FOUND, "Unknown platform").into_response(),
        };

        handler.exchange_code(code, &redirect_uri).await
    };

    match token_result {
        Ok(token) => {
            if platform == "x" {
                if let Some(st) = query.state.as_ref().filter(|s| !s.is_empty()) {
                    let pkce_sid = format!("oauth_pkce:{st}");
                    let _ = state.sessions.delete(&pkce_sid).await;
                    let _ = state.sessions.delete(&format!("oauth_state:{st}")).await;
                }
            }

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

/// GET /oauth/{platform}/nango-finalize — after Nango Connect succeeds, persist `connection_ref` from Nango.
pub async fn nango_finalize(State(state): State<Arc<AppState>>, Path(platform): Path<String>) -> Response {
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

    if config.engine != "nango" {
        return (StatusCode::BAD_REQUEST, "Connector engine is not nango").into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Set NANGO_BASE_URL and NANGO_SECRET_KEY to finalize a Nango connection",
        )
            .into_response();
    };

    let integration_key = config.provider_key.trim();
    if integration_key.is_empty() {
        return (StatusCode::BAD_REQUEST, "provider_key is required").into_response();
    }

    let end_user_id = crate::nango::end_user_id_for_connector(&platform);
    let connections = match crate::nango::list_connections(&base, &secret, &end_user_id, Some(integration_key)).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Nango list connections failed: {}", e);
            return (StatusCode::BAD_GATEWAY, format!("Nango list connections failed: {e}")).into_response();
        }
    };

    let Some(connection_id) = crate::nango::pick_connection_id(&connections, integration_key) else {
        return (
            StatusCode::NOT_FOUND,
            "No matching Nango connection yet. Complete Connect in Nango, then try this link again.",
        )
            .into_response();
    };

    let mut updated = config;
    updated.connection_ref = connection_id.clone();
    if let Err(e) = state.connectors.upsert(&updated).await {
        tracing::error!("Failed to persist connection_ref: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save connection").into_response();
    }

    tracing::info!("Nango connection_ref saved for {} ({})", platform, connection_id);
    Redirect::to("/").into_response()
}

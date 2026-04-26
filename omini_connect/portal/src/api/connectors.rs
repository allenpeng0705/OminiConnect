//! Connector management API.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::connector_engine::{is_api_key_platform, test_connector_by_engine};
use crate::connector_scope::{catalog_entry_key, find_duplicate_platform_for_entry, oauth_vault_platform_key};
use crate::oauth::models::ConnectorConfig;

/// Nango Connect / managed hub: explicit `engine=nango`, integration unique keys (`provider__suffix`),
/// or a legacy redirect like `/oauth/{connector-slug}/callback` where the slug is not a built-in native platform.
fn uses_nango_oauth_callback(engine: &str, provider_key: &str, redirect_uri: &str) -> bool {
    if engine.trim().eq_ignore_ascii_case("nango") || provider_key.contains("__") {
        return true;
    }
    let ru = redirect_uri.trim();
    if ru.is_empty() {
        return false;
    }
    let Ok(u) = reqwest::Url::parse(ru) else {
        return false;
    };
    let path = u.path().trim_end_matches('/');
    let Some(mid) = path.strip_prefix("/oauth/").and_then(|s| s.strip_suffix("/callback")) else {
        return false;
    };
    if mid.is_empty() || mid == "callback" {
        return false;
    }
    const NATIVE_OAUTH_PLATFORMS: &[&str] = &["feishu", "dingtalk", "wechatwork", "linkedin", "facebook", "x"];
    !NATIVE_OAUTH_PLATFORMS.contains(&mid)
}

fn managed_provider_and_integration_key(provider_key: &str) -> (String, String) {
    let trimmed = provider_key.trim();
    if let Some((provider, unique_key)) = trimmed.split_once("__") {
        let p = provider.trim();
        let u = unique_key.trim();
        if !p.is_empty() && !u.is_empty() {
            return (p.to_string(), trimmed.to_string());
        }
    }
    (trimmed.to_string(), trimmed.to_string())
}

/// GET /api/connectors
pub async fn list(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    };

    let connectors = state.connectors.list(&auth.username).await.map_err(|e| {
        tracing::error!("DB error listing connectors: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" })))
    });

    match connectors {
        Ok(list) => {
            let portal = crate::portal_env::portal_base_url()
                .trim()
                .trim_end_matches('/')
                .to_string();
            let json: Vec<serde_json::Value> = list
                .iter()
                .map(|c| {
                    let redirect_uri = if uses_nango_oauth_callback(&c.engine, &c.provider_key, &c.redirect_uri) && !portal.is_empty() {
                        format!("{portal}/oauth/callback")
                    } else {
                        c.redirect_uri.clone()
                    };
                    serde_json::json!({
                        "platform": c.platform,
                        "client_id": c.client_id,
                        "has_client_secret": !c.client_secret.is_empty(),
                        "redirect_uri": redirect_uri,
                        "scopes": c.scopes,
                        "engine": c.engine,
                        "provider_key": c.provider_key,
                        "connection_ref": c.connection_ref,
                        "agent_id": c.agent_id,
                        "enabled": c.enabled,
                        "catalog_entry": catalog_entry_key(&c.engine, &c.platform, &c.provider_key),
                        "connected": false,
                    })
                })
                .collect();
            Json(json).into_response()
        }
        Err(r) => r.into_response(),
    }
}

/// POST /api/connectors
pub async fn upsert(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(mut payload): Json<ConnectorConfig>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    };
    let owner = auth.username.as_str();

    let existing = state
        .connectors
        .get(owner, &payload.platform)
        .await
        .ok()
        .flatten();

    // Preserve existing client_secret if not provided (empty string means unchanged)
    if payload.client_secret.is_empty() {
        if let Some(existing) = existing.as_ref() {
            payload.client_secret = existing.client_secret.clone();
        }
    }

    // Keep previous engine/provider/connection_ref when omitted in partial updates.
    if payload.engine.trim().is_empty() {
        payload.engine = existing
            .as_ref()
            .map(|c| c.engine.clone())
            .unwrap_or_else(|| "omini_connect_native".to_string());
    }
    if payload.provider_key.trim().is_empty() {
        payload.provider_key = existing
            .as_ref()
            .map(|c| c.provider_key.clone())
            .unwrap_or_else(|| payload.platform.clone());
    }
    if payload.connection_ref.trim().is_empty() {
        payload.connection_ref = existing
            .as_ref()
            .map(|c| c.connection_ref.clone())
            .unwrap_or_default();
    }

    // Managed hub: OAuth always returns to Nango's `/oauth/callback` on the public API origin
    // (OminiConnect portal proxies it). Ignore client-sent `/oauth/{connector-id}/callback`.
    if uses_nango_oauth_callback(&payload.engine, &payload.provider_key, &payload.redirect_uri) {
        let portal = crate::portal_env::portal_base_url()
            .trim()
            .trim_end_matches('/')
            .to_string();
        if !portal.is_empty() {
            let fixed = format!("{portal}/oauth/callback");
            if payload.redirect_uri.trim() != fixed {
                tracing::info!(
                    platform = %payload.platform,
                    engine = %payload.engine,
                    provider_key = %payload.provider_key,
                    old = %payload.redirect_uri,
                    new = %fixed,
                    "normalized redirect_uri for managed hub (Nango)"
                );
            }
            payload.redirect_uri = fixed;
        }
    }

    let mine = match state.connectors.list(owner).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("DB error listing connectors for duplicate check: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
        }
    };

    if let Some(other) = find_duplicate_platform_for_entry(
        owner,
        &payload.platform,
        &payload.engine,
        &payload.platform,
        &payload.provider_key,
        &mine,
    ) {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "A connector for this integration already exists for your account.",
                "existing_platform": other,
                "catalog_entry": catalog_entry_key(&payload.engine, &payload.platform, &payload.provider_key),
            })),
        )
            .into_response();
    }

    if payload.engine == "nango" {
        let provider_key = payload.provider_key.trim();
        if provider_key.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "provider_key is required for managed connectors" })),
            )
                .into_response();
        }

        // Auto-provision integration in Nango. For OAuth2 providers (github, slack, etc.),
        // Nango requires client_id + client_secret at integration creation.
        // For API_KEY providers (airtable-pat, etc.), credentials are entered directly
        // in Nango Connect UI, so empty credentials are fine at integration creation.
        // Nango API will return "Missing credentials" if OAuth2 provider is missing credentials.
        let (provider, integration_unique_key) = managed_provider_and_integration_key(provider_key);
        let Some((base, secret)) = crate::nango::nango_credentials() else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "error": "NANGO_BASE_URL and NANGO_SECRET_KEY are required to create managed integrations" })),
            )
                .into_response();
        };
        if let Err(e) = crate::nango::ensure_integration_catalog(
            &base,
            &secret,
            &provider,
            &integration_unique_key,
            &payload.client_id,
            &payload.client_secret,
            &payload.scopes,
        )
        .await
        {
            tracing::error!(
                platform = %payload.platform,
                provider_key = %provider_key,
                provider = %provider,
                integration_key = %integration_unique_key,
                error = %e,
                "Failed to auto-provision Nango integration"
            );
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "error": format!("Failed to create managed integration in Nango: {e}")
                })),
            )
                .into_response();
        }
    }

    payload.owner_username = owner.to_string();

    if let Err(e) = state.connectors.upsert(owner, &payload).await {
        tracing::error!("DB error upserting connector: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
    }
    tracing::info!("Connector {} saved for {}", payload.platform, owner);
    (StatusCode::CREATED, Json(serde_json::json!({ "message": "saved" }))).into_response()
}

/// DELETE /api/connectors/:platform
pub async fn delete(State(state): State<Arc<AppState>>, headers: HeaderMap, Path(platform): Path<String>) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    };
    let owner = auth.username.as_str();

    if let Err(e) = state.connectors.delete(owner, &platform).await {
        tracing::error!("DB error deleting connector: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
    }
    let vk = oauth_vault_platform_key(owner, &platform);
    let _ = state.oauth_vault.delete_token(&vk, "user").await;
    tracing::info!("Connector {} deleted for {}", platform, owner);
    (StatusCode::OK, Json(serde_json::json!({ "message": "deleted" }))).into_response()
}

/// GET /api/connectors/:platform/status
pub async fn status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    };
    let owner = auth.username.as_str();

    let connector = state.connectors.get(owner, &platform).await.ok().flatten();
    let configured = connector.is_some();

    let connected = if let Some(c) = connector.as_ref() {
        match c.engine.as_str() {
            "nango" => !c.connection_ref.trim().is_empty(),
            _ if is_api_key_platform(&platform) => match platform.as_str() {
                "qqmail" => !c.client_id.trim().is_empty() && !c.client_secret.trim().is_empty(),
                "maton" | "github" => !c.client_id.trim().is_empty() || !c.client_secret.trim().is_empty(),
                _ => !c.client_id.is_empty(),
            },
            _ => {
                let vk = oauth_vault_platform_key(owner, &platform);
                state.oauth_vault.get_token(&vk, "user").await.is_ok()
            }
        }
    } else {
        false
    };

    Json(serde_json::json!({
        "platform": platform,
        "configured": configured,
        "connected": connected,
    }))
    .into_response()
}

/// POST /api/connectors/:platform/test
pub async fn test(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(platform): Path<String>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    };
    let owner = auth.username.as_str();

    let connector = match state.connectors.get(owner, &platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not configured" }))).into_response();
        }
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "db error" }))).into_response();
        }
    };
    if !connector.enabled {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "connector is disabled" })),
        )
            .into_response();
    }

    match test_connector_by_engine(&state, &connector, owner).await {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!({
            "platform": platform,
            "engine": connector.engine,
            "status": result.status,
            "message": result.message,
        })))
            .into_response(),
        Err(e) => {
            tracing::error!("Connector test failed for {}: {}", platform, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "connector test failed",
            })))
                .into_response()
        }
    }
}

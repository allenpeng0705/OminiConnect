//! Nango integration + provider catalog for the portal UI.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::middleware::try_auth;
use crate::oauth::models::ConnectorConfig;

#[derive(Debug, Deserialize, Default)]
pub struct NangoProvidersQuery {
    /// Forwarded to Nango `GET /providers?search=` (provider id filter).
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NangoConnectSessionBody {
    /// OminiConnect connector id (`/api/proxy/{platform}/…`).
    pub platform: String,
    /// Optional portal origin (`window.location.origin`) so local `localhost:9000` matches the iframe
    /// even when `NANGO_BROWSER_API_URL` points at a public tunnel.
    #[serde(default)]
    pub connect_api_base: Option<String>,
}

/// POST /api/nango/connections — create a Nango connection with credentials directly.
/// Used for API_KEY and BASIC auth to provision connections without the Nango Connect hosted UI.
pub async fn post_nango_connection(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<super::nango_connection::CreateNangoConnectionBody>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    };
    let owner = auth.username.as_str();

    let platform = body.platform.trim();
    if platform.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "platform is required" })),
        )
            .into_response();
    }

    let config: ConnectorConfig = match state.connectors.get(owner, platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "connector not found" })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("create-nango-connection DB: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "database error" })),
            )
                .into_response();
        }
    };

    if config.engine != "nango" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "connector engine is not nango" })),
        )
            .into_response();
    }
    if !config.enabled {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "connector is disabled" })),
        )
            .into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Set NANGO_BASE_URL and NANGO_SECRET_KEY to create managed connections"
            })),
        )
            .into_response();
    };

    let integration_key = config.provider_key.trim();
    if integration_key.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "provider_key must be set for managed connectors" })),
        )
            .into_response();
    }

    let end_user_id = crate::nango::end_user_id_for_connector(owner, platform);

    let credentials = match body.auth_mode.as_str() {
        "API_KEY" => {
            let api_key = body.api_key.as_deref().unwrap_or("").trim();
            if api_key.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": "api_key is required for API_KEY auth" })),
                )
                    .into_response();
            }
            serde_json::json!({ "type": "API_KEY", "apiKey": api_key })
        }
        "BASIC" => {
            let username = body.username.as_deref().unwrap_or("").trim();
            let password = body.password.as_deref().unwrap_or("").trim();
            if username.is_empty() && password.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": "username or password is required for BASIC auth" })),
                )
                    .into_response();
            }
            serde_json::json!({ "type": "BASIC", "username": username, "password": password })
        }
        "OAUTH2_CC" => {
            // OAuth 2.0 Client Credentials — app-level auth, no user interaction.
            // User provides client_id + client_secret in the portal form.
            let client_id = body.api_key.as_deref().unwrap_or("").trim();
            let client_secret = body.password.as_deref().unwrap_or("").trim();
            if client_id.is_empty() || client_secret.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": "client_id and client_secret are required for OAUTH2_CC auth" })),
                )
                    .into_response();
            }
            serde_json::json!({
                "type": "OAUTH2_CC",
                "client_id": client_id,
                "client_secret": client_secret
            })
        }
        "SIGNATURE" => {
            // Emarsys WSSE auth — username + password, Nango computes the WSSE digest.
            let username = body.username.as_deref().unwrap_or("").trim();
            let password = body.password.as_deref().unwrap_or("").trim();
            if username.is_empty() && password.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": "username or password is required for SIGNATURE auth" })),
                )
                    .into_response();
            }
            serde_json::json!({ "type": "SIGNATURE", "username": username, "password": password })
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "unsupported auth mode; use API_KEY, BASIC, OAUTH2_CC, or SIGNATURE" })),
            )
                .into_response();
        }
    };

    match crate::nango::create_nango_connection(
        &base,
        &secret,
        integration_key,
        &end_user_id,
        &credentials,
    )
    .await
    {
        Ok(connection_id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({ "connection_id": connection_id })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Nango POST /connections failed for {}: {}", platform, e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

/// POST /api/nango/connect-session — start Nango Connect for an `engine=nango` connector (portal user stays in OminiConnect; open `connect_url` in a popup).
pub async fn post_connect_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<NangoConnectSessionBody>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    };
    let owner = auth.username.as_str();

    let platform = body.platform.trim().to_string();
    if platform.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "platform is required" })),
        )
            .into_response();
    }

    let config: ConnectorConfig = match state.connectors.get(owner, &platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "connector not found" })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("connect-session DB: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "database error" })),
            )
                .into_response();
        }
    };

    if config.engine != "nango" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "connector engine is not nango" })),
        )
            .into_response();
    }
    if !config.enabled {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "connector is disabled" })),
        )
            .into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Set NANGO_BASE_URL and NANGO_SECRET_KEY to start Nango Connect"
            })),
        )
            .into_response();
    };

    let integration_key = config.provider_key.trim();
    if integration_key.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "provider_key must be set for managed connectors" })),
        )
            .into_response();
    }

    let end_user_id = crate::nango::end_user_id_for_connector(owner, &platform);
    let scopes_opt = if config.scopes.is_empty() {
        None
    } else {
        Some((integration_key, config.scopes.as_slice()))
    };
    let json_body = crate::nango::connect_session_body(&end_user_id, integration_key, scopes_opt);

    match crate::nango::create_connect_session(
        &base,
        &secret,
        &json_body,
        body.connect_api_base.as_deref(),
    )
    .await
    {
        Ok(connect_url) => (
            StatusCode::OK,
            Json(serde_json::json!({ "connect_url": connect_url })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Nango connect/sessions: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

/// GET /api/nango/connections?platform=xxx — list Nango connections for the current user.
pub async fn list_nango_connections(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<NangoConnectionsQuery>,
) -> impl IntoResponse {
    let Some(auth) = try_auth(&state, &headers).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    };

    let platform = q.platform.trim();
    if platform.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "platform query param is required" })),
        )
            .into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Nango is not configured" })),
        )
            .into_response();
    };

    let end_user_id = crate::nango::end_user_id_for_connector(&auth.username, platform);
    let connector = match state.connectors.get(&auth.username, platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "connector not found" })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "database error" })),
            )
                .into_response();
        }
    };

    let integration_id = Some(connector.provider_key.trim()).filter(|s| !s.is_empty());

    match crate::nango::list_connections(&base, &secret, &end_user_id, integration_id).await {
        Ok(connections) => (StatusCode::OK, Json(connections)).into_response(),
        Err(e) => {
            tracing::error!("Nango list_connections failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NangoConnectionsQuery {
    pub platform: String,
}

/// GET /api/nango/integrations — list Nango environment integrations (requires login).
pub async fn list_integrations(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if try_auth(&state, &headers).await.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (
            StatusCode::OK,
            Json(Vec::<crate::nango::NangoIntegrationCatalogItem>::new()),
        )
            .into_response();
    };

    match crate::nango::list_integrations_catalog(&base, &secret).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            tracing::error!("Nango catalog: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

/// GET /api/nango/providers — full Nango provider catalog (same set Nango exposes in `providers.yaml`).
pub async fn list_providers(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<NangoProvidersQuery>,
) -> impl IntoResponse {
    if try_auth(&state, &headers).await.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
            .into_response();
    }

    let search = q.search.as_deref();
    let base = std::env::var("NANGO_BASE_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let secret = std::env::var("NANGO_SECRET_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let result = match (base.as_ref(), secret.as_ref()) {
        (Some(b), Some(s)) => crate::nango::list_providers_catalog(b, s, search).await,
        (Some(b), None) => {
            tracing::info!("NANGO_SECRET_KEY unset: loading integration library from Nango GET /providers.json only");
            crate::nango::list_providers_catalog_public_only(b, search).await
        }
        (None, _) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Nango is not configured: set NANGO_BASE_URL (and usually NANGO_SECRET_KEY) in the portal environment — see repo-root .env.example."
                })),
            )
                .into_response();
        }
    };

    match result {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            tracing::error!("Nango providers: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

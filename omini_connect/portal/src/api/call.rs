//! `POST /api/call/{platform}` — Maton-style gateway call.
//!
//! Accepts a JSON body describing the upstream call and forwards it.
//! Simpler than the REST-style proxy because the method/path come in the body,
//! so any HTTP verb works with any path.

use std::{sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

use crate::app::AppState;
use crate::connector_scope::oauth_vault_platform_key;
use super::proxy::{get_platform_base_url, map_reqwest_to_axum, proxy_error_response, send_reqwest_response};

/// Request body for POST /api/call/{platform}
#[derive(Debug, Deserialize)]
pub struct CallRequest {
    /// HTTP method: GET, POST, PUT, DELETE, PATCH
    pub method: String,
    /// Upstream API path, e.g. "/user/repos"
    pub path: String,
    /// Query string parameters (optional)
    #[serde(default)]
    pub params: serde_json::Map<String, serde_json::Value>,
    /// Request body for POST/PUT/PATCH (optional)
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

/// Handle POST /api/call/{platform}
pub async fn handle_call(
    State(state): State<Arc<AppState>>,
    Path(platform): Path<String>,
    headers: HeaderMap,
    Json(req): Json<CallRequest>,
) -> impl IntoResponse {
    // 1. Auth — same logic as proxy
    let api_key = match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        Some(v) => v.strip_prefix("Bearer ").unwrap_or(v),
        None => {
            return proxy_error_response(StatusCode::UNAUTHORIZED, "missing authorization header");
        }
    };

    let api_keys = match state.api_keys.list_all().await {
        Ok(keys) => keys,
        Err(_) => {
            return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to list API keys");
        }
    };

    let mut key_owner: Option<String> = None;
    for ak in api_keys {
        if bcrypt::verify(api_key, &ak.key_hash).ok() == Some(true) {
            key_owner = Some(ak.username.clone());
            break;
        }
        let old_raw = format!("{}:{}", ak.username, api_key);
        if bcrypt::verify(&old_raw, &ak.key_hash).ok() == Some(true) {
            key_owner = Some(ak.username.clone());
            break;
        }
    }

    let Some(key_owner) = key_owner else {
        return proxy_error_response(StatusCode::UNAUTHORIZED, "invalid API key");
    };

    // 2. Connector lookup — same logic as proxy
    let connector = match state.connectors.get(&key_owner, &platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            tracing::warn!("call: no connector for owner={}, platform={}", key_owner, platform);
            return proxy_error_response(StatusCode::NOT_FOUND, "platform not configured");
        }
        Err(_) => {
            return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to get connector");
        }
    };

    if !connector.enabled {
        return proxy_error_response(StatusCode::FORBIDDEN, "connector is disabled");
    }

    // 3. Build upstream URL
    let method = Method::from_bytes(req.method.as_bytes())
        .unwrap_or(Method::GET);

    // Build query string from params
    let query_string: String = if req.params.is_empty() {
        String::new()
    } else {
        let pairs: Vec<String> = req.params
            .iter()
            .map(|(k, v)| {
                let val = match v {
                    serde_json::Value::String(s) => s.clone(),
                    _ => v.to_string(),
                };
                format!("{}={}", urlencoding::encode(k), urlencoding::encode(&val))
            })
            .collect();
        format!("?{}", pairs.join("&"))
    };

    let upstream_url = format!(
        "{}/{}{}",
        get_platform_base_url(&platform),
        req.path.trim_start_matches('/'),
        query_string
    );

    // 4. Build body bytes
    let body_bytes: Bytes = req.body
        .map(|b| serde_json::to_vec(&b).unwrap_or_default())
        .map(Bytes::from)
        .unwrap_or_default();

    // 5. Forward based on engine — same three paths as proxy
    let use_static_bearer = platform == "maton"
        || platform == "qqmail"
        || (platform == "github" && connector.engine != "nango");

    if use_static_bearer {
        let access_token = if platform == "maton" || platform == "github" {
            let t = connector.client_secret.trim();
            if t.is_empty() {
                connector.client_id.trim().to_string()
            } else {
                connector.client_secret.clone()
            }
        } else {
            connector.client_secret.clone()
        };

        if access_token.is_empty() {
            return proxy_error_response(StatusCode::UNAUTHORIZED, "api key not configured");
        }

        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to create HTTP client");
            }
        };

        let mut req_builder = client.request(method, &upstream_url);
        req_builder = req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
        req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");
        req_builder = req_builder.header("Content-Type", "application/json");
        if !body_bytes.is_empty() {
            req_builder = req_builder.body(body_bytes);
        }

        return send_reqwest_response(req_builder.send().await, &platform).await;
    }

    if connector.engine == "nango" {
        let Some((base, secret)) = crate::nango::nango_credentials() else {
            return proxy_error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                "NANGO_BASE_URL and NANGO_SECRET_KEY must be set for nango engine",
            );
        };

        let pk = connector.provider_key.trim();
        let cref = connector.connection_ref.trim();
        if pk.is_empty() || cref.is_empty() {
            return proxy_error_response(
                StatusCode::UNAUTHORIZED,
                "nango connector missing provider_key or connection_ref; finalize after Connect",
            );
        }

        let rq_method = reqwest::Method::from_bytes(req.method.as_bytes())
            .unwrap_or(reqwest::Method::GET);

        match crate::nango::forward_proxy(
            &base,
            &secret,
            pk,
            cref,
            rq_method,
            &req.path,
            body_bytes,
            Some("application/json"),
            None,
        )
        .await
        {
            Ok(resp) => map_reqwest_to_axum(resp).await,
            Err(e) => {
                tracing::error!("Nango call error for {}: {}", platform, e);
                proxy_error_response(StatusCode::BAD_GATEWAY, "nango proxy request failed")
            }
        }
    } else {
        // OAuth vault
        let vk = oauth_vault_platform_key(&key_owner, &platform);
        let access_token = match state.oauth_vault.get_token(&vk, "user").await {
            Ok(token) => token,
            Err(e) => {
                tracing::warn!("call: no token for {}: {}", platform, e);
                return proxy_error_response(StatusCode::UNAUTHORIZED, "no token available");
            }
        };

        let base = get_platform_base_url(&platform);
        if base.is_empty() {
            return proxy_error_response(
                StatusCode::BAD_REQUEST,
                "unknown native platform: use engine=nango for Nango-managed APIs",
            );
        }

        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to create HTTP client");
            }
        };

        let mut req_builder = client.request(method, &upstream_url);
        req_builder = req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
        req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");
        req_builder = req_builder.header("Content-Type", "application/json");
        if !body_bytes.is_empty() {
            req_builder = req_builder.body(body_bytes);
        }

        send_reqwest_response(req_builder.send().await, &platform).await
    }
}

/// Router for call endpoint.
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/call/{platform}", post(handle_call))
}

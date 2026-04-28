//! Proxy endpoint — forwards requests to connected platforms using stored OAuth tokens.
//!
//! Maton-style passthrough proxy: POST /api/proxy/{platform}/{native-api-path}
//! OminiConnect injects the stored OAuth token and forwards to native API.

use std::{sync::Arc, time::Duration};

use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use reqwest::header::AUTHORIZATION;
use reqwest::Method as ReqwestMethod;

use crate::app::AppState;
use crate::connector_scope::oauth_vault_platform_key;

/// Proxy endpoint — forwards requests to connected platforms using stored OAuth tokens.
pub async fn forward(
    State(state): State<Arc<AppState>>,
    Path((platform, native_path)): Path<(String, String)>,
    headers: HeaderMap,
    method: axum::http::Method,
    body: Bytes,
) -> axum::response::Response<Body> {
    // 1. Auth: require Bearer token (OminiConnect API key)
    let api_key = match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        Some(v) => v.strip_prefix("Bearer ").unwrap_or(v),
        None => {
            return proxy_error_response(StatusCode::UNAUTHORIZED, "missing authorization header");
        }
    };

    // Validate API key by checking against stored keys
    let api_keys = match state.api_keys.list_all().await {
        Ok(keys) => keys,
        Err(_) => {
            return proxy_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to list API keys",
            );
        }
    };

    let mut key_owner: Option<String> = None;
    for ak in api_keys {
        // New keys are plain UUIDs; old keys use "username:uuid" format.
        // Try plain key first, then fall back to old format with colon suffix.
        if bcrypt::verify(api_key, &ak.key_hash).ok() == Some(true) {
            key_owner = Some(ak.username.clone());
            break;
        }
        // Try treating api_key as the uuid part of an old "username:uuid" format
        let old_raw = format!("{}:{}", ak.username, api_key);
        if bcrypt::verify(&old_raw, &ak.key_hash).ok() == Some(true) {
            key_owner = Some(ak.username.clone());
            break;
        }
    }

    let Some(key_owner) = key_owner else {
        return proxy_error_response(StatusCode::UNAUTHORIZED, "invalid API key");
    };

    let connector = match state.connectors.get(&key_owner, &platform).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            tracing::warn!(
                "proxy: no connector found for owner={}, platform={}",
                key_owner,
                platform
            );
            return proxy_error_response(StatusCode::NOT_FOUND, "platform not configured");
        }
        Err(_) => {
            return proxy_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to get connector",
            );
        }
    };

    if !connector.enabled {
        return proxy_error_response(StatusCode::FORBIDDEN, "connector is disabled");
    }

    // 2–5. Forward: API-key / PAT platforms, Nango-backed OAuth, or native vault token
    let use_static_bearer = platform == "maton"
        || platform == "qqmail"
        || (platform == "github" && connector.engine != "nango");
    if use_static_bearer {
        // Maton: portal "API Key" is stored in `client_id` (Nango API_KEY style); `client_secret` if set also works.
        // GitHub (native PAT): same as Maton — classic or fine-grained PAT in `client_secret` and/or `client_id`.
        // QQ ExMail: corp id in `client_id`, access secret in `client_secret` (Bearer = secret for this passthrough).
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
        let upstream_url = format!("{}/{}", get_platform_base_url(&platform), native_path);
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                return proxy_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to create HTTP client",
                )
            }
        };
        let mut req_builder = client.request(method.clone(), &upstream_url);
        req_builder =
            req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
        req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");
        let content_type = headers.get("content-type").and_then(|v| v.to_str().ok());
        if let Some(ct) = content_type {
            req_builder = req_builder.header("Content-Type", ct);
        }
        if !body.is_empty() {
            req_builder = req_builder.body(body);
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
        let rq_method =
            ReqwestMethod::from_bytes(method.as_str().as_bytes()).unwrap_or(ReqwestMethod::GET);
        let content_type = headers.get("content-type").and_then(|v| v.to_str().ok());
        let linkedin_version = headers
            .get("x-linkedin-version")
            .and_then(|v| v.to_str().ok());
        match crate::nango::forward_proxy(
            &base,
            &secret,
            pk,
            cref,
            rq_method,
            &native_path,
            body,
            content_type,
            linkedin_version,
        )
        .await
        {
            Ok(resp) => map_reqwest_to_axum(resp).await,
            Err(e) => {
                tracing::error!("Nango proxy error for {}: {}", platform, e);
                proxy_error_response(StatusCode::BAD_GATEWAY, "nango proxy request failed")
            }
        }
    } else {
        let vk = oauth_vault_platform_key(&key_owner, &platform);
        let access_token = match state.oauth_vault.get_token(&vk, "user").await {
            Ok(token) => token,
            Err(e) => {
                tracing::warn!("No token for {}: {}", platform, e);
                return proxy_error_response(StatusCode::UNAUTHORIZED, "no token available");
            }
        };

        let base = get_platform_base_url(&platform);
        if base.is_empty() {
            return proxy_error_response(
                StatusCode::BAD_REQUEST,
                "unknown native platform: use engine=nango for Nango-managed APIs, or a built-in native connector",
            );
        }
        let upstream_url = format!("{}/{}", base, native_path);
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                return proxy_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to create HTTP client",
                )
            }
        };

        let mut req_builder = client.request(method.clone(), &upstream_url);
        req_builder =
            req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));
        req_builder = req_builder.header("User-Agent", "OminiConnect/1.0");

        let content_type = headers.get("content-type").and_then(|v| v.to_str().ok());
        if platform == "linkedin" {
            if let Some(version) = headers
                .get("x-linkedin-version")
                .and_then(|v| v.to_str().ok())
            {
                req_builder = req_builder.header("LinkedIn-Version", version);
            }
        }
        if let Some(ct) = content_type {
            req_builder = req_builder.header("Content-Type", ct);
        }
        if !body.is_empty() {
            req_builder = req_builder.body(body);
        }
        send_reqwest_response(req_builder.send().await, &platform).await
    }
}

pub(crate) async fn send_reqwest_response(
    result: Result<reqwest::Response, reqwest::Error>,
    platform: &str,
) -> axum::response::Response<Body> {
    match result {
        Ok(resp) => map_reqwest_to_axum(resp).await,
        Err(e) => {
            tracing::error!("Proxy request failed for {}: {}", platform, e);
            let err_msg = if e.is_timeout() {
                "upstream request timed out"
            } else if e.is_connect() {
                "upstream connection failed"
            } else {
                "upstream request failed"
            };
            proxy_error_response(StatusCode::BAD_GATEWAY, err_msg)
        }
    }
}

pub(crate) async fn map_reqwest_to_axum(resp: reqwest::Response) -> axum::response::Response<Body> {
    let status = resp.status();
    let body = resp.bytes().await.unwrap_or_default();
    let code = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let mut response = axum::response::Response::new(body.into());
    *response.status_mut() = code;
    response
}

pub(crate) fn proxy_error_response(
    status: StatusCode,
    message: &str,
) -> axum::response::Response<Body> {
    let body = serde_json::json!({ "error": message }).to_string();
    let mut response = axum::response::Response::new(body.into());
    *response.status_mut() = status;
    response
}

pub(crate) fn get_platform_base_url(platform: &str) -> &'static str {
    match platform {
        "feishu" => "https://open.feishu.cn/open-apis",
        "dingtalk" => "https://api.dingtalk.com",
        "wechatwork" => "https://qyapi.weixin.qq.com",
        "linkedin" => "https://api.linkedin.com",
        "facebook" => "https://graph.facebook.com/v21.0",
        "x" => "https://api.x.com/2",
        "github" | "github-pat" => "https://api.github.com",
        "maton" => "https://api.maton.ai",
        "qqmail" => "https://api.exmail.qq.com",
        _ => "",
    }
}

//! Proxy endpoint — forwards requests to connected platforms using stored OAuth tokens.
//!
//! Maton-style passthrough proxy: POST /api/proxy/{platform}/{native-api-path}
//! OmniConnect injects the stored OAuth token and forwards to native API.

use std::{sync::Arc, time::Duration};

use axum::{
    body::{Bytes, Body},
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use reqwest::header::AUTHORIZATION;

use crate::app::AppState;

/// Proxy endpoint — forwards requests to connected platforms using stored OAuth tokens.
pub async fn forward(
    State(state): State<Arc<AppState>>,
    Path((platform, native_path)): Path<(String, String)>,
    headers: HeaderMap,
    method: axum::http::Method,
    body: Bytes,
) -> axum::response::Response<Body> {
    tracing::info!("PROXY CALLED: platform={}, path={}, method={}", platform, native_path, method);
    // 1. Auth: require Bearer token (OmniConnect API key)
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
            return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to list API keys");
        }
    };

    let mut valid_key = false;
    for ak in api_keys {
        if bcrypt::verify(api_key, &ak.key_hash).ok() == Some(true) {
            valid_key = true;
            break;
        }
    }

    if !valid_key {
        tracing::warn!("PROXY: Invalid API key");
        return proxy_error_response(StatusCode::UNAUTHORIZED, "invalid API key");
    }

    tracing::info!("PROXY: API key validated, getting token for {}", platform);

    // 2. Get access token for platform
    let access_token = if platform == "maton" || platform == "qqmail" {
        // For API-key platforms, use client_secret as the API key
        match state.connectors.get(&platform).await {
            Ok(Some(conn)) => conn.client_secret.clone(),
            Ok(None) => {
                return proxy_error_response(StatusCode::NOT_FOUND, "platform not configured");
            }
            Err(_) => {
                return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to get connector");
            }
        }
    } else {
        // For OAuth platforms, get token from vault (auto-refreshed if needed)
        match state.oauth_vault.get_token(&platform, "user").await {
            Ok(token) => token,
            Err(e) => {
                tracing::warn!("No token for {}: {}", platform, e);
                return proxy_error_response(StatusCode::UNAUTHORIZED, "no token available");
            }
        }
    };

    // 3. Build upstream URL
    let upstream_url = format!("{}/{}", get_platform_base_url(&platform), native_path);
    tracing::debug!("Proxy request: platform={}, upstream_url={}", platform, upstream_url);

    // 4. Forward to native API
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(60)) // Longer timeout for uploads
        .build()
    {
        Ok(c) => c,
        Err(_) => {
            return proxy_error_response(StatusCode::INTERNAL_SERVER_ERROR, "failed to create HTTP client");
        }
    };

    let mut req_builder = client.request(method.clone(), &upstream_url);

    // Inject Authorization header with the access token
    req_builder = req_builder.header(AUTHORIZATION.as_str(), format!("Bearer {}", access_token));

    // Forward relevant headers
    let content_type = headers.get("content-type").and_then(|v| v.to_str().ok());
    let is_multipart = content_type.map(|ct| ct.starts_with("multipart/")).unwrap_or(false);

    // For LinkedIn, add version header if present in query params
    if platform == "linkedin" {
        if let Some(version) = headers.get("x-linkedin-version").and_then(|v| v.to_str().ok()) {
            req_builder = req_builder.header("LinkedIn-Version", version);
        }
    }

    // Forward Content-Type if present
    if let Some(ct) = content_type {
        req_builder = req_builder.header("Content-Type", ct);
    }

    // Add body - binary data for multipart, regular body otherwise
    if !body.is_empty() {
        req_builder = req_builder.body(body);
    }

    // 5. Send request and forward response
    tracing::info!("Sending request to upstream_url={}", upstream_url);
    let resp = req_builder.send().await;
    tracing::info!("Got response: {:?}", resp);
    match resp {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.bytes().await.unwrap_or_default();

            let mut response = axum::response::Response::new(body.into());
            *response.status_mut() = status;
            response
        }
        Err(e) => {
            tracing::error!("Proxy request failed for {}: {}", platform, e);
            proxy_error_response(StatusCode::BAD_GATEWAY, "upstream request failed")
        }
    }
}

fn proxy_error_response(status: StatusCode, message: &str) -> axum::response::Response<Body> {
    let body = serde_json::json!({ "error": message }).to_string();
    let mut response = axum::response::Response::new(body.into());
    *response.status_mut() = status;
    response
}

fn get_platform_base_url(platform: &str) -> &'static str {
    match platform {
        "feishu" => "https://open.feishu.cn/open-apis",
        "dingtalk" => "https://api.dingtalk.com",
        "wechatwork" => "https://qyapi.weixin.qq.com",
        "linkedin" => "https://api.linkedin.com",
        "facebook" => "https://graph.facebook.com/v21.0",
        "x" => "https://api.x.com/2",
        "maton" => "https://api.maton.ai",
        "qqmail" => "https://api.exmail.qq.com",
        _ => "",
    }
}

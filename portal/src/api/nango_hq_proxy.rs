//! Reverse proxy to the Nango HTTP API when only the portal is exposed on the public internet.
//!
//! Nango Connect UI builds requests with `new URL(apiURL)` then **assigns** `pathname` to
//! `/connect/session`, `/integrations`, etc. — so `apiURL` must be an **origin only** (e.g.
//! `https://portal.example`), not `{origin}/__omini/nango-hq`.
//!
//! With a single tunnel to the portal, set `NANGO_BROWSER_API_URL` to the same origin as
//! `PORTAL_BASE_URL` (no path, or `/` only). The portal forwards these routes at the **root**
//! to `NANGO_BASE_URL`. The nested `/__omini/nango-hq/*` proxy remains for backward compatibility.
//!
//! For `GET /connect/session`, if the client omits `Authorization` but passes `session_token` or
//! `connect_session_token` in the query string, the proxy adds `Authorization: Bearer …` before
//! forwarding (Nango otherwise returns `missing_auth_header`).

use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::header::{self, HeaderMap, HeaderName, HeaderValue};
use axum::http::{Method, Request, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use http_body_util::BodyExt;
use reqwest::Client;

use crate::app::AppState;

const MAX_PROXY_BODY: usize = 16 * 1024 * 1024;

fn internal_nango_base() -> Option<String> {
    std::env::var("NANGO_BASE_URL")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
}

fn hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
            | "host"
    )
}

fn apply_cors(res_headers: &mut HeaderMap, req_headers: &HeaderMap, is_options: bool) {
    let origin = req_headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty());
    let allow_origin = origin.unwrap_or("*");
    if let Ok(v) = HeaderValue::from_str(allow_origin) {
        res_headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, v.clone());
    }
    // Credentials require Allow-Credentials=true when origin is not "*".
    if allow_origin != "*" {
        res_headers.insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        );
    }
    res_headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET,POST,PUT,PATCH,DELETE,HEAD,OPTIONS"),
    );
    // Echo browser preflight list (Sentry, etc.) so fetch() with extra headers does not fail CORS.
    if is_options {
        if let Some(acrh) = req_headers.get(header::ACCESS_CONTROL_REQUEST_HEADERS) {
            res_headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, acrh.clone());
        } else {
            res_headers.insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static(
                    "authorization,content-type,accept,accept-language,x-nango-environment-id,x-omini-internal-key,baggage,sentry-trace",
                ),
            );
        }
    } else {
        res_headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static(
                "authorization,content-type,accept,accept-language,x-nango-environment-id,x-omini-internal-key,baggage,sentry-trace",
            ),
        );
    }
    res_headers.insert(header::ACCESS_CONTROL_MAX_AGE, HeaderValue::from_static("86400"));
}

fn proxy_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .no_proxy()
        // Reverse proxy must not follow OAuth redirects (e.g. `/oauth/connect/*` → 302 to LinkedIn).
        // If we fetch LinkedIn server-side, the response is often empty/blocked and the pop-up never shows login.
        .redirect(reqwest::redirect::Policy::none())
        .build()
}

fn build_upstream_url(base: &str, rest: &str, query: Option<&str>) -> String {
    let path = rest.trim_start_matches('/');
    let mut url = if path.is_empty() {
        base.to_string()
    } else {
        format!("{}/{}", base.trim_end_matches('/'), path)
    };
    if let Some(q) = query {
        if !q.is_empty() {
            url.push('?');
            url.push_str(q);
        }
    }
    url
}

/// Path after `/__omini/nango-hq` for upstream Nango. Axum `nest` usually strips the prefix, but nested
/// `fallback` may still see the full path — handle both.
fn nango_upstream_path_tail(uri: &Uri) -> String {
    let path = uri.path();
    const PREFIX: &str = "/__omini/nango-hq";
    if path.starts_with(PREFIX) {
        let tail = path[PREFIX.len()..].trim_start_matches('/');
        return tail.to_string();
    }
    path.trim_start_matches('/').to_string()
}

fn is_conditional_cache_header(name: &HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "if-none-match"
            | "if-modified-since"
            | "if-unmodified-since"
            | "if-match"
            | "if-range"
    )
}

fn forward_request_headers(from: &HeaderMap) -> HeaderMap {
    let mut out = HeaderMap::new();
    for (k, v) in from.iter() {
        if hop_by_hop(k) {
            continue;
        }
        if is_conditional_cache_header(k) {
            continue;
        }
        if k == header::CONTENT_LENGTH {
            continue;
        }
        out.append(k.clone(), v.clone());
    }
    out
}

/// Nango `GET /connect/session` expects `Authorization: Bearer <connect_session_jwt>`.
/// Some clients only pass the token in the URL (`?session_token=` from the hosted Connect link);
/// synthesize the header so the upstream check in `connectSessionAuth` succeeds.
fn upstream_rest_is_connect_session(rest: &str) -> bool {
    let r = rest.trim().trim_start_matches('/');
    r == "connect/session" || r.starts_with("connect/session/")
}

fn connect_session_token_from_url_query(query: Option<&str>) -> Option<String> {
    let q = query?.trim();
    if q.is_empty() {
        return None;
    }
    for pair in q.split('&') {
        let (raw_k, raw_v) = pair.split_once('=')?;
        let k = urlencoding::decode(raw_k).ok()?.into_owned();
        if k != "session_token" && k != "connect_session_token" {
            continue;
        }
        let v = urlencoding::decode(raw_v).ok()?.into_owned();
        let v = v.trim();
        if v.is_empty() {
            continue;
        }
        return Some(v.to_string());
    }
    None
}

fn augment_nango_connect_session_auth(rest: &str, query: Option<&str>, headers: &mut HeaderMap) {
    if !upstream_rest_is_connect_session(rest) {
        return;
    }
    if headers.get(header::AUTHORIZATION).is_some() {
        return;
    }
    let Some(tok) = connect_session_token_from_url_query(query) else {
        return;
    };
    let bearer = format!("Bearer {tok}");
    if let Ok(hv) = HeaderValue::from_str(&bearer) {
        headers.insert(header::AUTHORIZATION, hv);
    }
}

/// Paths the hosted Connect UI calls on whatever host is passed as `apiURL` / `connect_api_base`.
pub(crate) fn is_nango_connect_public_proxy_path(path: &str) -> bool {
    let p = path.split('?').next().unwrap_or(path);
    p == "/connect/session"
        || p == "/connect/telemetry"
        || p == "/integrations"
        || p.starts_with("/integrations/")
        || p.starts_with("/providers/")
}

/// Root routes (`/connect/session`, `/integrations`, …) — same upstream forwarding as
/// [`proxy_nango_hq_all`].
pub async fn proxy_nango_connect_public(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let path = req.uri().path();
    if !is_nango_connect_public_proxy_path(path) {
        return (StatusCode::NOT_FOUND, "not a nango connect public api path").into_response();
    }
    let rest = path.trim_start_matches('/').to_string();
    proxy_to_nango(rest, req).await
}

/// Nested under `/integrations` — Axum strips the prefix; rebuild `integrations/...` for Nango.
pub async fn proxy_nango_integrations_nested(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let inner = req.uri().path();
    let tail = inner.trim().trim_start_matches('/').trim_end_matches('/');
    let rest = if tail.is_empty() {
        "integrations".to_string()
    } else {
        format!("integrations/{tail}")
    };
    proxy_to_nango(rest, req).await
}

/// Nested under `/providers` — rebuild `providers/...` for Nango.
pub async fn proxy_nango_providers_nested(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let inner = req.uri().path();
    let tail = inner.trim().trim_start_matches('/').trim_end_matches('/');
    let rest = if tail.is_empty() {
        "providers".to_string()
    } else {
        format!("providers/{tail}")
    };
    proxy_to_nango(rest, req).await
}

/// Nested under `/oauth/connect` — Nango `GET /oauth/connect/:providerConfigKey` (OAuth popup target).
pub async fn proxy_nango_oauth_connect_nested(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let inner = req.uri().path();
    let tail = inner.trim().trim_start_matches('/').trim_end_matches('/');
    let rest = if tail.is_empty() {
        "oauth/connect".to_string()
    } else {
        format!("oauth/connect/{tail}")
    };
    proxy_to_nango(rest, req).await
}

/// `GET /oauth/callback` — Nango OAuth return URL (must hit Nango, not the portal SPA).
pub async fn proxy_nango_oauth_callback(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    proxy_to_nango("oauth/callback".to_string(), req).await
}

/// Nested under `/__omini/nango-hq` — forwards to Nango `NANGO_BASE_URL`.
pub async fn proxy_nango_hq_all(State(_state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let rest = nango_upstream_path_tail(req.uri());
    proxy_to_nango(rest, req).await
}

async fn proxy_to_nango(rest: String, req: Request<Body>) -> Response {
    let Some(base) = internal_nango_base() else {
        return (StatusCode::SERVICE_UNAVAILABLE, "NANGO_BASE_URL is not set").into_response();
    };

    let method = req.method().clone();
    let req_headers = req.headers().clone();

    if method == Method::OPTIONS {
        let mut res = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap_or_else(|_| Response::new(Body::empty()));
        apply_cors(res.headers_mut(), &req_headers, true);
        return res;
    }

    let client = match proxy_client() {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "proxy client init failed").into_response();
        }
    };

    let query = req.uri().query().map(|q| q.to_string());
    let upstream = build_upstream_url(&base, &rest, query.as_deref());

    let body_bytes = match req.into_body().collect().await {
        Ok(collected) => {
            let b = collected.to_bytes();
            if b.len() > MAX_PROXY_BODY {
                return (StatusCode::PAYLOAD_TOO_LARGE, "body too large").into_response();
            }
            b
        }
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "invalid body").into_response();
        }
    };

    let mut out_headers = forward_request_headers(&req_headers);
    augment_nango_connect_session_auth(&rest, query.as_deref(), &mut out_headers);

    let mut rb = client.request(method, upstream.as_str()).headers(out_headers);
    if !body_bytes.is_empty() {
        rb = rb.body(body_bytes);
    }

    let Ok(resp) = rb.send().await else {
        tracing::warn!(%upstream, "nango proxy: upstream request send failed");
        return (StatusCode::BAD_GATEWAY, "upstream request failed").into_response();
    };

    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    if status.is_client_error() || status.is_server_error() {
        tracing::warn!(%upstream, %status, "nango proxy: upstream returned error status");
    }
    let mut upstream_headers = HeaderMap::new();
    for (k, v) in resp.headers().iter() {
        if hop_by_hop(k) {
            continue;
        }
        let n = k.as_str();
        if n.eq_ignore_ascii_case("access-control-allow-origin")
            || n.eq_ignore_ascii_case("access-control-allow-methods")
            || n.eq_ignore_ascii_case("access-control-allow-headers")
            || n.eq_ignore_ascii_case("access-control-max-age")
        {
            continue;
        }
        // We buffer the decoded body; do not forward encoding / length from upstream.
        if n.eq_ignore_ascii_case("content-encoding")
            || n.eq_ignore_ascii_case("content-length")
            || n.eq_ignore_ascii_case("transfer-encoding")
        {
            continue;
        }
        upstream_headers.append(k.clone(), v.clone());
    }

    let body_bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(_) => Bytes::new(),
    };

    let mut out = Response::new(Body::from(body_bytes));
    *out.status_mut() = status;
    {
        let h = out.headers_mut();
        for (k, v) in upstream_headers.iter() {
            h.append(k.clone(), v.clone());
        }
        apply_cors(h, &req_headers, false);
    }
    out
}

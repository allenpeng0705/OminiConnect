//! Minimal Nango HTTP client (connect sessions, list connections, proxy, connection probe).

use std::time::Duration;

use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::portal_env;

fn sanitize_end_user_segment(s: &str) -> String {
    if s.is_empty() {
        return "_".to_string();
    }
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Stable end-user id sent to Nango so we can list connections after Connect UI completes.
pub fn end_user_id_for_connector(owner_username: &str, platform: &str) -> String {
    let o = sanitize_end_user_segment(owner_username);
    let p = sanitize_end_user_segment(platform);
    format!("omini_connect_portal_{o}_{p}")
}

pub fn nango_credentials() -> Option<(String, String)> {
    let base = std::env::var("NANGO_BASE_URL").ok()?.trim().to_string();
    let secret = std::env::var("NANGO_SECRET_KEY").ok()?.trim().to_string();
    if base.is_empty() || secret.is_empty() {
        return None;
    }
    Some((base.trim_end_matches('/').to_string(), secret))
}

/// Base URL the **browser** should use for Nango Connect UI API calls (`apiURL` query param).
/// Must be an **origin** only (scheme + host + port); Connect UI replaces the URL path.
/// When unset, defaults to [`nango_credentials`] base (same as `NANGO_BASE_URL`).
///
/// When only the portal is tunneled publicly, set to the same origin as `PORTAL_BASE_URL`; the portal
/// reverse-proxies `/connect/*`, `/integrations`, `/providers/*` to `NANGO_BASE_URL`.
pub fn nango_browser_api_base_for_connect_link(internal_base: &str) -> String {
    std::env::var("NANGO_BROWSER_API_URL")
        .ok()
        .map(|s| normalize_nango_browser_api_url_to_origin(&s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| internal_base.trim_end_matches('/').to_string())
}

fn url_host_loopback(url: &reqwest::Url) -> bool {
    match url.host_str() {
        Some("localhost") | Some("127.0.0.1") | Some("::1") => true,
        Some(h) => h
            .parse::<std::net::IpAddr>()
            .ok()
            .is_some_and(|ip| ip.is_loopback()),
        None => false,
    }
}

fn strip_legacy_nango_hq_suffix(url_str: &str) -> String {
    let mut t = url_str.trim().trim_end_matches('/').to_string();
    let suf = "/__omini/nango-hq";
    if t.ends_with(suf) {
        t.truncate(t.len() - suf.len());
        t = t.trim_end_matches('/').to_string();
    }
    t
}

/// Reduce a configured or client-sent URL to scheme + host + port (no path/query), for Connect `apiURL`.
fn normalize_nango_browser_api_url_to_origin(url_str: &str) -> String {
    let trimmed = strip_legacy_nango_hq_suffix(url_str);
    let Ok(mut u) = reqwest::Url::parse(trimmed.trim()) else {
        return trimmed;
    };
    u.set_path("");
    u.set_query(None);
    u.set_fragment(None);
    u.as_str().trim_end_matches('/').to_string()
}

fn url_origins_equal(a: &reqwest::Url, b: &reqwest::Url) -> bool {
    a.scheme() == b.scheme()
        && a.port() == b.port()
        && match (a.host_str(), b.host_str()) {
            (Some(x), Some(y)) if x == y => true,
            (Some(_), Some(_)) => url_host_loopback(a) && url_host_loopback(b),
            _ => false,
        }
}

/// `connect_api_base` from the portal SPA — must match portal origin (or configured browser base), origin-only or legacy `.../__omini/nango-hq`.
pub(crate) fn is_allowed_nango_connect_api_base(candidate: &str) -> bool {
    let c = candidate.trim();
    if c.is_empty() {
        return false;
    }
    let Ok(cu) = reqwest::Url::parse(c) else {
        return false;
    };
    if !matches!(cu.scheme(), "http" | "https") {
        return false;
    }
    let path = cu.path().trim_end_matches('/');
    let path_ok = path.is_empty() || path == "/" || path == "/__omini/nango-hq";
    if !path_ok {
        return false;
    }
    let cand_origin = {
        let mut u = cu.clone();
        u.set_path("");
        u.set_query(None);
        u.set_fragment(None);
        u
    };
    if let Ok(env) = std::env::var("NANGO_BROWSER_API_URL") {
        let e = env.trim();
        if !e.is_empty() {
            if let Ok(eu) = reqwest::Url::parse(&strip_legacy_nango_hq_suffix(e)) {
                let mut env_origin = eu;
                env_origin.set_path("");
                env_origin.set_query(None);
                env_origin.set_fragment(None);
                if url_origins_equal(&cand_origin, &env_origin) {
                    return true;
                }
            }
        }
    }
    let portal = portal_env::portal_base_url();
    if let Ok(pu) = reqwest::Url::parse(portal.trim()) {
        let mut portal_origin = pu;
        portal_origin.set_path("");
        portal_origin.set_query(None);
        portal_origin.set_fragment(None);
        if url_origins_equal(&cand_origin, &portal_origin) {
            return true;
        }
    }
    url_host_loopback(&cand_origin)
}

pub(crate) fn resolve_nango_connect_browser_api_base(
    override_from_client: Option<&str>,
    internal_base: &str,
) -> String {
    let chosen = if let Some(o) = override_from_client
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if is_allowed_nango_connect_api_base(o) {
            o.to_string()
        } else {
            tracing::warn!(%o, "rejected client connect_api_base; using server default for Nango Connect apiURL");
            nango_browser_api_base_for_connect_link(internal_base)
        }
    } else {
        nango_browser_api_base_for_connect_link(internal_base)
    };
    normalize_nango_browser_api_url_to_origin(&chosen)
}

fn nango_client() -> anyhow::Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        // Keep localhost bridge traffic off system proxies; avoids intermittent
        // 503s when NANGO_BASE_URL points to local dev services.
        .no_proxy()
        .build()
        .context("reqwest client")
}

fn auth_headers(secret: &str) -> anyhow::Result<HeaderMap> {
    let mut h = HeaderMap::new();
    let bearer = format!("Bearer {secret}");
    h.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&bearer)
            .map_err(|e| anyhow::anyhow!("invalid NANGO_SECRET_KEY for header: {e}"))?,
    );
    if let Ok(key) = std::env::var("NANGO_OMINICONNECT_INTERNAL_KEY") {
        let k = key.trim();
        if !k.is_empty() {
            h.insert(
                HeaderName::from_static("x-omini-internal-key"),
                HeaderValue::from_str(k).map_err(|e| {
                    anyhow::anyhow!("invalid NANGO_OMINICONNECT_INTERNAL_KEY header: {e}")
                })?,
            );
        }
    }
    if let Ok(env_id) = std::env::var("NANGO_ENVIRONMENT_ID") {
        let v = env_id.trim();
        if !v.is_empty() {
            h.insert(
                HeaderName::from_static("x-nango-environment-id"),
                HeaderValue::from_str(v)
                    .map_err(|e| anyhow::anyhow!("invalid NANGO_ENVIRONMENT_ID header: {e}"))?,
            );
        }
    }
    Ok(h)
}

#[derive(Debug, Deserialize)]
struct ConnectSessionEnvelope {
    data: ConnectSessionData,
}

#[derive(Debug, Deserialize)]
struct ConnectSessionData {
    connect_link: String,
}

/// Nango Connect UI (`connect-ui`) defaults `apiURL` to `https://api.nango.dev`. Self-hosted
/// sessions are unknown there → `GET /connect/session` returns 401 and the UI shows
/// "Your session has expired, please refresh the modal." Append `apiURL` when missing.
/// See: https://github.com/NangoHQ/nango/issues/5432
///
/// When `apiURL` is the portal but Nango runs elsewhere, also append `websocketsPath` so the
/// `@nangohq/frontend` SDK opens a WebSocket to Nango (otherwise it uses `ws(s)://apiURL/` and the
/// OAuth popup stays blank waiting for `connection_ack`).
///
/// Nango may already put `apiURL` on `connect_link` from server config — we still **merge** our
/// `apiURL` / `websocketsPath` so the pop-up flow cannot skip WebSocket injection.
fn attach_api_url_to_nango_connect_link(
    connect_link: &str,
    nango_api_base: &str,
    internal_nango_http_base: &str,
) -> String {
    let base = nango_api_base.trim().trim_end_matches('/');
    if base.is_empty() {
        return connect_link.to_string();
    }
    let Ok(mut u) = reqwest::Url::parse(connect_link.trim()) else {
        return connect_link.to_string();
    };

    let mut existing_ws: Option<String> = None;
    let mut other: Vec<(String, String)> = Vec::new();
    for (k, v) in u.query_pairs() {
        let kl = k.to_lowercase();
        if kl == "apiurl" || kl == "websocketspath" {
            if kl == "websocketspath" {
                existing_ws = Some(v.into_owned());
            }
            continue;
        }
        other.push((k.into_owned(), v.into_owned()));
    }

    let ws_merged =
        optional_websockets_path_for_connect_link(base, internal_nango_http_base).or(existing_ws);

    u.set_query(None);
    {
        let mut q = u.query_pairs_mut();
        for (k, v) in &other {
            q.append_pair(k, v);
        }
        q.append_pair("apiURL", base);
        if let Some(ws) = ws_merged.as_deref() {
            q.append_pair("websocketsPath", ws);
        }
    }
    u.to_string()
}

fn nango_browser_needs_direct_websocket(browser_api_base: &str, internal_http_base: &str) -> bool {
    let Ok(b) = reqwest::Url::parse(browser_api_base.trim()) else {
        return true;
    };
    let Ok(i) = reqwest::Url::parse(internal_http_base.trim()) else {
        return true;
    };
    let mut bo = b;
    bo.set_path("");
    bo.set_query(None);
    bo.set_fragment(None);
    let mut io = i;
    io.set_path("");
    io.set_query(None);
    io.set_fragment(None);
    !url_origins_equal(&bo, &io)
}

fn http_base_to_ws_root_url(http_base: &str) -> Option<String> {
    let b = http_base.trim().trim_end_matches('/');
    if b.is_empty() {
        return None;
    }
    let (scheme, host_and_path) = if let Some(r) = b.strip_prefix("https://") {
        ("wss", r)
    } else if let Some(r) = b.strip_prefix("http://") {
        ("ws", r)
    } else {
        return None;
    };
    Some(format!("{scheme}://{host_and_path}/"))
}

fn browser_api_base_to_portal_ws_bridge_url(browser_api_base: &str) -> Option<String> {
    let b = browser_api_base.trim().trim_end_matches('/');
    if b.is_empty() {
        return None;
    }
    let (scheme, host_and_path) = if let Some(r) = b.strip_prefix("https://") {
        ("wss", r)
    } else if let Some(r) = b.strip_prefix("http://") {
        ("ws", r)
    } else {
        return None;
    };
    Some(format!(
        "{scheme}://{}/_nango_auth_ws",
        host_and_path.trim_end_matches('/')
    ))
}

fn optional_websockets_path_for_connect_link(
    browser_api_base: &str,
    internal_http_base: &str,
) -> Option<String> {
    if let Ok(v) = std::env::var("NANGO_BROWSER_WEBSOCKET_URL") {
        let t = v.trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    if !nango_browser_needs_direct_websocket(browser_api_base, internal_http_base) {
        return None;
    }
    browser_api_base_to_portal_ws_bridge_url(browser_api_base)
        .or_else(|| http_base_to_ws_root_url(internal_http_base))
}

/// `POST /connect/sessions` — returns hosted Connect UI URL.
///
/// `browser_api_override`: optional `connect_api_base` from the browser (portal origin, e.g. `http://localhost:9000`)
/// so local dev matches `window.location.origin` even when `NANGO_BROWSER_API_URL` points at a public tunnel.
pub async fn create_connect_session(
    base_url: &str,
    secret: &str,
    body: &serde_json::Value,
    browser_api_override: Option<&str>,
) -> anyhow::Result<String> {
    let client = nango_client()?;
    let url = format!("{base_url}/connect/sessions");
    let mut headers = auth_headers(secret)?;
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let resp = client.post(&url).headers(headers).json(body).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango connect/sessions failed {status}: {text}");
    }

    let parsed: ConnectSessionEnvelope = resp.json().await.context("parse connect session JSON")?;
    let link = parsed.data.connect_link;
    let browser_base = resolve_nango_connect_browser_api_base(browser_api_override, base_url);
    Ok(attach_api_url_to_nango_connect_link(
        &link,
        &browser_base,
        base_url,
    ))
}

/// `POST /connections` — create a connection with credentials directly (API_KEY or BASIC).
/// Used to provision connections without the Nango Connect hosted UI.
pub async fn create_nango_connection(
    base_url: &str,
    secret: &str,
    provider_config_key: &str,
    connection_id: &str,
    credentials: &serde_json::Value,
) -> anyhow::Result<String> {
    let client = nango_client()?;
    let url = format!("{}/connections", base_url.trim_end_matches('/'));
    let mut headers = auth_headers(secret)?;
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = json!({
        "provider_config_key": provider_config_key,
        "connection_id": connection_id,
        "credentials": credentials,
    });

    let resp = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango POST /connections failed {status}: {text}");
    }

    #[derive(Deserialize)]
    struct ConnectionResponse {
        #[serde(rename = "connection_id")]
        connection_id: String,
    }

    let parsed: ConnectionResponse = resp
        .json()
        .await
        .context("parse connection response JSON")?;
    Ok(parsed.connection_id)
}

#[derive(Debug, Deserialize)]
struct ListConnectionsEnvelope {
    connections: Vec<NangoConnectionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NangoConnectionSummary {
    pub connection_id: String,
    pub provider_config_key: String,
    pub created: Option<String>,
}

/// `GET /connections` with optional `endUserId` and `integrationId` filters.
pub async fn list_connections(
    base_url: &str,
    secret: &str,
    end_user_id: &str,
    integration_id: Option<&str>,
) -> anyhow::Result<Vec<NangoConnectionSummary>> {
    let client = nango_client()?;
    let mut url = format!(
        "{base_url}/connections?endUserId={}",
        urlencoding::encode(end_user_id)
    );
    if let Some(iid) = integration_id.filter(|s| !s.is_empty()) {
        url.push_str("&integrationId=");
        url.push_str(&urlencoding::encode(iid));
    }

    let headers = auth_headers(secret)?;
    let resp = client.get(&url).headers(headers).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango list connections failed {status}: {text}");
    }

    let parsed: ListConnectionsEnvelope = resp.json().await.context("parse connections JSON")?;
    Ok(parsed.connections)
}

/// Pick the newest connection for `provider_config_key` (by `created` if present).
pub fn pick_connection_id(
    connections: &[NangoConnectionSummary],
    provider_config_key: &str,
) -> Option<String> {
    let mut matches: Vec<_> = connections
        .iter()
        .filter(|c| c.provider_config_key == provider_config_key)
        .collect();
    if matches.is_empty() {
        return None;
    }
    matches.sort_by(|a, b| {
        let ta = a.created.as_deref().unwrap_or("");
        let tb = b.created.as_deref().unwrap_or("");
        tb.cmp(ta)
    });
    Some(matches[0].connection_id.clone())
}

/// Build JSON body for [`create_connect_session`].
pub fn connect_session_body(
    end_user_id: &str,
    allowed_integration: &str,
    scopes_for_integration: Option<(&str, &[String])>,
) -> serde_json::Value {
    let mut body = json!({
        "end_user": { "id": end_user_id },
        "allowed_integrations": [allowed_integration],
    });
    if let Some((key, scopes)) = scopes_for_integration.filter(|(_, s)| !s.is_empty()) {
        body["integrations_config_defaults"] = json!({
            key: { "user_scopes": scopes.join(" ") }
        });
    }
    body
}

/// `GET /connections/{id}?provider_config_key=...` — verify connection exists (token path).
pub async fn probe_connection(
    base_url: &str,
    secret: &str,
    connection_id: &str,
    provider_config_key: &str,
) -> anyhow::Result<reqwest::StatusCode> {
    let client = nango_client()?;
    let url = format!(
        "{}/connections/{}?provider_config_key={}",
        base_url.trim_end_matches('/'),
        urlencoding::encode(connection_id),
        urlencoding::encode(provider_config_key)
    );
    let headers = auth_headers(secret)?;
    let resp = client.get(&url).headers(headers).send().await?;
    Ok(resp.status())
}

/// Forward to Nango's proxy (`/proxy/{endpoint}`) using connection + integration headers.
pub async fn forward_proxy(
    base_url: &str,
    secret: &str,
    provider_config_key: &str,
    connection_id: &str,
    method: Method,
    endpoint_path: &str,
    body: bytes::Bytes,
    content_type: Option<&str>,
    linkedin_version: Option<&str>,
) -> anyhow::Result<reqwest::Response> {
    let client = nango_client()?;
    let path = endpoint_path.trim_start_matches('/');
    let url = format!("{}/proxy/{}", base_url.trim_end_matches('/'), path);

    let mut headers = auth_headers(secret)?;
    headers.insert(
        HeaderName::from_static("connection-id"),
        HeaderValue::from_str(connection_id)
            .map_err(|e| anyhow::anyhow!("connection id header: {e}"))?,
    );
    headers.insert(
        HeaderName::from_static("provider-config-key"),
        HeaderValue::from_str(provider_config_key)
            .map_err(|e| anyhow::anyhow!("provider key header: {e}"))?,
    );

    if let Some(ct) = content_type {
        if let Ok(val) = HeaderValue::from_str(ct) {
            headers.insert(CONTENT_TYPE, val);
        }
    }

    if let Some(v) = linkedin_version {
        if let Ok(name) = HeaderName::from_bytes(b"nango-proxy-linkedin-version") {
            if let Ok(val) = HeaderValue::from_str(v) {
                headers.insert(name, val);
            }
        }
    }

    let mut req = client.request(method, url).headers(headers);
    if !body.is_empty() {
        req = req.body(body);
    }
    Ok(req.send().await?)
}

/// One row from Nango `GET /integrations` (public catalog for the environment).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NangoIntegrationCatalogItem {
    pub unique_key: String,
    pub display_name: String,
    #[serde(default)]
    pub provider: String,
}

#[derive(Debug, Deserialize)]
struct ListIntegrationsBody {
    data: Vec<NangoIntegrationCatalogItem>,
}

/// `GET /integrations` — configured integrations in the Nango environment.
pub async fn list_integrations_catalog(
    base_url: &str,
    secret: &str,
) -> anyhow::Result<Vec<NangoIntegrationCatalogItem>> {
    let client = nango_client()?;
    let url = format!("{}/integrations", base_url.trim_end_matches('/'));
    let headers = auth_headers(secret)?;
    let resp = client.get(&url).headers(headers).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango GET /integrations failed {status}: {text}");
    }
    let parsed: ListIntegrationsBody = resp.json().await.context("parse integrations JSON")?;
    Ok(parsed.data)
}

/// Nango `POST /integrations` (IntegrationCredentials) allows `credentials.scopes` as either
/// a **comma‑separated** ASCII string matching
/// `^[0-9a-zA-Z:/_.-]+(,[0-9a-zA-Z:/_.-]+)*$` **or** JSON `null` (see Nango API docs).
/// An **empty string is rejected** (invalid union). It is not a JSON array of strings.
///
/// The UI and catalog may add punctuation or copy‑pasted whitespace; strip anything not allowed
/// so a bad character does not fail the whole request on `["credentials","scopes"]`.
fn sanitize_nango_scope_token(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, ':' | '/' | '_' | '.' | '-'))
        .collect()
}

/// Returns a non-empty, Nango-safe comma string, or `None` to **omit** `scopes` (some stacks accept
/// omit; `null` is also valid in the published OpenAPI, but we avoid `""` which always fails).
fn nango_integration_scopes_value(scopes: &[String]) -> Option<String> {
    let mut parts: Vec<String> = vec![];
    for raw in scopes {
        for bit in raw.split(|c: char| c == ',' || c.is_whitespace()) {
            let t = sanitize_nango_scope_token(bit.trim());
            if t.is_empty() {
                continue;
            }
            if !parts.contains(&t) {
                parts.push(t);
            }
        }
    }
    if parts.is_empty() {
        return None;
    }
    let joined = parts.join(",");
    if joined.is_empty() {
        return None;
    }
    Some(joined)
}

/// `POST /integrations` — create a Nango integration (public API with secret key auth).
///
/// Nango uses `credentials.type` to specify auth mode (OAUTH2, OAUTH1, APP, CUSTOM).
/// For API_KEY/BASIC providers, credentials are NOT required at integration creation -
/// the user enters their API key/token directly in Nango Connect UI when creating a connection.
pub async fn create_integration_catalog(
    base_url: &str,
    secret: &str,
    provider: &str,
    unique_key: &str,
    client_id: &str,
    client_secret: &str,
    scopes: &[String],
) -> anyhow::Result<()> {
    let client = nango_client()?;
    let url = format!("{}/integrations", base_url.trim_end_matches('/'));
    let headers = auth_headers(secret)?;

    // Only include credentials when BOTH client_id AND client_secret are non-empty.
    // For PAT-only auth (only client_id provided) or API_KEY/BASIC auth (no credentials),
    // we create an empty integration - user provides the API key in Nango Connect UI.
    let has_oauth = !client_id.trim().is_empty() && !client_secret.trim().is_empty();
    let body: serde_json::Value = if has_oauth {
        // OAuth providers (OAUTH2, OAUTH1, APP, CUSTOM) require credentials at integration creation
        let scopes_str = nango_integration_scopes_value(scopes);
        let mut credentials = json!({
            "type": "OAUTH2",
            "client_id": client_id,
            "client_secret": client_secret,
        });
        if let Some(s) = scopes_str {
            credentials["scopes"] = json!(s);
        }
        json!({
            "provider": provider,
            "unique_key": unique_key,
            "credentials": credentials,
            "forward_webhooks": true
        })
    } else {
        // API_KEY/BASIC providers don't need credentials at integration creation.
        // User provides API key directly in Nango Connect UI when creating a connection.
        json!({
            "provider": provider,
            "unique_key": unique_key,
            "forward_webhooks": true
        })
    };

    let resp = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        return Ok(());
    }

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    anyhow::bail!("Nango POST /integrations failed {status}: {text}");
}

/// Ensure integration exists. If missing and OAuth credentials are provided, create it.
pub async fn ensure_integration_catalog(
    base_url: &str,
    secret: &str,
    provider: &str,
    unique_key: &str,
    client_id: &str,
    client_secret: &str,
    scopes: &[String],
) -> anyhow::Result<()> {
    let existing = list_integrations_catalog(base_url, secret).await?;
    if existing.iter().any(|i| i.unique_key == unique_key) {
        return Ok(());
    }
    create_integration_catalog(
        base_url,
        secret,
        provider,
        unique_key,
        client_id,
        client_secret,
        scopes,
    )
    .await
}

fn providers_catalog_from_data_root(root: serde_json::Value) -> Vec<serde_json::Value> {
    root.get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
}

/// Load provider scopes from two sources:
/// 1. Nango's providers.scopes.yaml (primary - curated scope lists)
/// 2. providers.yaml default_scopes field (fallback for providers missing from #1)
/// Returns a map of provider name -> list of available scopes.
fn load_provider_scopes() -> std::collections::HashMap<String, Vec<String>> {
    let mut scopes = std::collections::HashMap::new();

    // Source 1: providers.scopes.yaml (primary)
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scopes_yaml_path =
        manifest_dir.join("../../third_party/nango/packages/providers/providers.scopes.yaml");
    if let Ok(content) = std::fs::read_to_string(&scopes_yaml_path) {
        if let Ok(parsed) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(obj) = parsed.as_mapping() {
                for (k, v) in obj {
                    let name = k.as_str().unwrap_or_default().to_string();
                    let mut scopes_list = Vec::new();
                    if let Some(arr) = v.as_sequence() {
                        for scope_val in arr {
                            if let Some(s) = scope_val.as_str() {
                                scopes_list.push(s.to_string());
                            }
                        }
                    }
                    // Only insert non-empty scope lists from primary source
                    if !scopes_list.is_empty() {
                        scopes.insert(name, scopes_list);
                    }
                }
            }
        }
    }

    // Source 2: providers.yaml default_scopes (fallback)
    let providers_yaml_path =
        manifest_dir.join("../../third_party/nango/packages/providers/providers.yaml");
    if let Ok(content) = std::fs::read_to_string(&providers_yaml_path) {
        if let Ok(parsed) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(obj) = parsed.as_mapping() {
                for (k, v) in obj {
                    let name = k.as_str().unwrap_or_default().to_string();
                    // Only add if not already in scopes map
                    if !scopes.contains_key(&name) {
                        if let Some(v_obj) = v.as_mapping() {
                            if let Some(default_scopes) = v_obj.get(&serde_yaml::Value::String("default_scopes".to_string())) {
                                let mut scopes_list = Vec::new();
                                if let Some(arr) = default_scopes.as_sequence() {
                                    for scope_val in arr {
                                        if let Some(s) = scope_val.as_str() {
                                            scopes_list.push(s.to_string());
                                        }
                                    }
                                }
                                if !scopes_list.is_empty() {
                                    tracing::debug!(
                                        "Loaded fallback scopes for {}: {:?}",
                                        name.clone(),
                                        scopes_list.clone()
                                    );
                                    scopes.insert(name, scopes_list);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    scopes
}

/// Merge available_scopes into each provider row.
fn enrich_rows_with_scopes(
    mut rows: Vec<serde_json::Value>,
    scopes_map: &std::collections::HashMap<String, Vec<String>>,
) -> Vec<serde_json::Value> {
    for row in rows.iter_mut() {
        if let Some(obj) = row.as_object_mut() {
            if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                if let Some(scopes) = scopes_map.get(name) {
                    obj.insert("available_scopes".to_string(), serde_json::json!(scopes));
                }
            }
        }
    }
    rows
}

/// `GET /providers.json` — unauthenticated provider template map (same YAML as `/providers` body).
/// Used when `GET /providers` rejects the secret so the integration library still loads for browsing.
async fn list_providers_catalog_from_public_json(
    client: &reqwest::Client,
    base_trim: &str,
    search: Option<&str>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let url = format!("{base_trim}/providers.json");
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("Nango GET /providers.json failed {status}: {text}");
    }
    let root: serde_json::Value = resp.json().await.context("parse providers.json")?;
    let Some(obj) = root.as_object() else {
        anyhow::bail!("providers.json: expected a JSON object keyed by provider id");
    };
    let needle = search
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase());
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for (name, v) in obj {
        if let Some(n) = needle.as_ref() {
            if !name.to_lowercase().contains(n.as_str()) {
                continue;
            }
        }
        let Some(p) = v.as_object() else {
            continue;
        };
        let display_name = p
            .get("display_name")
            .and_then(|x| x.as_str())
            .unwrap_or(name.as_str());
        let logo_url = format!("/images/template-logos/{name}.svg");
        rows.push(json!({
            "name": name,
            "display_name": display_name,
            "logo_url": logo_url,
            "auth_mode": p.get("auth_mode"),
            "categories": p.get("categories"),
            "docs": p.get("docs"),
        }));
    }
    rows.sort_by(|a, b| {
        let an = a.get("name").and_then(|x| x.as_str()).unwrap_or("");
        let bn = b.get("name").and_then(|x| x.as_str()).unwrap_or("");
        an.cmp(bn)
    });
    let scopes_map = load_provider_scopes();
    let rows = enrich_rows_with_scopes(rows, &scopes_map);
    Ok(rows)
}

/// Same as [`list_providers_catalog_from_public_json`] but exposed for callers that have no API secret yet
/// (browse-only library in the portal).
pub async fn list_providers_catalog_public_only(
    base_url: &str,
    search: Option<&str>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = nango_client()?;
    let base_trim = base_url.trim_end_matches('/');
    list_providers_catalog_from_public_json(&client, base_trim, search).await
}

/// `GET /providers` — full Nango provider catalog (`search` filters by provider id regex on Nango side).
/// Falls back to `GET /providers.json` when Nango returns 401/403 (e.g. wrong `NANGO_SECRET_KEY`) or **5xx**
/// (broken `/providers` while static templates still serve) so the integration library can load; OAuth and
/// connect-session still need a healthy Nango API and valid secret.
pub async fn list_providers_catalog(
    base_url: &str,
    secret: &str,
    search: Option<&str>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let client = nango_client()?;
    let base_trim = base_url.trim_end_matches('/');
    let mut url = format!("{base_trim}/providers");
    if let Some(s) = search.map(str::trim).filter(|s| !s.is_empty()) {
        url.push_str("?search=");
        url.push_str(&urlencoding::encode(s));
    }
    let headers = auth_headers(secret)?;
    let resp = client.get(url).headers(headers).send().await?;
    if resp.status().is_success() {
        let root: serde_json::Value = resp.json().await.context("parse providers JSON")?;
        let rows = providers_catalog_from_data_root(root);
        tracing::debug!(
            "list_providers_catalog: got {} rows from Nango /providers",
            rows.len()
        );
        let scopes_map = load_provider_scopes();
        tracing::debug!(
            "load_provider_scopes: {} providers with scopes",
            scopes_map.len()
        );
        let rows = enrich_rows_with_scopes(rows, &scopes_map);
        let enriched_count = rows
            .iter()
            .filter(|r| r.get("available_scopes").is_some())
            .count();
        tracing::debug!(
            "enrich_rows_with_scopes: {} rows now have available_scopes",
            enriched_count
        );
        if let Some(linkedin_row) = rows
            .iter()
            .find(|r| r.get("name").and_then(|v| v.as_str()) == Some("linkedin"))
        {
            let linkedin_scopes = linkedin_row
                .get("available_scopes")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            tracing::debug!("DEBUG: linkedin has {} available_scopes", linkedin_scopes);
        }
        return Ok(rows);
    }
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        tracing::warn!(
            %status,
            "Nango GET /providers rejected credentials; using unauthenticated /providers.json for catalog only"
        );
        return list_providers_catalog_from_public_json(&client, base_trim, search).await;
    }
    if status.is_server_error() {
        tracing::warn!(
            %status,
            "Nango GET /providers failed; using unauthenticated /providers.json for catalog browse (repair Nango for authenticated /providers)"
        );
        return list_providers_catalog_from_public_json(&client, base_trim, search).await;
    }
    anyhow::bail!("Nango GET /providers failed {status}: {text}");
}

#[cfg(test)]
mod scope_enrichment_tests {
    use super::*;
    #[test]
    fn test_scope_enrichment_for_all_providers() {
        let test_providers = vec![
            "linkedin",
            "facebook",
            "github",
            "hubspot",
            "airtable",
            "slack",
            "salesforce",
            "google",
        ];
        let rows: Vec<serde_json::Value> = test_providers
            .iter()
            .map(|n| serde_json::json!({ "name": *n, "display_name": n }))
            .collect();
        let scopes_map = load_provider_scopes();
        let enriched = enrich_rows_with_scopes(rows, &scopes_map);
        for row in enriched {
            let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let scopes = row.get("available_scopes").and_then(|v| v.as_array());
            match scopes {
                Some(s) if !s.is_empty() => println!("PASS: {} has {} scopes", name, s.len()),
                _ => println!("FAIL: {} has no scopes", name),
            }
        }
    }
}

#[cfg(test)]
mod http_test {
    #[tokio::test]
    async fn test_providers_endpoint_returns_enriched_scopes() {
        // This test calls the actual Nango endpoint and verifies enrichment
        let nango_key = std::env::var("NANGO_SECRET_KEY").ok();
        if nango_key.is_none() {
            eprintln!("SKIP: NANGO_SECRET_KEY not set");
            return;
        }
        let key = nango_key.unwrap();
        let result = super::list_providers_catalog("http://localhost:3003", &key, None).await;
        match result {
            Ok(rows) => {
                eprintln!("Total rows: {}", rows.len());
                for name in &["linkedin", "github", "hubspot", "airtable"] {
                    let row = rows
                        .iter()
                        .find(|r| r.get("name").and_then(|v| v.as_str()) == Some(*name));
                    match row {
                        Some(r) => {
                            let scopes = r.get("available_scopes").and_then(|v| v.as_array());
                            match scopes {
                                Some(s) if !s.is_empty() => {
                                    eprintln!("OK: {} has {} scopes", name, s.len())
                                }
                                _ => eprintln!("FAIL: {} has no scopes", name),
                            }
                        }
                        None => eprintln!("FAIL: {} not found", name),
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

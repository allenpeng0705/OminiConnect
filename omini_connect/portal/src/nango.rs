//! Minimal Nango HTTP client (connect sessions, list connections, proxy, connection probe).

use std::time::Duration;

use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Stable end-user id sent to Nango so we can list connections after Connect UI completes.
pub fn end_user_id_for_connector(platform: &str) -> String {
    format!("omini_connect_portal_{platform}")
}

pub fn nango_credentials() -> Option<(String, String)> {
    let base = std::env::var("NANGO_BASE_URL").ok()?.trim().to_string();
    let secret = std::env::var("NANGO_SECRET_KEY").ok()?.trim().to_string();
    if base.is_empty() || secret.is_empty() {
        return None;
    }
    Some((base.trim_end_matches('/').to_string(), secret))
}

fn nango_client() -> anyhow::Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .context("reqwest client")
}

fn auth_headers(secret: &str) -> anyhow::Result<HeaderMap> {
    let mut h = HeaderMap::new();
    let bearer = format!("Bearer {secret}");
    h.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&bearer).map_err(|e| anyhow::anyhow!("invalid NANGO_SECRET_KEY for header: {e}"))?,
    );
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

/// `POST /connect/sessions` — returns hosted Connect UI URL.
pub async fn create_connect_session(
    base_url: &str,
    secret: &str,
    body: &serde_json::Value,
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
    Ok(parsed.data.connect_link)
}

#[derive(Debug, Deserialize)]
struct ListConnectionsEnvelope {
    connections: Vec<NangoConnectionSummary>,
}

#[derive(Debug, Deserialize)]
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
    let mut url = format!("{base_url}/connections?endUserId={}", urlencoding::encode(end_user_id));
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
pub fn pick_connection_id(connections: &[NangoConnectionSummary], provider_config_key: &str) -> Option<String> {
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
pub async fn probe_connection(base_url: &str, secret: &str, connection_id: &str, provider_config_key: &str) -> anyhow::Result<reqwest::StatusCode> {
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
        HeaderValue::from_str(connection_id).map_err(|e| anyhow::anyhow!("connection id header: {e}"))?,
    );
    headers.insert(
        HeaderName::from_static("provider-config-key"),
        HeaderValue::from_str(provider_config_key).map_err(|e| anyhow::anyhow!("provider key header: {e}"))?,
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
pub async fn list_integrations_catalog(base_url: &str, secret: &str) -> anyhow::Result<Vec<NangoIntegrationCatalogItem>> {
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

fn providers_catalog_from_data_root(root: serde_json::Value) -> Vec<serde_json::Value> {
    root.get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
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
    let needle = search.map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_lowercase());
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
        let logo_url = format!("{base_trim}/images/template-logos/{name}.svg");
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
    Ok(rows)
}

/// Same as [`list_providers_catalog_from_public_json`] but exposed for callers that have no API secret yet
/// (browse-only library in the portal).
pub async fn list_providers_catalog_public_only(base_url: &str, search: Option<&str>) -> anyhow::Result<Vec<serde_json::Value>> {
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
        return Ok(providers_catalog_from_data_root(root));
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

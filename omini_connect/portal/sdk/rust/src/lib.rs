//! OminiConnect Rust SDK.
//!
//! Example:
//!
//! ```ignore
//! use omini_connect_sdk::Client;
//!
//! let client = Client::new("sk-xxxxx", None);
//!
//! // List connected platforms
//! let connectors = client.connectors().list().await?;
//!
//! // Call GitHub API directly — Maton style
//! let user = client.call("github", "GET", "/user", None, None).await?;
//!
//! // Create a named API key
//! let key = client.api_keys().create("pr-reviewer-agent").await?;
//! println!("{}", key.key); // shown once — store securely
//! ```

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

// ─── Client ─────────────────────────────────────────────────────────────────

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl Client {
    /// Create a new OminiConnect client.
    pub fn new(api_key: impl Into<String>, base_url: Option<&'static str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url
                .unwrap_or("http://localhost:9000")
                .trim_end_matches('/')
                .to_string(),
            api_key: api_key.into(),
        }
    }

    /// Create a ToolsManager.
    pub fn tools(&self) -> ToolsManager<'_> {
        ToolsManager { client: self }
    }

    /// Create a ConnectorsManager.
    pub fn connectors(&self) -> ConnectorsManager<'_> {
        ConnectorsManager { client: self }
    }

    /// Create an ApiKeysManager.
    pub fn api_keys(&self) -> ApiKeysManager<'_> {
        ApiKeysManager { client: self }
    }

    /// Call a connected platform's API directly — Maton style.
    ///
    /// `params` are query string parameters. `body` is the request body for POST/PUT/PATCH.
    pub async fn call(
        &self,
        platform: &str,
        method: &str,
        path: &str,
        params: Option<HashMap<String, String>>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        let url = format!("{}/api/call/{}", self.base_url, platform);

        let mut payload = HashMap::new();
        payload.insert("method".to_string(), serde_json::Value::String(method.to_uppercase()));
        payload.insert("path".to_string(), serde_json::Value::String(path.to_string()));
        if let Some(p) = params {
            payload.insert("params".to_string(), serde_json::to_value(p).unwrap());
        }
        if let Some(b) = body {
            payload.insert("body".to_string(), b);
        }

        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE.as_str(), "application/json")
            .json(&payload)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Auth);
        }
        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimited);
        }

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Upstream { status, body });
        }

        let value: serde_json::Value = resp.json().await?;
        Ok(value)
    }
}

// ─── Tools ──────────────────────────────────────────────────────────────────

pub struct ToolsManager<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolSummary {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub method: String,
    pub endpoint: String,
    pub scopes: Vec<String>,
    pub scope_satisfied: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Toolkit {
    pub slug: String,
    pub name: String,
    pub logo: Option<String>,
    pub provider: String,
    pub tools: Vec<ToolSummary>,
}

#[derive(Debug, Deserialize)]
pub struct ToolkitsResponse {
    pub toolkits: Vec<Toolkit>,
}

#[derive(Debug, Deserialize)]
pub struct ToolsSearchResponse {
    pub tools: Vec<ToolSummary>,
    pub query: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolExecuteResult {
    pub ok: bool,
    pub body: Option<serde_json::Value>,
    pub error: Option<String>,
    pub call_id: Option<String>,
    pub status: Option<String>,
    pub duration_ms: Option<i64>,
}

impl ToolsManager<'_> {
    /// List available tools, optionally filtered by platform.
    pub async fn list(&self, platform: Option<&str>) -> Result<ToolkitsResponse, Error> {
        let mut url = format!("{}/api/tools", self.client.base_url);
        if let Some(p) = platform {
            url += &format!("?platform={}", urlencoding::encode(p));
        }
        let resp = self.client.http.get(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Search tools by query string.
    pub async fn search(
        &self,
        q: &str,
        platform: Option<&str>,
    ) -> Result<ToolsSearchResponse, Error> {
        let mut url = format!("{}/api/tools/search?q={}", self.client.base_url, urlencoding::encode(q));
        if let Some(p) = platform {
            url += &format!("&platform={}", urlencoding::encode(p));
        }
        let resp = self.client.http.get(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Execute a tool by slug.
    ///
    /// Set `callback_url` for async execution (returns immediately).
    pub async fn execute(
        &self,
        tool_slug: &str,
        arguments: Option<HashMap<String, serde_json::Value>>,
        callback_url: Option<&str>,
    ) -> Result<ToolExecuteResult, Error> {
        let url = format!("{}/api/tools/execute", self.client.base_url);
        let mut payload = serde_json::json!({
            "tool_slug": tool_slug,
            "arguments": arguments.unwrap_or_default(),
        });
        if let Some(cb) = callback_url {
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("callback_url".to_string(), serde_json::Value::String(cb.to_string()));
            }
        }
        let resp = self.client.http.post(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .header(CONTENT_TYPE.as_str(), "application/json")
            .json(&payload)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(resp: reqwest::Response) -> Result<T, Error> {
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Auth);
        }
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Upstream { status, body });
        }
        let value = resp.json().await?;
        Ok(value)
    }
}

// ─── Connectors ────────────────────────────────────────────────────────────────

pub struct ConnectorsManager<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct Connector {
    pub platform: String,
    pub enabled: bool,
    pub scopes: Vec<String>,
    pub created_at: String,
}

impl ConnectorsManager<'_> {
    /// List all connected platforms.
    pub async fn list(&self) -> Result<Vec<Connector>, Error> {
        let url = format!("{}/api/connectors", self.client.base_url);
        let resp = self.client.http.get(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Auth);
        }
        let value: Vec<Connector> = resp.json().await?;
        Ok(value)
    }

    /// Delete a connected platform.
    pub async fn delete(&self, platform: &str) -> Result<(), Error> {
        let url = format!("{}/api/connectors/{}", self.client.base_url, platform);
        let resp = self.client.http.delete(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Upstream { status, body });
        }
        Ok(())
    }
}

// ─── API Keys ────────────────────────────────────────────────────────────────

pub struct ApiKeysManager<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyCreated {
    /// The raw key — shown ONLY once. Store securely.
    pub key: String,
    pub label: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeySummary {
    pub key_hash: String,
    pub label: String,
    pub created_at: String,
}

impl ApiKeysManager<'_> {
    /// Create a new named API key. The raw key is returned ONLY here.
    pub async fn create(&self, label: &str) -> Result<ApiKeyCreated, Error> {
        let url = format!("{}/auth/apikey", self.client.base_url);
        let resp = self.client.http.post(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .header(CONTENT_TYPE.as_str(), "application/json")
            .json(&serde_json::json!({ "label": label }))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Auth);
        }
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Upstream { status, body });
        }
        let value: ApiKeyCreated = resp.json().await?;
        Ok(value)
    }

    /// List all API keys (raw key is never returned).
    pub async fn list(&self) -> Result<Vec<ApiKeySummary>, Error> {
        let url = format!("{}/auth/apikey", self.client.base_url);
        let resp = self.client.http.get(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(Error::Auth);
        }
        let value: Vec<ApiKeySummary> = resp.json().await?;
        Ok(value)
    }

    /// Revoke an API key.
    pub async fn delete(&self, key_hash: &str) -> Result<(), Error> {
        let url = format!("{}/auth/apikey/{}", self.client.base_url, key_hash);
        let resp = self.client.http.delete(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Upstream { status, body });
        }
        Ok(())
    }
}

// ─── Errors ────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication failed")]
    Auth,

    #[error("rate limited")]
    RateLimited,

    #[error("upstream error {status}: {body}")]
    Upstream { status: u16, body: String },

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
}

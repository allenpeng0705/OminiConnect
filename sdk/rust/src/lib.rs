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
use serde::{Deserialize, Serialize};
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

    /// Create an LlmManager.
    pub fn llm(&self) -> LlmManager<'_> {
        LlmManager { client: self }
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

// ─── LLM ─────────────────────────────────────────────────────────────────────

pub struct LlmManager<'a> {
    client: &'a Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmExecuteResponse {
    pub ok: bool,
    #[serde(default)]
    pub tool: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub arguments: Option<serde_json::Value>,
    #[serde(default)]
    pub explanation: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub candidates: Option<Vec<CandidateTool>>,
    #[serde(default)]
    pub available_tools_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateTool {
    pub tool: String,
    pub name: String,
    pub match_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmToolsResponse {
    pub platforms: HashMap<String, PlatformTools>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformTools {
    pub connected: bool,
    #[serde(default)]
    pub tools: Option<Vec<AvailableTool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTool {
    pub slug: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub example_queries: Vec<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    pub scope_satisfied: String,
}

impl LlmManager<'_> {
    /// Execute an LLM query, optionally constrained to a platform.
    pub async fn execute(&self, query: &str, platform: Option<&str>) -> Result<LlmExecuteResponse, Error> {
        let mut body = serde_json::json!({ "query": query });
        if let Some(p) = platform {
            body["platform"] = serde_json::json!(p);
        }
        let url = format!("{}/api/llm", self.client.base_url);
        let resp = self.client.http.post(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
            .header(CONTENT_TYPE.as_str(), "application/json")
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// List available tools for one or all platforms.
    pub async fn list_available_tools(&self, platform: Option<&str>) -> Result<LlmToolsResponse, Error> {
        let url = match platform {
            Some(p) => format!("{}/api/llm/tools?platform={}", self.client.base_url, urlencoding::encode(p)),
            None => format!("{}/api/llm/tools", self.client.base_url),
        };
        let resp = self.client.http.get(&url)
            .header(AUTHORIZATION.as_str(), format!("Bearer {}", self.client.api_key))
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

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Client::new ─────────────────────────────────────────────────────────

    #[test]
    fn client_new_default_base_url() {
        let client = Client::new("sk-test", None);
        assert_eq!(client.base_url, "http://localhost:9000");
    }

    #[test]
    fn client_new_custom_base_url() {
        let client = Client::new("sk-test", Some("https://api.example.com"));
        assert_eq!(client.base_url, "https://api.example.com");
    }

    #[test]
    fn client_new_strips_trailing_slash() {
        let client = Client::new("sk-test", Some("https://api.example.com///"));
        assert_eq!(client.base_url, "https://api.example.com");
    }

    // ─── ToolsManager::list ───────────────────────────────────────────────────

    #[tokio::test]
    async fn tools_manager_list_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!({
            "toolkits": [
                {
                    "slug": "github",
                    "name": "GitHub",
                    "logo": null,
                    "provider": "github",
                    "tools": [
                        {
                            "slug": "github_list_repos",
                            "name": "List Repositories",
                            "description": "List all repositories for the authenticated user",
                            "method": "GET",
                            "endpoint": "/user/repos",
                            "scopes": ["repo"],
                            "scopeSatisfied": "yes",
                            "tags": ["coding", "repository"]
                        }
                    ]
                }
            ]
        });

        let mock = mock_server.mock("GET", "/api/tools")
            .match_header("authorization", "Bearer sk-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.tools().list(None).await;
        mock.assert();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.toolkits.len(), 1);
        assert_eq!(response.toolkits[0].slug, "github");
        assert_eq!(response.toolkits[0].tools[0].slug, "github_list_repos");
    }

    // ─── ToolsManager::search ─────────────────────────────────────────────────

    #[tokio::test]
    async fn tools_manager_search_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!({
            "tools": [
                {
                    "slug": "github_list_repos",
                    "name": "List Repositories",
                    "description": "List all repositories",
                    "method": "GET",
                    "endpoint": "/user/repos",
                    "scopes": ["repo"],
                    "scopeSatisfied": "yes",
                    "tags": ["coding"]
                }
            ],
            "query": "list repos"
        });

        let mock = mock_server.mock("GET", mockito::Matcher::Regex(r"^/api/tools/search.*$".to_string()))
            .match_header("authorization", "Bearer sk-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.tools().search("list repos", None).await;
        mock.assert();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.tools.len(), 1);
        assert_eq!(response.query, "list repos");
    }

    // ─── ToolsManager::execute ────────────────────────────────────────────────

    #[tokio::test]
    async fn tools_manager_execute_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!({
            "ok": true,
            "body": { "id": 123, "name": "test-repo" },
            "error": null,
            "callId": "call_abc123",
            "status": "completed",
            "durationMs": 150
        });

        let mock = mock_server.mock("POST", "/api/tools/execute")
            .match_header("authorization", "Bearer sk-test")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.tools().execute("github_list_repos", None, None).await;
        mock.assert();

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.ok);
        assert_eq!(response.call_id.as_deref(), Some("call_abc123"));
    }

    // ─── ConnectorsManager::list ──────────────────────────────────────────────

    #[tokio::test]
    async fn connectors_manager_list_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!([
            {
                "platform": "github",
                "enabled": true,
                "scopes": ["repo", "user"],
                "created_at": "2024-01-15T10:30:00Z"
            },
            {
                "platform": "slack",
                "enabled": true,
                "scopes": ["chat:write"],
                "created_at": "2024-01-20T14:00:00Z"
            }
        ]);

        let mock = mock_server.mock("GET", "/api/connectors")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.connectors().list().await;
        mock.assert();

        assert!(result.is_ok());
        let connectors = result.unwrap();
        assert_eq!(connectors.len(), 2);
        assert_eq!(connectors[0].platform, "github");
        assert_eq!(connectors[1].platform, "slack");
    }

    // ─── ConnectorsManager::delete ───────────────────────────────────────────

    #[tokio::test]
    async fn connectors_manager_delete_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock = mock_server.mock("DELETE", "/api/connectors/github")
            .match_header("authorization", "Bearer sk-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{}")
            .create();

        let result = client.connectors().delete("github").await;
        mock.assert();

        assert!(result.is_ok());
    }

    // ─── ApiKeysManager::create ──────────────────────────────────────────────

    #[tokio::test]
    async fn api_keys_manager_create_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!({
            "key": "omk_live_abc123xyz",
            "label": "pr-reviewer-agent",
            "created_at": "2024-01-15T10:30:00Z"
        });

        let mock = mock_server.mock("POST", "/auth/apikey")
            .match_header("authorization", "Bearer sk-test")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.api_keys().create("pr-reviewer-agent").await;
        mock.assert();

        assert!(result.is_ok());
        let api_key = result.unwrap();
        assert_eq!(api_key.key, "omk_live_abc123xyz");
        assert_eq!(api_key.label, "pr-reviewer-agent");
    }

    // ─── ApiKeysManager::list ────────────────────────────────────────────────

    #[tokio::test]
    async fn api_keys_manager_list_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock_response = serde_json::json!([
            {
                "key_hash": "hash123",
                "label": "pr-reviewer-agent",
                "created_at": "2024-01-15T10:30:00Z"
            }
        ]);

        let mock = mock_server.mock("GET", "/auth/apikey")
            .match_header("authorization", "Bearer sk-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create();

        let result = client.api_keys().list().await;
        mock.assert();

        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].key_hash, "hash123");
        // Verify the raw key field is not present in ApiKeySummary (only keyHash)
        assert_eq!(keys[0].label, "pr-reviewer-agent");
    }

    // ─── ApiKeysManager::delete ───────────────────────────────────────────────

    #[tokio::test]
    async fn api_keys_manager_delete_mock() {
        let mut mock_server = mockito::Server::new_async().await;
        let url: String = mock_server.url();
        let static_url: &'static str = Box::leak(url.into_boxed_str());
        let client = Client::new("sk-test", Some(static_url));

        let mock = mock_server.mock("DELETE", "/auth/apikey/hash123")
            .match_header("authorization", "Bearer sk-test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{}")
            .create();

        let result = client.api_keys().delete("hash123").await;
        mock.assert();

        assert!(result.is_ok());
    }

    // ─── Error enum ──────────────────────────────────────────────────────────

    #[test]
    fn error_auth_display() {
        let err = Error::Auth;
        assert_eq!(err.to_string(), "authentication failed");
    }

    #[test]
    fn error_rate_limited_display() {
        let err = Error::RateLimited;
        assert_eq!(err.to_string(), "rate limited");
    }

    #[test]
    fn error_upstream_display() {
        let err = Error::Upstream { status: 500, body: "Internal Server Error".to_string() };
        assert_eq!(err.to_string(), "upstream error 500: Internal Server Error");
    }

    // ─── Error::Network ───────────────────────────────────────────────────────

    #[test]
    fn error_network_implements_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<Error>();
    }

    #[tokio::test]
    async fn error_network_from_reqwest_error() {
        // Create a reqwest error via a connection refused to localhost:1
        let reqwest_err = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(100))
            .build()
            .unwrap()
            .get("http://127.0.0.1:1")
            .send()
            .await
            .unwrap_err();
        let err = Error::Network(reqwest_err);
        // Verify it contains "network error"
        assert!(err.to_string().contains("network error"));
    }
}

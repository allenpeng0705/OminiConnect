//! Feishu API client with OAuth2 token management.

use serde_json::{json, Value};
use std::sync::Arc;

/// Token vault access trait for connector
pub trait TokenVaultAccess: Send + Sync {
    fn get_token(&self, platform: &str, subject: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send + '_>>;
}

/// Feishu API client
pub struct FeishuApiClient {
    /// OAuth vault for token management
    oauth_vault: Arc<dyn TokenVaultAccess>,
    /// Base URL for Feishu API
    base_url: String,
    /// App ID
    app_id: String,
    /// App secret
    app_secret: String,
}

impl FeishuApiClient {
    pub fn new(
        oauth_vault: Arc<dyn TokenVaultAccess>,
        app_id: String,
        app_secret: String,
    ) -> Self {
        Self {
            oauth_vault,
            base_url: "https://open.feishu.cn".to_string(),
            app_id,
            app_secret,
        }
    }

    /// Get tenant access token
    pub async fn get_tenant_token(&self) -> Result<String, anyhow::Error> {
        let url = format!("{}/open-apis/auth/v3/tenant_access_token/internal", self.base_url);

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&serde_json::json!({
                "app_id": self.app_id,
                "app_secret": self.app_secret
            }))
            .send()
            .await?;

        let body: Value = resp.json().await?;

        if body["code"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("Feishu auth failed: {:?}", body["msg"]);
        }

        Ok(body["tenant_access_token"].as_str().unwrap().to_string())
    }

    /// Make an API call with authorization
    pub async fn call_api(
        &self,
        method: &str,
        path: &str,
        token: &str,
        body: Option<Value>,
    ) -> Result<Value, anyhow::Error> {
        let url = format!("{}{}", self.base_url, path);

        let client = reqwest::Client::new();
        let mut req = client.request(method.parse()?, &url);

        req = req.header("Authorization", format!("Bearer {}", token));
        req = req.header("Content-Type", "application/json");

        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        let body: Value = resp.json().await?;

        if body["code"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("Feishu API error: {:?}", body["msg"]);
        }

        Ok(body)
    }
}

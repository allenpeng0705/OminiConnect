//! DingTalk API client.

use serde_json::Value;
use std::sync::Arc;

/// Token vault access trait for connector
pub trait TokenVaultAccess: Send + Sync {
    fn get_token(&self, platform: &str, subject: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send + '_>>;
}

/// DingTalk API client
pub struct DingTalkApiClient {
    /// OAuth vault for token management
    oauth_vault: Arc<dyn TokenVaultAccess>,
    /// Base URL for DingTalk API
    base_url: String,
    /// App ID
    app_key: String,
    /// App Secret
    app_secret: String,
}

impl DingTalkApiClient {
    pub fn new(
        oauth_vault: Arc<dyn TokenVaultAccess>,
        app_key: String,
        app_secret: String,
    ) -> Self {
        Self {
            oauth_vault,
            base_url: "https://oapi.dingtalk.com".to_string(),
            app_key,
            app_secret,
        }
    }

    /// Get access token
    pub async fn get_access_token(&self) -> Result<String, anyhow::Error> {
        let url = format!("{}/gettoken", self.base_url);

        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .query(&[
                ("appkey", self.app_key.as_str()),
                ("appsecret", self.app_secret.as_str()),
            ])
            .send()
            .await?;

        let body: Value = resp.json().await?;

        if body["errcode"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("DingTalk auth failed: {:?}", body["errmsg"]);
        }

        Ok(body["access_token"].as_str().unwrap().to_string())
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

        req = req.header("x-acs-dingtalk-access-token", token);
        req = req.header("Content-Type", "application/json");

        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        let body: Value = resp.json().await?;

        if body["errcode"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("DingTalk API error: {:?}", body["errmsg"]);
        }

        Ok(body)
    }
}

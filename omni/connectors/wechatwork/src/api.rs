//! WeChat Work API client.

use serde_json::Value;
use std::sync::Arc;

/// Token vault access trait for connector
pub trait TokenVaultAccess: Send + Sync {
    fn get_token(&self, platform: &str, subject: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send + '_>>;
}

/// WeChat Work API client
pub struct WeChatWorkApiClient {
    /// OAuth vault for token management
    oauth_vault: Arc<dyn TokenVaultAccess>,
    /// Base URL for WeChat Work API
    base_url: String,
    /// Corp ID
    corp_id: String,
    /// Corp Secret
    corp_secret: String,
}

impl WeChatWorkApiClient {
    pub fn new(
        oauth_vault: Arc<dyn TokenVaultAccess>,
        corp_id: String,
        corp_secret: String,
    ) -> Self {
        Self {
            oauth_vault,
            base_url: "https://qyapi.weixin.qq.com".to_string(),
            corp_id,
            corp_secret,
        }
    }

    /// Get access token
    pub async fn get_access_token(&self) -> Result<String, anyhow::Error> {
        let url = format!(
            "{}/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.base_url, self.corp_id, self.corp_secret
        );

        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;

        let body: Value = resp.json().await?;

        if body["errcode"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("WeChat Work auth failed: {:?}", body["errmsg"]);
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
        let url = format!("{}{}?access_token={}", self.base_url, path, token);

        let client = reqwest::Client::new();
        let mut req = client.request(method.parse()?, &url);

        req = req.header("Content-Type", "application/json");

        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await?;
        let body: Value = resp.json().await?;

        if body["errcode"].as_i64().unwrap_or(-1) != 0 {
            anyhow::bail!("WeChat Work API error: {:?}", body["errmsg"]);
        }

        Ok(body)
    }
}

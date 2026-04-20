//! WeChat Work API client.

use serde_json::Value;

/// WeChat Work API client
pub struct WeChatWorkApiClient {
    /// Base URL for WeChat Work API
    base_url: String,
}

impl WeChatWorkApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://qyapi.weixin.qq.com".to_string(),
        }
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

impl Default for WeChatWorkApiClient {
    fn default() -> Self {
        Self::new()
    }
}

//! Feishu API client.

use serde_json::Value;

/// Feishu API client
pub struct FeishuApiClient {
    /// Base URL for Feishu API
    base_url: String,
}

impl FeishuApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://open.feishu.cn".to_string(),
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

impl Default for FeishuApiClient {
    fn default() -> Self {
        Self::new()
    }
}

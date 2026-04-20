//! DingTalk API client.

use serde_json::Value;

/// DingTalk API client
pub struct DingTalkApiClient {
    /// Base URL for DingTalk API
    base_url: String,
}

impl DingTalkApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://oapi.dingtalk.com".to_string(),
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

impl Default for DingTalkApiClient {
    fn default() -> Self {
        Self::new()
    }
}

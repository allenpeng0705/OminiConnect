//! Feishu API client.

use serde_json::Value;

/// Feishu API client
pub struct FeishuApiClient {
    /// Base URL for Feishu API
    base_url: String,
    /// App ID
    app_id: String,
    /// App secret
    app_secret: String,
}

impl FeishuApiClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            base_url: "https://open.feishu.cn".to_string(),
            app_id,
            app_secret,
        }
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<String, anyhow::Error> {
        let url = format!("{}/open-apis/authen/v1/oid/connect/tenant_access_token", self.base_url);

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&serde_json::json!({
                "grant_type": "authorization_code",
                "code": code,
                "redirect_uri": redirect_uri,
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

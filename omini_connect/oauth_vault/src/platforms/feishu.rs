//! Feishu (Lark) OAuth2 platform implementation.

use crate::{OAuthError, OAuthToken};
use crate::platform::{OAuth2Platform, PlatformConfig};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

/// Feishu OAuth2 platform handler
pub struct FeishuPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct FeishuTokenResponse {
    code: i64,
    msg: String,
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "token_type")]
    token_type: Option<String>,
    #[serde(rename = "expires_in")]
    expires_in: Option<i64>,
    #[serde(rename = "scope")]
    _scope: Option<String>,
}

impl FeishuPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl OAuth2Platform for FeishuPlatform {
    fn name(&self) -> &str {
        "feishu"
    }

    /// Exchange authorization code for user access token
    async fn exchange_code(&self, code: &str, _redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://open.feishu.cn/open-apis/authen/v1/oid/connect/token";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&serde_json::json!({
                "grant_type": "authorization_code",
                "code": code,
                "app_id": self.config.client_id,
                "app_secret": self.config.client_secret,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body_text = resp
            .text()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: FeishuTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if body.code != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "Feishu auth error: {} - {}",
                body.code, body.msg
            )));
        }

        let access_token = body.access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + body.expires_in.unwrap_or(7200);

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    /// Refresh user access token using refresh token
    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://open.feishu.cn/open-apis/authen/v1/oid/connect/token";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
                "app_id": self.config.client_id,
                "app_secret": self.config.client_secret,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body_text = resp
            .text()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: FeishuTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if body.code != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "Feishu refresh error: {} - {}",
                body.code, body.msg
            )));
        }

        let access_token = body.access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                now + body.expires_in.unwrap_or(7200)
            },
            scopes: self.config.scopes.clone(),
        })
    }

    /// Get Feishu OAuth authorization URL for user consent
    fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        let auth_base = "https://open.feishu.cn/open-apis/authen/v1/authorize";
        format!(
            "{}?app_id={}&redirect_uri={}&scope={}&state={}",
            auth_base,
            self.config.client_id,
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            state
        )
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        let url = "https://open.feishu.cn/open-apis/authen/v1/oid/revoke";

        self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        Ok(())
    }
}

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
    tenant_access_token: Option<String>,
    user_access_token: Option<String>,
    refresh_token: Option<String>,
    expire: Option<i64>,
    token_type: Option<String>,
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

    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://open.feishu.cn/open-apis/authen/v1/oid/connect/tenant_access_token";

        let resp = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "grant_type": "authorization_code",
                "code": code,
                "redirect_uri": redirect_uri,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: FeishuTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.code != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "Feishu auth error: {}",
                body.msg
            )));
        }

        let access_token = body
            .tenant_access_token
            .or(body.user_access_token)
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let expires_at = body.expire.unwrap_or(7200);

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "tenant".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://open.feishu.cn/open-apis/authen/v1/oid/refresh_token";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: FeishuTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.code != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "Feishu refresh error: {}",
                body.msg
            )));
        }

        let access_token = body
            .tenant_access_token
            .or(body.user_access_token)
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: body.expire.unwrap_or(7200),
            scopes: self.config.scopes.clone(),
        })
    }

    fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(",");
        format!(
            "{}?app_id={}&redirect_uri={}&scope={}&state={}",
            self.config.auth_url, self.config.client_id, self.config.redirect_uri, scopes, state
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

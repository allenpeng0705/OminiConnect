//! DingTalk OAuth2 platform implementation.

use crate::{OAuthError, OAuthToken};
use crate::platform::{OAuth2Platform, PlatformConfig};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

/// DingTalk OAuth2 platform handler
pub struct DingTalkPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct DingTalkTokenResponse {
    errcode: i64,
    errmsg: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    token_type: Option<String>,
}

impl DingTalkPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl OAuth2Platform for DingTalkPlatform {
    fn name(&self) -> &str {
        "dingtalk"
    }

    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.dingtalk.com/v1.0/oauth2/accessToken";

        let resp = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "appKey": self.config.client_id,
                "appSecret": self.config.client_secret,
                "code": code,
                "redirect_uri": redirect_uri,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: DingTalkTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "DingTalk auth error: {}",
                body.errmsg
            )));
        }

        let access_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: body.expires_in.unwrap_or(7200),
            scopes: self.config.scopes.clone(),
        })
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.dingtalk.com/v1.0/oauth2/refreshAccessToken";

        let resp = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "appKey": self.config.client_id,
                "appSecret": self.config.client_secret,
                "refreshToken": refresh_token,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: DingTalkTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "DingTalk refresh error: {}",
                body.errmsg
            )));
        }

        let access_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: body.expires_in.unwrap_or(7200),
            scopes: self.config.scopes.clone(),
        })
    }

    fn get_auth_url(&self, state: &str) -> String {
        // Simple URL encoding without external dependency
        fn encode_url(s: &str) -> String {
            s.chars().map(|c| {
                match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                    _ => format!("%{:02X}", c as u8),
                }
            }).collect()
        }

        let scopes = self.config.scopes.join(",");
        format!(
            "{}?appid={}&redirect_uri={}&scope={}&state={}",
            self.config.auth_url,
            self.config.client_id,
            encode_url(&self.config.redirect_uri),
            scopes,
            state
        )
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        let url = "https://api.dingtalk.com/v1.0/oauth2/revokeAccessToken";

        self.client
            .post(url)
            .json(&serde_json::json!({
                "accessToken": token,
            }))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        Ok(())
    }
}

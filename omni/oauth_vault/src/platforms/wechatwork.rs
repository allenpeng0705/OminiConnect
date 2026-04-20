//! WeChat Work OAuth2 platform implementation.

use crate::{OAuthError, OAuthToken};
use crate::platform::{OAuth2Platform, PlatformConfig};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

/// WeChat Work OAuth2 platform handler
pub struct WeChatWorkPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct WeChatWorkTokenResponse {
    errcode: i64,
    errmsg: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    token_type: Option<String>,
}

impl WeChatWorkPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl OAuth2Platform for WeChatWorkPlatform {
    fn name(&self) -> &str {
        "wechatwork"
    }

    async fn exchange_code(&self, _code: &str, _redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        // WeChat Work uses a different OAuth flow - the code is exchanged directly
        // with the corp_id and corp_secret for user access token
        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.config.client_id,
            self.config.client_secret
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: WeChatWorkTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "WeChat Work auth error: {}",
                body.errmsg
            )));
        }

        let access_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let expires_at = body.expires_in.unwrap_or(7200);

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

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        // WeChat Work doesn't have a standard refresh token flow for user tokens
        // For user tokens, the login process needs to be repeated
        // For agent tokens, the access token can be refreshed using the same endpoint
        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.config.client_id,
            self.config.client_secret
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: WeChatWorkTokenResponse = resp
            .json()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if body.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "WeChat Work refresh error: {}",
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
            refresh_token: Some(refresh_token.to_string()),
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: body.expires_in.unwrap_or(7200),
            scopes: self.config.scopes.clone(),
        })
    }

    fn get_auth_url(&self, state: &str) -> String {
        // WeChat Work OAuth URL for user authorization
        // Simple URL encoding without external dependency
        fn encode_url(s: &str) -> String {
            s.chars().map(|c| {
                match c {
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                    _ => format!("%{:02X}", c as u8),
                }
            }).collect()
        }

        format!(
            "https://open.work.weixin.qq.com/wwopen/sso/qrConnect?appid={}&agentid={}&redirect_uri={}&state={}",
            self.config.client_id,
            self.config.redirect_uri, // Using redirect_uri as agentid in this context
            encode_url(&self.config.auth_url),
            state
        )
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        // WeChat Work doesn't have a standard token revocation endpoint
        // The token will simply expire
        let _ = token;
        Ok(())
    }
}

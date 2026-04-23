//! WeChat Work OAuth2 platform implementation.
//!
//! WeChat Work uses OAuth2 with a redirect flow. The authorization URL is:
//!   https://open.weixin.qq.com/connect/oauth2/authorize?appid=CORPID&redirect_uri=REDIRECT_URI&response_type=code&scope=snsapi_base&state=STATE#wechat_redirect
//!
//! After user authorization, the callback receives a `code` which is exchanged for a user access token.

use crate::{OAuthError, OAuthToken};
use crate::platform::{OAuth2Platform, PlatformConfig};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

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

    /// URL-encode string for WeChat Work OAuth
    fn encode_url(s: &str) -> String {
        s.chars().map(|c| {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                _ => format!("%{:02X}", c as u8),
            }
        }).collect()
    }
}

#[async_trait]
impl OAuth2Platform for WeChatWorkPlatform {
    fn name(&self) -> &str {
        "wechatwork"
    }

    /// Exchange authorization code for user access token.
    ///
    /// WeChat Work OAuth2 flow:
    /// 1. Redirect user to authorization URL
    /// 2. User authorizes, callback receives `code`
    /// 3. Exchange code for access token at /cgi-bin/oauth2/access_token
    async fn exchange_code(&self, code: &str, _redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        // Exchange authorization code for user access token
        // Endpoint: https://qyapi.weixin.qq.com/cgi-bin/oauth2/access_token
        //
        // For QR code login (wwopen/sso/qrConnect), we need to include agentid
        let agentid = if !self.config.agent_id.is_empty() {
            &self.config.agent_id
        } else {
            "0"
        };

        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code&agentid={}",
            self.config.client_id,
            self.config.client_secret,
            code,
            agentid
        );

        eprintln!("WeChat Work token exchange URL: {}", url);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        eprintln!("WeChat Work token response status: {}", resp.status());

        let body_text = resp
            .text()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        eprintln!("WeChat Work token response body: {}", body_text);

        if body_text.is_empty() {
            return Err(OAuthError::ExchangeFailed("Empty response body from WeChat Work".to_string()));
        }

        let token_resp: WeChatWorkTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if token_resp.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "WeChat Work token error: {} - {}",
                token_resp.errcode, token_resp.errmsg
            )));
        }

        let access_token = token_resp.access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let expires_at = token_resp.expires_in.unwrap_or(7200);

        // Note: WeChat Work user tokens don't have a separate refresh token.
        // They need to re-authenticate when expired. We store the access_token
        // as its own refresh (using same token to re-fetch).
        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: token_resp.refresh_token.or_else(|| Some(code.to_string())),
            token_type: token_resp.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    async fn refresh_token(&self, _refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        // WeChat Work user tokens don't have a standard refresh flow.
        // For user tokens, the login process needs to be repeated.
        // However, we can try to re-use the existing token info.
        //
        // If refresh_token is actually an old access_token, we can't refresh it.
        // We return an error suggesting re-authentication.
        //
        // Alternative: Get a new access token using corp secret (but this gives app token, not user token)
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

        let body_text = resp
            .text()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let token_resp: WeChatWorkTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if token_resp.errcode != 0 {
            return Err(OAuthError::ExchangeFailed(format!(
                "WeChat Work refresh error: {} - {}",
                token_resp.errcode, token_resp.errmsg
            )));
        }

        let access_token = token_resp.access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        // Note: This gives an application access token, not a user access token.
        // For full user context, re-authentication is needed.
        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: None,
            token_type: token_resp.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at: token_resp.expires_in.unwrap_or(7200),
            scopes: self.config.scopes.clone(),
        })
    }

    /// Get WeChat Work QR code OAuth authorization URL.
    ///
    /// WeChat Work QR code login URL format:
    /// https://open.work.weixin.qq.com/wwopen/sso/qrConnect?appid=CORPID&agentid=AGENTID&redirect_uri=CALLBACK_URL&state=STATE
    ///
    /// Users scan the QR code with their WeChat Work app to authorize.
    ///
    /// Config mapping:
    /// - client_id: Corp ID (e.g., wwxxxxxxxxxxxx)
    /// - agent_id: Agent ID (numeric, e.g., 1000001)
    /// - redirect_uri: The OAuth callback URL
    fn get_auth_url(&self, state: &str) -> String {
        let auth_base = "https://open.work.weixin.qq.com/wwopen/sso/qrConnect";

        // redirect_uri is the callback URL
        let redirect_uri = &self.config.redirect_uri;

        // agentid is required for WeChat Work QR code login
        let agentid = if !self.config.agent_id.is_empty() {
            &self.config.agent_id
        } else {
            "0"
        };

        format!(
            "{}?appid={}&agentid={}&redirect_uri={}&state={}",
            auth_base,
            self.config.client_id,
            agentid,
            Self::encode_url(redirect_uri),
            state
        )
    }

    async fn revoke_token(&self, _token: &str) -> Result<(), OAuthError> {
        // WeChat Work doesn't have a standard token revocation endpoint
        // The token will simply expire
        Ok(())
    }
}
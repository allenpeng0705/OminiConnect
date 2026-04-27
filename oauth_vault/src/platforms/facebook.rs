//! Facebook OAuth2 platform implementation.

use crate::platform::{OAuth2Platform, PlatformConfig};
use crate::{OAuthError, OAuthToken};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct FacebookPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields mirror Facebook token JSON; only some are read.
struct FacebookTokenResponse {
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "token_type")]
    token_type: Option<String>,
    #[serde(rename = "expires_in")]
    expires_in: Option<i64>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "error")]
    error: Option<String>,
    #[serde(rename = "error_message")]
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FacebookPage {
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "id")]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FacebookPagesResponse {
    data: Vec<FacebookPage>,
}

impl FacebookPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Exchange code for user token, then fetch Page token for the first managed Page
    async fn exchange_code_impl(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OAuthToken, OAuthError> {
        // Step 1: Exchange code for user access token
        let url = "https://graph.facebook.com/v21.0/oauth/access_token";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
                urlencoding::encode(code),
                urlencoding::encode(redirect_uri),
                urlencoding::encode(&self.config.client_id),
                urlencoding::encode(&self.config.client_secret),
            ))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body_text = resp
            .text()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let body: FacebookTokenResponse = serde_json::from_str(&body_text).map_err(|e| {
            OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text))
        })?;

        if let Some(err) = body.error {
            return Err(OAuthError::ExchangeFailed(format!(
                "Facebook auth error: {} - {}",
                err,
                body.error_message.unwrap_or_default()
            )));
        }

        let user_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        // Step 2: Exchange user token for long-lived token
        let extended_token = self.extend_token(&user_token).await?;

        // Step 3: Get Page access token
        let page_token = self.get_page_token(&extended_token).await?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + 3600; // Page tokens don't expire but we set a default

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token: page_token,
            refresh_token: Some(extended_token),
            token_type: "Bearer".to_string(),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    /// Extend user token to long-lived token (60 days)
    async fn extend_token(&self, user_token: &str) -> Result<String, OAuthError> {
        let url = format!(
            "https://graph.facebook.com/v21.0/oauth/access_token?grant_type=fb_exchange_token&client_id={}&client_secret={}&fb_exchange_token={}",
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.client_secret),
            urlencoding::encode(user_token),
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

        let body: FacebookTokenResponse = serde_json::from_str(&body_text).map_err(|e| {
            OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text))
        })?;

        if let Some(err) = body.error {
            return Err(OAuthError::ExchangeFailed(format!(
                "Facebook token extend error: {} - {}",
                err,
                body.error_message.unwrap_or_default()
            )));
        }

        body.access_token.ok_or_else(|| {
            OAuthError::ExchangeFailed("No access token in extended response".to_string())
        })
    }

    /// Get Page access token for the first managed Page
    async fn get_page_token(&self, user_token: &str) -> Result<String, OAuthError> {
        let url = format!(
            "https://graph.facebook.com/v21.0/me/accounts?access_token={}",
            urlencoding::encode(user_token),
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

        let pages: FacebookPagesResponse = serde_json::from_str(&body_text).map_err(|e| {
            OAuthError::ExchangeFailed(format!(
                "JSON parse error for pages: {} - body: {}",
                e, body_text
            ))
        })?;

        if let Some(page) = pages.data.first() {
            if let Some(token) = &page.access_token {
                return Ok(token.clone());
            }
        }

        // No Pages found, return the user token as fallback
        Ok(user_token.to_string())
    }
}

#[async_trait]
impl OAuth2Platform for FacebookPlatform {
    fn name(&self) -> &str {
        "facebook"
    }

    async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OAuthToken, OAuthError> {
        self.exchange_code_impl(code, redirect_uri).await
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        // Re-extend the stored refresh token (which is the extended user token)
        let extended_token = self.extend_token(refresh_token).await?;

        // Get page token again
        let page_token = self.get_page_token(&extended_token).await?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + 3600;

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: "user".to_string(),
            access_token: page_token,
            refresh_token: Some(extended_token),
            token_type: "Bearer".to_string(),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(",");
        let auth_base = "https://www.facebook.com/v21.0/dialog/oauth";
        format!(
            "{}?client_id={}&redirect_uri={}&scope={}&state={}",
            auth_base,
            self.config.client_id,
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            state
        )
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        let url = format!(
            "https://graph.facebook.com/v21.0/me/permissions?access_token={}",
            urlencoding::encode(token)
        );

        self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        Ok(())
    }
}

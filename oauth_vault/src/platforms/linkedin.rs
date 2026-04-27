//! LinkedIn OAuth2 platform implementation.

use crate::platform::{OAuth2Platform, PlatformConfig};
use crate::{OAuthError, OAuthToken};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct LinkedInPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields mirror LinkedIn token JSON; only some are read.
struct LinkedInTokenResponse {
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "token_type")]
    token_type: Option<String>,
    #[serde(rename = "expires_in")]
    expires_in: Option<i64>,
    #[serde(rename = "scope")]
    scope: Option<String>,
    #[serde(rename = "id_token")]
    id_token: Option<String>,
    pub sub: Option<String>,
}

impl LinkedInPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl OAuth2Platform for LinkedInPlatform {
    fn name(&self) -> &str {
        "linkedin"
    }

    /// Exchange authorization code for user access token
    async fn exchange_code(
        &self,
        code: &str,
        _redirect_uri: &str,
    ) -> Result<OAuthToken, OAuthError> {
        let url = "https://www.linkedin.com/oauth/v2/accessToken";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
                urlencoding::encode(code),
                urlencoding::encode(&self.config.redirect_uri),
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

        let body: LinkedInTokenResponse = serde_json::from_str(&body_text).map_err(|e| {
            OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text))
        })?;

        let access_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + body.expires_in.unwrap_or(5184000);

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: body.sub.clone().unwrap_or_else(|| "user".to_string()),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    /// Refresh access token (LinkedIn refresh tokens are long-lived, but may expire)
    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://www.linkedin.com/oauth/v2/accessToken";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}",
                urlencoding::encode(refresh_token),
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

        let body: LinkedInTokenResponse = serde_json::from_str(&body_text).map_err(|e| {
            OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text))
        })?;

        let access_token = body
            .access_token
            .ok_or_else(|| OAuthError::ExchangeFailed("No access token in response".to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + body.expires_in.unwrap_or(5184000);

        Ok(OAuthToken {
            platform: self.name().to_string(),
            subject: body.sub.unwrap_or_else(|| "user".to_string()),
            access_token,
            refresh_token: body
                .refresh_token
                .or_else(|| Some(refresh_token.to_string())),
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    /// Get LinkedIn OAuth authorization URL
    fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        let auth_base = "https://www.linkedin.com/oauth/v2/authorization";
        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            auth_base,
            self.config.client_id,
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            state
        )
    }

    async fn revoke_token(&self, token: &str) -> Result<(), OAuthError> {
        let url = "https://www.linkedin.com/oauth/v2/revocateToken";

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "token={}&client_id={}&client_secret={}",
                urlencoding::encode(token),
                urlencoding::encode(&self.config.client_id),
                urlencoding::encode(&self.config.client_secret),
            ))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(OAuthError::ExchangeFailed(format!(
                "Revoke failed: {}",
                resp.status()
            )));
        }

        Ok(())
    }
}

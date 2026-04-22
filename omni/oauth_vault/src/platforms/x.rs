//! X (Twitter) OAuth2 platform implementation.

use crate::{OAuthError, OAuthToken};
use crate::platform::{OAuth2Platform, PlatformConfig};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct XPlatform {
    config: PlatformConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct XTokenResponse {
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "token_type")]
    token_type: Option<String>,
    #[serde(rename = "expires_in")]
    expires_in: Option<i64>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "scope")]
    scope: Option<String>,
    #[serde(rename = "error")]
    error: Option<String>,
    #[serde(rename = "error_description")]
    error_description: Option<String>,
}

impl XPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl OAuth2Platform for XPlatform {
    fn name(&self) -> &str {
        "x"
    }

    async fn exchange_code(&self, code: &str, _redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.x.com/2/oauth2/token";

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

        let body: XTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if let Some(err) = body.error {
            return Err(OAuthError::ExchangeFailed(format!(
                "X auth error: {} - {}",
                err,
                body.error_description.unwrap_or_default()
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
            scopes: if let Some(scope) = body.scope {
                scope.split_whitespace().map(String::from).collect()
            } else {
                self.config.scopes.clone()
            },
        })
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.x.com/2/oauth2/token";

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

        let body: XTokenResponse = serde_json::from_str(&body_text)
            .map_err(|e| OAuthError::ExchangeFailed(format!("JSON parse error: {} - body: {}", e, body_text)))?;

        if let Some(err) = body.error {
            return Err(OAuthError::ExchangeFailed(format!(
                "X refresh error: {} - {}",
                err,
                body.error_description.unwrap_or_default()
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
            refresh_token: body.refresh_token.or_else(|| Some(refresh_token.to_string())),
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: self.config.scopes.clone(),
        })
    }

    fn get_auth_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        let auth_base = "https://twitter.com/i/oauth2/authorize";
        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            auth_base,
            self.config.client_id,
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            state
        )
    }

    async fn revoke_token(&self, _token: &str) -> Result<(), OAuthError> {
        // X has app-specific token revocation patterns. We treat delete/disconnect in OmniConnect
        // as local vault disconnect for now.
        Ok(())
    }
}

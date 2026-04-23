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

    /// Authorization URL including PKCE parameters (required by X).
    pub fn authorize_url(&self, state: &str, code_challenge: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        let auth_base = "https://twitter.com/i/oauth2/authorize";
        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            auth_base,
            self.config.client_id,
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            state,
            urlencoding::encode(code_challenge),
        )
    }

    /// Exchange authorization code using PKCE `code_verifier`.
    pub async fn exchange_authorization_code(
        &self,
        code: &str,
        redirect_uri: &str,
        code_verifier: &str,
    ) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.x.com/2/oauth2/token";

        // X OAuth2 requires Basic auth header with client_id:client_secret
        let credentials = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, credentials);

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Basic {}", encoded))
            .body(format!(
                "grant_type=authorization_code&code={}&redirect_uri={}&code_verifier={}",
                urlencoding::encode(code),
                urlencoding::encode(redirect_uri),
                urlencoding::encode(code_verifier),
            ))
            .send()
            .await
            .map_err(|e| OAuthError::ExchangeFailed(e.to_string()))?;

        let mut token = Self::parse_token_response(resp, self.config.scopes.clone()).await?;
        token.platform = self.name().to_string();
        Ok(token)
    }

    async fn parse_token_response(
        resp: reqwest::Response,
        default_scopes: Vec<String>,
    ) -> Result<OAuthToken, OAuthError> {
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
            platform: "x".to_string(),
            subject: "user".to_string(),
            access_token,
            refresh_token: body.refresh_token,
            token_type: body.token_type.unwrap_or_else(|| "Bearer".to_string()),
            expires_at,
            scopes: if let Some(scope) = body.scope {
                scope.split_whitespace().map(String::from).collect()
            } else {
                default_scopes
            },
        })
    }
}

#[async_trait]
impl OAuth2Platform for XPlatform {
    fn name(&self) -> &str {
        "x"
    }

    async fn exchange_code(&self, _code: &str, _redirect_uri: &str) -> Result<OAuthToken, OAuthError> {
        // X requires PKCE; portal must call `exchange_authorization_code` with a verifier.
        Err(OAuthError::ExchangeFailed(
            "X OAuth2 requires PKCE (use exchange_authorization_code)".to_string(),
        ))
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let url = "https://api.x.com/2/oauth2/token";

        // X OAuth2 requires Basic auth header with client_id:client_secret
        let credentials = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, credentials);

        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", format!("Basic {}", encoded))
            .body(format!(
                "grant_type=refresh_token&refresh_token={}",
                urlencoding::encode(refresh_token),
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

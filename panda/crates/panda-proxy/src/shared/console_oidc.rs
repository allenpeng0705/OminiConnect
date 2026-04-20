//! OIDC login for the Developer Console (Okta, Entra, etc.).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use base64::Engine;
use http::header::{self, HeaderMap, HeaderValue};
use http_body_util::{BodyExt, Empty, Full};
use hyper::Request;
use hyper::Response;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header as JwtHeader, Validation,
};
use panda_config::ConsoleOidcConfig;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::jwks::JwksResolver;
use crate::HttpClient;
use crate::{BoxBody, StatusCode};

#[derive(Debug, Clone)]
pub struct OpenIdDiscovery {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
}

pub struct ConsoleOidcRuntime {
    cfg: std::sync::Arc<ConsoleOidcConfig>,
    pub discovery: OpenIdDiscovery,
    pub jwks: std::sync::Arc<JwksResolver>,
    pending: Mutex<HashMap<String, PendingAuth>>,
}

#[derive(Debug, Clone)]
struct PendingAuth {
    created_at: Instant,
    code_verifier: Option<String>,
    nonce: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsoleSessionJwt {
    sub: String,
    exp: usize,
    iat: usize,
    #[serde(default)]
    roles: Vec<String>,
}

fn issuer_well_known(issuer: &str) -> String {
    let i = issuer.trim().trim_end_matches('/');
    format!("{i}/.well-known/openid-configuration")
}

pub async fn http_get_string(client: &HttpClient, uri: hyper::Uri) -> anyhow::Result<String> {
    let req = Request::builder()
        .method(hyper::Method::GET)
        .uri(uri)
        .body(
            Empty::<bytes::Bytes>::new()
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| anyhow::anyhow!("http get build: {e}"))?;
    let resp = client
        .request(req)
        .await
        .map_err(|e| anyhow::anyhow!("http get: {e}"))?;
    let body = resp
        .into_body()
        .collect()
        .await
        .map_err(|e| anyhow::anyhow!("http get body: {e}"))?;
    Ok(String::from_utf8_lossy(&body.to_bytes()).to_string())
}

pub async fn http_post_form(
    client: &HttpClient,
    uri: hyper::Uri,
    form_body: String,
) -> anyhow::Result<String> {
    let req = Request::builder()
        .method(hyper::Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(
            Full::new(bytes::Bytes::from(form_body))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| anyhow::anyhow!("http post build: {e}"))?;
    let resp = client
        .request(req)
        .await
        .map_err(|e| anyhow::anyhow!("http post: {e}"))?;
    let body = resp
        .into_body()
        .collect()
        .await
        .map_err(|e| anyhow::anyhow!("http post body: {e}"))?;
    Ok(String::from_utf8_lossy(&body.to_bytes()).to_string())
}

impl ConsoleOidcRuntime {
    pub async fn connect(
        oc: &ConsoleOidcConfig,
        client: &HttpClient,
    ) -> anyhow::Result<std::sync::Arc<Self>> {
        let disc_url = issuer_well_known(&oc.issuer_url);
        let disc_uri: hyper::Uri = disc_url.parse()?;
        let text = http_get_string(client, disc_uri).await?;
        let v: serde_json::Value = serde_json::from_str(&text)?;
        let authorization_endpoint = v
            .get("authorization_endpoint")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow::anyhow!("openid discovery: authorization_endpoint missing"))?
            .to_string();
        let token_endpoint = v
            .get("token_endpoint")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow::anyhow!("openid discovery: token_endpoint missing"))?
            .to_string();
        let jwks_uri = v
            .get("jwks_uri")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow::anyhow!("openid discovery: jwks_uri missing"))?
            .to_string();
        let jwks = std::sync::Arc::new(JwksResolver::new(
            client.clone(),
            jwks_uri.clone(),
            Duration::from_secs(3600),
        ));
        Ok(std::sync::Arc::new(Self {
            cfg: std::sync::Arc::new(oc.clone()),
            discovery: OpenIdDiscovery {
                authorization_endpoint,
                token_endpoint,
                jwks_uri,
            },
            jwks,
            pending: Mutex::new(HashMap::new()),
        }))
    }

    fn redirect_uri_public(&self) -> anyhow::Result<String> {
        let base = self.cfg.redirect_base_url.trim().trim_end_matches('/');
        if base.is_empty() {
            anyhow::bail!(
                "console_oidc.redirect_base_url must be set for production OIDC redirects"
            );
        }
        Ok(format!("{}{}", base, self.cfg.redirect_path))
    }

    pub fn handle_login(&self) -> Result<Response<BoxBody>, anyhow::Error> {
        let redirect_uri = self.redirect_uri_public()?;
        let state = uuid::Uuid::new_v4().to_string();
        if let Ok(mut g) = self.pending.lock() {
            g.retain(|_, t| t.created_at.elapsed() < Duration::from_secs(600));
            let code_verifier = if self.cfg.require_pkce {
                Some(generate_pkce_verifier())
            } else {
                None
            };
            let nonce = if self.cfg.require_nonce {
                Some(uuid::Uuid::new_v4().as_simple().to_string())
            } else {
                None
            };
            g.insert(
                state.clone(),
                PendingAuth {
                    created_at: Instant::now(),
                    code_verifier,
                    nonce,
                },
            );
        }
        let scope = self.cfg.scopes.join(" ");
        let (pkce_query, nonce_query) = {
            let g = self
                .pending
                .lock()
                .map_err(|_| anyhow::anyhow!("pending lock"))?;
            let p = g
                .get(&state)
                .ok_or_else(|| anyhow::anyhow!("pending oauth state missing"))?;
            let pkce = p
                .code_verifier
                .as_ref()
                .map(|v| {
                    let challenge = pkce_s256_challenge(v);
                    format!(
                        "&code_challenge={}&code_challenge_method=S256",
                        urlencoding::encode(&challenge)
                    )
                })
                .unwrap_or_default();
            let nonce = p
                .nonce
                .as_ref()
                .map(|n| format!("&nonce={}", urlencoding::encode(n)))
                .unwrap_or_default();
            (pkce, nonce)
        };
        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}{}{}",
            self.discovery.authorization_endpoint,
            urlencoding::encode(self.cfg.client_id.trim()),
            urlencoding::encode(redirect_uri.trim()),
            urlencoding::encode(&scope),
            urlencoding::encode(&state),
            pkce_query,
            nonce_query,
        );
        let mut resp = Response::builder()
            .status(StatusCode::FOUND)
            .body(
                Full::new(bytes::Bytes::from("redirecting"))
                    .map_err(|never: std::convert::Infallible| match never {})
                    .boxed_unsync(),
            )
            .map_err(|e| anyhow::anyhow!("response: {e}"))?;
        resp.headers_mut().insert(
            header::LOCATION,
            HeaderValue::try_from(url.as_str()).map_err(|e| anyhow::anyhow!("Location: {e}"))?,
        );
        Ok(resp)
    }

    pub async fn handle_callback(
        &self,
        client: &HttpClient,
        query: Option<&str>,
    ) -> Result<Response<BoxBody>, anyhow::Error> {
        let q = query.unwrap_or("");
        let mut code = None;
        let mut state = None;
        for pair in q.split('&') {
            let mut kv = pair.splitn(2, '=');
            let k = kv.next().unwrap_or("");
            let v = kv.next().unwrap_or("");
            let v = urlencoding::decode(v).unwrap_or_default();
            match k {
                "code" => code = Some(v.to_string()),
                "state" => state = Some(v.to_string()),
                _ => {}
            }
        }
        let code = code.ok_or_else(|| anyhow::anyhow!("missing code"))?;
        let state = state.ok_or_else(|| anyhow::anyhow!("missing state"))?;
        let pending = {
            let mut g = self
                .pending
                .lock()
                .map_err(|_| anyhow::anyhow!("pending lock"))?;
            let Some(p) = g.remove(&state) else {
                anyhow::bail!("invalid or expired oauth state");
            };
            p
        };
        let redirect_uri = self.redirect_uri_public()?;
        let secret = std::env::var(&self.cfg.client_secret_env)
            .map_err(|_| anyhow::anyhow!("client secret env not set"))?;
        let token_uri: hyper::Uri = self.discovery.token_endpoint.parse()?;
        let mut form = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
            urlencoding::encode(&code),
            urlencoding::encode(redirect_uri.trim()),
            urlencoding::encode(self.cfg.client_id.trim()),
            urlencoding::encode(&secret),
        );
        if let Some(ref verifier) = pending.code_verifier {
            form.push_str("&code_verifier=");
            form.push_str(&urlencoding::encode(verifier));
        }
        let traw = http_post_form(client, token_uri, form).await?;
        let tv: serde_json::Value = serde_json::from_str(&traw)?;
        let id_token = tv
            .get("id_token")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow::anyhow!("token response missing id_token"))?;

        let header = jsonwebtoken::decode_header(id_token)?;
        let alg = header.alg;
        let mut validation = Validation::new(alg);
        validation.validate_exp = true;
        validation.set_audience(&[self.cfg.client_id.as_str()]);
        let data = match alg {
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                let key = self
                    .jwks
                    .decoding_key_for(header.kid.as_deref(), alg)
                    .await
                    .map_err(|e| anyhow::anyhow!("jwks: {e}"))?;
                decode::<serde_json::Value>(id_token, &key, &validation)
            }
            _ => anyhow::bail!("id_token algorithm not supported: {alg:?}"),
        }
        .map_err(|e| anyhow::anyhow!("id_token invalid: {e}"))?;
        let claims = data.claims;
        if let Some(expected_nonce) = pending.nonce.as_ref() {
            let got_nonce = claims
                .get("nonce")
                .and_then(|x| x.as_str())
                .unwrap_or_default();
            if got_nonce != expected_nonce {
                anyhow::bail!("id_token nonce mismatch");
            }
        }
        let sub = claims
            .get("sub")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow::anyhow!("id_token missing sub"))?
            .to_string();
        let roles = extract_roles(&claims, &self.cfg.roles_claim);
        if !self.cfg.required_roles.is_empty() {
            let mode = self.cfg.required_roles_mode.to_ascii_lowercase();
            let ok = if mode == "all" {
                self.cfg
                    .required_roles
                    .iter()
                    .all(|r| roles.iter().any(|x| x == r))
            } else {
                self.cfg
                    .required_roles
                    .iter()
                    .any(|r| roles.iter().any(|x| x == r))
            };
            if !ok {
                anyhow::bail!("missing required role");
            }
        }
        let signing = std::env::var(&self.cfg.signing_secret_env)
            .map_err(|_| anyhow::anyhow!("console session signing secret env not set"))?;
        if signing.len() < 16 {
            anyhow::bail!("signing secret too short (min 16 bytes)");
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as usize)
            .unwrap_or(0);
        let exp = now.saturating_add(self.cfg.session_ttl_seconds as usize);
        let session = ConsoleSessionJwt {
            sub,
            exp,
            iat: now,
            roles,
        };
        let token = encode(
            &JwtHeader::default(),
            &session,
            &EncodingKey::from_secret(signing.as_bytes()),
        )
        .map_err(|e| anyhow::anyhow!("session jwt: {e}"))?;

        let secure_cookie = self.cfg.force_secure_cookie
            || self
                .cfg
                .redirect_base_url
                .trim()
                .to_ascii_lowercase()
                .starts_with("https://");
        let cookie_val = format!(
            "{}={}; Path=/console; HttpOnly; SameSite=Lax; Max-Age={}{}",
            self.cfg.cookie_name.trim(),
            token,
            self.cfg.session_ttl_seconds,
            if secure_cookie { "; Secure" } else { "" },
        );
        let resp = Response::builder()
            .status(StatusCode::FOUND)
            .header(header::SET_COOKIE, HeaderValue::try_from(cookie_val)?)
            .header(header::LOCATION, HeaderValue::from_static("/console"))
            .body(
                Full::new(bytes::Bytes::from("ok"))
                    .map_err(|never: std::convert::Infallible| match never {})
                    .boxed_unsync(),
            )
            .map_err(|e| anyhow::anyhow!("response: {e}"))?;
        Ok(resp)
    }

    pub fn validate_session_cookie(&self, headers: &HeaderMap) -> bool {
        validate_session_cookie_for_cfg(self.cfg.as_ref(), headers)
    }

    /// Valid console session cookie, and if `required_roles` is non-empty, role check (`mode`: `any` or `all`).
    pub fn control_plane_session_authorized(
        &self,
        headers: &HeaderMap,
        required_roles: &[String],
        mode: &str,
    ) -> bool {
        let Some(jwt) = decode_console_session_jwt(self.cfg.as_ref(), headers) else {
            return false;
        };
        if required_roles.is_empty() {
            return true;
        }
        roles_match(&jwt.roles, required_roles, mode)
    }

    /// Resolve control-plane access for a console session: read/write vs read-only from role lists.
    ///
    /// - If `required_rw_roles` is non-empty and the session matches, returns read/write.
    /// - Else if `read_only_roles` is non-empty and the session matches, returns read-only.
    /// - Else if `required_rw_roles` is non-empty (and the session did not match), returns `None`.
    /// - Else (no RW requirement), a valid session yields read/write unless it matched read-only above.
    pub fn control_plane_oidc_access(
        &self,
        headers: &HeaderMap,
        required_rw_roles: &[String],
        rw_mode: &str,
        read_only_roles: &[String],
        read_only_mode: &str,
    ) -> Option<ControlPlaneOidcAccess> {
        let jwt = decode_console_session_jwt(self.cfg.as_ref(), headers)?;
        if !required_rw_roles.is_empty() && roles_match(&jwt.roles, required_rw_roles, rw_mode) {
            return Some(ControlPlaneOidcAccess::ReadWrite);
        }
        if !read_only_roles.is_empty() && roles_match(&jwt.roles, read_only_roles, read_only_mode) {
            return Some(ControlPlaneOidcAccess::ReadOnly);
        }
        if !required_rw_roles.is_empty() {
            return None;
        }
        Some(ControlPlaneOidcAccess::ReadWrite)
    }
}

/// Control-plane access tier implied by a console OIDC session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPlaneOidcAccess {
    ReadWrite,
    ReadOnly,
}

fn roles_match(jwt_roles: &[String], required: &[String], mode: &str) -> bool {
    if required.is_empty() {
        return false;
    }
    let m = mode.to_ascii_lowercase();
    if m == "all" {
        required
            .iter()
            .all(|r| jwt_roles.iter().any(|x| x == r))
    } else {
        required
            .iter()
            .any(|r| jwt_roles.iter().any(|x| x == r))
    }
}

fn decode_console_session_jwt(
    cfg: &ConsoleOidcConfig,
    headers: &HeaderMap,
) -> Option<ConsoleSessionJwt> {
    let cookie_hdr = headers.get(header::COOKIE).and_then(|v| v.to_str().ok())?;
    let name = format!("{}=", cfg.cookie_name.trim());
    for part in cookie_hdr.split(';') {
        let p = part.trim();
        if let Some(rest) = p.strip_prefix(&name) {
            let token = rest.trim();
            if token.is_empty() {
                return None;
            }
            let signing = std::env::var(&cfg.signing_secret_env).ok()?;
            if signing.len() < 16 {
                return None;
            }
            let mut v = Validation::new(Algorithm::HS256);
            v.validate_exp = true;
            let data = decode::<ConsoleSessionJwt>(
                token,
                &DecodingKey::from_secret(signing.as_bytes()),
                &v,
            )
            .ok()?;
            return Some(data.claims);
        }
    }
    None
}

fn validate_session_cookie_for_cfg(cfg: &ConsoleOidcConfig, headers: &HeaderMap) -> bool {
    decode_console_session_jwt(cfg, headers).is_some()
}

fn generate_pkce_verifier() -> String {
    // 64 chars from UUID hex is valid and within PKCE verifier size constraints.
    let a = uuid::Uuid::new_v4().as_simple().to_string();
    let b = uuid::Uuid::new_v4().as_simple().to_string();
    format!("{a}{b}")
}

fn pkce_s256_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_session_cookie_accepts_valid_cookie() {
        let secret_env = "PANDA_TEST_CONSOLE_SESSION_SECRET";
        unsafe {
            std::env::set_var(secret_env, "0123456789abcdef0123456789abcdef");
        }
        let cfg = ConsoleOidcConfig {
            cookie_name: "panda_console_session".to_string(),
            signing_secret_env: secret_env.to_string(),
            ..Default::default()
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as usize)
            .unwrap_or(0);
        let token = encode(
            &JwtHeader::default(),
            &ConsoleSessionJwt {
                sub: "u1".to_string(),
                exp: now + 60,
                iat: now,
                roles: vec!["admin".to_string()],
            },
            &EncodingKey::from_secret("0123456789abcdef0123456789abcdef".as_bytes()),
        )
        .expect("session token");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_str(&format!("panda_console_session={token}")).expect("cookie"),
        );
        assert!(validate_session_cookie_for_cfg(&cfg, &headers));
        unsafe {
            std::env::remove_var(secret_env);
        }
    }

    #[test]
    fn roles_match_any_and_all() {
        let jwt = vec!["admin".to_string(), "viewer".to_string()];
        assert!(roles_match(&jwt, &["admin".to_string()], "any"));
        assert!(roles_match(&jwt, &["admin".to_string()], "all"));
        assert!(!roles_match(
            &jwt,
            &["admin".to_string(), "missing".to_string()],
            "all"
        ));
        assert!(roles_match(
            &jwt,
            &["admin".to_string(), "viewer".to_string()],
            "all"
        ));
    }

    #[test]
    fn validate_session_cookie_rejects_bad_cookie() {
        let secret_env = "PANDA_TEST_CONSOLE_SESSION_SECRET_BAD";
        unsafe {
            std::env::set_var(secret_env, "0123456789abcdef0123456789abcdef");
        }
        let cfg = ConsoleOidcConfig {
            cookie_name: "panda_console_session".to_string(),
            signing_secret_env: secret_env.to_string(),
            ..Default::default()
        };
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("panda_console_session=bad"),
        );
        assert!(!validate_session_cookie_for_cfg(&cfg, &headers));
        unsafe {
            std::env::remove_var(secret_env);
        }
    }
}

fn extract_roles(claims: &serde_json::Value, claim_name: &str) -> Vec<String> {
    if claim_name.trim().is_empty() {
        return vec![];
    }
    let Some(v) = claims.get(claim_name) else {
        return vec![];
    };
    match v {
        serde_json::Value::String(s) => s.split_whitespace().map(|x| x.to_string()).collect(),
        serde_json::Value::Array(a) => a
            .iter()
            .filter_map(|x| x.as_str().map(|s| s.to_string()))
            .collect(),
        _ => vec![],
    }
}

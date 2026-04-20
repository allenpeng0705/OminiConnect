//! JWKS fetch + cache for RSA JWT verification (`RS256` / `RS384` / `RS512`).

use std::collections::HashMap;
use std::time::{Duration, Instant};

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::Request;
use jsonwebtoken::{Algorithm, DecodingKey};
use serde::Deserialize;

use crate::HttpClient;

const JWKS_FETCH_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Deserialize)]
struct JwksDoc {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kty: String,
    kid: Option<String>,
    n: Option<String>,
    e: Option<String>,
}

pub struct JwksResolver {
    url: String,
    cache_ttl: Duration,
    client: HttpClient,
    cache: tokio::sync::RwLock<JwksCache>,
}

struct JwksCache {
    /// `kid` from JWKS → RSA (n, e) base64url components.
    rsa_by_kid: HashMap<String, (String, String)>,
    fetched_at: Instant,
}

impl JwksResolver {
    pub fn new(client: HttpClient, url: String, cache_ttl: Duration) -> Self {
        Self {
            url,
            cache_ttl,
            client,
            cache: tokio::sync::RwLock::new(JwksCache {
                rsa_by_kid: HashMap::new(),
                fetched_at: Instant::now() - Duration::from_secs(86400),
            }),
        }
    }

    /// Returns a decoding key for the JWT header's `kid` and algorithm.
    pub async fn decoding_key_for(
        &self,
        kid: Option<&str>,
        alg: Algorithm,
    ) -> Result<DecodingKey, &'static str> {
        if !matches!(alg, Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512) {
            return Err("unauthorized: jwks resolver supports RSA algorithms only");
        }
        self.ensure_fresh(kid).await?;
        let guard = self.cache.read().await;
        let (n, e) = select_rsa_components(&guard.rsa_by_kid, kid)?;
        DecodingKey::from_rsa_components(n, e).map_err(|_| "unauthorized: invalid jwks rsa key")
    }
}

fn select_rsa_components<'a>(
    rsa_by_kid: &'a HashMap<String, (String, String)>,
    kid: Option<&str>,
) -> Result<(&'a str, &'a str), &'static str> {
    match kid {
        Some(k) if !k.trim().is_empty() => rsa_by_kid
            .get(k)
            .map(|(n, e)| (n.as_str(), e.as_str()))
            .ok_or("unauthorized: jwks kid not found"),
        _ => {
            if rsa_by_kid.len() == 1 {
                rsa_by_kid
                    .values()
                    .next()
                    .map(|(n, e)| (n.as_str(), e.as_str()))
                    .ok_or("unauthorized: jwks empty")
            } else {
                Err("unauthorized: jwt kid required")
            }
        }
    }
}

impl JwksResolver {
    async fn ensure_fresh(&self, kid_hint: Option<&str>) -> Result<(), &'static str> {
        let need_fetch = {
            let g = self.cache.read().await;
            let stale = g.fetched_at.elapsed() > self.cache_ttl;
            let kid_miss = kid_hint
                .filter(|k| !k.is_empty())
                .map(|k| !g.rsa_by_kid.contains_key(k))
                .unwrap_or(false);
            stale || kid_miss
        };
        if !need_fetch {
            return Ok(());
        }
        let mut g = self.cache.write().await;
        let stale = g.fetched_at.elapsed() > self.cache_ttl;
        let kid_miss = kid_hint
            .filter(|k| !k.is_empty())
            .map(|k| !g.rsa_by_kid.contains_key(k))
            .unwrap_or(false);
        if !stale && !kid_miss {
            return Ok(());
        }
        self.fetch_and_store(&mut g).await
    }

    async fn fetch_and_store(&self, cache: &mut JwksCache) -> Result<(), &'static str> {
        let uri: hyper::Uri = self
            .url
            .parse()
            .map_err(|_| "unauthorized: invalid jwks url")?;
        let body = Full::new(Bytes::new())
            .map_err(|never: std::convert::Infallible| match never {})
            .boxed_unsync();
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(body)
            .map_err(|_| "unauthorized: jwks request build failed")?;
        let fut = self.client.request(req);
        let resp = tokio::time::timeout(JWKS_FETCH_TIMEOUT, fut)
            .await
            .map_err(|_| "unauthorized: jwks fetch timeout")?
            .map_err(|_| "unauthorized: jwks fetch failed")?;
        if !resp.status().is_success() {
            return Err("unauthorized: jwks fetch bad status");
        }
        let body = resp
            .into_body()
            .collect()
            .await
            .map_err(|_| "unauthorized: jwks read body")?;
        let bytes = body.to_bytes();
        let doc: JwksDoc =
            serde_json::from_slice(&bytes).map_err(|_| "unauthorized: jwks json invalid")?;
        let mut rsa_by_kid = HashMap::new();
        for jwk in doc.keys {
            if jwk.kty != "RSA" {
                continue;
            }
            let (Some(n), Some(e)) = (jwk.n, jwk.e) else {
                continue;
            };
            let kid = jwk.kid.unwrap_or_default();
            rsa_by_kid.insert(kid, (n, e));
        }
        if rsa_by_kid.is_empty() {
            return Err("unauthorized: jwks has no usable rsa keys");
        }
        cache.rsa_by_kid = rsa_by_kid;
        cache.fetched_at = Instant::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_static_jwks_rsa_keys() {
        let json = r#"{"keys":[{"kty":"RSA","kid":"a","n":"abc","e":"AQAB"}]}"#;
        let doc: JwksDoc = serde_json::from_str(json).unwrap();
        assert_eq!(doc.keys.len(), 1);
        assert_eq!(doc.keys[0].kid.as_deref(), Some("a"));
    }

    #[test]
    fn select_single_rsa_without_kid() {
        let mut m = HashMap::new();
        m.insert("kid1".to_string(), ("n".into(), "e".into()));
        let (n, e) = select_rsa_components(&m, None).unwrap();
        assert_eq!(n, "n");
        assert_eq!(e, "e");
    }

    #[test]
    fn select_multi_requires_kid() {
        let mut m = HashMap::new();
        m.insert("a".into(), ("n1".into(), "e1".into()));
        m.insert("b".into(), ("n2".into(), "e2".into()));
        assert!(select_rsa_components(&m, None).is_err());
        assert!(select_rsa_components(&m, Some("a")).is_ok());
    }
}

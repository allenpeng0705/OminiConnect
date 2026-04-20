//! Kong (or any edge) handshake: verify attestation, extract identity, strip spoofed headers.

use anyhow::Context;
use constant_time_eq::constant_time_eq;
use hmac::{Hmac, Mac};
use http::header::{HeaderMap, HeaderName, HeaderValue};
use panda_config::TrustedGatewayConfig;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Parsed identity for TPM keys, audit, and future JWT validation (Phase 3).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RequestContext {
    pub subject: Option<String>,
    pub tenant: Option<String>,
    pub scopes: Vec<String>,
    /// From JWT when `budget_hierarchy` is enabled (`budget_hierarchy.jwt_claim`).
    pub department: Option<String>,
    pub trusted_hop: bool,
    pub correlation_id: String,
    /// When `agent_sessions` is enabled, value from the configured request header (trimmed).
    pub agent_session: Option<String>,
    /// When `agent_sessions` is enabled, planner / agent profile from JWT claim and/or profile header (trimmed).
    pub agent_profile: Option<String>,
}

/// Shared secret set by the edge (Kong plugin / mesh); must not appear in YAML.
pub fn trusted_gateway_secret_from_env() -> Option<String> {
    std::env::var("PANDA_TRUSTED_GATEWAY_SECRET")
        .ok()
        .filter(|s| !s.trim().is_empty())
}

/// Compare two strings in time independent of *position of first byte difference*, using
/// `HMAC-SHA256(key, ·)` digests (`key` should be the server secret bytes).
pub fn attestation_equals(got: &str, expected: &str, key: &[u8]) -> bool {
    if got.len() > 16 * 1024 || expected.len() > 16 * 1024 {
        return false;
    }
    let Ok(mut m1) = HmacSha256::new_from_slice(key) else {
        return false;
    };
    m1.update(got.as_bytes());
    let a = m1.finalize().into_bytes();

    let Ok(mut m2) = HmacSha256::new_from_slice(key) else {
        return false;
    };
    m2.update(expected.as_bytes());
    let b = m2.finalize().into_bytes();

    constant_time_eq(&a[..], &b[..])
}

fn header_name(s: &str) -> HeaderName {
    HeaderName::from_bytes(s.as_bytes()).expect("header validated in panda-config")
}

/// W3C trace id (32 hex chars) from `traceparent`.
pub fn trace_id_from_traceparent(tp: &str) -> Option<String> {
    let parts: Vec<&str> = tp.split('-').collect();
    if parts.len() >= 2 && parts[1].len() == 32 && parts[1].chars().all(|c| c.is_ascii_hexdigit()) {
        Some(parts[1].to_string())
    } else {
        None
    }
}

/// Ensure an upstream-bound correlation id: reuse client header, else trace id, else UUID.
pub fn ensure_correlation_id(headers: &mut HeaderMap, header_name: &str) -> anyhow::Result<String> {
    let hn = HeaderName::from_bytes(header_name.as_bytes())
        .map_err(|_| anyhow::anyhow!("invalid correlation header name"))?;

    if let Some(v) = headers.get(&hn).and_then(|v| v.to_str().ok()) {
        let t = v.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }

    if let Some(tp) = headers
        .get("traceparent")
        .or_else(|| headers.get("Traceparent"))
        .and_then(|v| v.to_str().ok())
    {
        if let Some(id) = trace_id_from_traceparent(tp) {
            headers.insert(
                hn.clone(),
                HeaderValue::from_str(&id).context("trace id as header value")?,
            );
            return Ok(id);
        }
    }

    let id = uuid::Uuid::new_v4().to_string();
    headers.insert(
        hn,
        HeaderValue::from_str(&id).context("uuid as header value")?,
    );
    Ok(id)
}

/// Apply after generic hop-by-hop filtering.
pub fn apply_trusted_gateway(
    headers: &mut HeaderMap,
    cfg: &TrustedGatewayConfig,
    secret: Option<&str>,
) -> RequestContext {
    let att_name = cfg.attestation_header.as_deref().map(header_name);
    let sub_name = cfg.subject_header.as_deref().map(header_name);
    let ten_name = cfg.tenant_header.as_deref().map(header_name);
    let sco_name = cfg.scopes_header.as_deref().map(header_name);

    let trusted = match (&att_name, secret) {
        (Some(att), Some(sec)) => headers
            .get(att)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|got| attestation_equals(got, sec, sec.as_bytes())),
        _ => false,
    };

    let mut ctx = RequestContext {
        trusted_hop: trusted,
        ..Default::default()
    };

    if trusted {
        if let Some(ref h) = sub_name {
            ctx.subject = headers
                .get(h)
                .and_then(|v| v.to_str().ok())
                .map(str::to_string);
        }
        if let Some(ref h) = ten_name {
            ctx.tenant = headers
                .get(h)
                .and_then(|v| v.to_str().ok())
                .map(str::to_string);
        }
        if let Some(ref h) = sco_name {
            if let Some(raw) = headers.get(h).and_then(|v| v.to_str().ok()) {
                ctx.scopes = parse_scopes(raw);
            }
        }
    }

    if let Some(att) = att_name {
        headers.remove(att);
    }

    if !trusted {
        for h in [sub_name, ten_name, sco_name].into_iter().flatten() {
            headers.remove(h);
        }
    }

    ctx
}

fn parse_scopes(raw: &str) -> Vec<String> {
    raw.split(|c| c == ',' || c == ' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_trusted_cfg() -> TrustedGatewayConfig {
        TrustedGatewayConfig {
            attestation_header: Some("X-Panda-Internal".to_string()),
            subject_header: Some("X-User-Id".to_string()),
            tenant_header: Some("X-Tenant-Id".to_string()),
            scopes_header: Some("X-User-Scopes".to_string()),
        }
    }

    #[test]
    fn attestation_digest_matches_when_equal() {
        let sec = "s3cret";
        assert!(attestation_equals(sec, sec, sec.as_bytes()));
        assert!(!attestation_equals("wrong", sec, sec.as_bytes()));
    }

    #[test]
    fn traceparent_parse() {
        assert_eq!(
            trace_id_from_traceparent("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
                .as_deref(),
            Some("4bf92f3577b34da6a3ce929d0e0e4736")
        );
    }

    #[test]
    fn untrusted_strips_identity_headers() {
        let mut h = HeaderMap::new();
        h.insert(
            http::HeaderName::from_static("x-user-id"),
            http::HeaderValue::from_static("eve"),
        );
        let ctx = apply_trusted_gateway(&mut h, &sample_trusted_cfg(), None);
        assert!(!ctx.trusted_hop);
        assert!(h.get("X-User-Id").is_none());
    }

    #[test]
    fn trusted_keeps_identity_removes_attestation() {
        let mut h = HeaderMap::new();
        h.insert(
            http::HeaderName::from_static("x-panda-internal"),
            http::HeaderValue::from_static("s3cret"),
        );
        h.insert(
            http::HeaderName::from_static("x-user-id"),
            http::HeaderValue::from_static("alice"),
        );
        h.insert(
            http::HeaderName::from_static("x-user-scopes"),
            http::HeaderValue::from_static("read, write"),
        );
        let ctx = apply_trusted_gateway(&mut h, &sample_trusted_cfg(), Some("s3cret"));
        assert!(ctx.trusted_hop);
        assert_eq!(ctx.subject.as_deref(), Some("alice"));
        assert_eq!(ctx.scopes, vec!["read", "write"]);
        assert!(h.get("X-Panda-Internal").is_none());
        assert!(h.get("X-User-Id").is_some());
    }

    #[test]
    fn wrong_secret_untrusted() {
        let mut h = HeaderMap::new();
        h.insert(
            http::HeaderName::from_static("x-panda-internal"),
            http::HeaderValue::from_static("wrong"),
        );
        h.insert(
            http::HeaderName::from_static("x-user-id"),
            http::HeaderValue::from_static("alice"),
        );
        let ctx = apply_trusted_gateway(&mut h, &sample_trusted_cfg(), Some("s3cret"));
        assert!(!ctx.trusted_hop);
        assert!(h.get("X-User-Id").is_none());
    }

    #[test]
    fn untrusted_strips_tenant_and_scopes() {
        let mut h = HeaderMap::new();
        h.insert(
            http::HeaderName::from_static("x-user-id"),
            http::HeaderValue::from_static("eve"),
        );
        h.insert(
            http::HeaderName::from_static("x-tenant-id"),
            http::HeaderValue::from_static("t1"),
        );
        h.insert(
            http::HeaderName::from_static("x-user-scopes"),
            http::HeaderValue::from_static("admin"),
        );
        let ctx = apply_trusted_gateway(&mut h, &sample_trusted_cfg(), Some("s3cret"));
        assert!(!ctx.trusted_hop);
        assert!(h.get("X-User-Id").is_none());
        assert!(h.get("X-Tenant-Id").is_none());
        assert!(h.get("X-User-Scopes").is_none());
        assert!(ctx.subject.is_none());
        assert!(ctx.tenant.is_none());
        assert!(ctx.scopes.is_empty());
    }

    #[test]
    fn attestation_rejects_extremely_long_values() {
        let sec = "s3cret";
        let long = "a".repeat(20_000);
        assert!(!attestation_equals(&long, sec, sec.as_bytes()));
        assert!(!attestation_equals(sec, &long, sec.as_bytes()));
    }

    #[test]
    fn ensure_correlation_id_prefers_existing_header() {
        let mut h = HeaderMap::new();
        h.insert(
            HeaderName::from_static("x-request-id"),
            HeaderValue::from_static("corr-existing"),
        );
        let id = ensure_correlation_id(&mut h, "x-request-id").unwrap();
        assert_eq!(id, "corr-existing");
    }

    #[test]
    fn ensure_correlation_id_derives_from_traceparent() {
        let mut h = HeaderMap::new();
        h.insert(
            HeaderName::from_static("traceparent"),
            HeaderValue::from_static("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01"),
        );
        let id = ensure_correlation_id(&mut h, "x-request-id").unwrap();
        assert_eq!(id, "4bf92f3577b34da6a3ce929d0e0e4736");
        assert_eq!(
            h.get("x-request-id").and_then(|v| v.to_str().ok()),
            Some("4bf92f3577b34da6a3ce929d0e0e4736")
        );
    }

    #[test]
    fn ensure_correlation_id_generates_uuid_when_missing() {
        let mut h = HeaderMap::new();
        let id = ensure_correlation_id(&mut h, "x-request-id").unwrap();
        assert_eq!(id.len(), 36);
        assert!(id.chars().filter(|c| *c == '-').count() == 4);
        assert!(h.get("x-request-id").is_some());
    }
}

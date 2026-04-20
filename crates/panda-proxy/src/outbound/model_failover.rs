//! Model parity / upstream failover (Enterprise): capability-aware chains and optional Anthropic hops.

use http::header::{self, HeaderMap, HeaderName, HeaderValue};
use http_body_util::{BodyExt, Full, Limited};
use hyper::body::Incoming;
use hyper::Request;
use panda_config::{
    ModelFailoverBackend, ModelFailoverConfig, ModelFailoverOperation, ModelFailoverProtocol,
};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::adapter;
use super::upstream;
use crate::BoxBody;
use crate::HttpClient;
use crate::ProxyError;

/// Logical API surface for parity / failover (ingress is usually OpenAI-shaped).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailoverApiOperation {
    ChatCompletions,
    Embeddings,
    ResponsesApi,
    ImagesApi,
    AudioApi,
}

#[derive(Debug, Clone, Default)]
pub struct FailoverRequestFeatures {
    pub streaming: bool,
    pub tools: bool,
}

#[derive(Debug, Clone)]
pub struct ClassifiedFailoverRequest {
    pub operation: FailoverApiOperation,
    pub features: FailoverRequestFeatures,
}

/// Snapshot of the failover chain after a winning upstream is chosen (for mid-stream SSE replay).
#[derive(Debug, Clone)]
pub struct FailoverStreamContext {
    pub chain: Vec<ModelFailoverBackend>,
    pub classified: ClassifiedFailoverRequest,
    pub winner_index: usize,
}

async fn collect_incoming_limited(
    body: Incoming,
    max_bytes: usize,
) -> Result<Vec<u8>, (bool, ProxyError)> {
    let limited = Limited::new(body, max_bytes);
    match limited.collect().await {
        Ok(collected) => Ok(collected.to_bytes().to_vec()),
        Err(e) => {
            if e.downcast_ref::<http_body_util::LengthLimitError>()
                .is_some()
            {
                let pe = ProxyError::Upstream(anyhow::anyhow!(
                    "midstream SSE response exceeded model_failover.midstream_sse_max_buffer_bytes"
                ));
                return Err((true, pe));
            }
            Err((
                false,
                ProxyError::Upstream(anyhow::anyhow!("midstream SSE body read: {e}")),
            ))
        }
    }
}

/// Buffer a successful streaming OpenAI-style SSE body (up to `max_bytes`). If the winning upstream
/// fails mid-body, retry later backends in `ctx.chain` with the same logical request.
///
/// Returns `(bytes, midstream_retry_count)` where `midstream_retry_count` is how many **additional**
/// upstream attempts were needed after the winner (for metrics).
pub async fn collect_openai_sse_with_midstream_failover(
    client: &HttpClient,
    failover_cfg: &ModelFailoverConfig,
    ctx: &FailoverStreamContext,
    upstream_template: &hyper::http::request::Parts,
    openai_request_body: &[u8],
    anthropic_version: &str,
    winner_body: Incoming,
    max_bytes: usize,
    upstream_timeout: Duration,
) -> Result<(Vec<u8>, u32), ProxyError> {
    let mut retries: u32 = 0;
    match collect_incoming_limited(winner_body, max_bytes).await {
        Ok(b) => return Ok((b, retries)),
        Err((limit, e)) => {
            if limit {
                return Err(e);
            }
            record_circuit_retryable_failure(failover_cfg, &ctx.chain[ctx.winner_index]);
        }
    }

    let n = ctx.chain.len();
    let mut last_err = ProxyError::Upstream(anyhow::anyhow!(
        "model_failover: mid-stream SSE collect failed on winner and no fallback error preserved"
    ));

    for j in (ctx.winner_index + 1)..n {
        let backend = &ctx.chain[j];
        if !circuit_allows_attempt(failover_cfg, backend) {
            continue;
        }
        retries = retries.saturating_add(1);
        let mut p_try = upstream_template.clone();
        let mut h_try = p_try.headers.clone();
        if apply_backend_auth(&mut h_try, backend).is_err() {
            continue;
        }
        p_try.headers = h_try;
        let hop_body = match prepare_failover_hop(
            backend,
            &ctx.classified,
            &mut p_try,
            openai_request_body,
            anthropic_version,
        ) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let body_stream = Full::new(bytes::Bytes::from(hop_body))
            .map_err(|never: std::convert::Infallible| match never {})
            .boxed_unsync();
        let req_try = match build_upstream_request(&p_try, body_stream, backend.backend_base.trim()) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let resp = match crate::request_upstream_with_timeout(
            client,
            req_try,
            upstream_timeout,
            "failover_midstream",
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                last_err = e;
                record_circuit_retryable_failure(failover_cfg, backend);
                continue;
            }
        };
        let st = resp.status();
        if should_retry_failover(st) {
            record_circuit_retryable_failure(failover_cfg, backend);
            let (_, drain) = resp.into_parts();
            let _ = crate::collect_body_bounded(drain, max_bytes.max(256 * 1024)).await;
            continue;
        }
        if !st.is_success() {
            last_err = ProxyError::Upstream(anyhow::anyhow!(
                "model_failover midstream retry: upstream HTTP {}",
                st.as_u16()
            ));
            let (_, drain) = resp.into_parts();
            let _ = crate::collect_body_bounded(drain, max_bytes.max(256 * 1024)).await;
            continue;
        }
        record_circuit_success(failover_cfg, backend);
        let (_, rb) = resp.into_parts();
        match collect_incoming_limited(rb, max_bytes).await {
            Ok(b) => return Ok((b, retries)),
            Err((limit, e)) => {
                if limit {
                    return Err(e);
                }
                last_err = e;
                record_circuit_retryable_failure(failover_cfg, backend);
            }
        }
    }

    Err(last_err)
}

fn parse_chat_features(body: Option<&[u8]>) -> FailoverRequestFeatures {
    let Some(b) = body else {
        return FailoverRequestFeatures::default();
    };
    let Ok(v) = serde_json::from_slice::<serde_json::Value>(b) else {
        return FailoverRequestFeatures::default();
    };
    let streaming = v.get("stream").and_then(|x| x.as_bool()).unwrap_or(false);
    let tools = v
        .get("tools")
        .map(|t| !t.is_null() && t.as_array().map(|a| !a.is_empty()).unwrap_or(!t.is_null()))
        .unwrap_or(false)
        || v.get("tool_choice").map(|x| !x.is_null()).unwrap_or(false);
    FailoverRequestFeatures { streaming, tools }
}

/// Classify request for failover using configured path prefixes.
pub fn classify_failover_request(
    cfg: &ModelFailoverConfig,
    path: &str,
    method: &hyper::Method,
    body: Option<&[u8]>,
) -> Option<ClassifiedFailoverRequest> {
    if !cfg.enabled || method != &hyper::Method::POST {
        return None;
    }
    let chat_p = cfg.path_prefix.trim_end_matches('/');
    if !chat_p.is_empty() && path.starts_with(chat_p) {
        return Some(ClassifiedFailoverRequest {
            operation: FailoverApiOperation::ChatCompletions,
            features: parse_chat_features(body),
        });
    }
    if let Some(ref ep) = cfg.embeddings_path_prefix {
        let p = ep.trim().trim_end_matches('/');
        if !p.is_empty() && path.starts_with(p) {
            return Some(ClassifiedFailoverRequest {
                operation: FailoverApiOperation::Embeddings,
                features: FailoverRequestFeatures::default(),
            });
        }
    }
    if let Some(ref rp) = cfg.responses_path_prefix {
        let p = rp.trim().trim_end_matches('/');
        if !p.is_empty() && path.starts_with(p) {
            let tools = body
                .and_then(|b| serde_json::from_slice::<serde_json::Value>(b).ok())
                .map(|v| v.get("tools").is_some() || v.get("tool_choice").is_some())
                .unwrap_or(false);
            return Some(ClassifiedFailoverRequest {
                operation: FailoverApiOperation::ResponsesApi,
                features: FailoverRequestFeatures {
                    streaming: false,
                    tools,
                },
            });
        }
    }
    if let Some(ref ip) = cfg.images_path_prefix {
        let p = ip.trim().trim_end_matches('/');
        if !p.is_empty() && path.starts_with(p) {
            return Some(ClassifiedFailoverRequest {
                operation: FailoverApiOperation::ImagesApi,
                features: FailoverRequestFeatures::default(),
            });
        }
    }
    if let Some(ref ap) = cfg.audio_path_prefix {
        let p = ap.trim().trim_end_matches('/');
        if !p.is_empty() && path.starts_with(p) {
            return Some(ClassifiedFailoverRequest {
                operation: FailoverApiOperation::AudioApi,
                features: FailoverRequestFeatures::default(),
            });
        }
    }
    None
}

fn extract_model_for_group_match(
    operation: &FailoverApiOperation,
    body: Option<&[u8]>,
) -> Option<String> {
    let b = body?;
    let v: serde_json::Value = serde_json::from_slice(b).ok()?;
    match operation {
        FailoverApiOperation::ChatCompletions
        | FailoverApiOperation::ResponsesApi
        | FailoverApiOperation::Embeddings
        | FailoverApiOperation::ImagesApi
        | FailoverApiOperation::AudioApi => v
            .get("model")
            .and_then(|m| m.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
    }
}

fn config_op(op: &FailoverApiOperation) -> ModelFailoverOperation {
    match op {
        FailoverApiOperation::ChatCompletions => ModelFailoverOperation::ChatCompletions,
        FailoverApiOperation::Embeddings => ModelFailoverOperation::Embeddings,
        FailoverApiOperation::ResponsesApi => ModelFailoverOperation::Responses,
        FailoverApiOperation::ImagesApi => ModelFailoverOperation::Images,
        FailoverApiOperation::AudioApi => ModelFailoverOperation::Audio,
    }
}

fn default_operations_for_protocol(p: ModelFailoverProtocol) -> &'static [ModelFailoverOperation] {
    match p {
        ModelFailoverProtocol::OpenaiCompatible => {
            const ALL: [ModelFailoverOperation; 5] = [
                ModelFailoverOperation::ChatCompletions,
                ModelFailoverOperation::Embeddings,
                ModelFailoverOperation::Responses,
                ModelFailoverOperation::Images,
                ModelFailoverOperation::Audio,
            ];
            &ALL
        }
        ModelFailoverProtocol::Anthropic => {
            const CHAT: [ModelFailoverOperation; 1] = [ModelFailoverOperation::ChatCompletions];
            &CHAT
        }
    }
}

/// True if this backend may serve the classified request (protocol × operation × features).
pub fn backend_eligible(
    backend: &ModelFailoverBackend,
    classified: &ClassifiedFailoverRequest,
) -> bool {
    let want = config_op(&classified.operation);
    let supported: &[ModelFailoverOperation] = if backend.supports.is_empty() {
        default_operations_for_protocol(backend.protocol)
    } else {
        &backend.supports
    };
    if !supported.contains(&want) {
        return false;
    }
    if backend.protocol == ModelFailoverProtocol::Anthropic {
        if classified.operation != FailoverApiOperation::ChatCompletions {
            return false;
        }
        if classified.features.tools {
            // Tool payloads are mapped by the adapter for Anthropic hops.
        }
    }
    if matches!(classified.operation, FailoverApiOperation::ChatCompletions) {
        if let Some(false) = backend.supports_streaming {
            if classified.features.streaming {
                return false;
            }
        }
    }
    true
}

pub fn filter_eligible_backends(
    backends: Vec<ModelFailoverBackend>,
    classified: &ClassifiedFailoverRequest,
) -> Vec<ModelFailoverBackend> {
    backends
        .into_iter()
        .filter(|b| backend_eligible(b, classified))
        .collect()
}

/// Resolve ordered failover chain and classification. `None` if failover does not apply or no eligible backend.
///
/// **Streaming:** The gateway retries on retryable HTTP statuses before returning headers to the client. When
/// `ModelFailoverConfig::allow_failover_after_first_byte` is enabled, successful OpenAI-style **SSE** chat responses may be
/// fully buffered (see [`collect_openai_sse_with_midstream_failover`]) so a later backend can be tried if the winning
/// connection fails mid-body (TTFT increases; Anthropic adapter streaming excluded at the gateway).
pub fn resolve_failover_chain(
    cfg: &ModelFailoverConfig,
    ingress_path: &str,
    method: &hyper::Method,
    body: Option<&[u8]>,
) -> Option<(Vec<ModelFailoverBackend>, ClassifiedFailoverRequest)> {
    let classified = classify_failover_request(cfg, ingress_path, method, body)?;
    let model = extract_model_for_group_match(&classified.operation, body);
    for g in &cfg.groups {
        let matches = g.match_models.is_empty()
            || model
                .as_ref()
                .is_some_and(|m| g.match_models.iter().any(|x| x == m));
        if matches {
            let filtered = filter_eligible_backends(g.backends.clone(), &classified);
            if filtered.is_empty() {
                return None;
            }
            return Some((filtered, classified));
        }
    }
    None
}

pub fn apply_backend_auth(
    headers: &mut HeaderMap,
    backend: &ModelFailoverBackend,
) -> Result<(), &'static str> {
    if let Some(ref env_name) = backend.api_key_env {
        let key = std::env::var(env_name).map_err(|_| "model_failover: api key env not set")?;
        if key.trim().is_empty() {
            return Err("model_failover: api key empty");
        }
        if backend.use_api_key_header {
            let hv =
                HeaderValue::try_from(key.trim()).map_err(|_| "model_failover: api-key header")?;
            headers.insert(HeaderName::from_static("api-key"), hv);
            headers.remove(header::AUTHORIZATION);
        } else {
            let bearer = format!("Bearer {}", key.trim());
            let hv = HeaderValue::try_from(bearer)
                .map_err(|_| "model_failover: Authorization header")?;
            headers.insert(header::AUTHORIZATION, hv);
        }
    }
    Ok(())
}

/// Map OpenAI chat JSON to Anthropic Messages for this hop; otherwise pass body through unchanged.
pub fn prepare_failover_hop(
    backend: &ModelFailoverBackend,
    classified: &ClassifiedFailoverRequest,
    parts: &mut hyper::http::request::Parts,
    body_bytes: &[u8],
    anthropic_version: &str,
) -> anyhow::Result<Vec<u8>> {
    if backend.protocol != ModelFailoverProtocol::Anthropic {
        return Ok(body_bytes.to_vec());
    }
    if classified.operation != FailoverApiOperation::ChatCompletions {
        anyhow::bail!("model_failover: anthropic hop only supports chat completions");
    }
    let (mapped, _st) = adapter::openai_chat_to_anthropic(body_bytes)?;
    parts.uri = upstream::rewrite_uri_path_preserving_query(&parts.uri, "/v1/messages")?;
    let hv = HeaderValue::try_from(anthropic_version)
        .map_err(|_| anyhow::anyhow!("anthropic-version header value"))?;
    parts
        .headers
        .insert(HeaderName::from_static("anthropic-version"), hv);
    Ok(mapped)
}

pub fn build_upstream_request(
    parts: &hyper::http::request::Parts,
    body: BoxBody,
    upstream_base: &str,
) -> anyhow::Result<Request<BoxBody>> {
    let suffix = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let joined = format!("{}{}", upstream_base.trim_end_matches('/'), suffix);
    let uri: hyper::Uri = joined
        .parse()
        .map_err(|e| anyhow::anyhow!("model_failover join uri: {e}"))?;
    let mut p = parts.clone();
    p.uri = uri;
    Ok(Request::from_parts(p, body))
}

pub fn should_retry_failover(status: hyper::StatusCode) -> bool {
    status.is_server_error()
        || status == hyper::StatusCode::TOO_MANY_REQUESTS
        || status == hyper::StatusCode::BAD_GATEWAY
        || status == hyper::StatusCode::SERVICE_UNAVAILABLE
        || status == hyper::StatusCode::GATEWAY_TIMEOUT
}

#[derive(Debug, Clone, Copy, Default)]
struct CircuitState {
    consecutive_failures: u32,
    open_until_epoch_ms: u128,
}

fn now_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn circuits() -> &'static Mutex<HashMap<String, CircuitState>> {
    static CIRCUITS: OnceLock<Mutex<HashMap<String, CircuitState>>> = OnceLock::new();
    CIRCUITS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn circuit_allows_attempt(cfg: &ModelFailoverConfig, backend: &ModelFailoverBackend) -> bool {
    if !cfg.circuit_breaker_enabled {
        return true;
    }
    let Ok(g) = circuits().lock() else {
        return true;
    };
    let Some(st) = g.get(&backend.backend_base) else {
        return true;
    };
    st.open_until_epoch_ms <= now_epoch_ms()
}

pub fn record_circuit_success(cfg: &ModelFailoverConfig, backend: &ModelFailoverBackend) {
    if !cfg.circuit_breaker_enabled {
        return;
    }
    if let Ok(mut g) = circuits().lock() {
        g.insert(
            backend.backend_base.clone(),
            CircuitState {
                consecutive_failures: 0,
                open_until_epoch_ms: 0,
            },
        );
    }
}

pub fn record_circuit_retryable_failure(cfg: &ModelFailoverConfig, backend: &ModelFailoverBackend) {
    if !cfg.circuit_breaker_enabled {
        return;
    }
    if let Ok(mut g) = circuits().lock() {
        let st = g.entry(backend.backend_base.clone()).or_default();
        st.consecutive_failures = st.consecutive_failures.saturating_add(1);
        if st.consecutive_failures >= cfg.circuit_breaker_failure_threshold {
            st.open_until_epoch_ms = now_epoch_ms()
                .saturating_add((cfg.circuit_breaker_open_seconds as u128).saturating_mul(1000));
            st.consecutive_failures = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_failover_chain_matches_model_and_prefix() {
        let cfg = ModelFailoverConfig {
            enabled: true,
            path_prefix: "/v1/chat".to_string(),
            embeddings_path_prefix: None,
            responses_path_prefix: None,
            images_path_prefix: None,
            audio_path_prefix: None,
            allow_failover_after_first_byte: false,
            midstream_sse_max_buffer_bytes: 16 * 1024 * 1024,
            circuit_breaker_enabled: false,
            circuit_breaker_failure_threshold: 3,
            circuit_breaker_open_seconds: 30,
            groups: vec![
                panda_config::ModelFailoverGroup {
                    match_models: vec!["gpt-4o-mini".to_string()],
                    backends: vec![ModelFailoverBackend {
                backend_base: "https://a.example".to_string(),
                        api_key_env: None,
                        use_api_key_header: false,
                        protocol: ModelFailoverProtocol::OpenaiCompatible,
                        supports: vec![],
                        supports_streaming: None,
                    }],
                },
                panda_config::ModelFailoverGroup {
                    match_models: vec![],
                    backends: vec![ModelFailoverBackend {
                backend_base: "https://b.example".to_string(),
                        api_key_env: None,
                        use_api_key_header: false,
                        protocol: ModelFailoverProtocol::OpenaiCompatible,
                        supports: vec![],
                        supports_streaming: None,
                    }],
                },
            ],
        };
        let body = br#"{"model":"gpt-4o-mini","messages":[]}"#;
        let (r, _) = resolve_failover_chain(
            &cfg,
            "/v1/chat/completions",
            &hyper::Method::POST,
            Some(body),
        )
        .expect("expected chain");
        assert_eq!(r[0].backend_base, "https://a.example");
    }

    #[test]
    fn anthropic_backend_kept_when_tools_present() {
        let cfg = ModelFailoverConfig {
            enabled: true,
            path_prefix: "/v1/chat".to_string(),
            embeddings_path_prefix: None,
            responses_path_prefix: None,
            images_path_prefix: None,
            audio_path_prefix: None,
            allow_failover_after_first_byte: false,
            midstream_sse_max_buffer_bytes: 16 * 1024 * 1024,
            circuit_breaker_enabled: false,
            circuit_breaker_failure_threshold: 3,
            circuit_breaker_open_seconds: 30,
            groups: vec![panda_config::ModelFailoverGroup {
                match_models: vec![],
                backends: vec![ModelFailoverBackend {
                backend_base: "https://api.anthropic.com".to_string(),
                    api_key_env: None,
                    use_api_key_header: false,
                    protocol: ModelFailoverProtocol::Anthropic,
                    supports: vec![],
                    supports_streaming: None,
                }],
            }],
        };
        let body =
            br#"{"model":"x","messages":[],"tools":[{"type":"function","function":{"name":"f"}}]}"#;
        let r = resolve_failover_chain(
            &cfg,
            "/v1/chat/completions",
            &hyper::Method::POST,
            Some(body),
        );
        assert!(r.is_some(), "anthropic hop now supports tool mapping");
    }

    #[test]
    fn embeddings_prefix_classifies_and_matches_group() {
        let cfg = ModelFailoverConfig {
            enabled: true,
            path_prefix: "/v1/chat".to_string(),
            embeddings_path_prefix: Some("/v1/embeddings".to_string()),
            responses_path_prefix: None,
            images_path_prefix: None,
            audio_path_prefix: None,
            allow_failover_after_first_byte: false,
            midstream_sse_max_buffer_bytes: 16 * 1024 * 1024,
            circuit_breaker_enabled: false,
            circuit_breaker_failure_threshold: 3,
            circuit_breaker_open_seconds: 30,
            groups: vec![panda_config::ModelFailoverGroup {
                match_models: vec!["text-embedding-3-small".to_string()],
                backends: vec![ModelFailoverBackend {
                backend_base: "https://embeddings.example".to_string(),
                    api_key_env: None,
                    use_api_key_header: false,
                    protocol: ModelFailoverProtocol::OpenaiCompatible,
                    supports: vec![ModelFailoverOperation::Embeddings],
                    supports_streaming: None,
                }],
            }],
        };
        let body = br#"{"model":"text-embedding-3-small","input":"hi"}"#;
        let (r, c) =
            resolve_failover_chain(&cfg, "/v1/embeddings", &hyper::Method::POST, Some(body))
                .unwrap();
        assert_eq!(c.operation, FailoverApiOperation::Embeddings);
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn should_retry_failover_for_429_and_5xx_only() {
        assert!(should_retry_failover(hyper::StatusCode::TOO_MANY_REQUESTS));
        assert!(should_retry_failover(hyper::StatusCode::BAD_GATEWAY));
        assert!(should_retry_failover(
            hyper::StatusCode::INTERNAL_SERVER_ERROR
        ));
        assert!(!should_retry_failover(hyper::StatusCode::OK));
        assert!(!should_retry_failover(hyper::StatusCode::BAD_REQUEST));
    }
}

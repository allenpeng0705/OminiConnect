//! Human-in-the-loop MCP gates, 429 failover, and chat summarization.

use std::time::Duration;

use anyhow::Context as _;
use http::header::{self, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use panda_config::{ContextManagementConfig, McpHitlConfig, PandaConfig, RateLimitFallbackConfig};
use serde_json::{json, Value};

use crate::outbound::adapter;
use crate::{
    collect_body_bounded, request_upstream_with_timeout, HttpClient, ProxyError, ProxyState,
};

const HITL_APPROVAL_MAX_BODY: usize = 64 * 1024;
const SUMMARIZER_RESPONSE_MAX_BODY: usize = 512 * 1024;

/// True when this MCP invocation should wait for external approval.
pub fn mcp_hitl_matches(
    hitl: &McpHitlConfig,
    openai_function_name: &str,
    server: &str,
    tool: &str,
) -> bool {
    if !hitl.enabled || hitl.tools.is_empty() {
        return false;
    }
    let canonical = format!("{server}.{tool}");
    hitl.tools
        .iter()
        .any(|t| t == openai_function_name || t == &canonical)
}

/// POST to [`McpHitlConfig::approval_url`]; returns `Ok(())` only when the body approves.
pub async fn mcp_hitl_approve(
    client: &HttpClient,
    hitl: &McpHitlConfig,
    correlation_id: &str,
    openai_function_name: &str,
    server: &str,
    tool: &str,
    arguments: &Value,
) -> Result<(), ProxyError> {
    let uri: hyper::Uri = hitl
        .approval_url
        .trim()
        .parse()
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("mcp.hitl.approval_url: {e}")))?;
    let payload = json!({
        "correlation_id": correlation_id,
        "openai_function_name": openai_function_name,
        "server": server,
        "tool": tool,
        "arguments": arguments,
    });
    let body = serde_json::to_vec(&payload).map_err(|e| ProxyError::Upstream(e.into()))?;
    let timeout = Duration::from_millis(hitl.timeout_ms.max(1));

    let mut req_builder = Request::builder()
        .method(hyper::Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(ref envname) = hitl.bearer_token_env {
        if let Ok(tok) = std::env::var(envname) {
            let t = tok.trim();
            if !t.is_empty() {
                let hv = HeaderValue::try_from(format!("Bearer {t}")).map_err(|_| {
                    ProxyError::Upstream(anyhow::anyhow!("mcp.hitl bearer header value"))
                })?;
                req_builder = req_builder.header(header::AUTHORIZATION, hv);
            }
        }
    }
    let req = req_builder
        .body(
            Full::new(bytes::Bytes::from(body))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("hitl request build: {e}")))?;

    let resp = request_upstream_with_timeout(client, req, timeout, "mcp_hitl_approval").await?;
    let status = resp.status();
    let (parts, b) = resp.into_parts();
    let bytes = collect_body_bounded(b, HITL_APPROVAL_MAX_BODY).await?;
    drop(parts);
    if !status.is_success() {
        let snippet = String::from_utf8_lossy(&bytes[..bytes.len().min(512)]);
        return Err(ProxyError::Upstream(anyhow::anyhow!(
            "mcp.hitl approval HTTP {} body={snippet}",
            status
        )));
    }
    let v: Value = serde_json::from_slice(&bytes)
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("mcp.hitl approval JSON: {e}")))?;
    let approved = v.get("approved").and_then(|x| x.as_bool()) == Some(true)
        || v.get("status").and_then(|x| x.as_str()) == Some("approved");
    if !approved {
        return Err(ProxyError::Upstream(anyhow::anyhow!(
            "mcp.hitl approval denied or missing approved=true"
        )));
    }
    Ok(())
}

/// If the chat `messages` array is long, summarize the prefix and keep a tail of recent turns.
pub async fn maybe_summarize_openai_chat_body(
    client: &HttpClient,
    cfg: &ContextManagementConfig,
    body: &[u8],
) -> Result<Vec<u8>, ProxyError> {
    if !cfg.enabled {
        return Ok(body.to_vec());
    }
    let mut root: Value =
        serde_json::from_slice(body).map_err(|e| ProxyError::Upstream(e.into()))?;
    let Some(messages) = root.get_mut("messages").and_then(|m| m.as_array_mut()) else {
        return Ok(body.to_vec());
    };
    if messages.len() <= cfg.max_messages {
        return Ok(body.to_vec());
    }
    let keep = cfg.keep_recent_messages.min(messages.len());
    if keep == 0 || messages.len() <= keep {
        return Ok(body.to_vec());
    }
    let tail: Vec<Value> = messages.split_off(messages.len() - keep);
    let head = std::mem::take(messages);
    let key = std::env::var(&cfg.summarizer_api_key_env)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .with_context(|| format!("env {} for summarizer", cfg.summarizer_api_key_env))
        .map_err(ProxyError::Upstream)?;
    let head_json = serde_json::to_string(&head).map_err(|e| ProxyError::Upstream(e.into()))?;
    let summarize_req = json!({
        "model": cfg.summarizer_model,
        "messages": [
            {"role": "system", "content": cfg.system_prompt},
            {"role": "user", "content": head_json}
        ],
        "max_tokens": cfg.summarization_max_tokens
    });
    let summarize_bytes =
        serde_json::to_vec(&summarize_req).map_err(|e| ProxyError::Upstream(e.into()))?;
    let base = cfg.summarizer_backend_base.trim_end_matches('/');
    let uri: hyper::Uri = format!("{base}/v1/chat/completions")
        .parse()
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("summarizer URI: {e}")))?;
    let auth = HeaderValue::try_from(format!("Bearer {}", key.trim()))
        .map_err(|_| ProxyError::Upstream(anyhow::anyhow!("summarizer auth header")))?;
    let req = Request::builder()
        .method(hyper::Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, auth)
        .body(
            Full::new(bytes::Bytes::from(summarize_bytes))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("summarizer request: {e}")))?;
    let timeout = Duration::from_millis(cfg.request_timeout_ms.max(1));
    let resp = request_upstream_with_timeout(client, req, timeout, "context_summarize").await?;
    if resp.status() != StatusCode::OK {
        return Err(ProxyError::Upstream(anyhow::anyhow!(
            "summarizer upstream HTTP {}",
            resp.status()
        )));
    }
    let (_p, b) = resp.into_parts();
    let out_bytes = collect_body_bounded(b, SUMMARIZER_RESPONSE_MAX_BODY).await?;
    let sresp: Value =
        serde_json::from_slice(&out_bytes).map_err(|e| ProxyError::Upstream(e.into()))?;
    let summary_text = sresp
        .pointer("/choices/0/message/content")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if summary_text.is_empty() {
        return Err(ProxyError::Upstream(anyhow::anyhow!(
            "summarizer returned empty content"
        )));
    }
    let summary_msg = json!({
        "role": "system",
        "content": format!("Prior conversation summary:\n{summary_text}")
    });
    let mut new_messages = vec![summary_msg];
    new_messages.extend(tail);
    if let Some(obj) = root.as_object_mut() {
        obj.insert("messages".to_string(), Value::Array(new_messages));
    }
    serde_json::to_vec(&root).map_err(|e| ProxyError::Upstream(e.into()))
}

/// True when a 429 from the primary upstream should drain the error body and attempt [`try_rate_limit_fallback_chat`].
pub fn rate_limit_fallback_can_attempt(cfg: &PandaConfig, snapshot: Option<&[u8]>) -> bool {
    let fb = &cfg.rate_limit_fallback;
    if !fb.enabled || snapshot.is_none() {
        return false;
    }
    if !matches!(fb.provider.as_str(), "anthropic" | "openai_compatible") {
        return false;
    }
    std::env::var(&fb.api_key_env)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .is_some()
}

/// After primary upstream returns 429, optionally replay the (OpenAI-shaped) chat body to the fallback hop.
pub async fn try_rate_limit_fallback_chat(
    state: &ProxyState,
    openai_body: &[u8],
) -> Result<Option<Response<Incoming>>, ProxyError> {
    let cfg = &state.config.rate_limit_fallback;
    if !cfg.enabled {
        return Ok(None);
    }
    let key = std::env::var(&cfg.api_key_env)
        .ok()
        .filter(|s| !s.trim().is_empty());
    let Some(key) = key else {
        eprintln!(
            "panda: rate_limit_fallback skipped: env {} unset or empty",
            cfg.api_key_env
        );
        return Ok(None);
    };
    let timeout = Duration::from_secs(crate::DEFAULT_UPSTREAM_REQUEST_TIMEOUT_SECONDS);
    match cfg.provider.as_str() {
        "anthropic" => anthropic_fallback_request(state, openai_body, &key, timeout).await,
        "openai_compatible" => {
            openai_compatible_fallback_request(openai_body, cfg, &key, timeout, &state.client).await
        }
        _ => Ok(None),
    }
}

async fn anthropic_fallback_request(
    state: &ProxyState,
    openai_body: &[u8],
    api_key: &str,
    timeout: Duration,
) -> Result<Option<Response<Incoming>>, ProxyError> {
    let cfg = &state.config.rate_limit_fallback;
    let (body_bytes, _) =
        adapter::openai_chat_to_anthropic(openai_body).map_err(ProxyError::Upstream)?;
    let base = cfg.backend_base.trim_end_matches('/');
    let uri: hyper::Uri = format!("{base}/v1/messages")
        .parse()
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("rate_limit_fallback.backend_base: {e}")))?;
    let version = state.config.adapter.anthropic_version.trim();
    let key_h = HeaderValue::try_from(api_key)
        .map_err(|_| ProxyError::Upstream(anyhow::anyhow!("rate_limit_fallback api key header")))?;
    let ver_h = HeaderValue::try_from(version)
        .map_err(|_| ProxyError::Upstream(anyhow::anyhow!("anthropic-version header")))?;
    let req = Request::builder()
        .method(hyper::Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("x-api-key", key_h)
        .header("anthropic-version", ver_h)
        .body(
            Full::new(bytes::Bytes::from(body_bytes))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("fallback request: {e}")))?;
    let resp =
        request_upstream_with_timeout(&state.client, req, timeout, "rate_limit_fallback_anthropic")
            .await?;
    Ok(Some(resp))
}

async fn openai_compatible_fallback_request(
    openai_body: &[u8],
    cfg: &RateLimitFallbackConfig,
    api_key: &str,
    timeout: Duration,
    client: &HttpClient,
) -> Result<Option<Response<Incoming>>, ProxyError> {
    let uri: hyper::Uri =
        cfg.backend_base.trim().parse().map_err(|e| {
            ProxyError::Upstream(anyhow::anyhow!("rate_limit_fallback.backend_base: {e}"))
        })?;
    let mut req_builder = Request::builder()
        .method(hyper::Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if cfg.use_api_key_header {
        let hv = HeaderValue::try_from(api_key)
            .map_err(|_| ProxyError::Upstream(anyhow::anyhow!("api-key header value")))?;
        req_builder = req_builder.header("api-key", hv);
    } else {
        let hv = HeaderValue::try_from(format!("Bearer {}", api_key.trim()))
            .map_err(|_| ProxyError::Upstream(anyhow::anyhow!("Authorization header value")))?;
        req_builder = req_builder.header(header::AUTHORIZATION, hv);
    }
    let req = req_builder
        .body(
            Full::new(bytes::Bytes::copy_from_slice(openai_body))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("fallback request: {e}")))?;
    let resp =
        request_upstream_with_timeout(client, req, timeout, "rate_limit_fallback_openai").await?;
    Ok(Some(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::McpHitlConfig;

    #[test]
    fn hitl_matches_openai_name_or_server_dot_tool() {
        let hitl = McpHitlConfig {
            enabled: true,
            tools: vec!["sql.drop".to_string(), "other".to_string()],
            ..Default::default()
        };
        assert!(mcp_hitl_matches(&hitl, "sql_drop", "sql", "drop"));
        assert!(mcp_hitl_matches(&hitl, "anything", "sql", "drop"));
        assert!(!mcp_hitl_matches(&hitl, "sql_select", "sql", "select"));
    }
}

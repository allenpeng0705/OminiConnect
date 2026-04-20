//! Semantic upstream selection: `routing.semantic.mode` embed, classifier, or llm_judge.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Bytes;
use http::header::{self, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::Request;
use panda_config::PandaConfig;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;

use crate::{HttpClient, ProxyError};

const MAX_PROMPT_EMBEDDING_CACHE_ENTRIES: usize = 1024;
const MAX_PROMPT_ROUTER_CACHE_ENTRIES: usize = 1024;

/// One prompt-routing cache slot: `created` enforces fixed TTL; `last_touch` drives LRU eviction under `max_entries`.
#[derive(Clone)]
struct SemanticPromptCacheEntry<T> {
    created: Instant,
    last_touch: Instant,
    value: T,
}

fn semantic_prompt_cache_touch_or_miss<T: Clone>(
    map: &mut HashMap<String, SemanticPromptCacheEntry<T>>,
    key: &str,
    ttl: Duration,
) -> Option<T> {
    let e = map.get_mut(key)?;
    if e.created.elapsed() >= ttl {
        return None;
    }
    e.last_touch = Instant::now();
    Some(e.value.clone())
}

fn semantic_prompt_cache_insert<T>(
    map: &mut HashMap<String, SemanticPromptCacheEntry<T>>,
    key: String,
    value: T,
    ttl: Duration,
    max_entries: usize,
) {
    map.retain(|_, e| e.created.elapsed() < ttl);
    if !map.contains_key(&key) {
        while map.len() >= max_entries {
            let Some(lru_key) = map
                .iter()
                .min_by_key(|(_, e)| e.last_touch)
                .map(|(k, _)| k.clone())
            else {
                break;
            };
            map.remove(&lru_key);
        }
    }
    let now = Instant::now();
    map.insert(
        key,
        SemanticPromptCacheEntry {
            created: now,
            last_touch: now,
            value,
        },
    );
}

/// Result of the semantic routing stage (upstream selection, response headers, metrics).
#[derive(Debug, Clone)]
pub struct SemanticRouteOutcome {
    /// When set, the request should use this upstream base (non-shadow match only).
    pub upstream: Option<String>,
    pub kind: SemanticRouteKind,
}

#[derive(Debug, Clone)]
pub enum SemanticRouteKind {
    /// Semantic routing did not run for this request.
    NotRun,
    NoPromptText,
    EmbedFailedStatic,
    /// Router HTTP/parse failure with `fallback: static`.
    RouterFailedStatic,
    BelowThreshold,
    /// `shadow`: would route but `routing.shadow_mode` kept static upstream.
    Match {
        target: String,
        score: f32,
        shadow: bool,
    },
}

impl Default for SemanticRouteOutcome {
    fn default() -> Self {
        Self {
            upstream: None,
            kind: SemanticRouteKind::NotRun,
        }
    }
}

#[derive(Clone)]
struct TargetEmbeddings {
    name: String,
    upstream: String,
    vector: Vec<f32>,
}

#[derive(Clone)]
struct RouterTargetRow {
    name: String,
    upstream: String,
    hint: String,
}

enum SemanticRoutingInner {
    Embed(EmbedRouting),
    Router(RouterRouting),
}

pub struct SemanticRoutingRuntime {
    inner: SemanticRoutingInner,
}

#[derive(Clone)]
struct EmbedRouting {
    client: HttpClient,
    embed_url: String,
    embed_model: String,
    api_key_env: String,
    timeout: Duration,
    threshold: f32,
    cache_ttl: Duration,
    targets: Vec<TargetEmbeddings>,
    prompt_cache: Arc<Mutex<HashMap<String, SemanticPromptCacheEntry<Vec<f32>>>>>,
}

#[derive(Clone)]
struct RouterRouting {
    client: HttpClient,
    chat_url: String,
    router_model: String,
    api_key_env: String,
    timeout: Duration,
    /// Minimum model-reported confidence (same field as embed cosine threshold).
    threshold: f32,
    cache_ttl: Duration,
    targets: Vec<RouterTargetRow>,
    names: HashSet<String>,
    judge_style: bool,
    router_response_json: bool,
    prompt_cache: Arc<Mutex<HashMap<String, SemanticPromptCacheEntry<CachedRouterOutcome>>>>,
}

#[derive(Clone)]
struct CachedRouterOutcome {
    target: Option<String>,
    confidence: f32,
}

#[derive(Deserialize)]
struct EmbeddingsResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

/// OpenAI-compatible `POST .../embeddings` → single L2-normalized vector.
pub(crate) async fn openai_fetch_embedding_normalized(
    client: &HttpClient,
    embed_url: &str,
    model: &str,
    api_key: &str,
    input: &str,
    timeout: Duration,
) -> Result<Vec<f32>, ProxyError> {
    let body = serde_json::json!({
        "model": model,
        "input": input,
    });
    let body_bytes = serde_json::to_vec(&body).map_err(|e| ProxyError::Upstream(e.into()))?;
    let auth = HeaderValue::from_str(&format!("Bearer {api_key}")).map_err(|_| {
        ProxyError::Upstream(anyhow::anyhow!("semantic cache embed: bad bearer token"))
    })?;
    let req = Request::builder()
        .method(hyper::Method::POST)
        .uri(embed_url)
        .header(header::AUTHORIZATION, auth)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            Full::new(Bytes::from(body_bytes))
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("semantic cache embed request: {e}")))?;
    let resp =
        crate::request_upstream_with_timeout(client, req, timeout, "semantic_cache_embed").await?;
    let st = resp.status();
    let body = resp
        .into_body()
        .collect()
        .await
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("semantic cache embed body: {e}")))?;
    let bytes = body.to_bytes();
    if !st.is_success() {
        let snippet = String::from_utf8_lossy(&bytes[..bytes.len().min(512)]);
        return Err(ProxyError::Upstream(anyhow::anyhow!(
            "semantic cache embeddings HTTP {st}: {snippet}"
        )));
    }
    let parsed: EmbeddingsResponse = serde_json::from_slice(&bytes)
        .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("embeddings JSON: {e}")))?;
    let mut emb = parsed
        .data
        .into_iter()
        .next()
        .and_then(|d| {
            if d.embedding.is_empty() {
                None
            } else {
                Some(d.embedding)
            }
        })
        .ok_or_else(|| ProxyError::Upstream(anyhow::anyhow!("semantic cache: empty embedding")))?;
    l2_normalize(&mut emb);
    Ok(emb)
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessageBody,
}

#[derive(Deserialize)]
struct ChatMessageBody {
    content: Option<String>,
}

#[derive(Deserialize)]
struct RouterJudgePayload {
    target: Option<String>,
    confidence: Option<f32>,
}

fn sha256_key_prefix(text: &str) -> String {
    let d = Sha256::digest(text.as_bytes());
    format!("{:x}", d)
}

pub(crate) fn l2_normalize(v: &mut [f32]) {
    let s: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if s > 1e-10 {
        for x in v.iter_mut() {
            *x /= s;
        }
    }
}

pub(crate) fn cosine_dot(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    Some(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

/// Strip optional ```json fences and return the JSON object substring.
fn extract_json_object_slice(text: &str) -> String {
    let t = text.trim();
    if let Some(rest) = t.strip_prefix("```") {
        let rest = rest.trim_start();
        let rest = rest.strip_prefix("json").unwrap_or(rest).trim_start();
        if let Some(end) = rest.rfind("```") {
            return rest[..end].trim().to_string();
        }
        return rest.trim().to_string();
    }
    t.to_string()
}

fn parse_router_judge_json(content: &str) -> Option<RouterJudgePayload> {
    let slice = extract_json_object_slice(content);
    serde_json::from_str::<RouterJudgePayload>(&slice).ok()
}

/// Extract user-visible text from an OpenAI-style chat JSON body for embedding.
pub fn extract_openai_chat_text_for_routing(raw: &[u8], max_chars: usize) -> Option<String> {
    let v: serde_json::Value = serde_json::from_slice(raw).ok()?;
    let messages = v.get("messages")?.as_array()?;
    let mut parts = Vec::new();
    for m in messages {
        let role = m.get("role").and_then(|r| r.as_str()).unwrap_or("");
        let text = match m.get("content") {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Array(arr)) => arr
                .iter()
                .filter_map(|b| {
                    if b.get("type").and_then(|t| t.as_str()) == Some("text") {
                        b.get("text")
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
            _ => String::new(),
        };
        let t = text.trim();
        if !t.is_empty() {
            parts.push(format!("{role}: {t}"));
        }
    }
    let joined = parts.join("\n");
    let t = joined.trim();
    if t.is_empty() {
        return None;
    }
    Some(t.chars().take(max_chars).collect())
}

impl SemanticRoutingRuntime {
    /// Build runtime (embed warmup or router env check). Returns `None` when semantic routing is off.
    pub async fn connect(
        cfg: &PandaConfig,
        client: HttpClient,
    ) -> anyhow::Result<Option<Arc<Self>>> {
        let r = &cfg.routing;
        if !r.enabled || !r.semantic.enabled {
            return Ok(None);
        }
        let mode = r.semantic.mode.to_ascii_lowercase();
        match mode.as_str() {
            "embed" => {
                let inner = EmbedRouting::connect(cfg, client).await?;
                Ok(Some(Arc::new(Self {
                    inner: SemanticRoutingInner::Embed(inner),
                })))
            }
            "classifier" | "llm_judge" => {
                let judge_style = mode == "llm_judge";
                let inner = RouterRouting::connect(cfg, client, judge_style).await?;
                Ok(Some(Arc::new(Self {
                    inner: SemanticRoutingInner::Router(inner),
                })))
            }
            _ => Ok(None),
        }
    }

    pub async fn resolve(
        &self,
        prompt_text: Option<&str>,
        fallback: &str,
        shadow: bool,
    ) -> Result<SemanticRouteOutcome, ProxyError> {
        match &self.inner {
            SemanticRoutingInner::Embed(e) => e.resolve(prompt_text, fallback, shadow).await,
            SemanticRoutingInner::Router(r) => r.resolve(prompt_text, fallback, shadow).await,
        }
    }
}

impl EmbedRouting {
    async fn connect(cfg: &PandaConfig, client: HttpClient) -> anyhow::Result<Self> {
        let r = &cfg.routing;
        if r.semantic.mode.to_ascii_lowercase() != "embed" {
            anyhow::bail!("semantic routing: embed connect called with non-embed mode");
        }
        let timeout = Duration::from_millis(r.semantic.timeout_ms);
        let threshold = r.semantic.similarity_threshold;
        let cache_ttl = Duration::from_secs(r.semantic.cache_ttl_seconds);
        let base = r.semantic.embed_backend_base.trim_end_matches('/');
        let embed_url = format!("{base}/embeddings");
        let api_key = std::env::var(r.semantic.embed_api_key_env.trim()).map_err(|_| {
            anyhow::anyhow!(
                "semantic routing: env {} not set",
                r.semantic.embed_api_key_env
            )
        })?;
        if api_key.trim().is_empty() {
            anyhow::bail!(
                "semantic routing: API key from {} is empty",
                r.semantic.embed_api_key_env
            );
        }
        let embed_model = r.semantic.embed_model.trim().to_string();
        let mut targets = Vec::new();
        for t in &r.semantic.targets {
            let vec = Self::fetch_embedding(
                &client,
                &embed_url,
                &embed_model,
                api_key.trim(),
                &t.routing_text,
                timeout,
            )
            .await
            .map_err(|e| anyhow::anyhow!("semantic routing warmup target {:?}: {e:?}", t.name))?;
            targets.push(TargetEmbeddings {
                name: t.name.clone(),
                upstream: t.backend_base.trim().to_string(),
                vector: vec,
            });
        }
        eprintln!(
            "panda: semantic routing enabled (embed, {} target(s), threshold={})",
            targets.len(),
            threshold
        );
        Ok(Self {
            client,
            embed_url,
            embed_model,
            api_key_env: r.semantic.embed_api_key_env.trim().to_string(),
            timeout,
            threshold,
            cache_ttl,
            targets,
            prompt_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    async fn fetch_embedding(
        client: &HttpClient,
        embed_url: &str,
        model: &str,
        api_key: &str,
        input: &str,
        timeout: Duration,
    ) -> Result<Vec<f32>, ProxyError> {
        openai_fetch_embedding_normalized(client, embed_url, model, api_key, input, timeout).await
    }

    async fn prompt_embedding(&self, text: &str) -> Result<Vec<f32>, ProxyError> {
        let key = sha256_key_prefix(text);
        {
            let mut guard = self.prompt_cache.lock().await;
            if let Some(v) = semantic_prompt_cache_touch_or_miss(&mut guard, &key, self.cache_ttl) {
                return Ok(v);
            }
        }
        let api_key = std::env::var(self.api_key_env.trim()).map_err(|_| {
            ProxyError::SemanticRoutingFailed("embedding API key env not set".to_string())
        })?;
        if api_key.trim().is_empty() {
            return Err(ProxyError::SemanticRoutingFailed(
                "embedding API key empty".to_string(),
            ));
        }
        let emb = Self::fetch_embedding(
            &self.client,
            &self.embed_url,
            &self.embed_model,
            api_key.trim(),
            text,
            self.timeout,
        )
        .await?;
        let mut guard = self.prompt_cache.lock().await;
        semantic_prompt_cache_insert(
            &mut guard,
            key,
            emb.clone(),
            self.cache_ttl,
            MAX_PROMPT_EMBEDDING_CACHE_ENTRIES,
        );
        Ok(emb)
    }

    async fn resolve(
        &self,
        prompt_text: Option<&str>,
        fallback: &str,
        shadow: bool,
    ) -> Result<SemanticRouteOutcome, ProxyError> {
        let Some(text) = prompt_text.map(|s| s.trim()).filter(|s| !s.is_empty()) else {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::NoPromptText,
            });
        };
        let prompt_vec = match self.prompt_embedding(text).await {
            Ok(v) => v,
            Err(e) => {
                if fallback != "deny" {
                    eprintln!("panda: semantic routing embed failed (fallback=static): {e:?}");
                    return Ok(SemanticRouteOutcome {
                        upstream: None,
                        kind: SemanticRouteKind::EmbedFailedStatic,
                    });
                }
                return Err(match e {
                    ProxyError::Upstream(x) => ProxyError::SemanticRoutingFailed(x.to_string()),
                    ProxyError::SemanticRoutingFailed(s) => ProxyError::SemanticRoutingFailed(s),
                    other => other,
                });
            }
        };
        let mut best: Option<(f32, &TargetEmbeddings)> = None;
        for t in &self.targets {
            let Some(sim) = cosine_dot(&prompt_vec, &t.vector) else {
                continue;
            };
            if best.map(|(s, _)| sim > s).unwrap_or(true) {
                best = Some((sim, t));
            }
        }
        let Some((score, target)) = best else {
            return Ok(SemanticRouteOutcome::default());
        };
        if score < self.threshold {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::BelowThreshold,
            });
        }
        if shadow {
            eprintln!(
                "panda: semantic routing shadow — would use target={} upstream={} score={score:.4}",
                target.name, target.upstream
            );
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::Match {
                    target: target.name.clone(),
                    score,
                    shadow: true,
                },
            });
        }
        Ok(SemanticRouteOutcome {
            upstream: Some(target.upstream.clone()),
            kind: SemanticRouteKind::Match {
                target: target.name.clone(),
                score,
                shadow: false,
            },
        })
    }
}

impl RouterRouting {
    async fn connect(
        cfg: &PandaConfig,
        client: HttpClient,
        judge_style: bool,
    ) -> anyhow::Result<Self> {
        let r = &cfg.routing;
        let mode = r.semantic.mode.to_ascii_lowercase();
        if mode != "classifier" && mode != "llm_judge" {
            anyhow::bail!("semantic routing: router connect with wrong mode");
        }
        let api_key = std::env::var(r.semantic.router_api_key_env.trim()).map_err(|_| {
            anyhow::anyhow!(
                "semantic routing: env {} not set",
                r.semantic.router_api_key_env
            )
        })?;
        if api_key.trim().is_empty() {
            anyhow::bail!(
                "semantic routing: API key from {} is empty",
                r.semantic.router_api_key_env
            );
        }
        let base = r.semantic.router_backend_base.trim_end_matches('/');
        let chat_url = format!("{base}/chat/completions");
        let mut names = HashSet::new();
        let mut targets = Vec::new();
        for t in &r.semantic.targets {
            names.insert(t.name.clone());
            targets.push(RouterTargetRow {
                name: t.name.clone(),
                upstream: t.backend_base.trim().to_string(),
                hint: t.routing_text.trim().to_string(),
            });
        }
        let label = if judge_style {
            "llm_judge"
        } else {
            "classifier"
        };
        eprintln!(
            "panda: semantic routing enabled ({label}, {} target(s), confidence threshold={})",
            targets.len(),
            r.semantic.similarity_threshold
        );
        Ok(Self {
            client,
            chat_url,
            router_model: r.semantic.router_model.trim().to_string(),
            api_key_env: r.semantic.router_api_key_env.trim().to_string(),
            timeout: Duration::from_millis(r.semantic.timeout_ms),
            threshold: r.semantic.similarity_threshold,
            cache_ttl: Duration::from_secs(r.semantic.cache_ttl_seconds),
            targets,
            names,
            judge_style,
            router_response_json: r.semantic.router_response_json,
            prompt_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn system_prompt(&self) -> String {
        let mut lines = Vec::new();
        if self.judge_style {
            lines.push(
                "You are a careful routing judge for an AI gateway. Read the user request and pick at most one route."
                    .to_string(),
            );
            lines.push(
                "Respond with a single JSON object only (no markdown). Keys: \"target\" (string route id or null) and \"confidence\" (number 0.0–1.0)."
                    .to_string(),
            );
        } else {
            lines.push(
                "You classify requests for an AI gateway. Pick at most one route id from the list."
                    .to_string(),
            );
            lines.push(
                "Respond with a single JSON object only (no markdown). Keys: \"target\" (string route id or null) and \"confidence\" (number 0.0–1.0)."
                    .to_string(),
            );
        }
        lines.push("Valid route ids:".to_string());
        for t in &self.targets {
            if t.hint.is_empty() {
                lines.push(format!("- {}", t.name));
            } else {
                lines.push(format!("- {}: {}", t.name, t.hint));
            }
        }
        lines.push(
            "If none fit, use {\"target\": null, \"confidence\": <0.0-1.0>}. Never invent route ids."
                .to_string(),
        );
        lines.join("\n")
    }

    async fn call_router(&self, user_text: &str) -> Result<RouterJudgePayload, ProxyError> {
        let api_key = std::env::var(self.api_key_env.trim()).map_err(|_| {
            ProxyError::SemanticRoutingFailed("router API key env not set".to_string())
        })?;
        if api_key.trim().is_empty() {
            return Err(ProxyError::SemanticRoutingFailed(
                "router API key empty".to_string(),
            ));
        }
        let system = self.system_prompt();
        let mut body = serde_json::json!({
            "model": self.router_model,
            "temperature": 0,
            "max_tokens": 200,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": format!(
                    "Classify this request (may include role labels).\n\n---\n{user_text}\n---"
                )}
            ]
        });
        if self.router_response_json {
            if let Some(obj) = body.as_object_mut() {
                obj.insert(
                    "response_format".to_string(),
                    serde_json::json!({"type": "json_object"}),
                );
            }
        }
        let body_bytes = serde_json::to_vec(&body).map_err(|e| ProxyError::Upstream(e.into()))?;
        let auth = HeaderValue::from_str(&format!("Bearer {}", api_key.trim())).map_err(|_| {
            ProxyError::Upstream(anyhow::anyhow!("semantic routing: bad router bearer token"))
        })?;
        let req = Request::builder()
            .method(hyper::Method::POST)
            .uri(&self.chat_url)
            .header(header::AUTHORIZATION, auth)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                Full::new(Bytes::from(body_bytes))
                    .map_err(|never: std::convert::Infallible| match never {})
                    .boxed_unsync(),
            )
            .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("semantic router request: {e}")))?;
        let resp = crate::request_upstream_with_timeout(
            &self.client,
            req,
            self.timeout,
            "semantic_router",
        )
        .await?;
        let st = resp.status();
        let body = resp
            .into_body()
            .collect()
            .await
            .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("semantic router body: {e}")))?;
        let bytes = body.to_bytes();
        if !st.is_success() {
            let snippet = String::from_utf8_lossy(&bytes[..bytes.len().min(512)]);
            return Err(ProxyError::Upstream(anyhow::anyhow!(
                "semantic router HTTP {st}: {snippet}"
            )));
        }
        let parsed: ChatCompletionResponse = serde_json::from_slice(&bytes)
            .map_err(|e| ProxyError::Upstream(anyhow::anyhow!("router chat JSON: {e}")))?;
        let content = parsed
            .choices
            .first()
            .and_then(|c| c.message.content.as_deref())
            .unwrap_or("")
            .to_string();
        parse_router_judge_json(&content).ok_or_else(|| {
            ProxyError::Upstream(anyhow::anyhow!(
                "semantic router: could not parse JSON from model output: {}",
                content.chars().take(200).collect::<String>()
            ))
        })
    }

    async fn cached_decision(&self, text: &str) -> Result<CachedRouterOutcome, ProxyError> {
        let key = sha256_key_prefix(text);
        {
            let mut guard = self.prompt_cache.lock().await;
            if let Some(v) = semantic_prompt_cache_touch_or_miss(&mut guard, &key, self.cache_ttl) {
                return Ok(v);
            }
        }
        let payload = self.call_router(text).await?;
        let confidence = payload.confidence.unwrap_or(1.0).clamp(0.0, 1.0);
        let target = payload
            .target
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let cached = CachedRouterOutcome { target, confidence };
        let mut guard = self.prompt_cache.lock().await;
        semantic_prompt_cache_insert(
            &mut guard,
            key,
            cached.clone(),
            self.cache_ttl,
            MAX_PROMPT_ROUTER_CACHE_ENTRIES,
        );
        Ok(cached)
    }

    async fn resolve(
        &self,
        prompt_text: Option<&str>,
        fallback: &str,
        shadow: bool,
    ) -> Result<SemanticRouteOutcome, ProxyError> {
        let Some(text) = prompt_text.map(|s| s.trim()).filter(|s| !s.is_empty()) else {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::NoPromptText,
            });
        };

        let cached = match self.cached_decision(text).await {
            Ok(c) => c,
            Err(e) => {
                if fallback != "deny" {
                    eprintln!("panda: semantic routing router failed (fallback=static): {e:?}");
                    return Ok(SemanticRouteOutcome {
                        upstream: None,
                        kind: SemanticRouteKind::RouterFailedStatic,
                    });
                }
                return Err(match e {
                    ProxyError::Upstream(x) => ProxyError::SemanticRoutingFailed(x.to_string()),
                    ProxyError::SemanticRoutingFailed(s) => ProxyError::SemanticRoutingFailed(s),
                    other => other,
                });
            }
        };

        let Some(ref name) = cached.target else {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::BelowThreshold,
            });
        };
        if !self.names.contains(name) {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::BelowThreshold,
            });
        }
        if cached.confidence < self.threshold {
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::BelowThreshold,
            });
        }
        let upstream = self
            .targets
            .iter()
            .find(|t| t.name == *name)
            .map(|t| t.upstream.clone())
            .unwrap_or_default();
        let score = cached.confidence;
        if shadow {
            eprintln!("panda: semantic routing shadow — would use target={name} upstream={upstream} score={score:.4}");
            return Ok(SemanticRouteOutcome {
                upstream: None,
                kind: SemanticRouteKind::Match {
                    target: name.clone(),
                    score,
                    shadow: true,
                },
            });
        }
        Ok(SemanticRouteOutcome {
            upstream: Some(upstream),
            kind: SemanticRouteKind::Match {
                target: name.clone(),
                score,
                shadow: false,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_chat_flattens_string_content() {
        let raw = br#"{"model":"m","messages":[{"role":"user","content":"hello world"}]}"#;
        let t = extract_openai_chat_text_for_routing(raw, 100).expect("text");
        assert!(t.contains("user"));
        assert!(t.contains("hello"));
    }

    #[test]
    fn cosine_normalized_orthogonal_low() {
        let mut a = vec![1.0_f32, 0.0];
        let mut b = vec![0.0_f32, 1.0];
        l2_normalize(&mut a);
        l2_normalize(&mut b);
        let d = cosine_dot(&a, &b).unwrap();
        assert!(d.abs() < 0.01, "{d}");
    }

    #[test]
    fn cosine_normalized_parallel_high() {
        let mut a = vec![3.0_f32, 4.0];
        let mut b = vec![6.0_f32, 8.0];
        l2_normalize(&mut a);
        l2_normalize(&mut b);
        let d = cosine_dot(&a, &b).unwrap();
        assert!(d > 0.99, "{d}");
    }

    #[test]
    fn parse_router_json_plain_and_fenced() {
        let p = parse_router_judge_json(r#"{"target":"coding","confidence":0.9}"#).expect("plain");
        assert_eq!(p.target.as_deref(), Some("coding"));
        assert!((p.confidence.unwrap() - 0.9).abs() < 0.001);

        let fenced = "```json\n{\"target\": null, \"confidence\": 0.2}\n```";
        let p2 = parse_router_judge_json(fenced).expect("fenced");
        assert!(p2.target.is_none());
    }

    #[test]
    fn semantic_prompt_cache_lru_evicts_least_recently_touched() {
        let mut map: HashMap<String, SemanticPromptCacheEntry<u8>> = HashMap::new();
        let ttl = Duration::from_secs(60);
        let max = 3usize;
        semantic_prompt_cache_insert(&mut map, "k1".into(), 1, ttl, max);
        std::thread::sleep(Duration::from_millis(15));
        semantic_prompt_cache_insert(&mut map, "k2".into(), 2, ttl, max);
        std::thread::sleep(Duration::from_millis(15));
        semantic_prompt_cache_insert(&mut map, "k3".into(), 3, ttl, max);
        assert_eq!(map.len(), 3);
        let _ = semantic_prompt_cache_touch_or_miss(&mut map, "k1", ttl);
        semantic_prompt_cache_insert(&mut map, "k4".into(), 4, ttl, max);
        assert_eq!(map.len(), 3);
        assert!(map.contains_key("k1"), "touched entry should remain");
        assert!(map.contains_key("k4"), "new entry should be present");
        assert!(
            !map.contains_key("k2"),
            "middle LRU slot should be evicted (k1 touched, k3 newest insert)"
        );
        assert!(map.contains_key("k3"));
    }

    #[test]
    fn semantic_prompt_cache_expired_entries_dropped_before_eviction() {
        let mut map: HashMap<String, SemanticPromptCacheEntry<u8>> = HashMap::new();
        let ttl = Duration::from_millis(1);
        let max = 2usize;
        semantic_prompt_cache_insert(&mut map, "old".into(), 1, ttl, max);
        std::thread::sleep(Duration::from_millis(20));
        semantic_prompt_cache_insert(&mut map, "new".into(), 2, ttl, max);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("new"));
        assert!(!map.contains_key("old"));
    }
}

/// Integration-style tests: local TCP server mimics OpenAI-compatible `/embeddings` and `/chat/completions`.
#[cfg(test)]
mod mock_upstream_tests {
    use super::{SemanticRouteKind, SemanticRoutingRuntime};
    use panda_config::PandaConfig;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn embed_resolve_with_tcp_mock_openai_embeddings() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let emb = r#"{"data":[{"embedding":[1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]}]}"#;

        tokio::spawn(async move {
            for _ in 0..2 {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let mut buf = vec![0u8; 24_576];
                let Ok(n) = sock.read(&mut buf).await else {
                    continue;
                };
                let req = std::str::from_utf8(&buf[..n]).expect("utf8");
                assert!(
                    req.contains("embeddings"),
                    "expected embeddings path, got {}",
                    req.chars().take(120).collect::<String>()
                );
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
                    emb.len(),
                    emb
                );
                let _ = sock.write_all(resp.as_bytes()).await;
            }
        });

        let key = "PANDA_TEST_SEM_EMB_KEY";
        std::env::set_var(key, "test-secret");

        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
  fallback: static
  semantic:
    enabled: true
    mode: embed
    embed_backend_base: 'http://{addr}/v1'
    embed_api_key_env: '{key}'
    embed_model: 'test'
    similarity_threshold: 0.5
    targets:
      - name: t1
        routing_text: warmup
        backend_base: 'http://127.0.0.1:9/v1'
"#,
            addr = addr,
            key = key
        );

        let cfg = PandaConfig::from_yaml_str(&yaml).expect("cfg");
        let client = crate::build_http_client().expect("client");
        let rt = SemanticRoutingRuntime::connect(&cfg, client)
            .await
            .expect("connect")
            .expect("some");
        let out = rt
            .resolve(Some("user request text"), "static", false)
            .await
            .expect("resolve");

        std::env::remove_var(key);

        match out.kind {
            SemanticRouteKind::Match {
                ref target,
                shadow: false,
                ..
            } => assert_eq!(target, "t1"),
            k => panic!("unexpected outcome: {k:?}"),
        }
        assert!(out
            .upstream
            .as_deref()
            .is_some_and(|u| u.contains("127.0.0.1:9")));
    }

    #[tokio::test]
    async fn router_resolve_with_tcp_mock_chat_completions() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let chat =
            r#"{"choices":[{"message":{"content":"{\"target\":\"t1\",\"confidence\":0.95}"}}]}"#;

        tokio::spawn(async move {
            let Ok((mut sock, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 65_536];
            let Ok(n) = sock.read(&mut buf).await else {
                return;
            };
            let req = std::str::from_utf8(&buf[..n]).expect("utf8");
            assert!(
                req.contains("chat/completions"),
                "expected chat path, got {}",
                req.chars().take(120).collect::<String>()
            );
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
                chat.len(),
                chat
            );
            let _ = sock.write_all(resp.as_bytes()).await;
        });

        let key = "PANDA_TEST_SEM_ROUTER_KEY";
        std::env::set_var(key, "test-secret");

        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routing:
  enabled: true
  fallback: static
  semantic:
    enabled: true
    mode: classifier
    router_backend_base: 'http://{addr}/v1'
    router_api_key_env: '{key}'
    router_model: 'gpt-test'
    similarity_threshold: 0.5
    router_response_json: true
    targets:
      - name: t1
        routing_text: ''
        backend_base: 'http://127.0.0.1:8/v1'
"#,
            addr = addr,
            key = key
        );

        let cfg = PandaConfig::from_yaml_str(&yaml).expect("cfg");
        let client = crate::build_http_client().expect("client");
        let rt = SemanticRoutingRuntime::connect(&cfg, client)
            .await
            .expect("connect")
            .expect("some");
        let out = rt
            .resolve(Some("please route me"), "static", false)
            .await
            .expect("resolve");

        std::env::remove_var(key);

        match out.kind {
            SemanticRouteKind::Match {
                ref target, score, ..
            } => {
                assert_eq!(target, "t1");
                assert!(score > 0.9);
            }
            k => panic!("unexpected outcome: {k:?}"),
        }
    }
}

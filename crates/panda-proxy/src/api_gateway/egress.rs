//! HTTP egress toward corporate API gateways (Phase B). Gated by `api_gateway.egress.enabled`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bytes::Bytes;
use http::header::{self, HeaderMap, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request, Uri};
use panda_config::{
    ApiGatewayEgressConfig, ApiGatewayEgressDefaultHeader, ApiGatewayEgressProfile,
};
use redis::AsyncCommands;
use tokio::sync::{RwLock, Semaphore};

use crate::{build_egress_http_client, HttpClient};

const MAX_EGRESS_RESPONSE_BODY_BYTES: usize = 4 * 1024 * 1024;

/// Low-cardinality route label for metrics (e.g. `default`, `integration_test`).
#[derive(Debug, Clone)]
pub struct EgressHttpRequest {
    pub method: Method,
    /// Absolute `http(s)://...` URL, or a path starting with `/` when `corporate.default_base` / `pool_bases` is configured.
    pub target: String,
    pub route_label: String,
    /// Optional [`panda_config::ApiGatewayEgressProfile::name`]; merged after global egress `default_headers`.
    pub egress_profile: Option<String>,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

#[derive(Debug)]
pub struct EgressHttpResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Bytes,
}

#[derive(Debug)]
pub enum EgressError {
    Misconfigured(&'static str),
    AllowlistDenied,
    InvalidUrl(String),
    Timeout,
    Upstream(String),
    BodyTooLarge,
    /// Process-local `api_gateway.egress.rate_limit` (in-flight or RPS) exceeded.
    RateLimited,
}

impl std::fmt::Display for EgressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EgressError::Misconfigured(m) => write!(f, "egress misconfigured: {m}"),
            EgressError::AllowlistDenied => write!(f, "egress allowlist denied request"),
            EgressError::InvalidUrl(m) => write!(f, "egress invalid URL: {m}"),
            EgressError::Timeout => write!(f, "egress request timed out"),
            EgressError::Upstream(m) => write!(f, "egress upstream error: {m}"),
            EgressError::BodyTooLarge => write!(f, "egress response body exceeds cap"),
            EgressError::RateLimited => write!(f, "egress rate limit exceeded"),
        }
    }
}

impl std::error::Error for EgressError {}

#[derive(Default)]
struct EgressMetricsInner {
    /// Key: `result\x1froute_label`
    requests: HashMap<String, u64>,
    duration_sum_ms: u64,
    duration_count: u64,
    duration_bucket: HashMap<String, u64>,
    /// Key: route label — attempts after the first toward the same logical request.
    retries: HashMap<String, u64>,
    /// Key: `scope\x1froute\x1fallowed|denied` for [`panda_egress_rps_total`] (global + per-route gates).
    rps_gate: HashMap<String, u64>,
}

#[derive(Clone, Default)]
pub struct EgressMetrics {
    inner: Arc<Mutex<EgressMetricsInner>>,
}

impl EgressMetrics {
    /// `scope`: `global` or `per_route`; `route` is `-` for global.
    fn record_rps_gate(&self, scope: &str, route: &str, denied: bool) {
        let r = if route.is_empty() { "-" } else { route };
        let result = if denied { "denied" } else { "allowed" };
        let key = format!("{scope}\x1f{r}\x1f{result}");
        if let Ok(mut g) = self.inner.lock() {
            *g.rps_gate.entry(key).or_insert(0) += 1;
        }
    }

    fn record_retry(&self, route: &str) {
        let route = if route.is_empty() { "-" } else { route };
        if let Ok(mut g) = self.inner.lock() {
            *g.retries.entry(route.to_string()).or_insert(0) += 1;
        }
    }

    fn record(&self, route: &str, result: &str, latency_ms: u64, pool_slot: &str) {
        let route = if route.is_empty() { "-" } else { route };
        let ps = if pool_slot.is_empty() { "-" } else { pool_slot };
        let key = format!("{result}\x1f{route}\x1f{ps}");
        if let Ok(mut g) = self.inner.lock() {
            *g.requests.entry(key).or_insert(0) += 1;
            g.duration_sum_ms = g.duration_sum_ms.saturating_add(latency_ms);
            g.duration_count = g.duration_count.saturating_add(1);
            const BUCKETS: &[u64] = &[25, 50, 100, 250, 500, 1000, 2500, 5000, 10_000, 30_000];
            for b in BUCKETS {
                if latency_ms <= *b {
                    *g.duration_bucket.entry(b.to_string()).or_insert(0) += 1;
                }
            }
            *g.duration_bucket.entry("+Inf".to_string()).or_insert(0) += 1;
        }
    }

    pub fn prometheus_text(&self) -> String {
        fn esc(s: &str) -> String {
            s.replace('\\', "\\\\")
                .replace('\n', "\\n")
                .replace('"', "\\\"")
        }
        let mut out = String::new();
        out.push_str("# HELP panda_egress_requests_total API gateway egress HTTP requests by result and route label.\n");
        out.push_str("# TYPE panda_egress_requests_total counter\n");
        if let Ok(g) = self.inner.lock() {
            let mut entries: Vec<(&String, &u64)> = g.requests.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            for (key, count) in entries {
                let mut p = key.splitn(3, '\x1f');
                let result = esc(p.next().unwrap_or("-"));
                let route = esc(p.next().unwrap_or("-"));
                let pool_slot = esc(p.next().unwrap_or("-"));
                out.push_str(&format!(
                    "panda_egress_requests_total{{result=\"{result}\",route=\"{route}\",pool_slot=\"{pool_slot}\"}} {count}\n",
                ));
            }
        }
        out.push_str("# HELP panda_egress_request_duration_ms_bucket Cumulative egress request latency (ms) histogram buckets.\n");
        out.push_str("# TYPE panda_egress_request_duration_ms_bucket counter\n");
        const BUCKETS: &[&str] = &[
            "25", "50", "100", "250", "500", "1000", "2500", "5000", "10000", "30000", "+Inf",
        ];
        if let Ok(g) = self.inner.lock() {
            for b in BUCKETS {
                let n = *g.duration_bucket.get(*b).unwrap_or(&0);
                let bv = esc(b);
                out.push_str(&format!(
                    "panda_egress_request_duration_ms_bucket{{le=\"{bv}\"}} {n}\n",
                ));
            }
        }
        out.push_str(
            "# HELP panda_egress_request_duration_ms_sum Total egress request latency (ms).\n",
        );
        out.push_str("# TYPE panda_egress_request_duration_ms_sum counter\n");
        if let Ok(g) = self.inner.lock() {
            out.push_str(&format!(
                "panda_egress_request_duration_ms_sum {}\n",
                g.duration_sum_ms
            ));
        }
        out.push_str("# HELP panda_egress_request_duration_ms_count Egress requests with recorded latency.\n");
        out.push_str("# TYPE panda_egress_request_duration_ms_count counter\n");
        if let Ok(g) = self.inner.lock() {
            out.push_str(&format!(
                "panda_egress_request_duration_ms_count {}\n",
                g.duration_count
            ));
        }
        out.push_str(
            "# HELP panda_egress_retries_total Egress retry attempts before a final outcome.\n",
        );
        out.push_str("# TYPE panda_egress_retries_total counter\n");
        if let Ok(g) = self.inner.lock() {
            let mut entries: Vec<(&String, &u64)> = g.retries.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            for (route, count) in entries {
                let rv = esc(route);
                out.push_str(&format!(
                    "panda_egress_retries_total{{route=\"{rv}\"}} {count}\n",
                ));
            }
        }
        out.push_str(
            "# HELP panda_egress_rps_total Egress RPS limiter outcomes (global cap and per-route_label caps; Redis when configured).\n",
        );
        out.push_str("# TYPE panda_egress_rps_total counter\n");
        if let Ok(g) = self.inner.lock() {
            let mut entries: Vec<(&String, &u64)> = g.rps_gate.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            for (key, count) in entries {
                let mut p = key.splitn(3, '\x1f');
                let scope = esc(p.next().unwrap_or("-"));
                let route = esc(p.next().unwrap_or("-"));
                let result = esc(p.next().unwrap_or("-"));
                out.push_str(&format!(
                    "panda_egress_rps_total{{scope=\"{scope}\",route=\"{route}\",result=\"{result}\"}} {count}\n",
                ));
            }
        }
        out
    }
}

#[derive(Clone)]
struct AllowedHost {
    host_lower: String,
    /// When set, only this port matches; otherwise scheme default port only.
    port: Option<u16>,
}

pub struct EgressClient {
    client: Arc<RwLock<HttpClient>>,
    pool_idle: Option<Duration>,
    tls: panda_config::ApiGatewayEgressTlsConfig,
    /// Non-empty bases for joining relative tool paths: either `corporate.pool_bases` or `default_base`.
    corporate_bases: Vec<String>,
    corporate_rr: Arc<AtomicUsize>,
    timeout: Duration,
    allow_hosts: Vec<AllowedHost>,
    path_prefixes: Vec<String>,
    default_headers: HeaderMap,
    /// Key: profile `name` from config.
    profile_headers: HashMap<String, HeaderMap>,
    max_retries: u32,
    retry_initial_backoff: Duration,
    retry_max_backoff: Duration,
    metrics: EgressMetrics,
    /// When set, limits concurrent `request` calls (fail-fast `try_acquire`).
    in_flight: Option<Arc<Semaphore>>,
    /// Global egress RPS cap (`api_gateway.egress.rate_limit.max_rps`). `0` = none.
    rps_global_cap: u32,
    /// Process-local 1s window for global cap when Redis is not configured.
    rps_global_local: Mutex<(Instant, u32)>,
    /// Optional Redis for cluster-wide RPS (global + per-route).
    rps_redis: Option<redis::aio::ConnectionManager>,
    /// Normalized `api_gateway.egress.rate_limit.redis.key_prefix` (ends with `:`).
    rps_redis_prefix: String,
    /// Per `route_label` RPS caps.
    rps_per_route_caps: HashMap<String, u32>,
    /// Process-local 1s windows for per-route caps when Redis is not configured.
    rps_per_route_local: Mutex<HashMap<String, (Instant, u32)>>,
}

impl EgressClient {
    /// Returns `None` when egress is disabled in config.
    pub async fn connect(cfg: &ApiGatewayEgressConfig) -> anyhow::Result<Option<Arc<Self>>> {
        if !cfg.enabled {
            return Ok(None);
        }
        let pool_idle = if cfg.pool_idle_timeout_ms > 0 {
            Some(Duration::from_millis(cfg.pool_idle_timeout_ms))
        } else {
            None
        };
        let client = Arc::new(RwLock::new(build_egress_http_client(pool_idle, &cfg.tls)?));
        let tls = cfg.tls.clone();
        let timeout_ms = if cfg.timeout_ms > 0 {
            cfg.timeout_ms
        } else {
            30_000
        };
        let allow_hosts = parse_allow_hosts(&cfg.allowlist.allow_hosts)?;
        let mut path_prefixes: Vec<String> = cfg
            .allowlist
            .allow_path_prefixes
            .iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
        if path_prefixes.is_empty() {
            path_prefixes.push("/".to_string());
        }
        path_prefixes.sort_by_key(|p| std::cmp::Reverse(p.len()));
        let default_base = cfg
            .corporate
            .default_base
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let mut corporate_bases: Vec<String> = cfg
            .corporate
            .pool_bases
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if corporate_bases.is_empty() {
            if let Some(b) = default_base {
                corporate_bases.push(b);
            }
        }
        let default_headers = build_default_egress_headers(&cfg.default_headers)?;
        let profile_headers = build_profile_header_maps(&cfg.profiles)?;
        let max_retries = cfg.retry.max_retries;
        let retry_initial_backoff = Duration::from_millis(cfg.retry.initial_backoff_ms);
        let retry_max_backoff = Duration::from_millis(cfg.retry.max_backoff_ms);
        let in_flight = if cfg.rate_limit.max_in_flight > 0 {
            Some(Arc::new(Semaphore::new(
                cfg.rate_limit.max_in_flight as usize,
            )))
        } else {
            None
        };
        let mut rps_redis_prefix = cfg.rate_limit.redis.key_prefix.trim().to_string();
        if !rps_redis_prefix.ends_with(':') {
            rps_redis_prefix.push(':');
        }
        let rps_redis = if let Some(url) = cfg.effective_rate_limit_redis_url() {
            let client = redis::Client::open(url.as_str())?;
            Some(redis::aio::ConnectionManager::new(client).await?)
        } else {
            None
        };
        let mut rps_per_route_caps = HashMap::new();
        for pr in &cfg.rate_limit.per_route {
            let label = pr.route_label.trim().to_string();
            if !label.is_empty() && pr.max_rps > 0 {
                rps_per_route_caps.insert(label, pr.max_rps);
            }
        }
        let rps_global_cap = cfg.rate_limit.max_rps;
        Ok(Some(Arc::new(Self {
            client,
            pool_idle,
            tls,
            corporate_bases,
            corporate_rr: Arc::new(AtomicUsize::new(0)),
            timeout: Duration::from_millis(timeout_ms),
            allow_hosts,
            path_prefixes,
            default_headers,
            profile_headers,
            max_retries,
            retry_initial_backoff,
            retry_max_backoff,
            metrics: EgressMetrics::default(),
            in_flight,
            rps_global_cap,
            rps_global_local: Mutex::new((Instant::now(), 0u32)),
            rps_redis,
            rps_redis_prefix,
            rps_per_route_caps,
            rps_per_route_local: Mutex::new(HashMap::new()),
        })))
    }

    /// Rebuild the HTTPS client from [`Self::tls`] (re-read PEM files from disk).
    pub async fn reload_http_client(&self) -> anyhow::Result<()> {
        let c = build_egress_http_client(self.pool_idle, &self.tls)?;
        *self.client.write().await = c;
        Ok(())
    }

    pub fn metrics(&self) -> EgressMetrics {
        self.metrics.clone()
    }

    pub fn prometheus_text(&self) -> String {
        self.metrics.prometheus_text()
    }

    pub async fn request(&self, req: EgressHttpRequest) -> Result<EgressHttpResponse, EgressError> {
        let route = req.route_label.trim();
        if route.is_empty() {
            return Err(EgressError::Misconfigured("route_label must be non-empty"));
        }
        let t = req.target.trim();
        let (pool_slot, relative_base): (String, Option<String>) = if t.starts_with("http://")
            || t.starts_with("https://")
        {
            ("-".to_string(), None)
        } else {
            let n = self.corporate_bases.len();
            if n == 0 {
                return Err(EgressError::Misconfigured(
                        "relative egress path requires api_gateway.egress.corporate.default_base or pool_bases",
                    ));
            }
            let idx = self.corporate_rr.fetch_add(1, Ordering::Relaxed) % n;
            (idx.to_string(), Some(self.corporate_bases[idx].clone()))
        };
        let _in_flight_permit = match &self.in_flight {
            None => None,
            Some(sem) => match sem.try_acquire() {
                Ok(p) => Some(p),
                Err(_) => {
                    self.record_final_error_metrics(
                        route,
                        &EgressError::RateLimited,
                        0,
                        pool_slot.as_str(),
                    );
                    return Err(EgressError::RateLimited);
                }
            },
        };
        if let Err(e) = self.consume_rps(route).await {
            self.record_final_error_metrics(route, &e, 0, pool_slot.as_str());
            return Err(e);
        }
        let started = Instant::now();
        let mut attempt: u32 = 0;
        loop {
            let once = self
                .request_once_without_metrics(&req, relative_base.as_deref())
                .await;
            match once {
                Ok(resp) => {
                    if egress_http_status_retryable(resp.status) && attempt < self.max_retries {
                        self.metrics.record_retry(route);
                        tokio::time::sleep(egress_retry_delay(
                            self.retry_initial_backoff,
                            self.retry_max_backoff,
                            attempt,
                        ))
                        .await;
                        attempt = attempt.saturating_add(1);
                        continue;
                    }
                    self.record_http_success_metrics(
                        route,
                        resp.status,
                        started.elapsed().as_millis() as u64,
                        pool_slot.as_str(),
                    );
                    return Ok(resp);
                }
                Err(ref e) if egress_err_retryable(e) && attempt < self.max_retries => {
                    self.metrics.record_retry(route);
                    tokio::time::sleep(egress_retry_delay(
                        self.retry_initial_backoff,
                        self.retry_max_backoff,
                        attempt,
                    ))
                    .await;
                    attempt = attempt.saturating_add(1);
                    continue;
                }
                Err(e) => {
                    self.record_final_error_metrics(
                        route,
                        &e,
                        started.elapsed().as_millis() as u64,
                        pool_slot.as_str(),
                    );
                    return Err(e);
                }
            }
        }
    }

    fn record_http_success_metrics(
        &self,
        route: &str,
        status: u16,
        latency_ms: u64,
        pool_slot: &str,
    ) {
        let result = if status >= 500 {
            "5xx"
        } else if status >= 400 {
            "4xx"
        } else {
            "ok"
        };
        self.metrics.record(route, result, latency_ms, pool_slot);
    }

    fn record_final_error_metrics(
        &self,
        route: &str,
        e: &EgressError,
        latency_ms: u64,
        pool_slot: &str,
    ) {
        let result = match e {
            EgressError::AllowlistDenied => "allowlist_denied",
            EgressError::Timeout => "timeout",
            EgressError::BodyTooLarge => "5xx",
            EgressError::RateLimited => "rate_limited",
            _ => "error",
        };
        self.metrics.record(route, result, latency_ms, pool_slot);
    }

    async fn request_once_without_metrics(
        &self,
        req: &EgressHttpRequest,
        relative_base: Option<&str>,
    ) -> Result<EgressHttpResponse, EgressError> {
        let uri = resolve_uri(&req.target, relative_base)?;
        self.check_allowlist(&uri)?;
        let host = uri
            .host()
            .ok_or_else(|| EgressError::InvalidUrl("missing host".into()))?;
        let port = uri
            .port_u16()
            .unwrap_or_else(|| default_port(uri.scheme_str().unwrap_or("http")));
        let body = if let Some(b) = req.body.clone() {
            Full::new(b)
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync()
        } else {
            Full::new(Bytes::new())
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync()
        };
        let profile_name = req
            .egress_profile
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let profile_layer: Option<&HeaderMap> = if let Some(n) = profile_name {
            Some(
                self.profile_headers
                    .get(n)
                    .ok_or(EgressError::Misconfigured(
                        "unknown egress_profile (not in api_gateway.egress.profiles)",
                    ))?,
            )
        } else {
            None
        };
        let merged = merge_egress_header_chain(&self.default_headers, profile_layer, &req.headers);
        let mut builder = Request::builder()
            .method(req.method.clone())
            .uri(uri.clone());
        for (k, v) in merged.iter() {
            if k == header::HOST {
                continue;
            }
            builder = builder.header(k, v);
        }
        let authority = format!("{host}:{port}");
        let hyper_req = builder
            .header(
                header::HOST,
                HeaderValue::from_str(&authority)
                    .map_err(|_| EgressError::InvalidUrl("invalid Host header value".into()))?,
            )
            .body(body)
            .map_err(|e| EgressError::InvalidUrl(e.to_string()))?;
        let client = self.client.read().await.clone();
        let send = client.request(hyper_req);
        let resp = match tokio::time::timeout(self.timeout, send).await {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => return Err(EgressError::Upstream(e.to_string())),
            Err(_) => return Err(EgressError::Timeout),
        };
        let status = resp.status().as_u16();
        let (parts, body_in) = resp.into_parts();
        let mut out_headers = HeaderMap::new();
        for (k, v) in parts.headers.iter() {
            out_headers.append(k.clone(), v.clone());
        }
        let collected =
            match crate::collect_body_bounded(body_in, MAX_EGRESS_RESPONSE_BODY_BYTES).await {
                Ok(b) => b,
                Err(_) => return Err(EgressError::BodyTooLarge),
            };
        Ok(EgressHttpResponse {
            status,
            headers: out_headers,
            body: collected,
        })
    }

    fn rps_local_fixed(lock: &Mutex<(Instant, u32)>, cap: u32) -> Result<(), EgressError> {
        let mut g = lock.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let (window_start, count) = &mut *g;
        if now.duration_since(*window_start) >= Duration::from_secs(1) {
            *window_start = now;
            *count = 0;
        }
        if *count >= cap {
            return Err(EgressError::RateLimited);
        }
        *count += 1;
        Ok(())
    }

    fn rps_local_map(
        lock: &Mutex<HashMap<String, (Instant, u32)>>,
        key: &str,
        cap: u32,
    ) -> Result<(), EgressError> {
        let mut g = lock.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let e = g.entry(key.to_string()).or_insert((now, 0u32));
        if now.duration_since(e.0) >= Duration::from_secs(1) {
            e.0 = now;
            e.1 = 0;
        }
        if e.1 >= cap {
            return Err(EgressError::RateLimited);
        }
        e.1 += 1;
        Ok(())
    }

    async fn egress_rps_redis_or_local<F>(
        conn: &redis::aio::ConnectionManager,
        full_key: &str,
        cap: u32,
        local: F,
    ) -> Result<(), EgressError>
    where
        F: FnOnce() -> Result<(), EgressError>,
    {
        let mut c = conn.clone();
        match c.incr::<_, _, i64>(full_key, 1i64).await {
            Ok(n) => {
                if n == 1 {
                    let _: redis::RedisResult<()> = c.expire(full_key, 1).await;
                }
                if n > cap as i64 {
                    return Err(EgressError::RateLimited);
                }
                Ok(())
            }
            Err(e) => {
                tracing::warn!(target: "panda::egress_rps", "redis egress RPS incr failed, using local window: {e}");
                local()
            }
        }
    }

    async fn consume_rps(&self, route: &str) -> Result<(), EgressError> {
        if self.rps_global_cap == 0 && self.rps_per_route_caps.is_empty() {
            return Ok(());
        }
        if self.rps_global_cap > 0 {
            let full_key = format!("{}global", self.rps_redis_prefix);
            let r = if let Some(ref conn) = self.rps_redis {
                Self::egress_rps_redis_or_local(conn, &full_key, self.rps_global_cap, || {
                    Self::rps_local_fixed(&self.rps_global_local, self.rps_global_cap)
                })
                .await
            } else {
                Self::rps_local_fixed(&self.rps_global_local, self.rps_global_cap)
            };
            match r {
                Ok(()) => self.metrics.record_rps_gate("global", "-", false),
                Err(e) => {
                    self.metrics.record_rps_gate("global", "-", true);
                    return Err(e);
                }
            }
        }
        if let Some(cap) = self.rps_per_route_caps.get(route).copied() {
            if cap > 0 {
                let route_label = route.to_string();
                let full_key = format!("{}route:{route_label}", self.rps_redis_prefix);
                let r = if let Some(ref conn) = self.rps_redis {
                    Self::egress_rps_redis_or_local(conn, &full_key, cap, || {
                        Self::rps_local_map(&self.rps_per_route_local, route_label.as_str(), cap)
                    })
                    .await
                } else {
                    Self::rps_local_map(&self.rps_per_route_local, route_label.as_str(), cap)
                };
                match r {
                    Ok(()) => self.metrics.record_rps_gate("per_route", route, false),
                    Err(e) => {
                        self.metrics.record_rps_gate("per_route", route, true);
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    fn check_allowlist(&self, uri: &Uri) -> Result<(), EgressError> {
        let scheme = uri.scheme_str().unwrap_or("");
        if scheme != "http" && scheme != "https" {
            return Err(EgressError::AllowlistDenied);
        }
        let host = uri.host().ok_or(EgressError::AllowlistDenied)?;
        let port = uri.port_u16().unwrap_or_else(|| default_port(scheme));
        if !host_allowed(&self.allow_hosts, host, port, scheme) {
            return Err(EgressError::AllowlistDenied);
        }
        let path = uri.path();
        if !path_allowed(&self.path_prefixes, path) {
            return Err(EgressError::AllowlistDenied);
        }
        Ok(())
    }
}

fn egress_http_status_retryable(status: u16) -> bool {
    matches!(status, 429 | 502 | 503 | 504)
}

fn egress_err_retryable(e: &EgressError) -> bool {
    matches!(e, EgressError::Timeout | EgressError::Upstream(_))
}

fn egress_retry_delay(initial: Duration, max: Duration, attempt: u32) -> Duration {
    let pow = attempt.min(24);
    let mult = 1u64 << pow;
    let ms_u128 = initial.as_millis().saturating_mul(mult as u128);
    let cap = max.as_millis().max(1);
    let ms = ms_u128.min(cap) as u64;
    Duration::from_millis(ms.max(1))
}

fn merge_egress_header_chain(
    defaults: &HeaderMap,
    profile: Option<&HeaderMap>,
    req: &HeaderMap,
) -> HeaderMap {
    let mut out = HeaderMap::new();
    for (k, v) in defaults.iter() {
        out.append(k, v.clone());
    }
    if let Some(p) = profile {
        for (k, v) in p.iter() {
            out.insert(k, v.clone());
        }
    }
    for (k, v) in req.iter() {
        out.insert(k, v.clone());
    }
    out
}

fn build_profile_header_maps(
    profiles: &[ApiGatewayEgressProfile],
) -> anyhow::Result<HashMap<String, HeaderMap>> {
    let mut out = HashMap::new();
    for prof in profiles {
        let name = prof.name.trim();
        if name.is_empty() {
            continue;
        }
        let hm = build_default_egress_headers(&prof.default_headers)?;
        out.insert(name.to_string(), hm);
    }
    Ok(out)
}

fn build_default_egress_headers(
    entries: &[ApiGatewayEgressDefaultHeader],
) -> anyhow::Result<HeaderMap> {
    let mut m = HeaderMap::new();
    for h in entries {
        let name = h.name.trim();
        let raw = if let Some(ref v) = h.value {
            v.clone()
        } else if let Some(ref k) = h.value_env {
            std::env::var(k.trim()).map_err(|_| {
                anyhow::anyhow!("egress default_headers: missing env {:?}", k.trim())
            })?
        } else {
            continue;
        };
        let hn = http::HeaderName::from_bytes(name.as_bytes()).map_err(|_| {
            anyhow::anyhow!("egress default_headers: invalid header name {:?}", h.name)
        })?;
        let hv = HeaderValue::from_str(raw.trim()).map_err(|_| {
            anyhow::anyhow!(
                "egress default_headers {:?}: value is not a valid header value",
                h.name
            )
        })?;
        m.insert(hn, hv);
    }
    Ok(m)
}

fn default_port(scheme: &str) -> u16 {
    match scheme {
        "https" => 443,
        _ => 80,
    }
}

fn parse_allow_hosts(entries: &[String]) -> Result<Vec<AllowedHost>, EgressError> {
    let mut out = Vec::new();
    for e in entries {
        let e = e.trim();
        if e.is_empty() {
            continue;
        }
        if e.contains('/') {
            return Err(EgressError::Misconfigured(
                "allow_hosts must be hostname or host:port only",
            ));
        }
        if let Some(colon) = e.rfind(':') {
            let tail = &e[colon + 1..];
            if !tail.is_empty() && tail.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(p) = tail.parse::<u16>() {
                    out.push(AllowedHost {
                        host_lower: e[..colon].to_ascii_lowercase(),
                        port: Some(p),
                    });
                    continue;
                }
            }
        }
        out.push(AllowedHost {
            host_lower: e.to_ascii_lowercase(),
            port: None,
        });
    }
    if out.is_empty() {
        return Err(EgressError::Misconfigured(
            "allow_hosts resolved empty (configure api_gateway.egress.allowlist.allow_hosts)",
        ));
    }
    Ok(out)
}

fn host_allowed(allow: &[AllowedHost], host: &str, port: u16, scheme: &str) -> bool {
    let h = host.to_ascii_lowercase();
    let default_p = default_port(scheme);
    for a in allow {
        if a.host_lower != h {
            continue;
        }
        match a.port {
            Some(p) => {
                if p == port {
                    return true;
                }
            }
            None => {
                if port == default_p {
                    return true;
                }
            }
        }
    }
    false
}

fn path_allowed(prefixes: &[String], path: &str) -> bool {
    prefixes.iter().any(|p| path.starts_with(p))
}

fn resolve_uri(target: &str, default_base: Option<&str>) -> Result<Uri, EgressError> {
    let t = target.trim();
    if t.is_empty() {
        return Err(EgressError::InvalidUrl("empty target".into()));
    }
    if t.starts_with("http://") || t.starts_with("https://") {
        return t
            .parse()
            .map_err(|e: http::uri::InvalidUri| EgressError::InvalidUrl(e.to_string()));
    }
    if !t.starts_with('/') {
        return Err(EgressError::InvalidUrl(
            "relative target must start with '/'".into(),
        ));
    }
    let Some(base) = default_base else {
        return Err(EgressError::Misconfigured(
            "relative egress path requires corporate default_base or pool_bases",
        ));
    };
    let base = base.trim_end_matches('/');
    format!("{base}{t}")
        .parse()
        .map_err(|e: http::uri::InvalidUri| EgressError::InvalidUrl(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::{ApiGatewayEgressCorporateConfig, ApiGatewayEgressProfile};
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[test]
    fn allowlist_host_port_and_path() {
        let hosts = parse_allow_hosts(&["127.0.0.1:9".to_string()]).unwrap();
        assert!(host_allowed(&hosts, "127.0.0.1", 9, "http"));
        assert!(!host_allowed(&hosts, "127.0.0.1", 80, "http"));
        let prefixes = vec!["/allowed".to_string()];
        assert!(path_allowed(&prefixes, "/allowed/x"));
        assert!(!path_allowed(&prefixes, "/denied"));
    }

    #[test]
    fn resolve_relative_requires_base() {
        assert!(matches!(
            resolve_uri("/x", None),
            Err(EgressError::Misconfigured(_))
        ));
        let u = resolve_uri("/x", Some("https://a.example.com")).unwrap();
        assert_eq!(u.to_string(), "https://a.example.com/x");
    }

    #[tokio::test]
    async fn integration_hits_mock_upstream_when_allowed() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let body_json = r#"{"ok":true}"#;

        tokio::spawn(async move {
            let Ok((mut sock, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let Ok(n) = sock.read(&mut buf).await else {
                return;
            };
            let req = std::str::from_utf8(&buf[..n]).expect("utf8");
            assert!(
                req.contains("GET /allowed/health"),
                "unexpected request head: {}",
                req.chars().take(160).collect::<String>()
            );
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
                body_json.len(),
                body_json
            );
            let _ = sock.write_all(resp.as_bytes()).await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static("panda-egress-test"),
        );
        let res = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/health".to_string(),
                route_label: "integration_test".to_string(),
                egress_profile: None,
                headers,
                body: None,
            })
            .await
            .expect("egress ok");
        assert_eq!(res.status, 200);
        assert_eq!(res.body.as_ref(), body_json.as_bytes());
        let prom = client.prometheus_text();
        assert!(prom.contains("panda_egress_requests_total"));
        assert!(prom.contains("result=\"ok\""));
        assert!(prom.contains("route=\"integration_test\""));
        assert!(prom.contains("pool_slot=\"0\""));
    }

    #[tokio::test]
    async fn corporate_pool_round_robin_two_bases() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let l1 = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let l2 = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let a1 = l1.local_addr().unwrap();
        let a2 = l2.local_addr().unwrap();
        let p1 = a1.port();
        let p2 = a2.port();

        tokio::spawn(async move {
            let Ok((mut sock, _)) = l1.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let _ = sock.read(&mut buf).await;
            let _ = sock
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\none")
                .await;
        });
        tokio::spawn(async move {
            let Ok((mut sock, _)) = l2.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let _ = sock.read(&mut buf).await;
            let _ = sock
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\ntwo")
                .await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: None,
                pool_bases: vec![
                    format!("http://127.0.0.1:{p1}"),
                    format!("http://127.0.0.1:{p2}"),
                ],
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{p1}"), format!("127.0.0.1:{p2}")],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let headers = HeaderMap::new();
        let r1 = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/a".to_string(),
                route_label: "pool_rr".to_string(),
                egress_profile: None,
                headers: headers.clone(),
                body: None,
            })
            .await
            .expect("r1");
        let r2 = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/b".to_string(),
                route_label: "pool_rr".to_string(),
                egress_profile: None,
                headers,
                body: None,
            })
            .await
            .expect("r2");
        assert_eq!(r1.body.as_ref(), b"one");
        assert_eq!(r2.body.as_ref(), b"two");
        let prom = client.prometheus_text();
        assert!(prom.contains("pool_slot=\"0\""));
        assert!(prom.contains("pool_slot=\"1\""));
    }

    #[tokio::test]
    async fn allowlist_denies_wrong_path_prefix() {
        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some("http://127.0.0.1:1".to_string()),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec!["127.0.0.1".to_string()],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let err = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/secret".to_string(),
                route_label: "test".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, EgressError::AllowlistDenied));
    }

    #[tokio::test]
    async fn retries_on_503_then_succeeds() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc as StdArc;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let hits = StdArc::new(AtomicUsize::new(0));
        let hits_t = StdArc::clone(&hits);
        tokio::spawn(async move {
            for _ in 0..2 {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let mut buf = vec![0u8; 16_384];
                let Ok(n) = sock.read(&mut buf).await else {
                    break;
                };
                let _ = std::str::from_utf8(&buf[..n]).expect("utf8");
                let n = hits_t.fetch_add(1, Ordering::SeqCst) + 1;
                let resp = if n == 1 {
                    "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 2\r\nConnection: close\r\n\r\nno"
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
                };
                let _ = sock.write_all(resp.as_bytes()).await;
            }
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            retry: panda_config::ApiGatewayEgressRetryConfig {
                max_retries: 2,
                initial_backoff_ms: 5,
                max_backoff_ms: 50,
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let res = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/r".to_string(),
                route_label: "retry_test".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .expect("egress ok after retry");
        assert_eq!(res.status, 200);
        assert_eq!(hits.load(Ordering::SeqCst), 2);
        let prom = client.prometheus_text();
        assert!(prom.contains("panda_egress_retries_total"));
        assert!(prom.contains("route=\"retry_test\""));
    }

    #[tokio::test]
    async fn default_headers_sent_to_upstream() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let Ok((mut sock, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let Ok(n) = sock.read(&mut buf).await else {
                return;
            };
            let req = std::str::from_utf8(&buf[..n]).expect("utf8");
            assert!(
                req.to_ascii_lowercase().contains("x-test-header: alpha"),
                "{req:?}"
            );
            let resp = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            let _ = sock.write_all(resp.as_bytes()).await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            default_headers: vec![panda_config::ApiGatewayEgressDefaultHeader {
                name: "X-Test-Header".to_string(),
                value: Some("alpha".to_string()),
                value_env: None,
            }],
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/z".to_string(),
                route_label: "hdr_test".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .expect("ok");
    }

    #[tokio::test]
    async fn retries_on_429_then_succeeds() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc as StdArc;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let hits = StdArc::new(AtomicUsize::new(0));
        let hits_t = StdArc::clone(&hits);
        tokio::spawn(async move {
            for _ in 0..2 {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let mut buf = vec![0u8; 16_384];
                let Ok(n) = sock.read(&mut buf).await else {
                    break;
                };
                let _ = std::str::from_utf8(&buf[..n]).expect("utf8");
                let n = hits_t.fetch_add(1, Ordering::SeqCst) + 1;
                let resp = if n == 1 {
                    "HTTP/1.1 429 Too Many Requests\r\nContent-Length: 2\r\nConnection: close\r\n\r\nno"
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok"
                };
                let _ = sock.write_all(resp.as_bytes()).await;
            }
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            retry: panda_config::ApiGatewayEgressRetryConfig {
                max_retries: 2,
                initial_backoff_ms: 5,
                max_backoff_ms: 50,
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let res = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/r429".to_string(),
                route_label: "retry_429_test".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .expect("egress ok after 429 retry");
        assert_eq!(res.status, 200);
        assert_eq!(hits.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn egress_profile_merges_headers_request_wins_on_dup() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let Ok((mut sock, _)) = listener.accept().await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let Ok(n) = sock.read(&mut buf).await else {
                return;
            };
            let req = std::str::from_utf8(&buf[..n]).expect("utf8");
            let lower = req.to_ascii_lowercase();
            assert!(
                lower.contains("x-global: alpha"),
                "global header missing: {req:?}"
            );
            assert!(
                lower.contains("x-from-profile: beta"),
                "profile header missing: {req:?}"
            );
            let resp = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            let _ = sock.write_all(resp.as_bytes()).await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            default_headers: vec![panda_config::ApiGatewayEgressDefaultHeader {
                name: "X-Global".to_string(),
                value: Some("ignored".to_string()),
                value_env: None,
            }],
            profiles: vec![ApiGatewayEgressProfile {
                name: "corp".to_string(),
                default_headers: vec![panda_config::ApiGatewayEgressDefaultHeader {
                    name: "X-From-Profile".to_string(),
                    value: Some("beta".to_string()),
                    value_env: None,
                }],
            }],
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let mut headers = HeaderMap::new();
        headers.insert(
            http::HeaderName::from_static("x-global"),
            HeaderValue::from_static("alpha"),
        );
        client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/prof".to_string(),
                route_label: "prof_test".to_string(),
                egress_profile: Some("corp".to_string()),
                headers,
                body: None,
            })
            .await
            .expect("ok");
    }

    #[tokio::test]
    async fn unknown_egress_profile_is_misconfigured() {
        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some("http://127.0.0.1:1".to_string()),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec!["127.0.0.1:1".to_string()],
                allow_path_prefixes: vec!["/".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let err = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/x".to_string(),
                route_label: "bad_prof".to_string(),
                egress_profile: Some("nope".to_string()),
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, EgressError::Misconfigured(_)));
    }

    /// Local HTTPS listener that **requires** a client certificate (mTLS). Proves egress uses
    /// `api_gateway.egress.tls` client cert + corporate CA trust for the pooled Hyper client.
    #[tokio::test]
    async fn integration_https_mtls_presents_client_cert_to_upstream() {
        use rcgen::{BasicConstraints, CertificateParams, IsCa, KeyPair, SanType};
        use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
        use rustls::server::WebPkiClientVerifier;
        use rustls::RootCertStore;
        use std::sync::Arc;
        use tokio_rustls::TlsAcceptor;

        crate::ensure_rustls_ring_provider();

        let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
        let ca_key = KeyPair::generate_for(alg).expect("ca key");
        let mut ca_params = CertificateParams::new(vec![]).expect("ca params");
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        let ca_cert = ca_params.self_signed(&ca_key).expect("ca cert");

        let mut server_params =
            CertificateParams::new(vec!["egress-mtls-test".to_string()]).expect("srv params");
        server_params.subject_alt_names = vec![SanType::IpAddress(std::net::IpAddr::V4(
            std::net::Ipv4Addr::LOCALHOST,
        ))];
        let server_key = KeyPair::generate_for(alg).expect("srv key");
        let server_cert = server_params
            .signed_by(&server_key, &ca_cert, &ca_key)
            .expect("srv cert");

        let client_params =
            CertificateParams::new(vec!["panda-egress".to_string()]).expect("cli params");
        let client_key = KeyPair::generate_for(alg).expect("cli key");
        let client_cert = client_params
            .signed_by(&client_key, &ca_cert, &ca_key)
            .expect("cli cert");

        let dir = tempfile::tempdir().expect("tempdir");
        let ca_path = dir.path().join("ca.pem");
        let client_cert_path = dir.path().join("client.pem");
        let client_key_path = dir.path().join("client-key.pem");
        std::fs::write(&ca_path, ca_cert.pem()).unwrap();
        std::fs::write(&client_cert_path, client_cert.pem()).unwrap();
        std::fs::write(&client_key_path, client_key.serialize_pem()).unwrap();

        let ca_der = CertificateDer::from(ca_cert.der().to_vec());
        let mut client_auth_roots = RootCertStore::empty();
        client_auth_roots
            .add(ca_der)
            .expect("add ca to client-auth roots");
        let verifier = WebPkiClientVerifier::builder(Arc::new(client_auth_roots))
            .build()
            .expect("WebPkiClientVerifier");

        let server_chain = vec![CertificateDer::from(server_cert.der().to_vec())];
        let server_key_der =
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(server_key.serialize_der()));
        let server_config = rustls::ServerConfig::builder()
            .with_client_cert_verifier(verifier)
            .with_single_cert(server_chain, server_key_der)
            .expect("server tls config");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();

        let server_cfg = Arc::new(server_config);
        tokio::spawn(async move {
            let Ok((stream, _)) = listener.accept().await else {
                return;
            };
            let acceptor = TlsAcceptor::from(server_cfg);
            let Ok(mut tls) = acceptor.accept(stream).await else {
                return;
            };
            let mut buf = vec![0u8; 16_384];
            let Ok(n) = tls.read(&mut buf).await else {
                return;
            };
            let req = std::str::from_utf8(&buf[..n]).expect("utf8");
            assert!(
                req.contains("GET /allowed/mtls"),
                "unexpected request: {}",
                req.chars().take(200).collect::<String>()
            );
            let peer = tls.get_ref().1.peer_certificates();
            assert!(
                peer.is_some_and(|c| !c.is_empty()),
                "server should receive a client certificate over mTLS"
            );
            let resp = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok";
            let _ = tls.write_all(resp.as_bytes()).await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 10_000,
            pool_idle_timeout_ms: 0,
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("https://127.0.0.1:{port}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{port}")],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            tls: panda_config::ApiGatewayEgressTlsConfig {
                client_cert_pem: Some(client_cert_path.to_str().unwrap().to_string()),
                client_key_pem: Some(client_key_path.to_str().unwrap().to_string()),
                extra_ca_pem: Some(ca_path.to_str().unwrap().to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let res = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/mtls".to_string(),
                route_label: "mtls_test".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .expect("egress mTLS");
        assert_eq!(res.status, 200);
        assert_eq!(res.body.as_ref(), b"ok");
    }

    #[tokio::test]
    async fn rate_limit_max_rps_denies_excess_requests_in_same_second() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let mut buf = vec![0u8; 16_384];
                let _ = sock.read(&mut buf).await;
                let _ = sock
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                    )
                    .await;
            }
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            rate_limit: panda_config::ApiGatewayEgressRateLimitConfig {
                max_in_flight: 0,
                max_rps: 2,
                ..Default::default()
            },
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let headers = HeaderMap::new();
        for i in 0..2 {
            client
                .request(EgressHttpRequest {
                    method: Method::GET,
                    target: "/allowed/rps".to_string(),
                    route_label: "rps_cap".to_string(),
                    egress_profile: None,
                    headers: headers.clone(),
                    body: None,
                })
                .await
                .unwrap_or_else(|e| panic!("request {i}: {e}"));
        }
        let err = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/rps3".to_string(),
                route_label: "rps_cap".to_string(),
                egress_profile: None,
                headers,
                body: None,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, EgressError::RateLimited));
        let prom = client.prometheus_text();
        assert!(prom.contains("rate_limited"));
        assert!(prom.contains("panda_egress_rps_total"));
        assert!(prom.contains("scope=\"global\""));
        assert!(prom.contains("result=\"denied\""));
    }

    #[tokio::test]
    async fn rate_limit_max_in_flight_blocks_second_concurrent_call() {
        use tokio::sync::Notify;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let ready = Arc::new(Notify::new());
        let proceed = Arc::new(Notify::new());
        let ready_s = Arc::clone(&ready);
        let proceed_s = Arc::clone(&proceed);

        tokio::spawn(async move {
            let Ok((mut sock, _)) = listener.accept().await else {
                return;
            };
            ready_s.notify_one();
            proceed_s.notified().await;
            let mut buf = vec![0u8; 16_384];
            let _ = sock.read(&mut buf).await;
            let _ = sock
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
                .await;
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            rate_limit: panda_config::ApiGatewayEgressRateLimitConfig {
                max_in_flight: 1,
                max_rps: 0,
                ..Default::default()
            },
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = Arc::new(EgressClient::connect(&cfg).await.expect("new").expect("some"));
        let c1 = Arc::clone(&client);
        let j1 = tokio::spawn(async move {
            c1.request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/a".to_string(),
                route_label: "in_flight".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
        });
        ready.notified().await;
        let err = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/b".to_string(),
                route_label: "in_flight".to_string(),
                egress_profile: None,
                headers: HeaderMap::new(),
                body: None,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, EgressError::RateLimited));
        proceed.notify_one();
        j1.await.expect("join").expect("first request ok");
    }

    #[tokio::test]
    async fn rate_limit_per_route_rps_metrics_scope() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let mut buf = vec![0u8; 16_384];
                let _ = sock.read(&mut buf).await;
                let _ = sock
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                    )
                    .await;
            }
        });

        let cfg = ApiGatewayEgressConfig {
            enabled: true,
            timeout_ms: 5_000,
            pool_idle_timeout_ms: 0,
            rate_limit: panda_config::ApiGatewayEgressRateLimitConfig {
                max_in_flight: 0,
                max_rps: 0,
                per_route: vec![panda_config::ApiGatewayEgressPerRouteRateLimit {
                    route_label: "per_route_only".to_string(),
                    max_rps: 1,
                }],
                ..Default::default()
            },
            corporate: ApiGatewayEgressCorporateConfig {
                default_base: Some(format!("http://{addr}")),
                ..Default::default()
            },
            allowlist: panda_config::ApiGatewayEgressAllowlistConfig {
                allow_hosts: vec![format!("127.0.0.1:{}", addr.port())],
                allow_path_prefixes: vec!["/allowed".to_string()],
            },
            ..Default::default()
        };
        let client = EgressClient::connect(&cfg).await.expect("new").expect("some");
        let headers = HeaderMap::new();
        client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/one".to_string(),
                route_label: "per_route_only".to_string(),
                egress_profile: None,
                headers: headers.clone(),
                body: None,
            })
            .await
            .expect("first");
        let err = client
            .request(EgressHttpRequest {
                method: Method::GET,
                target: "/allowed/two".to_string(),
                route_label: "per_route_only".to_string(),
                egress_profile: None,
                headers,
                body: None,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, EgressError::RateLimited));
        let prom = client.prometheus_text();
        assert!(prom.contains("panda_egress_rps_total"));
        assert!(prom.contains("scope=\"per_route\""));
        assert!(prom.contains("route=\"per_route_only\""));
        assert!(prom.contains("result=\"allowed\""));
        assert!(prom.contains("result=\"denied\""));
    }
}

//! Per-route and ingress-row HTTP RPS limits ([`panda_config::RouteRateLimitConfig`]).
//!
//! Default: process-local fixed **1 second** windows. When `api_gateway.ingress.rate_limit_redis.url_env`
//! resolves to a Redis URL, matching counters use **`INCR` + `EXPIRE 1`** for shared limits across replicas.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use panda_config::PandaConfig;
use redis::AsyncCommands;

use crate::api_gateway::ingress::IngressRpsKey;

/// Longest-prefix route with `rate_limit` shares one fixed 1s window per `path_prefix` (process-local
/// unless Redis is configured).
pub struct RouteRpsLimiters {
    legacy_limits: HashMap<String, u32>,
    windows: Mutex<HashMap<String, (Instant, u32)>>,
    redis: Option<redis::aio::ConnectionManager>,
    redis_key_prefix: String,
}

impl RouteRpsLimiters {
    /// Whether any code path may call [`Self::check_route`] / [`Self::check_ingress`].
    pub fn needs_layer(cfg: &PandaConfig) -> bool {
        cfg.routes.iter().any(|r| r.rate_limit.is_some())
            || cfg
                .api_gateway
                .ingress
                .routes
                .iter()
                .any(|r| r.rate_limit.is_some())
            || cfg.api_gateway.ingress.enabled
    }

    pub async fn connect(cfg: Arc<PandaConfig>) -> anyhow::Result<Option<Arc<Self>>> {
        if !Self::needs_layer(&cfg) {
            return Ok(None);
        }
        let mut limits = HashMap::new();
        for r in &cfg.routes {
            if let Some(ref rl) = r.rate_limit {
                limits.insert(r.path_prefix.clone(), rl.rps);
            }
        }
        let redis_url = cfg.effective_api_gateway_ingress_rate_limit_redis_url();
        let redis = if let Some(url) = redis_url {
            let client = redis::Client::open(url.as_str())?;
            Some(redis::aio::ConnectionManager::new(client).await?)
        } else {
            None
        };
        let mut pfx = cfg
            .api_gateway
            .ingress
            .rate_limit_redis
            .key_prefix
            .trim()
            .to_string();
        if pfx.is_empty() {
            pfx = "panda:gw:ingress_rps".to_string();
        }
        if !pfx.ends_with(':') {
            pfx.push(':');
        }
        Ok(Some(Arc::new(Self {
            legacy_limits: limits,
            windows: Mutex::new(HashMap::new()),
            redis,
            redis_key_prefix: pfx,
        })))
    }

    fn legacy_map_key(path_prefix: &str) -> String {
        format!("legacy:{path_prefix}")
    }

    fn ingress_map_key(key: &IngressRpsKey) -> String {
        format!("i:{}:{}", key.tenant_id, key.path_prefix)
    }

    fn check_local(&self, map_key: &str, cap: u32) -> Result<(), u32> {
        let mut g = self.windows.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let e = g.entry(map_key.to_string()).or_insert((now, 0));
        if now.duration_since(e.0) >= Duration::from_secs(1) {
            e.0 = now;
            e.1 = 0;
        }
        if e.1 >= cap {
            return Err(cap);
        }
        e.1 += 1;
        Ok(())
    }

    async fn check_redis_or_local(&self, map_key: &str, cap: u32) -> Result<(), u32> {
        let full_key = format!("{}{}", self.redis_key_prefix, map_key);
        if let Some(ref conn) = self.redis {
            let mut c = conn.clone();
            match c.incr::<_, _, i64>(&full_key, 1i64).await {
                Ok(n) => {
                    if n == 1 {
                        let _: redis::RedisResult<()> = c.expire(&full_key, 1).await;
                    }
                    if n > cap as i64 {
                        return Err(cap);
                    }
                    Ok(())
                }
                Err(e) => {
                    tracing::warn!(target: "panda::route_rps", "redis RPS incr failed, using local window: {e}");
                    self.check_local(map_key, cap)
                }
            }
        } else {
            self.check_local(map_key, cap)
        }
    }

    /// Whether [`Self::check_route`] will enforce (not no-op) for this path.
    pub fn legacy_limit_applies(&self, cfg: &PandaConfig, ingress_path: &str) -> bool {
        let Some(route) = cfg.effective_route_for_path(ingress_path) else {
            return false;
        };
        route.rate_limit.is_some() && self.legacy_limits.contains_key(&route.path_prefix)
    }

    /// Top-level [`PandaConfig::routes`] `rate_limit` for the longest matching prefix.
    pub async fn check_route(&self, cfg: &PandaConfig, ingress_path: &str) -> Result<(), u32> {
        let Some(route) = cfg.effective_route_for_path(ingress_path) else {
            return Ok(());
        };
        let Some(ref rl) = route.rate_limit else {
            return Ok(());
        };
        if !self.legacy_limits.contains_key(&route.path_prefix) {
            return Ok(());
        }
        let cap = rl.rps;
        let map_key = Self::legacy_map_key(&route.path_prefix);
        self.check_redis_or_local(&map_key, cap).await
    }

    /// Ingress row RPS after a successful [`crate::api_gateway::ingress::classify_merged`].
    pub async fn check_ingress(&self, key: &IngressRpsKey) -> Result<(), u32> {
        let cap = key.rps;
        if cap == 0 {
            return Ok(());
        }
        let map_key = Self::ingress_map_key(key);
        self.check_redis_or_local(&map_key, cap).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::PandaConfig;

    #[tokio::test]
    async fn rps_enforces_cap_within_window() {
        let cfg = Arc::new(
            PandaConfig::from_yaml_str(
                r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api
    backend_base: 'http://127.0.0.1:2'
    rate_limit:
      rps: 2
"#,
            )
            .unwrap(),
        );
        let lim = RouteRpsLimiters::connect(Arc::clone(&cfg))
            .await
            .unwrap()
            .expect("limiters");
        assert!(lim.check_route(&cfg, "/api/a").await.is_ok());
        assert!(lim.check_route(&cfg, "/api/b").await.is_ok());
        assert!(lim.check_route(&cfg, "/api/c").await.is_err());
    }

    #[tokio::test]
    async fn rps_skips_when_no_matching_route() {
        let cfg = Arc::new(
            PandaConfig::from_yaml_str(
                r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
routes:
  - path_prefix: /api
    backend_base: 'http://127.0.0.1:2'
    rate_limit:
      rps: 1
"#,
            )
            .unwrap(),
        );
        let lim = RouteRpsLimiters::connect(Arc::clone(&cfg))
            .await
            .unwrap()
            .expect("limiters");
        assert!(lim.check_route(&cfg, "/other").await.is_ok());
        assert!(lim.check_route(&cfg, "/other").await.is_ok());
    }
}

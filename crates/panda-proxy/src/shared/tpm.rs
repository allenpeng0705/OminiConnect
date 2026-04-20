//! Prompt / completion counters: in-process and optional Redis (`INCRBY`).
//!
//! When Redis is configured but fails (connect or command), optional **degraded mode** tightens
//! effective per-minute limits (see `panda-config` `tpm.redis_*_degraded_*`).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use panda_config::TpmConfig;
use redis::AsyncCommands;

const PROMPT_BUDGET_LUA: &str = r#"
local cur = tonumber(redis.call('GET', KEYS[1]) or 0)
local n = tonumber(ARGV[1])
local limit = tonumber(ARGV[2])
if cur + n > limit then
  return 0
end
redis.call('INCRBY', KEYS[1], n)
redis.call('EXPIRE', KEYS[1], 120)
redis.call('INCRBY', KEYS[2], n)
return 1
"#;

/// Runtime TPM behavior when Redis is missing or flaky (from [`TpmConfig`]).
#[derive(Debug, Clone)]
pub struct TpmPolicy {
    pub degraded_limit_ratio: f64,
    pub tighten_on_redis_unavailable: bool,
    pub tighten_on_redis_command_error: bool,
}

impl TpmPolicy {
    pub fn from_config(tpm: &TpmConfig) -> Self {
        Self {
            degraded_limit_ratio: tpm.redis_degraded_limit_ratio,
            tighten_on_redis_unavailable: tpm.redis_unavailable_degraded_limits,
            tighten_on_redis_command_error: tpm.redis_command_error_degraded_limits,
        }
    }

    /// Tests / in-memory only: never tighten limits from Redis failures.
    pub fn passive() -> Self {
        Self {
            degraded_limit_ratio: 1.0,
            tighten_on_redis_unavailable: false,
            tighten_on_redis_command_error: false,
        }
    }
}

pub struct TpmCounters {
    mem: Mutex<HashMap<String, (u64, u64)>>,
    mem_prompt_window: Mutex<HashMap<String, (std::time::Instant, u64)>>,
    redis: Option<redis::aio::ConnectionManager>,
    degraded: AtomicBool,
    policy: TpmPolicy,
}

impl TpmCounters {
    /// In-memory only; ignores Redis (for tests and passive policy).
    pub async fn connect(url: Option<&str>) -> anyhow::Result<Self> {
        Self::connect_with_policy(url, TpmPolicy::passive()).await
    }

    pub async fn connect_with_policy(
        effective_redis_url: Option<&str>,
        policy: TpmPolicy,
    ) -> anyhow::Result<Self> {
        let url_nonempty = effective_redis_url.filter(|u| !u.trim().is_empty());
        let (redis, initial_degraded) = match url_nonempty {
            Some(url) => match connect_redis(url).await {
                Ok(r) => (Some(r), false),
                Err(e) => {
                    eprintln!("panda tpm: redis connect failed: {e}");
                    let d = policy.tighten_on_redis_unavailable;
                    (None, d)
                }
            },
            None => (None, false),
        };
        Ok(Self {
            mem: Mutex::new(HashMap::new()),
            mem_prompt_window: Mutex::new(HashMap::new()),
            redis,
            degraded: AtomicBool::new(initial_degraded),
            policy,
        })
    }

    fn mark_degraded(&self) {
        if self.policy.tighten_on_redis_command_error && self.redis.is_some() {
            self.degraded.store(true, Ordering::Release);
        }
    }

    fn mark_healthy_redis(&self) {
        if self.redis.is_some() {
            self.degraded.store(false, Ordering::Release);
        }
    }

    /// Effective per-minute limit while accounting for degraded “safe mode”.
    pub fn effective_budget_limit(&self, configured_limit_per_minute: u64) -> u64 {
        Self::scale_limit(
            configured_limit_per_minute,
            self.degraded.load(Ordering::Relaxed),
            self.policy.degraded_limit_ratio,
        )
    }

    pub fn redis_budget_degraded(&self) -> bool {
        self.degraded.load(Ordering::Relaxed)
    }

    fn scale_limit(limit: u64, degraded: bool, ratio: f64) -> u64 {
        if limit == 0 {
            return 0;
        }
        if !degraded {
            return limit;
        }
        let r = ratio.clamp(0.0, 1.0);
        if r <= 0.0 {
            return 1;
        }
        let scaled = ((limit as f64) * r).floor() as u64;
        scaled.max(1)
    }

    /// Atomically reserve `n` prompt tokens for the current minute window when under `limit`.
    /// Returns false when the request would exceed the budget (nothing is applied).
    pub async fn try_reserve_prompt_budget(
        &self,
        bucket: &str,
        n: u64,
        limit_per_minute: u64,
    ) -> bool {
        if n == 0 {
            return true;
        }
        let eff = self.effective_budget_limit(limit_per_minute);
        if eff == 0 {
            return false;
        }
        if let Some(ref mgr) = self.redis {
            let mut conn = mgr.clone();
            let minute = unix_minute();
            let window_key = format!("panda:tpm:v1:prompt_window:{bucket}:{minute}");
            let total_key = format!("panda:tpm:v1:prompt:{bucket}");
            match redis::cmd("EVAL")
                .arg(PROMPT_BUDGET_LUA)
                .arg(2usize)
                .arg(&window_key)
                .arg(&total_key)
                .arg(n as i64)
                .arg(eff as i64)
                .query_async::<i64>(&mut conn)
                .await
            {
                Ok(1) => {
                    self.mark_healthy_redis();
                    return true;
                }
                Ok(0) => return false,
                Ok(_) => {
                    eprintln!("panda tpm: unexpected lua return from prompt budget script");
                    self.mark_degraded();
                    return self.try_reserve_prompt_budget_memory(bucket, n, eff);
                }
                Err(e) => {
                    eprintln!("panda tpm: redis prompt budget script failed: {e}; falling back to in-memory window");
                    self.mark_degraded();
                    return self.try_reserve_prompt_budget_memory(bucket, n, eff);
                }
            }
        }
        self.try_reserve_prompt_budget_memory(bucket, n, eff)
    }

    fn try_reserve_prompt_budget_memory(
        &self,
        bucket: &str,
        n: u64,
        limit_per_minute: u64,
    ) -> bool {
        let now = std::time::Instant::now();
        let mut g = self.mem_prompt_window.lock().expect("tpm mutex poisoned");
        let e = g.entry(bucket.to_string()).or_insert((now, 0));
        if now.duration_since(e.0) >= std::time::Duration::from_secs(60) {
            *e = (now, 0);
        }
        if e.1.saturating_add(n) > limit_per_minute {
            return false;
        }
        e.1 = e.1.saturating_add(n);
        drop(g);
        let mut g = self.mem.lock().expect("tpm mutex poisoned");
        g.entry(bucket.to_string()).or_default().0 += n;
        true
    }

    pub async fn add_prompt_tokens(&self, bucket: &str, n: u64) {
        if n == 0 {
            return;
        }
        {
            let mut g = self.mem.lock().expect("tpm mutex poisoned");
            g.entry(bucket.to_string()).or_default().0 += n;
        }
        {
            let now = std::time::Instant::now();
            let mut g = self.mem_prompt_window.lock().expect("tpm mutex poisoned");
            let e = g.entry(bucket.to_string()).or_insert((now, 0));
            if now.duration_since(e.0) >= std::time::Duration::from_secs(60) {
                *e = (now, 0);
            }
            e.1 = e.1.saturating_add(n);
        }
        if let Some(ref mgr) = self.redis {
            let mut conn = mgr.clone();
            let key = format!("panda:tpm:v1:prompt:{bucket}");
            if redis::cmd("INCRBY")
                .arg(&key)
                .arg(n as i64)
                .query_async::<i64>(&mut conn)
                .await
                .is_err()
            {
                self.mark_degraded();
            } else {
                self.mark_healthy_redis();
            }
            let minute = unix_minute();
            let window_key = format!("panda:tpm:v1:prompt_window:{bucket}:{minute}");
            let _: Result<i64, _> = redis::cmd("INCRBY")
                .arg(&window_key)
                .arg(n as i64)
                .query_async(&mut conn)
                .await;
            let _: Result<bool, _> = conn.expire(&window_key, 120).await;
        }
    }

    pub async fn add_completion_tokens(&self, bucket: &str, n: u64) {
        if n == 0 {
            return;
        }
        {
            let mut g = self.mem.lock().expect("tpm mutex poisoned");
            g.entry(bucket.to_string()).or_default().1 += n;
        }
        if let Some(ref mgr) = self.redis {
            let mut conn = mgr.clone();
            let key = format!("panda:tpm:v1:completion:{bucket}");
            if redis::cmd("INCRBY")
                .arg(&key)
                .arg(n as i64)
                .query_async::<i64>(&mut conn)
                .await
                .is_err()
            {
                self.mark_degraded();
            } else {
                self.mark_healthy_redis();
            }
        }
    }

    /// Lifetime prompt + completion token totals for `bucket` (in-process; mirrors Redis when enabled).
    pub fn bucket_token_totals(&self, bucket: &str) -> (u64, u64) {
        let g = self.mem.lock().expect("tpm mutex poisoned");
        g.get(bucket).copied().unwrap_or((0, 0))
    }

    /// Returns `(used, remaining)` for the active prompt budget window.
    pub async fn prompt_budget_snapshot(&self, bucket: &str, limit_per_minute: u64) -> (u64, u64) {
        let eff = self.effective_budget_limit(limit_per_minute);
        let used = if let Some(ref mgr) = self.redis {
            let mut conn = mgr.clone();
            let minute = unix_minute();
            let window_key = format!("panda:tpm:v1:prompt_window:{bucket}:{minute}");
            match redis::cmd("GET")
                .arg(&window_key)
                .query_async::<Option<u64>>(&mut conn)
                .await
            {
                Ok(current) => {
                    self.mark_healthy_redis();
                    current.unwrap_or(0)
                }
                Err(_) => {
                    self.mark_degraded();
                    self.local_prompt_window_used(bucket)
                }
            }
        } else {
            self.local_prompt_window_used(bucket)
        };
        (used, eff.saturating_sub(used))
    }

    fn local_prompt_window_used(&self, bucket: &str) -> u64 {
        let now = std::time::Instant::now();
        let mut g = self.mem_prompt_window.lock().expect("tpm mutex poisoned");
        let e = g.entry(bucket.to_string()).or_insert((now, 0));
        if now.duration_since(e.0) >= std::time::Duration::from_secs(60) {
            *e = (now, 0);
        }
        e.1
    }

    /// Seconds until current prompt budget window rolls over (1..=60).
    pub async fn prompt_budget_retry_after_seconds(&self, bucket: &str) -> u64 {
        if self.redis.is_some() {
            let now = unix_seconds();
            let rem = 60 - (now % 60);
            return rem.max(1);
        }
        let now = std::time::Instant::now();
        let mut g = self.mem_prompt_window.lock().expect("tpm mutex poisoned");
        let e = g.entry(bucket.to_string()).or_insert((now, 0));
        let elapsed = now.duration_since(e.0).as_secs();
        if elapsed >= 60 {
            *e = (now, 0);
            60
        } else {
            (60 - elapsed).max(1)
        }
    }

    #[cfg(test)]
    pub(crate) fn test_set_degraded(&self, v: bool) {
        self.degraded.store(v, Ordering::SeqCst);
    }
}

async fn connect_redis(url: &str) -> anyhow::Result<redis::aio::ConnectionManager> {
    let client = redis::Client::open(url)?;
    Ok(redis::aio::ConnectionManager::new(client).await?)
}

fn unix_minute() -> u64 {
    unix_seconds() / 60
}

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{TpmCounters, TpmPolicy};

    #[tokio::test]
    async fn in_memory_prompt_budget_enforced() {
        let counters = TpmCounters::connect(None).await.unwrap();
        assert!(counters.try_reserve_prompt_budget("u1", 50, 100).await);
        assert!(!counters.try_reserve_prompt_budget("u1", 51, 100).await);
        assert!(counters.try_reserve_prompt_budget("u1", 50, 100).await);
    }

    #[tokio::test]
    async fn in_memory_budget_snapshot_reports_remaining() {
        let counters = TpmCounters::connect(None).await.unwrap();
        counters.add_prompt_tokens("u2", 25).await;
        let (used, remaining) = counters.prompt_budget_snapshot("u2", 100).await;
        assert_eq!(used, 25);
        assert_eq!(remaining, 75);
    }

    #[tokio::test]
    async fn in_memory_retry_after_is_positive() {
        let counters = TpmCounters::connect(None).await.unwrap();
        let secs = counters.prompt_budget_retry_after_seconds("u3").await;
        assert!((1..=60).contains(&secs));
    }

    #[tokio::test]
    async fn degraded_mode_halves_effective_limit() {
        let policy = TpmPolicy {
            degraded_limit_ratio: 0.5,
            tighten_on_redis_unavailable: true,
            tighten_on_redis_command_error: true,
        };
        let c = TpmCounters::connect_with_policy(None, policy)
            .await
            .unwrap();
        c.test_set_degraded(true);
        assert_eq!(c.effective_budget_limit(100), 50);
        assert!(!c.try_reserve_prompt_budget("d1", 51, 100).await);
        assert!(c.try_reserve_prompt_budget("d1", 50, 100).await);
    }
}

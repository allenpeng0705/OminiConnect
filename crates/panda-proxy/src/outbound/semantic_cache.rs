use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use redis::AsyncCommands;
use sha2::{Digest, Sha256};

use super::semantic_routing::cosine_dot;

pub struct SemanticCache {
    backend: SemanticCacheBackend,
}

enum SemanticCacheBackend {
    Memory(MemorySemanticCache),
    Redis(RedisSemanticCache),
}

struct MemorySemanticCache {
    max_entries: usize,
    ttl: Duration,
    similarity_threshold: f32,
    similarity_fallback: bool,
    inner: Mutex<Inner>,
}

struct RedisSemanticCache {
    ttl_seconds: u64,
    redis: redis::aio::ConnectionManager,
}

struct Inner {
    map: HashMap<String, (Instant, Vec<u8>)>,
    order: VecDeque<String>,
    /// L2-normalized embedding per cache key (subset of keys in `map`).
    embeddings: HashMap<String, Vec<f32>>,
}

impl SemanticCache {
    pub async fn connect(
        backend: &str,
        max_entries: usize,
        ttl: Duration,
        similarity_threshold: f32,
        similarity_fallback: bool,
        redis_url: Option<&str>,
    ) -> anyhow::Result<Self> {
        match backend {
            "memory" => Ok(Self::new_memory(
                max_entries,
                ttl,
                similarity_threshold,
                similarity_fallback,
            )),
            "redis" => {
                let url = redis_url
                    .filter(|u| !u.trim().is_empty())
                    .ok_or_else(|| anyhow::anyhow!("semantic cache backend=redis requires redis_url or PANDA_SEMANTIC_CACHE_REDIS_URL"))?;
                let client = redis::Client::open(url)?;
                let redis = redis::aio::ConnectionManager::new(client).await?;
                Ok(Self {
                    backend: SemanticCacheBackend::Redis(RedisSemanticCache {
                        ttl_seconds: ttl.as_secs().max(1),
                        redis,
                    }),
                })
            }
            _ => anyhow::bail!("unsupported semantic cache backend: {backend}"),
        }
    }

    pub fn new_memory(
        max_entries: usize,
        ttl: Duration,
        similarity_threshold: f32,
        similarity_fallback: bool,
    ) -> Self {
        Self {
            backend: SemanticCacheBackend::Memory(MemorySemanticCache {
                max_entries,
                ttl,
                similarity_threshold,
                similarity_fallback,
                inner: Mutex::new(Inner {
                    map: HashMap::new(),
                    order: VecDeque::new(),
                    embeddings: HashMap::new(),
                }),
            }),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        match &self.backend {
            SemanticCacheBackend::Memory(mem) => mem.get(key),
            SemanticCacheBackend::Redis(redis_backend) => redis_backend.get(key).await,
        }
    }

    pub async fn put(&self, key: String, value: Vec<u8>) {
        self.put_with_embedding(key, value, None).await
    }

    pub(crate) async fn put_with_embedding(
        &self,
        key: String,
        value: Vec<u8>,
        embedding: Option<Vec<f32>>,
    ) {
        match &self.backend {
            SemanticCacheBackend::Memory(mem) => mem.put(key, value, embedding),
            SemanticCacheBackend::Redis(redis_backend) => redis_backend.put(key, value).await,
        }
    }

    /// Memory backend only: best-effort cosine match on stored embeddings (same model/tools contract as Jaccard fallback).
    pub(crate) fn get_by_embedding_match(
        &self,
        query_key: &str,
        query_vec: &[f32],
        threshold: f32,
    ) -> Option<Vec<u8>> {
        match &self.backend {
            SemanticCacheBackend::Memory(mem) => {
                mem.get_by_embedding_match(query_key, query_vec, threshold)
            }
            SemanticCacheBackend::Redis(_) => None,
        }
    }
}

impl MemorySemanticCache {
    fn get(&self, key: &str) -> Option<Vec<u8>> {
        let now = Instant::now();
        let mut g = self.inner.lock().expect("semantic cache mutex poisoned");
        if let Some((ts, v)) = g.map.get(key).cloned() {
            if now.duration_since(ts) <= self.ttl {
                return Some(v);
            }
            g.map.remove(key);
            g.embeddings.remove(key);
            remove_all_from_order(&mut g.order, key);
        }
        if !self.similarity_fallback {
            return None;
        }
        // Similarity fallback (MVP): choose best unexpired key by Jaccard token overlap.
        let (model, tools_sig, _msgs_sig) = parse_cache_key_contract(key)?;
        let mut best: Option<(String, f32, Vec<u8>)> = None;
        let mut expired = Vec::new();
        for (k, (ts, v)) in &g.map {
            if now.duration_since(*ts) > self.ttl {
                expired.push(k.clone());
                continue;
            }
            let Some((k_model, k_tools_sig, _)) = parse_cache_key_contract(k) else {
                continue;
            };
            // Hard compatibility gate before any semantic fallback.
            if k_model != model || k_tools_sig != tools_sig {
                continue;
            }
            let sim = jaccard_similarity(key, k);
            if sim >= self.similarity_threshold {
                match best {
                    Some((_, cur, _)) if sim <= cur => {}
                    _ => best = Some((k.clone(), sim, v.clone())),
                }
            }
        }
        for k in expired {
            g.map.remove(&k);
            g.embeddings.remove(&k);
            remove_all_from_order(&mut g.order, &k);
        }
        best.map(|(_, _, v)| v)
    }

    fn get_by_embedding_match(
        &self,
        query_key: &str,
        query_vec: &[f32],
        threshold: f32,
    ) -> Option<Vec<u8>> {
        let now = Instant::now();
        let (model, tools_sig, _) = parse_cache_key_contract(query_key)?;
        let g = self.inner.lock().expect("semantic cache mutex poisoned");
        let mut best: Option<(f32, Vec<u8>)> = None;
        for (k, emb) in &g.embeddings {
            let Some((ts, v)) = g.map.get(k).cloned() else {
                continue;
            };
            if now.duration_since(ts) > self.ttl {
                continue;
            }
            let Some((k_model, k_tools, _)) = parse_cache_key_contract(k) else {
                continue;
            };
            if k_model != model || k_tools != tools_sig {
                continue;
            }
            let sim = cosine_dot(query_vec, emb)?;
            if sim >= threshold {
                match best {
                    Some((cur, _)) if sim <= cur => {}
                    _ => best = Some((sim, v)),
                }
            }
        }
        best.map(|(_, v)| v)
    }

    fn put(&self, key: String, value: Vec<u8>, embedding: Option<Vec<f32>>) {
        let now = Instant::now();
        let mut g = self.inner.lock().expect("semantic cache mutex poisoned");
        if g.map.contains_key(&key) {
            remove_all_from_order(&mut g.order, &key);
        }
        g.map.insert(key.clone(), (now, value));
        match embedding {
            Some(e) if !e.is_empty() => {
                g.embeddings.insert(key.clone(), e);
            }
            _ => {
                g.embeddings.remove(&key);
            }
        }
        g.order.push_back(key);
        while g.map.len() > self.max_entries {
            if let Some(k) = g.order.pop_front() {
                g.map.remove(&k);
                g.embeddings.remove(&k);
            } else {
                break;
            }
        }
    }
}

impl RedisSemanticCache {
    async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut conn = self.redis.clone();
        let redis_key = semantic_cache_redis_key(key);
        let result: Result<Option<Vec<u8>>, _> = conn.get(redis_key).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                eprintln!("panda semantic-cache(redis): get failed: {e}");
                None
            }
        }
    }

    async fn put(&self, key: String, value: Vec<u8>) {
        let mut conn = self.redis.clone();
        let redis_key = semantic_cache_redis_key(&key);
        let result: Result<(), _> = conn.set_ex(redis_key, value, self.ttl_seconds).await;
        if let Err(e) = result {
            eprintln!("panda semantic-cache(redis): set_ex failed: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SemanticCache;
    use std::time::Duration;

    #[tokio::test]
    async fn cache_put_get_roundtrip() {
        let c = SemanticCache::new_memory(10, Duration::from_secs(30), 0.92, false);
        c.put("k".to_string(), b"v".to_vec()).await;
        assert_eq!(c.get("k").await.as_deref(), Some(&b"v"[..]));
    }

    #[tokio::test]
    async fn cache_evicts_oldest_when_full() {
        let c = SemanticCache::new_memory(1, Duration::from_secs(30), 0.92, false);
        c.put("k1".to_string(), b"v1".to_vec()).await;
        c.put("k2".to_string(), b"v2".to_vec()).await;
        assert!(c.get("k1").await.is_none());
        assert_eq!(c.get("k2").await.as_deref(), Some(&b"v2"[..]));
    }

    #[tokio::test]
    async fn cache_similarity_hit() {
        let c = SemanticCache::new_memory(10, Duration::from_secs(30), 0.5, true);
        c.put(
            r#"{"model":"a","messages":"list users","tools":"t"}"#.to_string(),
            b"v".to_vec(),
        )
        .await;
        assert_eq!(
            c.get(r#"{"model":"a","messages":"list all users","tools":"t"}"#)
                .await
                .as_deref(),
            Some(&b"v"[..])
        );
    }

    #[tokio::test]
    async fn cache_similarity_hit_with_upstream_suffix_on_key() {
        let c = SemanticCache::new_memory(10, Duration::from_secs(30), 0.5, true);
        let k_store =
            r#"{"model":"a","messages":"list users","tools":"t"}|https://upstream.example"#
                .to_string();
        let k_query =
            r#"{"model":"a","messages":"list all users","tools":"t"}|https://upstream.example"#;
        c.put(k_store, b"v".to_vec()).await;
        assert_eq!(c.get(k_query).await.as_deref(), Some(&b"v"[..]));
    }

    #[tokio::test]
    async fn cache_similarity_respects_model_and_tools_contract() {
        let c = SemanticCache::new_memory(10, Duration::from_secs(30), 0.1, true);
        c.put(
            r#"{"model":"a","messages":"list users","tools":"t1"}"#.to_string(),
            b"v1".to_vec(),
        )
        .await;
        assert!(c
            .get(r#"{"model":"b","messages":"list users","tools":"t1"}"#)
            .await
            .is_none());
        assert!(c
            .get(r#"{"model":"a","messages":"list users","tools":"t2"}"#)
            .await
            .is_none());
    }

    #[tokio::test]
    async fn cache_similarity_disabled_skips_fuzzy_match() {
        let c = SemanticCache::new_memory(10, Duration::from_secs(30), 0.1, false);
        c.put(
            r#"{"model":"a","messages":"list users","tools":"t"}"#.to_string(),
            b"v".to_vec(),
        )
        .await;
        assert!(c
            .get(r#"{"model":"a","messages":"list all users","tools":"t"}"#)
            .await
            .is_none());
    }
}

fn jaccard_similarity(a: &str, b: &str) -> f32 {
    use std::collections::HashSet;
    let ta: HashSet<String> = a
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect();
    let tb: HashSet<String> = b
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect();
    if ta.is_empty() && tb.is_empty() {
        return 1.0;
    }
    let inter = ta.intersection(&tb).count() as f32;
    let union = ta.union(&tb).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

fn remove_all_from_order(order: &mut VecDeque<String>, key: &str) {
    order.retain(|k| k != key);
}

fn parse_cache_key_contract(key: &str) -> Option<(String, String, String)> {
    let json_part = key.splitn(2, '|').next()?;
    let v: serde_json::Value = serde_json::from_str(json_part).ok()?;
    let model = v
        .get("model")
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string();
    let tools_sig =
        serde_json::to_string(v.get("tools").unwrap_or(&serde_json::Value::Null)).ok()?;
    let messages_sig =
        serde_json::to_string(v.get("messages").unwrap_or(&serde_json::Value::Null)).ok()?;
    Some((model, tools_sig, messages_sig))
}

fn semantic_cache_redis_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let digest = hasher.finalize();
    let hex = digest
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    format!("panda:semantic:v1:{hex}")
}

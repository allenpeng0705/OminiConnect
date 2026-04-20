//! Ingress path routing (Phase C): longest-prefix match to `ai` / `mcp` / `ops` / `deny` before handlers run.
//!
//! Dynamic overlay (Epic E): [`DynamicIngressRoutes`] merges with the static table; longest prefix wins,
//! with ties resolved in favor of the dynamic row.

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use hyper::Method;
use panda_config::{
    ApiGatewayIngressAuthMode, ApiGatewayIngressBackend, ApiGatewayIngressConfig,
    ApiGatewayIngressRoute,
};

use super::control_plane_store::ControlPlanePersist;

/// Key material for ingress row RPS limits (after a successful classify).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IngressRpsKey {
    pub tenant_id: String,
    pub path_prefix: String,
    pub rps: u32,
}

/// [`IngressClassify`] plus optional per-row rate limit for the **winning** ingress entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IngressClassifyMerged {
    pub classify: IngressClassify,
    /// Set when the matched static or dynamic row defines `rate_limit`.
    pub ingress_rps: Option<IngressRpsKey>,
    /// JWT policy for the winning ingress row (built-in defaults use [`ApiGatewayIngressAuthMode::Inherit`]).
    pub auth_mode: ApiGatewayIngressAuthMode,
}

/// Result of classifying a request path + method against the ingress table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressClassify {
    /// Hand off to the existing handler for this backend.
    Allow {
        backend: ApiGatewayIngressBackend,
        /// When `backend == Ai`, optional upstream base URL for [`crate::forward_to_upstream`].
        upstream: Option<String>,
    },
    /// Path matched but HTTP method is not allowed for that row.
    MethodNotAllowed { allow: Vec<String> },
    /// No `path_prefix` matched.
    NoMatch,
}

#[derive(Debug, Clone)]
pub(crate) struct IngressEntry {
    /// Empty = global (applies to every request for dynamic rows when tenant header is used).
    pub(crate) tenant_id: String,
    pub(crate) prefix: String,
    pub(crate) backend: ApiGatewayIngressBackend,
    /// Empty = any method allowed.
    pub(crate) methods: Vec<Method>,
    pub(crate) upstream: Option<String>,
    /// When set, enforce RPS for this row (ingress + optional Redis in `RouteRpsLimiters`).
    pub(crate) ingress_rps: Option<u32>,
    /// Per-prefix JWT policy vs global `identity.require_jwt`.
    pub(crate) auth: ApiGatewayIngressAuthMode,
}

/// Convert a YAML/config row into a router entry (`None` when `path_prefix` is empty after trim).
pub(crate) fn ingress_entry_from_route(r: &ApiGatewayIngressRoute) -> Option<IngressEntry> {
    let p = r.path_prefix.trim();
    if p.is_empty() {
        return None;
    }
    let methods: Vec<Method> = r
        .methods
        .iter()
        .filter_map(|m| {
            let t = m.trim();
            if t.is_empty() {
                None
            } else {
                Method::from_bytes(t.as_bytes()).ok()
            }
        })
        .collect();
    let upstream = r
        .backend_base
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let tenant_id = r
        .tenant_id
        .as_deref()
        .map(str::trim)
        .unwrap_or("")
        .to_string();
    let ingress_rps = r.rate_limit.as_ref().map(|rl| rl.rps).filter(|n| *n > 0);
    Some(IngressEntry {
        tenant_id,
        prefix: p.to_string(),
        backend: r.backend,
        methods,
        upstream,
        ingress_rps,
        auth: r.auth,
    })
}

fn longest_prefix_match_iter<'a>(
    entries: impl Iterator<Item = &'a IngressEntry>,
    path: &str,
) -> Option<&'a IngressEntry> {
    entries
        .filter(|e| path.starts_with(e.prefix.as_str()))
        .max_by_key(|e| e.prefix.len())
}

fn dynamic_entry_applies_to_request(e: &IngressEntry, request_tenant: Option<&str>) -> bool {
    if e.tenant_id.is_empty() {
        return true;
    }
    request_tenant.is_some_and(|t| t == e.tenant_id.as_str())
}

/// Merge static ingress (from config) with a dynamic snapshot. Longest matching prefix wins; on equal
/// length, the **dynamic** row wins (so control-plane upserts can override static routes for the same prefix).
pub(crate) fn classify_merged(
    static_router: &IngressRouter,
    dynamic: &[IngressEntry],
    path: &str,
    method: &Method,
    request_tenant: Option<&str>,
) -> IngressClassifyMerged {
    let s = longest_prefix_match_iter(
        static_router
            .entries()
            .iter()
            .filter(|e| dynamic_entry_applies_to_request(e, request_tenant)),
        path,
    );
    let d = longest_prefix_match_iter(
        dynamic
            .iter()
            .filter(|e| dynamic_entry_applies_to_request(e, request_tenant)),
        path,
    );
    let chosen = match (s, d) {
        (None, None) => {
            return IngressClassifyMerged {
                classify: IngressClassify::NoMatch,
                ingress_rps: None,
                auth_mode: ApiGatewayIngressAuthMode::Inherit,
            };
        }
        (Some(x), None) => x,
        (None, Some(x)) => x,
        (Some(a), Some(b)) => {
            if b.prefix.len() > a.prefix.len() {
                b
            } else if a.prefix.len() > b.prefix.len() {
                a
            } else {
                b
            }
        }
    };
    if !chosen.methods.is_empty() && !chosen.methods.iter().any(|m| m == method) {
        let allow: Vec<String> = chosen.methods.iter().map(|m| m.to_string()).collect();
        return IngressClassifyMerged {
            classify: IngressClassify::MethodNotAllowed { allow },
            ingress_rps: None,
            auth_mode: ApiGatewayIngressAuthMode::Inherit,
        };
    }
    let auth_mode = chosen.auth;
    let ingress_rps = chosen.ingress_rps.map(|rps| IngressRpsKey {
        tenant_id: chosen.tenant_id.clone(),
        path_prefix: chosen.prefix.clone(),
        rps,
    });
    IngressClassifyMerged {
        classify: IngressClassify::Allow {
            backend: chosen.backend,
            upstream: chosen.upstream.clone(),
        },
        ingress_rps,
        auth_mode,
    }
}

/// Process-local dynamic ingress rows (control plane), optionally backed by a [`ControlPlanePersist`] store.
pub struct DynamicIngressRoutes {
    inner: RwLock<Vec<IngressEntry>>,
    persist: Option<Arc<dyn ControlPlanePersist>>,
}

impl DynamicIngressRoutes {
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            inner: RwLock::new(Vec::new()),
            persist: None,
        })
    }

    pub fn new_arc_with(
        persist: Option<Arc<dyn ControlPlanePersist>>,
        preloaded: Vec<ApiGatewayIngressRoute>,
    ) -> Result<Arc<Self>, String> {
        let mut v = Vec::new();
        for r in preloaded {
            let e = ingress_entry_from_route(&r).ok_or_else(|| {
                format!(
                    "invalid stored route (empty path_prefix): {:?}",
                    r.path_prefix
                )
            })?;
            if !e.prefix.starts_with('/') {
                return Err(format!("path_prefix must start with `/`: {}", e.prefix));
            }
            v.push(e);
        }
        Self::sort_entries(&mut v[..]);
        Ok(Arc::new(Self {
            inner: RwLock::new(v),
            persist,
        }))
    }

    pub fn persist_handle(&self) -> Option<Arc<dyn ControlPlanePersist>> {
        self.persist.clone()
    }

    fn sort_entries(entries: &mut [IngressEntry]) {
        entries.sort_by(|a, b| b.prefix.len().cmp(&a.prefix.len()));
    }

    /// Memory only (used after import has already written the backing store).
    pub fn upsert_route_memory_only(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        r.validate_for_control_plane()
            .map_err(|e| e.to_string())?;
        let e = ingress_entry_from_route(r)
            .ok_or_else(|| "path_prefix must be non-empty".to_string())?;
        if !e.prefix.starts_with('/') {
            return Err("path_prefix must start with `/`".to_string());
        }
        let mut g = self
            .inner
            .write()
            .map_err(|_| "dynamic ingress lock poisoned".to_string())?;
        if let Some(i) = g
            .iter()
            .position(|x| x.prefix == e.prefix && x.tenant_id == e.tenant_id)
        {
            g[i] = e;
        } else {
            g.push(e);
        }
        Self::sort_entries(&mut g[..]);
        Ok(())
    }

    /// Insert or replace by trimmed `path_prefix`, then asynchronously mirror to [`Self::persist`] when set.
    pub fn upsert_route(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        self.upsert_route_memory_only(r)?;
        if let Some(p) = &self.persist {
            let p = Arc::clone(p);
            let r = r.clone();
            tokio::spawn(async move {
                if let Err(e) = p.upsert(&r).await {
                    tracing::warn!(target: "panda::control_plane", "dynamic ingress persist upsert failed: {e}");
                }
            });
        }
        Ok(())
    }

    pub fn replace_all_from_routes(
        &self,
        routes: Vec<ApiGatewayIngressRoute>,
    ) -> Result<(), String> {
        let mut seen = HashSet::new();
        let mut v = Vec::new();
        for r in routes {
            r.validate_for_control_plane()
                .map_err(|e| e.to_string())?;
            let e = ingress_entry_from_route(&r)
                .ok_or_else(|| format!("invalid route (empty path_prefix): {:?}", r.path_prefix))?;
            if !e.prefix.starts_with('/') {
                return Err(format!("path_prefix must start with `/`: {}", e.prefix));
            }
            let tid = e.tenant_id.as_str();
            if !seen.insert((tid.to_string(), e.prefix.clone())) {
                return Err(format!(
                    "duplicate (tenant_id, path_prefix) in import batch: tenant={:?} path={}",
                    tid,
                    e.prefix
                ));
            }
            v.push(e);
        }
        Self::sort_entries(&mut v[..]);
        let mut g = self
            .inner
            .write()
            .map_err(|_| "dynamic ingress lock poisoned".to_string())?;
        *g = v;
        Ok(())
    }

    /// Await backing-store write, then update memory (for HTTP handlers + Redis/NOTIFY fan-out ordering).
    pub async fn upsert_route_persisted(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        if let Some(p) = &self.persist {
            p.upsert(r).await?;
        }
        self.upsert_route_memory_only(r)
    }

    /// Await backing-store delete, then drop from memory.
    pub async fn remove_route_persisted(
        &self,
        tenant_id: &str,
        path_prefix: &str,
    ) -> Result<bool, String> {
        let tid = tenant_id.trim();
        let pfx = path_prefix.trim();
        if let Some(p) = &self.persist {
            let _ = p.remove(tid, pfx).await?;
        }
        let mut g = self
            .inner
            .write()
            .map_err(|_| "dynamic ingress lock poisoned".to_string())?;
        let before = g.len();
        g.retain(|x| x.prefix != pfx || x.tenant_id != tid);
        Ok(g.len() < before)
    }

    pub(crate) fn entries_snapshot(&self) -> Vec<IngressEntry> {
        self.inner.read().map(|g| g.clone()).unwrap_or_default()
    }

    pub fn route_count(&self) -> usize {
        self.inner.read().map(|g| g.len()).unwrap_or(0)
    }

    /// Serialize current dynamic rows for `GET …/export` / GitOps bundles.
    pub fn api_routes_snapshot(&self) -> Vec<ApiGatewayIngressRoute> {
        self.entries_snapshot()
            .iter()
            .map(|e| ApiGatewayIngressRoute {
                tenant_id: if e.tenant_id.is_empty() {
                    None
                } else {
                    Some(e.tenant_id.clone())
                },
                path_prefix: e.prefix.clone(),
                backend: e.backend,
                methods: e.methods.iter().map(|m| m.to_string()).collect(),
                backend_base: e.upstream.clone(),
                rate_limit: e
                    .ingress_rps
                    .map(|rps| panda_config::RouteRateLimitConfig { rps }),
                auth: e.auth,
            })
            .collect()
    }
}

/// Longest-prefix ingress router (same listener as main `listen`; split listener is out of scope for this phase).
#[derive(Debug)]
pub struct IngressRouter {
    /// Sorted by `path_prefix` length descending.
    entries: Vec<IngressEntry>,
}

impl IngressRouter {
    /// Returns `None` when ingress is disabled in config.
    pub fn try_new(cfg: &ApiGatewayIngressConfig) -> Option<Arc<Self>> {
        if !cfg.enabled {
            return None;
        }
        let entries: Vec<IngressEntry> = if cfg.routes.is_empty() {
            builtin_default_routes()
        } else {
            cfg.routes
                .iter()
                .filter_map(ingress_entry_from_route)
                .collect()
        };
        let mut entries = entries;
        entries.sort_by(|a, b| b.prefix.len().cmp(&a.prefix.len()));
        Some(Arc::new(Self { entries }))
    }

    pub(crate) fn entries(&self) -> &[IngressEntry] {
        &self.entries
    }

    /// Longest matching `path_prefix` wins; then optional HTTP method filter.
    pub fn classify(
        &self,
        path: &str,
        method: &Method,
        request_tenant: Option<&str>,
    ) -> IngressClassify {
        let Some(e) = longest_prefix_match_iter(
            self.entries
                .iter()
                .filter(|ent| dynamic_entry_applies_to_request(ent, request_tenant)),
            path,
        ) else {
            return IngressClassify::NoMatch;
        };
        if !e.methods.is_empty() && !e.methods.iter().any(|m| m == method) {
            let allow: Vec<String> = e.methods.iter().map(|m| m.to_string()).collect();
            return IngressClassify::MethodNotAllowed { allow };
        }
        IngressClassify::Allow {
            backend: e.backend,
            upstream: e.upstream.clone(),
        }
    }
}

fn entry(prefix: &str, backend: ApiGatewayIngressBackend, methods: Vec<Method>) -> IngressEntry {
    IngressEntry {
        tenant_id: String::new(),
        prefix: prefix.to_string(),
        backend,
        methods,
        upstream: None,
        ingress_rps: None,
        auth: ApiGatewayIngressAuthMode::Inherit,
    }
}

/// Matches [`dispatch`](crate::dispatch) ops paths first (longer prefixes before shorter), then AI and MCP.
fn builtin_default_routes() -> Vec<IngressEntry> {
    let any: Vec<Method> = vec![];
    vec![
        entry(
            "/compliance/status",
            ApiGatewayIngressBackend::Ops,
            any.clone(),
        ),
        entry(
            "/ops/fleet/status",
            ApiGatewayIngressBackend::Ops,
            any.clone(),
        ),
        entry(
            "/plugins/status",
            ApiGatewayIngressBackend::Ops,
            any.clone(),
        ),
        entry("/mcp/status", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/tpm/status", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/metrics", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/ready", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/health", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/portal", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/console", ApiGatewayIngressBackend::Ops, any.clone()),
        entry("/v1", ApiGatewayIngressBackend::Ai, any.clone()),
        entry("/mcp", ApiGatewayIngressBackend::Mcp, any),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::{ApiGatewayIngressConfig, ApiGatewayIngressRoute};

    fn router_from_builtin() -> Arc<IngressRouter> {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![],
            ..Default::default()
        };
        IngressRouter::try_new(&cfg).expect("some")
    }

    #[test]
    fn builtin_health_is_ops() {
        let r = router_from_builtin();
        assert_eq!(
            r.classify("/health", &Method::GET, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ops,
                upstream: None,
            }
        );
    }

    #[test]
    fn builtin_v1_is_ai() {
        let r = router_from_builtin();
        assert_eq!(
            r.classify("/v1/chat/completions", &Method::POST, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ai,
                upstream: None,
            }
        );
    }

    #[test]
    fn mcp_status_ops_before_mcp_prefix() {
        let r = router_from_builtin();
        assert_eq!(
            r.classify("/mcp/status", &Method::GET, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ops,
                upstream: None,
            }
        );
        assert_eq!(
            r.classify("/mcp/rpc", &Method::GET, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Mcp,
                upstream: None,
            }
        );
    }

    #[test]
    fn unknown_path_no_match() {
        let r = router_from_builtin();
        assert_eq!(
            r.classify("/zzz", &Method::GET, None),
            IngressClassify::NoMatch
        );
    }

    #[test]
    fn custom_routes_replace_defaults() {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![ApiGatewayIngressRoute {
                tenant_id: None,
                path_prefix: "/api".to_string(),
                backend: ApiGatewayIngressBackend::Ai,
                methods: vec![],
                backend_base: None,
                rate_limit: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let r = IngressRouter::try_new(&cfg).expect("some");
        assert_eq!(
            r.classify("/api/x", &Method::GET, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ai,
                upstream: None,
            }
        );
        assert_eq!(
            r.classify("/health", &Method::GET, None),
            IngressClassify::NoMatch
        );
    }

    #[test]
    fn custom_methods_reject_disallowed_verb() {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![ApiGatewayIngressRoute {
                tenant_id: None,
                path_prefix: "/hooks".to_string(),
                backend: ApiGatewayIngressBackend::Ai,
                methods: vec!["POST".to_string()],
                backend_base: None,
                rate_limit: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let r = IngressRouter::try_new(&cfg).expect("some");
        match r.classify("/hooks/x", &Method::GET, None) {
            IngressClassify::MethodNotAllowed { allow } => {
                assert_eq!(allow, vec!["POST".to_string()]);
            }
            o => panic!("expected MethodNotAllowed, got {o:?}"),
        }
        assert_eq!(
            r.classify("/hooks/x", &Method::POST, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ai,
                upstream: None,
            }
        );
    }

    #[test]
    fn custom_ai_upstream_carried_in_classify() {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![ApiGatewayIngressRoute {
                tenant_id: None,
                path_prefix: "/alt".to_string(),
                backend: ApiGatewayIngressBackend::Ai,
                methods: vec![],
                backend_base: Some("https://llm.other.example/v1".to_string()),
                rate_limit: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let r = IngressRouter::try_new(&cfg).expect("some");
        assert_eq!(
            r.classify("/alt/chat", &Method::POST, None),
            IngressClassify::Allow {
                backend: ApiGatewayIngressBackend::Ai,
                upstream: Some("https://llm.other.example/v1".to_string()),
            }
        );
    }

    #[test]
    fn merge_prefers_longer_dynamic_prefix() {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![ApiGatewayIngressRoute {
                tenant_id: None,
                path_prefix: "/api".to_string(),
                backend: ApiGatewayIngressBackend::Ai,
                methods: vec![],
                backend_base: None,
                rate_limit: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let r = IngressRouter::try_new(&cfg).expect("some");
        let dynamic = vec![IngressEntry {
            tenant_id: String::new(),
            prefix: "/api/v2".to_string(),
            backend: ApiGatewayIngressBackend::Gone,
            methods: vec![],
            upstream: None,
            ingress_rps: None,
            auth: ApiGatewayIngressAuthMode::Inherit,
        }];
        assert_eq!(
            classify_merged(r.as_ref(), &dynamic, "/api/v2/x", &Method::GET, None),
            IngressClassifyMerged {
                classify: IngressClassify::Allow {
                    backend: ApiGatewayIngressBackend::Gone,
                    upstream: None,
                },
                ingress_rps: None,
                auth_mode: ApiGatewayIngressAuthMode::Inherit,
            }
        );
        assert_eq!(
            classify_merged(r.as_ref(), &dynamic, "/api/v1/x", &Method::GET, None),
            IngressClassifyMerged {
                classify: IngressClassify::Allow {
                    backend: ApiGatewayIngressBackend::Ai,
                    upstream: None,
                },
                ingress_rps: None,
                auth_mode: ApiGatewayIngressAuthMode::Inherit,
            }
        );
    }

    #[test]
    fn merge_tie_length_prefers_dynamic() {
        let cfg = ApiGatewayIngressConfig {
            enabled: true,
            routes: vec![ApiGatewayIngressRoute {
                tenant_id: None,
                path_prefix: "/x".to_string(),
                backend: ApiGatewayIngressBackend::Ai,
                methods: vec![],
                backend_base: None,
                rate_limit: None,
                ..Default::default()
            }],
            ..Default::default()
        };
        let r = IngressRouter::try_new(&cfg).expect("some");
        let dynamic = vec![IngressEntry {
            tenant_id: String::new(),
            prefix: "/x".to_string(),
            backend: ApiGatewayIngressBackend::Deny,
            methods: vec![],
            upstream: None,
            ingress_rps: None,
            auth: ApiGatewayIngressAuthMode::Inherit,
        }];
        assert_eq!(
            classify_merged(r.as_ref(), &dynamic, "/x/y", &Method::GET, None),
            IngressClassifyMerged {
                classify: IngressClassify::Allow {
                    backend: ApiGatewayIngressBackend::Deny,
                    upstream: None,
                },
                ingress_rps: None,
                auth_mode: ApiGatewayIngressAuthMode::Inherit,
            }
        );
    }
}

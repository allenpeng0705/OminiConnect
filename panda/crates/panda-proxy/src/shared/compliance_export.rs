//! Stub implementation of EU AI Act–oriented **audit export** (see `docs/compliance_export.md`).
//! Current release: append-only **local JSONL** with optional **HMAC-SHA256** over a canonical payload.
//! S3 / GCS with WORM buckets are design-only until object-store writers are added.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use hmac::{Hmac, Mac};
use panda_config::ComplianceExportConfig;
use serde::Serialize;
use sha2::{Digest, Sha256};
use tracing::warn;

/// At most one warn log per this interval while JSONL append keeps failing (avoids log storms).
const APPEND_IO_WARN_THROTTLE: Duration = Duration::from_secs(60);

type HmacSha256 = Hmac<Sha256>;

pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let d = Sha256::digest(data);
    hex_lower(d.as_slice())
}

#[derive(Clone)]
pub struct ComplianceSink {
    dir: PathBuf,
    secret: Option<Vec<u8>>,
    last_append_io_warn: Arc<Mutex<Option<Instant>>>,
}

impl ComplianceSink {
    pub fn try_from_config(cfg: &ComplianceExportConfig) -> anyhow::Result<Option<Self>> {
        if !cfg.enabled {
            return Ok(None);
        }
        let mode = cfg.mode.to_ascii_lowercase();
        if mode != "local_jsonl" {
            anyhow::bail!("compliance_export: only local_jsonl is implemented");
        }
        let dir = PathBuf::from(cfg.local_path.trim());
        std::fs::create_dir_all(&dir)?;
        let secret = cfg
            .signing_secret_env
            .as_ref()
            .map(|env| std::env::var(env))
            .transpose()?
            .map(|s| s.into_bytes())
            .filter(|b| !b.is_empty());
        Ok(Some(Self {
            dir,
            secret,
            last_append_io_warn: Arc::new(Mutex::new(None)),
        }))
    }

    fn warn_append_io_throttled(&self, op: &'static str, path: &Path, err: &std::io::Error) {
        let now = Instant::now();
        let mut log = false;
        if let Ok(mut g) = self.last_append_io_warn.lock() {
            match *g {
                None => {
                    *g = Some(now);
                    log = true;
                }
                Some(prev) if now.saturating_duration_since(prev) >= APPEND_IO_WARN_THROTTLE => {
                    *g = Some(now);
                    log = true;
                }
                _ => {}
            }
        }
        if log {
            warn!(
                path = %path.display(),
                error = %err,
                "compliance_export: failed to {op} local JSONL (best-effort sink; events may be dropped)"
            );
        }
    }

    /// Ingress row. When the request body was buffered, `request_body_sha256_hex` is the SHA-256 of raw bytes as received.
    pub fn record_ingress(
        &self,
        request_id: &str,
        path: &str,
        method: &str,
        request_body_sha256_hex: Option<&str>,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        let ts_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let mut rec = ComplianceIngressV1 {
            schema: "panda.compliance.ingress.v1",
            ts_unix_ms: ts_ms,
            event: "ingress",
            request_id: request_id.to_string(),
            path: path.to_string(),
            method: method.to_string(),
            request_body_sha256_hex: request_body_sha256_hex.map(str::to_string),
            budget_hierarchy_nodes,
            hmac_sha256_hex: None,
        };
        let signing_bytes = match serde_json::to_vec(&rec) {
            Ok(b) => b,
            Err(_) => return,
        };
        if let Some(ref key) = self.secret {
            if let Ok(mut mac) = HmacSha256::new_from_slice(key) {
                mac.update(&signing_bytes);
                let hex = hex_lower(mac.finalize().into_bytes().as_slice());
                rec.hmac_sha256_hex = Some(hex);
            }
        }
        let line = match serde_json::to_string(&rec) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.append_line(&line);
    }

    /// Egress row when the response body is fully available (`response_streamed` false) or omitted for SSE / streaming tails.
    pub fn record_egress(
        &self,
        request_id: &str,
        status: u16,
        response_body_sha256_hex: Option<&str>,
        response_streamed: bool,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        let ts_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let mut rec = ComplianceEgressV1 {
            schema: "panda.compliance.egress.v1",
            ts_unix_ms: ts_ms,
            event: "egress",
            request_id: request_id.to_string(),
            status,
            response_body_sha256_hex: response_body_sha256_hex.map(str::to_string),
            response_streamed,
            budget_hierarchy_nodes,
            hmac_sha256_hex: None,
        };
        let signing_bytes = match serde_json::to_vec(&rec) {
            Ok(b) => b,
            Err(_) => return,
        };
        if let Some(ref key) = self.secret {
            if let Ok(mut mac) = HmacSha256::new_from_slice(key) {
                mac.update(&signing_bytes);
                let hex = hex_lower(mac.finalize().into_bytes().as_slice());
                rec.hmac_sha256_hex = Some(hex);
            }
        }
        let line = match serde_json::to_string(&rec) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.append_line(&line);
    }

    /// MCP tool-result cache decision (`hit`, `store`, `bypass`, or `miss` when enabled in config).
    pub fn record_tool_cache(
        &self,
        request_id: &str,
        decision: &str,
        server: &str,
        tool: &str,
        bypass_reason: Option<&str>,
        entry_key_sha256_hex: &str,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        let ts_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let mut rec = ComplianceToolCacheV1 {
            schema: "panda.compliance.tool_cache.v1",
            ts_unix_ms: ts_ms,
            event: "tool_cache",
            request_id: request_id.to_string(),
            decision: decision.to_string(),
            server: server.to_string(),
            tool: tool.to_string(),
            bypass_reason: bypass_reason.map(str::to_string),
            entry_key_sha256_hex: entry_key_sha256_hex.to_string(),
            budget_hierarchy_nodes,
            hmac_sha256_hex: None,
        };
        let signing_bytes = match serde_json::to_vec(&rec) {
            Ok(b) => b,
            Err(_) => return,
        };
        if let Some(ref key) = self.secret {
            if let Ok(mut mac) = HmacSha256::new_from_slice(key) {
                mac.update(&signing_bytes);
                let hex = hex_lower(mac.finalize().into_bytes().as_slice());
                rec.hmac_sha256_hex = Some(hex);
            }
        }
        let line = match serde_json::to_string(&rec) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.append_line(&line);
    }

    fn append_line(&self, line: &str) {
        let file = self.dir.join("panda-compliance.jsonl");
        match OpenOptions::new().create(true).append(true).open(&file) {
            Ok(mut f) => {
                if let Err(e) = writeln!(f, "{line}") {
                    self.warn_append_io_throttled("write", &file, &e);
                }
            }
            Err(e) => self.warn_append_io_throttled("open", &file, &e),
        }
    }
}

/// Thread-safe wrapper for [`ProxyState`].
pub struct ComplianceSinkShared {
    inner: Mutex<ComplianceSink>,
}

impl ComplianceSinkShared {
    pub fn new(inner: ComplianceSink) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }

    pub fn record_ingress(
        &self,
        request_id: &str,
        path: &str,
        method: &str,
        request_body_sha256_hex: Option<&str>,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        if let Ok(g) = self.inner.lock() {
            g.record_ingress(
                request_id,
                path,
                method,
                request_body_sha256_hex,
                budget_hierarchy_nodes,
            );
        }
    }

    pub fn record_egress(
        &self,
        request_id: &str,
        status: u16,
        response_body_sha256_hex: Option<&str>,
        response_streamed: bool,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        if let Ok(g) = self.inner.lock() {
            g.record_egress(
                request_id,
                status,
                response_body_sha256_hex,
                response_streamed,
                budget_hierarchy_nodes,
            );
        }
    }

    pub fn record_tool_cache(
        &self,
        request_id: &str,
        decision: &str,
        server: &str,
        tool: &str,
        bypass_reason: Option<&str>,
        entry_key_sha256_hex: &str,
        budget_hierarchy_nodes: Option<Vec<String>>,
    ) {
        if let Ok(g) = self.inner.lock() {
            g.record_tool_cache(
                request_id,
                decision,
                server,
                tool,
                bypass_reason,
                entry_key_sha256_hex,
                budget_hierarchy_nodes,
            );
        }
    }
}

#[derive(Serialize)]
struct ComplianceIngressV1 {
    schema: &'static str,
    ts_unix_ms: u128,
    event: &'static str,
    request_id: String,
    path: String,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_body_sha256_hex: Option<String>,
    /// Logical budget nodes when `budget_hierarchy` is enabled (`org`, `dept:<name>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_hierarchy_nodes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hmac_sha256_hex: Option<String>,
}

#[derive(Serialize)]
struct ComplianceEgressV1 {
    schema: &'static str,
    ts_unix_ms: u128,
    event: &'static str,
    request_id: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_body_sha256_hex: Option<String>,
    response_streamed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_hierarchy_nodes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hmac_sha256_hex: Option<String>,
}

#[derive(Serialize)]
struct ComplianceToolCacheV1 {
    schema: &'static str,
    ts_unix_ms: u128,
    event: &'static str,
    request_id: String,
    /// `hit`, `store`, `bypass`, or `miss` (miss only if `mcp.tool_cache.compliance_log_misses`).
    decision: String,
    server: String,
    tool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_reason: Option<String>,
    /// SHA-256 (hex) of the internal cache key string (includes scoped identity digest, not raw arguments).
    entry_key_sha256_hex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_hierarchy_nodes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hmac_sha256_hex: Option<String>,
}

fn hex_lower(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn status_json(cfg: &ComplianceExportConfig) -> serde_json::Value {
    serde_json::json!({
        "enabled": cfg.enabled,
        "mode": cfg.mode,
        "local_path": cfg.local_path,
        "signing_configured": cfg.signing_secret_env.as_ref().is_some_and(|e| {
            !e.trim().is_empty() && std::env::var(e).map(|v| !v.is_empty()).unwrap_or(false)
        }),
        "local_jsonl_append": {
            "on_io_error": "best_effort_drop_event",
            "operator_visibility": format!(
                "tracing::warn at most once per {}s while open/write keeps failing (set RUST_LOG e.g. panda_proxy=warn)",
                APPEND_IO_WARN_THROTTLE.as_secs()
            )
        },
        "design_doc": "docs/compliance_export.md",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::ComplianceExportConfig;

    #[test]
    fn ingress_and_egress_lines_use_expected_schemas() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = ComplianceExportConfig {
            enabled: true,
            mode: "local_jsonl".to_string(),
            local_path: dir.path().to_string_lossy().into_owned(),
            signing_secret_env: None,
        };
        let sink = ComplianceSink::try_from_config(&cfg)
            .expect("config ok")
            .expect("some sink");
        sink.record_ingress("rid", "/v1/chat", "POST", Some("abc"), None);
        sink.record_egress("rid", 200, Some("def"), false, None);
        let txt = std::fs::read_to_string(dir.path().join("panda-compliance.jsonl")).expect("read");
        assert!(txt.contains("panda.compliance.ingress.v1"));
        assert!(txt.contains("request_body_sha256_hex"));
        assert!(txt.contains("panda.compliance.egress.v1"));
        assert!(txt.contains("\"response_streamed\":false"));
    }

    #[test]
    fn tool_cache_line_uses_expected_schema() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = ComplianceExportConfig {
            enabled: true,
            mode: "local_jsonl".to_string(),
            local_path: dir.path().to_string_lossy().into_owned(),
            signing_secret_env: None,
        };
        let sink = ComplianceSink::try_from_config(&cfg)
            .expect("config ok")
            .expect("some sink");
        sink.record_tool_cache(
            "rid-tc",
            "bypass",
            "srv",
            "ping",
            Some("not_allowlisted"),
            "deadbeef",
            None,
        );
        let txt = std::fs::read_to_string(dir.path().join("panda-compliance.jsonl")).expect("read");
        assert!(txt.contains("panda.compliance.tool_cache.v1"));
        assert!(txt.contains("\"decision\":\"bypass\""));
        assert!(txt.contains("entry_key_sha256_hex"));
        assert!(txt.contains("not_allowlisted"));
    }

    #[test]
    fn tool_cache_miss_line_uses_expected_schema() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = ComplianceExportConfig {
            enabled: true,
            mode: "local_jsonl".to_string(),
            local_path: dir.path().to_string_lossy().into_owned(),
            signing_secret_env: None,
        };
        let sink = ComplianceSink::try_from_config(&cfg)
            .expect("config ok")
            .expect("some sink");
        sink.record_tool_cache("rid-miss", "miss", "srv", "get", None, "abc123", None);
        let txt = std::fs::read_to_string(dir.path().join("panda-compliance.jsonl")).expect("read");
        assert!(txt.contains("panda.compliance.tool_cache.v1"));
        assert!(txt.contains("\"decision\":\"miss\""));
    }

    #[test]
    fn ingress_and_egress_include_budget_hierarchy_nodes_when_set() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = ComplianceExportConfig {
            enabled: true,
            mode: "local_jsonl".to_string(),
            local_path: dir.path().to_string_lossy().into_owned(),
            signing_secret_env: None,
        };
        let sink = ComplianceSink::try_from_config(&cfg)
            .expect("config ok")
            .expect("some sink");
        sink.record_ingress(
            "rid2",
            "/v1/chat",
            "POST",
            None,
            Some(vec!["org".into(), "dept:marketing".into()]),
        );
        sink.record_egress(
            "rid2",
            200,
            None,
            true,
            Some(vec!["org".into(), "dept:marketing".into()]),
        );
        let txt = std::fs::read_to_string(dir.path().join("panda-compliance.jsonl")).expect("read");
        assert!(txt.contains("budget_hierarchy_nodes"));
        assert!(txt.contains("dept:marketing"));
    }
}

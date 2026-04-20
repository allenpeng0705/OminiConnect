//! MCP host wiring (Phase 4). Stdio transport uses JSON-RPC (see `mcp_stdio`).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use panda_config::{McpConfig, McpServerConfig, McpToolRoutesConfig, PandaConfig};

use super::mcp_http_remote::McpHttpRemoteClient;
use super::mcp_http_tool::McpHttpToolClient;
use super::mcp_stdio::StdioMcpClient;

/// Embedded in timeout errors from [`McpRuntime::call_tool`] so callers can classify failures.
pub const MCP_TOOL_CALL_TIMEOUT_MARKER: &str = "PANDA_MCP_TOOL_CALL_TIMEOUT";

/// Stable, LLM-facing tool `content` when `mcp.fail_open` is true and the host times out.
pub const FAIL_OPEN_TOOL_USER_MESSAGE_TIMEOUT: &str = "PANDA_TOOL_UNAVAILABLE: The tool did not respond before the gateway timeout. Proceed without this tool's output, or retry once the tool may be responsive again.";

/// Stable, LLM-facing tool `content` when `mcp.fail_open` is true and the call fails for other reasons.
pub const FAIL_OPEN_TOOL_USER_MESSAGE_ERROR: &str = "PANDA_TOOL_UNAVAILABLE: The tool could not be executed (gateway or MCP host error). Proceed without this tool's output.";

pub fn mcp_call_error_is_timeout(e: &anyhow::Error) -> bool {
    e.chain()
        .any(|c| c.to_string().contains(MCP_TOOL_CALL_TIMEOUT_MARKER))
}

/// One tool exposed by an MCP server (model-facing name is derived in `mcp_openai`).
#[derive(Debug, Clone)]
pub struct McpToolDescriptor {
    pub server: String,
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct McpToolCallRequest {
    pub server: String,
    pub tool: String,
    pub arguments: serde_json::Value,
    pub correlation_id: String,
}

#[derive(Debug, Clone)]
pub struct McpToolCallResult {
    pub content: serde_json::Value,
    pub is_error: bool,
}

#[async_trait]
pub trait McpClient: Send + Sync {
    async fn list_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>>;
    async fn call_tool(&self, req: McpToolCallRequest) -> anyhow::Result<McpToolCallResult>;
}

/// Placeholder client until real MCP transports are implemented.
struct StubMcpClient {
    server_name: String,
}

impl StubMcpClient {
    fn new(server_name: String) -> Self {
        Self { server_name }
    }
}

#[async_trait]
impl McpClient for StubMcpClient {
    async fn list_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>> {
        Ok(vec![])
    }

    async fn call_tool(&self, _req: McpToolCallRequest) -> anyhow::Result<McpToolCallResult> {
        anyhow::bail!(
            "MCP transport not implemented for server {:?}",
            self.server_name
        )
    }
}

pub struct McpRuntime {
    fail_open: bool,
    tool_timeout_ms: u64,
    max_tool_payload_bytes: usize,
    clients: HashMap<String, Arc<dyn McpClient + Send + Sync>>,
}

impl McpRuntime {
    pub async fn connect(
        config: &PandaConfig,
        egress: Option<&Arc<crate::api_gateway::egress::EgressClient>>,
    ) -> anyhow::Result<Option<Arc<Self>>> {
        if !config.mcp.enabled {
            return Ok(None);
        }
        let mut clients: HashMap<String, Arc<dyn McpClient + Send + Sync>> = HashMap::new();
        let corr = config.observability.correlation_header.trim().to_string();
        for s in &config.mcp.servers {
            if !s.enabled {
                continue;
            }
            let client: Arc<dyn McpClient + Send + Sync> = if !s.http_tools.is_empty() {
                let Some(eg) = egress else {
                    anyhow::bail!(
                        "mcp.servers[{}]: http_tools requires api_gateway.egress (enabled client)",
                        s.name
                    );
                };
                Arc::new(McpHttpToolClient::new(
                    s.name.clone(),
                    s.http_tools.clone(),
                    Arc::clone(eg),
                    corr.clone(),
                )?)
            } else if let Some(url) = s
                .remote_mcp_url
                .as_deref()
                .map(str::trim)
                .filter(|u| !u.is_empty())
            {
                let Some(eg) = egress else {
                    anyhow::bail!(
                        "mcp.servers[{}]: remote_mcp_url requires api_gateway.egress (enabled client)",
                        s.name
                    );
                };
                let prof = s
                    .remote_mcp_egress_profile
                    .as_ref()
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty());
                Arc::new(McpHttpRemoteClient::new(
                    s.name.clone(),
                    url.to_string(),
                    Arc::clone(eg),
                    corr.clone(),
                    prof,
                )?)
            } else if let Some(ref ht) = s.http_tool {
                let Some(eg) = egress else {
                    anyhow::bail!(
                        "mcp.servers[{}]: http_tool requires api_gateway.egress (enabled client)",
                        s.name
                    );
                };
                Arc::new(McpHttpToolClient::new(
                    s.name.clone(),
                    vec![ht.clone()],
                    Arc::clone(eg),
                    corr.clone(),
                )?)
            } else if let Some(cmd) = s.command.as_ref().filter(|c| !c.trim().is_empty()) {
                Arc::new(StdioMcpClient::spawn(&s.name, cmd, &s.args).await?)
            } else {
                Arc::new(StubMcpClient::new(s.name.clone()))
            };
            clients.insert(s.name.clone(), client);
        }
        if clients.is_empty() {
            anyhow::bail!("mcp.enabled with no enabled servers");
        }
        Ok(Some(Arc::new(Self {
            fail_open: config.mcp.fail_open,
            tool_timeout_ms: config.mcp.tool_timeout_ms,
            max_tool_payload_bytes: config.mcp.max_tool_payload_bytes,
            clients,
        })))
    }

    pub fn fail_open(&self) -> bool {
        self.fail_open
    }

    pub fn enabled_server_count(&self) -> usize {
        self.clients.len()
    }

    pub async fn list_all_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>> {
        let mut out = Vec::new();
        for (name, client) in &self.clients {
            let mut ts = client.list_tools().await?;
            for t in &mut ts {
                if t.server.is_empty() {
                    t.server.clone_from(name);
                }
            }
            out.extend(ts);
        }
        Ok(out)
    }

    pub async fn call_tool(&self, req: McpToolCallRequest) -> anyhow::Result<McpToolCallResult> {
        let payload = serde_json::to_string(&req.arguments).unwrap_or_default();
        if payload.len() > self.max_tool_payload_bytes {
            anyhow::bail!("tool arguments exceed mcp.max_tool_payload_bytes");
        }
        let client = self
            .clients
            .get(&req.server)
            .ok_or_else(|| anyhow::anyhow!("unknown MCP server {:?}", req.server))?;
        tokio::time::timeout(
            Duration::from_millis(self.tool_timeout_ms),
            client.call_tool(req),
        )
        .await
        .map_err(|_| anyhow::anyhow!("{}: MCP tool call timed out", MCP_TOOL_CALL_TIMEOUT_MARKER))?
    }
}

pub fn parse_openai_function_name(
    function_name: &str,
    servers: &[McpServerConfig],
) -> Option<(String, String)> {
    let raw = function_name.strip_prefix("mcp_")?;
    for s in servers.iter().filter(|s| s.enabled) {
        let sn = super::mcp_openai::sanitize_openai_function_name(&s.name);
        let p = format!("{sn}_");
        if raw.starts_with(&p) {
            let tool = raw.trim_start_matches(&p);
            if !tool.is_empty() {
                return Some((s.name.clone(), tool.to_string()));
            }
        }
    }
    None
}

pub fn classify_intent_from_chat_request(raw: &[u8]) -> String {
    let Ok(v) = serde_json::from_slice::<serde_json::Value>(raw) else {
        return "general".to_string();
    };
    let Some(messages) = v.get("messages").and_then(|m| m.as_array()) else {
        return "general".to_string();
    };
    let text = messages
        .iter()
        .rev()
        .find_map(|m| {
            if m.get("role").and_then(|r| r.as_str()) == Some("user") {
                m.get("content")
                    .and_then(|c| c.as_str())
                    .map(|s| s.to_ascii_lowercase())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let score = |terms: &[&str]| -> i32 {
        terms
            .iter()
            .map(|t| if text.contains(t) { 1 } else { 0 })
            .sum()
    };
    let write = score(&[
        "write", "delete", "update", "create", "insert", "drop", "remove", "modify",
    ]);
    let read = score(&[
        "read", "list", "show", "query", "search", "find", "fetch", "select",
    ]);
    let fs = score(&[
        "file",
        "directory",
        "path",
        "folder",
        "open",
        "save",
        "rename",
        "filesystem",
    ]);
    if fs >= write && fs >= read && fs > 0 {
        "filesystem".to_string()
    } else if write > read && write > 0 {
        "data_write".to_string()
    } else if read > 0 {
        "data_read".to_string()
    } else {
        "general".to_string()
    }
}

pub fn tool_allowed_for_intent(cfg: &McpConfig, intent: &str, function_name: &str) -> bool {
    if cfg.intent_tool_policies.is_empty() {
        return true;
    }
    let Some((server, tool)) = parse_openai_function_name(function_name, &cfg.servers) else {
        return false;
    };
    let canonical = format!("{server}.{tool}");
    let Some(rule) = cfg.intent_tool_policies.iter().find(|r| r.intent == intent) else {
        return false;
    };
    rule.allowed_tools
        .iter()
        .any(|a| a == function_name || a == &canonical)
}

pub fn filter_tools_for_intent(
    cfg: &McpConfig,
    intent: &str,
    tools: Vec<McpToolDescriptor>,
) -> Vec<McpToolDescriptor> {
    if cfg.intent_tool_policies.is_empty() {
        return tools;
    }
    tools
        .into_iter()
        .filter(|t| {
            let fname = super::mcp_openai::openai_function_name(&t.server, &t.name);
            tool_allowed_for_intent(cfg, intent, &fname)
        })
        .collect()
}

/// When `allowed` is set, keep only tools whose `server` is in the list.
pub fn filter_tools_by_allowed_servers(
    tools: Vec<McpToolDescriptor>,
    allowed: Option<&[String]>,
) -> Vec<McpToolDescriptor> {
    let Some(allowed) = allowed else {
        return tools;
    };
    if allowed.is_empty() {
        return vec![];
    }
    tools
        .into_iter()
        .filter(|t| allowed.iter().any(|a| a == &t.server))
        .collect()
}

/// `*` matches any substring; otherwise exact match. Empty pattern never matches.
pub fn glob_star_match(pattern: &str, text: &str) -> bool {
    let pattern = pattern.trim();
    if pattern.is_empty() {
        return false;
    }
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == text;
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = text;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i == 0 {
            if !rest.starts_with(part) {
                return false;
            }
            rest = &rest[part.len()..];
        } else {
            match rest.find(part) {
                Some(idx) => rest = &rest[idx + part.len()..],
                None => return false,
            }
        }
    }
    if pattern.ends_with('*') {
        true
    } else {
        rest.is_empty()
    }
}

/// `Ok(())` if the tool may be advertised and invoked; `Err(reason)` carries a short label for metrics (pattern or `unmatched`).
pub fn mcp_tool_allowed_by_route_rules(
    tr: &McpToolRoutesConfig,
    server: &str,
    tool: &str,
) -> Result<(), String> {
    if !tr.enabled {
        return Ok(());
    }
    let fname = super::mcp_openai::openai_function_name(server, tool);
    let canonical = format!("{server}.{tool}");
    for rule in &tr.rules {
        if !glob_star_match(&rule.pattern, &fname) && !glob_star_match(&rule.pattern, &canonical) {
            continue;
        }
        match rule.action.trim().to_ascii_lowercase().as_str() {
            "deny" => return Err(rule.pattern.clone()),
            "allow" => {
                if !rule.servers.is_empty() && !rule.servers.iter().any(|s| s == server) {
                    return Err(rule.pattern.clone());
                }
                return Ok(());
            }
            _ => {}
        }
    }
    match tr.unmatched.trim().to_ascii_lowercase().as_str() {
        "deny" => Err("unmatched".to_string()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config_with_mcp() -> PandaConfig {
        PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  tool_timeout_ms: 1000
  max_tool_payload_bytes: 1024
  servers:
    - name: a
    - name: b
      enabled: false
"#,
        )
        .expect("yaml")
    }

    #[test]
    fn mcp_call_error_is_timeout_detects_marker() {
        let e = anyhow::anyhow!("{}: MCP tool call timed out", MCP_TOOL_CALL_TIMEOUT_MARKER);
        assert!(mcp_call_error_is_timeout(&e));
        let e2 = anyhow::anyhow!("unknown MCP server");
        assert!(!mcp_call_error_is_timeout(&e2));
    }

    #[tokio::test]
    async fn connect_disabled_yields_none() {
        let cfg =
            PandaConfig::from_yaml_str("listen: '127.0.0.1:0'\ndefault_backend: 'http://127.0.0.1:1'\n")
                .unwrap();
        assert!(McpRuntime::connect(&cfg, None).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn connect_enabled_builds_runtime() {
        let cfg = sample_config_with_mcp();
        let rt = McpRuntime::connect(&cfg, None)
            .await
            .unwrap()
            .expect("some");
        assert_eq!(rt.enabled_server_count(), 1);
        assert!(rt.fail_open);
    }

    #[tokio::test]
    async fn list_all_tools_empty_for_stub() {
        let cfg = sample_config_with_mcp();
        let rt = McpRuntime::connect(&cfg, None)
            .await
            .unwrap()
            .expect("some");
        let tools = rt.list_all_tools().await.unwrap();
        assert!(tools.is_empty());
    }

    #[tokio::test]
    async fn call_tool_rejects_oversized_arguments() {
        let cfg = sample_config_with_mcp();
        let rt = McpRuntime::connect(&cfg, None)
            .await
            .unwrap()
            .expect("some");
        let big = serde_json::Value::String("x".repeat(2048));
        let err = rt
            .call_tool(McpToolCallRequest {
                server: "a".into(),
                tool: "t".into(),
                arguments: big,
                correlation_id: "c".into(),
            })
            .await
            .unwrap_err();
        assert!(err.to_string().contains("max_tool_payload_bytes"));
    }

    #[test]
    fn filter_tools_by_allowed_servers_keeps_subset() {
        let tools = vec![
            McpToolDescriptor {
                server: "a".into(),
                name: "t1".into(),
                description: None,
                input_schema: serde_json::json!({}),
            },
            McpToolDescriptor {
                server: "b".into(),
                name: "t2".into(),
                description: None,
                input_schema: serde_json::json!({}),
            },
        ];
        let allowed = vec!["a".to_string()];
        let out = filter_tools_by_allowed_servers(tools, Some(&allowed));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].server, "a");
    }

    #[test]
    fn parse_openai_function_name_matches_server_prefix() {
        let cfg = sample_config_with_mcp();
        let got = parse_openai_function_name("mcp_a_demo_tool", &cfg.mcp.servers).unwrap();
        assert_eq!(got.0, "a");
        assert_eq!(got.1, "demo_tool");
    }

    #[test]
    fn classify_intent_from_chat_request_detects_read_write() {
        let read = br#"{"messages":[{"role":"user","content":"list users and query orders"}]}"#;
        let write = br#"{"messages":[{"role":"user","content":"delete user 42"}]}"#;
        assert_eq!(classify_intent_from_chat_request(read), "data_read");
        assert_eq!(classify_intent_from_chat_request(write), "data_write");
    }

    #[test]
    fn tool_allowed_for_intent_uses_policy() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  proof_of_intent_mode: enforce
  intent_tool_policies:
    - intent: data_read
      allowed_tools: ["a.read_users"]
  servers:
    - name: a
"#,
        )
        .unwrap();
        assert!(tool_allowed_for_intent(
            &cfg.mcp,
            "data_read",
            "mcp_a_read_users"
        ));
        assert!(!tool_allowed_for_intent(
            &cfg.mcp,
            "data_read",
            "mcp_a_delete_user"
        ));
    }

    #[test]
    fn glob_star_match_cases() {
        assert!(glob_star_match("*", "mcp_a_x"));
        assert!(glob_star_match("mcp_fs_*", "mcp_fs_read"));
        assert!(!glob_star_match("mcp_fs_*", "mcp_other_read"));
        assert!(glob_star_match("*_read", "mcp_fs_read"));
        assert!(glob_star_match("a.b", "a.b"));
    }

    #[test]
    fn mcp_tool_route_rules_deny_wins() {
        use panda_config::McpToolRouteRule;
        let tr = McpToolRoutesConfig {
            enabled: true,
            rules: vec![McpToolRouteRule {
                pattern: "mcp_bad_*".to_string(),
                action: "deny".to_string(),
                servers: vec![],
            }],
            unmatched: "allow".to_string(),
        };
        assert!(mcp_tool_allowed_by_route_rules(&tr, "bad", "x").is_err());
        assert!(mcp_tool_allowed_by_route_rules(&tr, "good", "x").is_ok());
    }

    #[test]
    fn mcp_tool_route_rules_allow_servers() {
        use panda_config::McpToolRouteRule;
        let tr = McpToolRoutesConfig {
            enabled: true,
            rules: vec![McpToolRouteRule {
                pattern: "*".to_string(),
                action: "allow".to_string(),
                servers: vec!["a".to_string()],
            }],
            unmatched: "deny".to_string(),
        };
        assert!(mcp_tool_allowed_by_route_rules(&tr, "a", "t").is_ok());
        assert!(mcp_tool_allowed_by_route_rules(&tr, "b", "t").is_err());
    }

    #[test]
    fn filter_tools_for_intent_applies_allowlist() {
        let cfg = PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
mcp:
  enabled: true
  intent_tool_policies:
    - intent: data_read
      allowed_tools: ["a.read_users"]
  servers:
    - name: a
"#,
        )
        .unwrap();
        let tools = vec![
            McpToolDescriptor {
                server: "a".into(),
                name: "read_users".into(),
                description: None,
                input_schema: serde_json::json!({}),
            },
            McpToolDescriptor {
                server: "a".into(),
                name: "delete_user".into(),
                description: None,
                input_schema: serde_json::json!({}),
            },
        ];
        let filtered = filter_tools_for_intent(&cfg.mcp, "data_read", tools);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "read_users");
    }

    #[tokio::test]
    async fn stdio_mcp_lists_and_calls_tools() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/mcp_mock_stdio.py");
        if !path.is_file() {
            return;
        }
        let py = if std::process::Command::new("python3")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            "python3"
        } else if std::process::Command::new("python")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            "python"
        } else {
            return;
        };
        let client = StdioMcpClient::spawn("mock", py, &[path.to_string_lossy().into_owned()])
            .await
            .expect("spawn mock MCP");
        let tools = client.list_tools().await.expect("list");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "ping");
        let res = client
            .call_tool(McpToolCallRequest {
                server: "mock".into(),
                tool: "ping".into(),
                arguments: serde_json::json!({}),
                correlation_id: "c".into(),
            })
            .await
            .expect("call");
        assert!(res.content.to_string().contains("pong"));
    }

    #[tokio::test]
    async fn stdio_mcp_zombie_process_returns_error_without_hanging() {
        let py = if std::process::Command::new("python3")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            "python3"
        } else if std::process::Command::new("python")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            "python"
        } else {
            return;
        };
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("zombie_mcp.py");
        std::fs::write(
            &script,
            r#"#!/usr/bin/env python3
import json, sys
for line in sys.stdin:
    msg = json.loads(line)
    mid = msg.get("id")
    method = msg.get("method")
    if method == "initialize":
        print(json.dumps({"jsonrpc":"2.0","id":mid,"result":{"protocolVersion":"2024-11-05","capabilities":{},"serverInfo":{"name":"z","version":"1"}}}), flush=True)
    elif method == "notifications/initialized":
        continue
    elif method == "tools/list":
        print(json.dumps({"jsonrpc":"2.0","id":mid,"result":{"tools":[{"name":"boom","description":"x","inputSchema":{"type":"object"}}]}}), flush=True)
    elif method == "tools/call":
        sys.exit(0)
"#,
        )
        .unwrap();
        let client = StdioMcpClient::spawn("zombie", py, &[script.to_string_lossy().into_owned()])
            .await
            .expect("spawn zombie MCP");
        let tools = client.list_tools().await.expect("list");
        assert_eq!(tools.len(), 1);
        let err = client
            .call_tool(McpToolCallRequest {
                server: "zombie".into(),
                tool: "boom".into(),
                arguments: serde_json::json!({}),
                correlation_id: "c".into(),
            })
            .await
            .unwrap_err();
        assert!(err.to_string().contains("closed stdout"));
    }

    #[tokio::test]
    async fn mcp_http_tool_invokes_egress() {
        use crate::api_gateway::egress::EgressClient;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();
        let body_json = r#"{"hello":"world"}"#;

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
                req.starts_with("GET /allowed/toolpath "),
                "unexpected request head: {}",
                req.chars().take(200).collect::<String>()
            );
            assert!(
                req.to_ascii_lowercase().contains("x-request-id: corr-1"),
                "expected correlation header in {req:?}"
            );
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
                body_json.len(),
                body_json
            );
            let _ = sock.write_all(resp.as_bytes()).await;
        });

        let yaml = format!(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  egress:
    enabled: true
    timeout_ms: 5000
    pool_idle_timeout_ms: 0
    corporate:
      default_base: 'http://127.0.0.1:{port}'
    allowlist:
      allow_hosts: ['127.0.0.1:{port}']
      allow_path_prefixes: ['/allowed']
mcp:
  enabled: true
  servers:
    - name: corpapi
      http_tool:
        path: /allowed/toolpath
        method: GET
        tool_name: fetch
"#
        );
        let cfg = PandaConfig::from_yaml_str(&yaml).expect("yaml");
        let egress = EgressClient::connect(&cfg.api_gateway.egress)
            .await
            .expect("egress connect")
            .expect("egress some");
        let rt = McpRuntime::connect(&cfg, Some(&egress))
            .await
            .expect("mcp connect")
            .expect("mcp some");
        let tools = rt.list_all_tools().await.expect("list");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "fetch");
        let res = rt
            .call_tool(McpToolCallRequest {
                server: "corpapi".into(),
                tool: "fetch".into(),
                arguments: serde_json::json!({}),
                correlation_id: "corr-1".into(),
            })
            .await
            .expect("call");
        assert!(!res.is_error);
        assert_eq!(res.content["status"], 200);
        assert_eq!(res.content["body"]["hello"], "world");
    }
}

//! MCP client over HTTP: JSON-RPC 2.0 POST to a remote MCP server, via [`crate::api_gateway::egress::EgressClient`]
//! (allowlist, headers, timeouts, metrics).

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::header::{self, HeaderMap, HeaderValue};
use hyper::Method;
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::api_gateway::egress::{EgressClient, EgressHttpRequest, EgressHttpResponse};

use super::mcp::{McpClient, McpToolCallRequest, McpToolCallResult, McpToolDescriptor};

const ROUTE_LABEL: &str = "mcp_remote";

struct Session {
    initialized: bool,
    next_id: u64,
}

pub(crate) struct McpHttpRemoteClient {
    server_name: String,
    endpoint: String,
    egress: Arc<EgressClient>,
    correlation_header: String,
    egress_profile: Option<String>,
    session: Mutex<Session>,
}

impl McpHttpRemoteClient {
    pub fn new(
        server_name: String,
        endpoint: String,
        egress: Arc<EgressClient>,
        correlation_header: String,
        egress_profile: Option<String>,
    ) -> anyhow::Result<Self> {
        let ep = endpoint.trim();
        if ep.is_empty() {
            anyhow::bail!("remote_mcp_url must be non-empty");
        }
        if !ep.starts_with("http://") && !ep.starts_with("https://") {
            anyhow::bail!("remote_mcp_url must be an absolute http(s) URL");
        }
        Ok(Self {
            server_name,
            endpoint: ep.to_string(),
            egress,
            correlation_header,
            egress_profile,
            session: Mutex::new(Session {
                initialized: false,
                next_id: 1,
            }),
        })
    }

    fn next_id(s: &mut Session) -> u64 {
        let id = s.next_id;
        s.next_id = s.next_id.saturating_add(1);
        id
    }

    async fn post_json(
        &self,
        body: &Value,
        correlation_value: Option<&str>,
    ) -> anyhow::Result<EgressHttpResponse> {
        let bytes = serde_json::to_vec(body)?;
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json, text/event-stream"),
        );
        let hn = self.correlation_header.trim();
        if !hn.is_empty() {
            if let Some(cv) = correlation_value
                .map(str::trim)
                .filter(|c| !c.is_empty() && *c != "-")
            {
                if let (Ok(name), Ok(hv)) = (
                    http::HeaderName::from_bytes(hn.as_bytes()),
                    HeaderValue::from_str(cv),
                ) {
                    headers.insert(name, hv);
                }
            }
        }
        let res = self
            .egress
            .request(EgressHttpRequest {
                method: Method::POST,
                target: self.endpoint.clone(),
                route_label: ROUTE_LABEL.to_string(),
                egress_profile: self.egress_profile.clone(),
                headers,
                body: Some(Bytes::from(bytes)),
            })
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        if res.status >= 400 {
            anyhow::bail!("remote MCP HTTP status {}", res.status);
        }
        Ok(res)
    }

    /// Remote MCP may respond with `application/json` or **streamable HTTP** `text/event-stream` (first `data:` line).
    fn decode_remote_mcp_body(res: &EgressHttpResponse) -> anyhow::Result<Vec<u8>> {
        let ct = res
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ct.contains("text/event-stream") {
            Self::sse_first_data_line_json_bytes(&res.body)
        } else {
            Ok(res.body.to_vec())
        }
    }

    fn sse_first_data_line_json_bytes(raw: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s = std::str::from_utf8(raw)
            .map_err(|e| anyhow::anyhow!("remote MCP SSE body utf8: {e}"))?;
        for line in s.lines() {
            let t = line.trim_start();
            if let Some(rest) = t.strip_prefix("data:") {
                let data = rest.trim();
                if !data.is_empty() {
                    return Ok(data.as_bytes().to_vec());
                }
            }
        }
        anyhow::bail!("remote MCP SSE: no non-empty data: line");
    }

    fn rpc_id_matches(msg: &Value, expect_id: u64) -> bool {
        match msg.get("id") {
            Some(Value::Number(n)) => n.as_u64() == Some(expect_id),
            Some(Value::String(s)) => s.parse::<u64>().ok() == Some(expect_id),
            _ => false,
        }
    }

    fn parse_rpc_response(body: &[u8], expect_id: u64) -> anyhow::Result<Value> {
        if body.is_empty() {
            return Ok(Value::Null);
        }
        let v: Value = serde_json::from_slice(body)
            .map_err(|e| anyhow::anyhow!("remote MCP response JSON: {e}"))?;
        if let Some(err) = v.get("error") {
            anyhow::bail!("remote MCP JSON-RPC error: {err}");
        }
        if !Self::rpc_id_matches(&v, expect_id) {
            anyhow::bail!("remote MCP JSON-RPC id mismatch");
        }
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    }

    async fn ensure_initialized(&self) -> anyhow::Result<()> {
        let id = {
            let mut g = self.session.lock().await;
            if g.initialized {
                return Ok(());
            }
            Self::next_id(&mut g)
        };
        let init = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "panda-proxy", "version": env!("CARGO_PKG_VERSION") }
            }
        });
        let resp = self.post_json(&init, None).await?;
        let body = Self::decode_remote_mcp_body(&resp)?;
        let _ = Self::parse_rpc_response(&body, id)?;
        let note = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
            "params": {}
        });
        let _ = self.post_json(&note, None).await?;
        let mut g = self.session.lock().await;
        g.initialized = true;
        Ok(())
    }
}

#[async_trait]
impl McpClient for McpHttpRemoteClient {
    async fn list_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>> {
        self.ensure_initialized().await?;
        let mut g = self.session.lock().await;
        let id = Self::next_id(&mut g);
        drop(g);
        let req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/list",
            "params": {}
        });
        let resp = self.post_json(&req, None).await?;
        let body = Self::decode_remote_mcp_body(&resp)?;
        let result = Self::parse_rpc_response(&body, id)?;
        let tools = result
            .get("tools")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut out = Vec::new();
        for t in tools {
            let name = t
                .get("name")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                continue;
            }
            let description = t
                .get("description")
                .and_then(|x| x.as_str())
                .map(|s| s.to_string());
            let input_schema = t
                .get("inputSchema")
                .cloned()
                .unwrap_or_else(|| json!({"type": "object"}));
            out.push(McpToolDescriptor {
                server: self.server_name.clone(),
                name,
                description,
                input_schema,
            });
        }
        Ok(out)
    }

    async fn call_tool(&self, req: McpToolCallRequest) -> anyhow::Result<McpToolCallResult> {
        self.ensure_initialized().await?;
        let mut g = self.session.lock().await;
        let id = Self::next_id(&mut g);
        drop(g);
        let rpc = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": req.tool,
                "arguments": req.arguments
            }
        });
        let resp = self
            .post_json(&rpc, Some(req.correlation_id.as_str()))
            .await?;
        let body = Self::decode_remote_mcp_body(&resp)?;
        let result = Self::parse_rpc_response(&body, id)?;
        let is_error = result
            .get("isError")
            .and_then(|x| x.as_bool())
            .unwrap_or(false);
        let content = if let Some(c) = result.get("content") {
            c.clone()
        } else {
            Value::Null
        };
        Ok(McpToolCallResult { content, is_error })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panda_config::PandaConfig;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    /// Minimal HTTP/1.1 server: one request per connection, returns JSON-RPC for `initialize`, empty 200 for notification, `tools/list` / `tools/call`.
    async fn mock_mcp_upstream(listener: &tokio::net::TcpListener) {
        let Ok((mut sock, _)) = listener.accept().await else {
            return;
        };
        let mut buf = vec![0u8; 64 * 1024];
        let Ok(n) = sock.read(&mut buf).await else {
            return;
        };
        let req = std::str::from_utf8(&buf[..n]).expect("utf8");
        let Some(body_start) = req.find("\r\n\r\n") else {
            return;
        };
        let body = req[body_start + 4..].trim();
        let Ok(v) = serde_json::from_str::<Value>(body) else {
            return;
        };
        let method = v.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let mid = v
            .get("id")
            .and_then(|x| {
                x.as_u64()
                    .or_else(|| x.as_str().and_then(|s| s.parse().ok()))
            })
            .unwrap_or(0);
        let (status_line, resp_body) = match method {
            "initialize" => (
                "HTTP/1.1 200 OK",
                format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{}},\"serverInfo\":{{\"name\":\"mock\"}}}}}}"
                ),
            ),
            "notifications/initialized" => ("HTTP/1.1 200 OK", String::new()),
            "tools/list" => (
                "HTTP/1.1 200 OK",
                format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"tools\":[{{\"name\":\"alpha\",\"description\":\"d\",\"inputSchema\":{{\"type\":\"object\"}}}}]}}}}"
                ),
            ),
            "tools/call" => (
                "HTTP/1.1 200 OK",
                format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":\"ok\"}}],\"isError\":false}}}}"
                ),
            ),
            _ => ("HTTP/1.1 400 Bad Request", "{}".to_string()),
        };
        let cl = resp_body.len();
        let resp = if cl == 0 {
            format!("{status_line}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n")
        } else {
            format!("{status_line}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {cl}\r\n\r\n{resp_body}")
        };
        let _ = sock.write_all(resp.as_bytes()).await;
    }

    #[tokio::test]
    async fn remote_mcp_list_and_call_via_egress() {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();

        let h = tokio::spawn(async move {
            for _ in 0..4 {
                mock_mcp_upstream(&listener).await;
            }
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
      allow_path_prefixes: ['/']
mcp:
  enabled: true
  servers:
    - name: remote1
      remote_mcp_url: 'http://127.0.0.1:{port}/mcp'
"#
        );
        let cfg = PandaConfig::from_yaml_str(&yaml).expect("yaml");
        let egress = EgressClient::connect(&cfg.api_gateway.egress)
            .await
            .expect("egress")
            .expect("some");
        let client = McpHttpRemoteClient::new(
            "remote1".into(),
            format!("http://127.0.0.1:{port}/mcp"),
            Arc::clone(&egress),
            "x-request-id".into(),
            None,
        )
        .expect("new");
        let tools = client.list_tools().await.expect("list");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "alpha");
        let res = client
            .call_tool(McpToolCallRequest {
                server: "remote1".into(),
                tool: "alpha".into(),
                arguments: json!({}),
                correlation_id: "c1".into(),
            })
            .await
            .expect("call");
        assert!(!res.is_error);
        h.await.ok();
    }
}

//! MCP over stdio: JSON-RPC 2.0, one message per line (NDJSON).

use std::process::Stdio;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use super::mcp::{McpClient, McpToolCallRequest, McpToolCallResult, McpToolDescriptor};

pub(crate) struct StdioMcpClient {
    server_name: String,
    inner: Arc<Mutex<StdioSession>>,
}

struct StdioSession {
    _child: Child,
    stdin: BufWriter<tokio::process::ChildStdin>,
    stdout: BufReader<tokio::process::ChildStdout>,
    next_id: u64,
    initialized: bool,
}

impl StdioMcpClient {
    pub async fn spawn(server_name: &str, command: &str, args: &[String]) -> anyhow::Result<Self> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());
        cmd.kill_on_drop(true);
        let mut child = cmd.spawn()?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("MCP child missing stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("MCP child missing stdout"))?;
        Ok(Self {
            server_name: server_name.to_string(),
            inner: Arc::new(Mutex::new(StdioSession {
                _child: child,
                stdin: BufWriter::new(stdin),
                stdout: BufReader::new(stdout),
                next_id: 1,
                initialized: false,
            })),
        })
    }

    fn next_id(session: &mut StdioSession) -> u64 {
        let id = session.next_id;
        session.next_id = session.next_id.saturating_add(1);
        id
    }

    async fn write_line(session: &mut StdioSession, value: &Value) -> anyhow::Result<()> {
        let mut line = serde_json::to_string(value)?;
        line.push('\n');
        session.stdin.write_all(line.as_bytes()).await?;
        session.stdin.flush().await?;
        Ok(())
    }

    async fn read_result_for_id(
        session: &mut StdioSession,
        expect_id: u64,
    ) -> anyhow::Result<Value> {
        let mut line = String::new();
        loop {
            line.clear();
            let n = session.stdout.read_line(&mut line).await?;
            if n == 0 {
                anyhow::bail!("MCP server closed stdout while waiting for JSON-RPC response");
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let v: Value = serde_json::from_str(trimmed)
                .map_err(|e| anyhow::anyhow!("invalid MCP JSON line: {e}: {trimmed:?}"))?;
            if v.get("id").is_none() {
                continue;
            }
            let rid = jsonrpc_id_as_u64(&v);
            let Some(rid) = rid else {
                continue;
            };
            if rid != expect_id {
                continue;
            }
            if let Some(err) = v.get("error") {
                anyhow::bail!("MCP JSON-RPC error: {err}");
            }
            return Ok(v.get("result").cloned().unwrap_or(Value::Null));
        }
    }

    async fn ensure_initialized(session: &mut StdioSession) -> anyhow::Result<()> {
        if session.initialized {
            return Ok(());
        }
        let id = Self::next_id(session);
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
        Self::write_line(session, &init).await?;
        let _ = Self::read_result_for_id(session, id).await?;
        let note = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
            "params": {}
        });
        Self::write_line(session, &note).await?;
        session.initialized = true;
        Ok(())
    }
}

fn jsonrpc_id_as_u64(msg: &Value) -> Option<u64> {
    match msg.get("id") {
        Some(Value::Number(n)) => n.as_u64(),
        Some(Value::String(s)) => s.parse().ok(),
        _ => None,
    }
}

#[async_trait]
impl McpClient for StdioMcpClient {
    async fn list_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>> {
        let mut g = self.inner.lock().await;
        Self::ensure_initialized(&mut g).await?;
        let id = StdioMcpClient::next_id(&mut g);
        let req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/list",
            "params": {}
        });
        StdioMcpClient::write_line(&mut g, &req).await?;
        let result = StdioMcpClient::read_result_for_id(&mut g, id).await?;
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
        let mut g = self.inner.lock().await;
        Self::ensure_initialized(&mut g).await?;
        let id = StdioMcpClient::next_id(&mut g);
        let rpc = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": req.tool,
                "arguments": req.arguments
            }
        });
        StdioMcpClient::write_line(&mut g, &rpc).await?;
        let result = StdioMcpClient::read_result_for_id(&mut g, id).await?;
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

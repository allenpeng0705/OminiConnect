//! MCP tools backed by HTTP calls through [`crate::api_gateway::egress::EgressClient`].

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::header::{self, HeaderMap, HeaderValue};
use hyper::Method;
use panda_config::McpHttpToolConfig;
use serde_json::{json, Value};

use crate::api_gateway::egress::{EgressClient, EgressError, EgressHttpRequest};

use super::mcp::{McpClient, McpToolCallRequest, McpToolCallResult, McpToolDescriptor};

const ROUTE_LABEL: &str = "mcp_http_tool";

pub(crate) struct McpHttpToolClient {
    server_name: String,
    tools: Vec<McpHttpToolConfig>,
    egress: Arc<EgressClient>,
    correlation_header: String,
}

impl McpHttpToolClient {
    pub fn new(
        server_name: String,
        tools: Vec<McpHttpToolConfig>,
        egress: Arc<EgressClient>,
        correlation_header: String,
    ) -> anyhow::Result<Self> {
        if tools.is_empty() {
            anyhow::bail!("McpHttpToolClient requires at least one http_tool entry");
        }
        Ok(Self {
            server_name,
            tools,
            egress,
            correlation_header,
        })
    }

    fn http_method(server: &str, ht: &McpHttpToolConfig) -> anyhow::Result<Method> {
        let m = ht.method.trim();
        Method::from_bytes(m.as_bytes()).map_err(|_| {
            anyhow::anyhow!("invalid http_tool.method for server {:?}: {:?}", server, m)
        })
    }

    fn body_for_method(method: &Method, arguments: &Value) -> Option<Bytes> {
        match *method {
            Method::POST | Method::PUT | Method::PATCH => {
                let bytes = serde_json::to_vec(arguments).unwrap_or_else(|_| b"{}".to_vec());
                Some(Bytes::from(bytes))
            }
            _ => None,
        }
    }

    fn content_type_for_method(method: &Method) -> Option<&'static str> {
        match *method {
            Method::POST | Method::PUT | Method::PATCH => Some("application/json"),
            _ => None,
        }
    }

    async fn execute_one_tool(
        &self,
        ht: &McpHttpToolConfig,
        req: &McpToolCallRequest,
    ) -> anyhow::Result<McpToolCallResult> {
        let method = Self::http_method(&self.server_name, ht)?;
        let mut headers = HeaderMap::new();
        if let Some(ct) = Self::content_type_for_method(&method) {
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(ct));
        }
        let corr = req.correlation_id.trim();
        if !corr.is_empty() && corr != "-" {
            let name = self.correlation_header.trim();
            if !name.is_empty() {
                if let Ok(hv) = HeaderValue::from_str(corr) {
                    if let Ok(hn) = http::HeaderName::from_bytes(name.as_bytes()) {
                        headers.insert(hn, hv);
                    }
                }
            }
        }
        let body = Self::body_for_method(&method, &req.arguments);
        let egress_profile = ht
            .egress_profile
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let res = self
            .egress
            .request(EgressHttpRequest {
                method,
                target: ht.path.clone(),
                route_label: ROUTE_LABEL.to_string(),
                egress_profile,
                headers,
                body,
            })
            .await;
        let (status, body_bytes, err_opt) = match res {
            Ok(r) => (r.status, r.body, None),
            Err(e) => {
                let msg = e.to_string();
                let code = match e {
                    EgressError::AllowlistDenied => 403u16,
                    EgressError::RateLimited => 429,
                    EgressError::Timeout => 504,
                    EgressError::BodyTooLarge => 502,
                    _ => 502,
                };
                (code, Bytes::new(), Some(msg))
            }
        };
        let is_error = status >= 400 || err_opt.is_some();
        let content = if let Some(err) = err_opt {
            json!({
                "error": err,
                "status": status,
            })
        } else {
            match serde_json::from_slice::<Value>(&body_bytes) {
                Ok(v) => json!({
                    "status": status,
                    "body": v,
                }),
                Err(_) => json!({
                    "status": status,
                    "body_raw": String::from_utf8_lossy(&body_bytes).to_string(),
                }),
            }
        };
        Ok(McpToolCallResult { content, is_error })
    }
}

#[async_trait]
impl McpClient for McpHttpToolClient {
    async fn list_tools(&self) -> anyhow::Result<Vec<McpToolDescriptor>> {
        let mut out = Vec::with_capacity(self.tools.len());
        for ht in &self.tools {
            out.push(McpToolDescriptor {
                server: self.server_name.clone(),
                name: ht.tool_name.clone(),
                description: ht.description.clone(),
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": true,
                    "description": "Forwarded as JSON body for POST/PUT/PATCH; ignored for GET/HEAD/DELETE."
                }),
            });
        }
        Ok(out)
    }

    async fn call_tool(&self, req: McpToolCallRequest) -> anyhow::Result<McpToolCallResult> {
        let ht = self
            .tools
            .iter()
            .find(|t| t.tool_name == req.tool)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "unknown tool {:?} for MCP HTTP server {:?} (available: {:?})",
                    req.tool,
                    self.server_name,
                    self.tools
                        .iter()
                        .map(|t| t.tool_name.as_str())
                        .collect::<Vec<_>>()
                )
            })?;
        self.execute_one_tool(ht, &req).await
    }
}

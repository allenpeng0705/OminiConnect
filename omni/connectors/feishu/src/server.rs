//! Feishu MCP Server.

use std::sync::Arc;
use tokio::sync::RwLock;

use serde_json::{json, Value};

use crate::api::FeishuApiClient;
use crate::tools::FeishuTool;

/// MCP JSON-RPC request
#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// MCP JSON-RPC response
#[derive(Debug, serde::Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl JsonRpcResponse {
    pub fn success(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<Value>, code: i64, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(json!({
                "code": code,
                "message": message,
            })),
        }
    }
}

/// Feishu MCP Server implementation
pub struct FeishuMcpServer {
    api_client: Arc<RwLock<FeishuApiClient>>,
    tools: Vec<FeishuTool>,
}

impl FeishuMcpServer {
    pub fn new(api_client: FeishuApiClient) -> Self {
        Self {
            api_client: Arc::new(RwLock::new(api_client)),
            tools: FeishuTool::all_tools(),
        }
    }

    pub async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
            "initialize" => {
                JsonRpcResponse::success(req.id, json!({
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "feishu",
                        "version": "0.1.0"
                    }
                }))
            }
            "tools/list" => {
                let tools: Vec<Value> = self.tools.iter().map(|t| json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.input_schema,
                })).collect();

                JsonRpcResponse::success(req.id, json!({
                    "tools": tools
                }))
            }
            "tools/call" => {
                let tool_name = req.params.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let arguments = req.params.get("arguments")
                    .and_then(|v| v.as_object())
                    .cloned()
                    .map(serde_json::Value::Object)
                    .unwrap_or(json!({}));

                let api = self.api_client.read().await;
                let token_result = api.get_tenant_token().await;

                let token = match token_result {
                    Ok(t) => t,
                    Err(e) => {
                        return JsonRpcResponse::error(req.id, 500, e.to_string());
                    }
                };

                let result = self.call_tool_internal(&api, &token, tool_name, arguments).await;

                match result {
                    Ok(data) => JsonRpcResponse::success(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&data).unwrap_or_default()
                        }]
                    })),
                    Err(e) => JsonRpcResponse::error(req.id, 500, e.to_string()),
                }
            }
            "ping" => {
                JsonRpcResponse::success(req.id, json!({ "status": "ok" }))
            }
            _ => {
                JsonRpcResponse::error(req.id, -32601, format!("Unknown method: {}", req.method))
            }
        }
    }

    async fn call_tool_internal(
        &self,
        api: &FeishuApiClient,
        token: &str,
        tool_name: &str,
        arguments: Value,
    ) -> anyhow::Result<Value> {
        match tool_name {
            "calendar_list" => {
                api.call_api("GET", "/open-apis/calendar/v4/calendars", token, None).await
            }
            "calendar_event_list" => {
                let calendar_id = arguments.get("calendar_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("primary");
                let path = format!("/open-apis/calendar/v4/calendars/{}/events", calendar_id);
                api.call_api("GET", &path, token, None).await
            }
            "bitable_list" => {
                api.call_api("GET", "/open-apis/bitable/v1/apps", token, None).await
            }
            "bitable_table_list" => {
                let app_token = arguments.get("app_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let path = format!("/open-apis/bitable/v1/apps/{}/tables", app_token);
                api.call_api("GET", &path, token, None).await
            }
            "bitable_record_list" => {
                let app_token = arguments.get("app_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let table_id = arguments.get("table_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let path = format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records",
                    app_token, table_id
                );
                api.call_api("GET", &path, token, None).await
            }
            "message_send" => {
                let body = json!({
                    "receive_id": arguments.get("receive_id"),
                    "receive_id_type": arguments.get("receive_id_type"),
                    "msg_type": arguments.get("msg_type"),
                    "content": arguments.get("content"),
                });
                api.call_api(
                    "POST",
                    "/open-apis/im/v1/messages?receive_id_type=open_id",
                    token,
                    Some(body),
                )
                .await
            }
            "message_list" => {
                let container_id = arguments.get("container_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let container_id_type = arguments.get("container_id_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("p2p");
                let path = format!(
                    "/open-apis/im/v1/messages?container_id_type={}&container_id={}",
                    container_id_type, container_id
                );
                api.call_api("GET", &path, token, None).await
            }
            _ => {
                anyhow::bail!("unknown tool: {}", tool_name)
            }
        }
    }
}

//! DingTalk MCP Server.

use std::sync::Arc;
use tokio::sync::RwLock;

use serde_json::{json, Value};

use crate::api::DingTalkApiClient;
use crate::tools::DingTalkTool;

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

/// DingTalk MCP Server
pub struct DingTalkMcpServer {
    api_client: Arc<RwLock<DingTalkApiClient>>,
    tools: Vec<DingTalkTool>,
}

impl DingTalkMcpServer {
    pub fn new(api_client: DingTalkApiClient) -> Self {
        Self {
            api_client: Arc::new(RwLock::new(api_client)),
            tools: DingTalkTool::all_tools(),
        }
    }

    pub async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
            "initialize" => {
                JsonRpcResponse::success(req.id, json!({
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "dingtalk",
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
                let token_result = api.get_access_token().await;

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
        api: &DingTalkApiClient,
        token: &str,
        tool_name: &str,
        arguments: Value,
    ) -> anyhow::Result<Value> {
        match tool_name {
            "workflow_list" => {
                let process_code = arguments.get("process_code")
                    .and_then(|v| v.as_str());
                let path = if let Some(code) = process_code {
                    format!("/topapi/process/instance/list?process_code={}", code)
                } else {
                    "/topapi/process/instance/list".to_string()
                };
                api.call_api("POST", &path, token, None).await
            }
            "workflow_instance_create" => {
                let body = json!({
                    "process_code": arguments.get("process_code"),
                    "requester": arguments.get("requester"),
                    "approvers": arguments.get("approvers"),
                    "data": arguments.get("data"),
                });
                api.call_api("POST", "/topapi/process/instance/create", token, Some(body)).await
            }
            "workflow_instance_detail" => {
                let process_instance_id = arguments.get("process_instance_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let body = json!({ "process_instance_id": process_instance_id });
                api.call_api("POST", "/topapi/process/instance/get", token, Some(body)).await
            }
            "message_send" => {
                let body = json!({
                    "receive_id": arguments.get("receive_id"),
                    "msgtype": arguments.get("msg_type"),
                    "message": arguments.get("content"),
                });
                api.call_api("POST", "/message/send_to_conversation", token, Some(body)).await
            }
            "message_list" => {
                let chat_id = arguments.get("chat_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let path = format!("/message/list?chat_id={}", chat_id);
                api.call_api("GET", &path, token, None).await
            }
            "user_get" => {
                let userid = arguments.get("userid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let body = json!({ "userid": userid });
                api.call_api("POST", "/topapi/v2/user/get", token, Some(body)).await
            }
            "user_list" => {
                let dept_id = arguments.get("dept_id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1);
                let body = json!({ "dept_id": dept_id });
                api.call_api("POST", "/topapi/user/list", token, Some(body)).await
            }
            _ => {
                anyhow::bail!("unknown tool: {}", tool_name)
            }
        }
    }
}

//! WeChat Work MCP Server.

use std::sync::Arc;
use tokio::sync::RwLock;

use serde_json::{json, Value};

use crate::api::WeChatWorkApiClient;
use crate::tools::WeChatWorkTool;

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

/// WeChat Work MCP Server
pub struct WeChatWorkMcpServer {
    api_client: Arc<RwLock<WeChatWorkApiClient>>,
    tools: Vec<WeChatWorkTool>,
}

impl WeChatWorkMcpServer {
    pub fn new(api_client: WeChatWorkApiClient) -> Self {
        Self {
            api_client: Arc::new(RwLock::new(api_client)),
            tools: WeChatWorkTool::all_tools(),
        }
    }

    pub async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
            "initialize" => {
                JsonRpcResponse::success(req.id, json!({
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "wechatwork",
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
        api: &WeChatWorkApiClient,
        token: &str,
        tool_name: &str,
        arguments: Value,
    ) -> anyhow::Result<Value> {
        match tool_name {
            "external_contact_list" => {
                let userid = arguments.get("userid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let path = format!("/cgi-bin/externalcontact/list?userid={}", userid);
                api.call_api("GET", &path, token, None).await
            }
            "external_contact_get" => {
                let external_userid = arguments.get("external_userid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let body = json!({ "external_userid": external_userid });
                api.call_api("POST", "/cgi-bin/externalcontact/get", token, Some(body)).await
            }
            "external_contact_tag_list" => {
                api.call_api("GET", "/cgi-bin/externalcontact/tag/list", token, None).await
            }
            "external_contact_tag_add" => {
                let body = json!({
                    "external_userid": arguments.get("external_userid"),
                    "tag_id": arguments.get("tag_id"),
                });
                api.call_api("POST", "/cgi-bin/externalcontact/tag/add", token, Some(body)).await
            }
            "message_send" => {
                let body = json!({
                    "touser": arguments.get("to_user"),
                    "toparty": arguments.get("to_party"),
                    "msgtype": arguments.get("msg_type"),
                    "agentid": arguments.get("agent_id"),
                    "text": { "content": arguments.get("content") },
                });
                api.call_api("POST", "/cgi-bin/message/send", token, Some(body)).await
            }
            "message_list" => {
                let chat_id = arguments.get("chat_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let path = format!("/cgi-bin/message/list?chat_id={}", chat_id);
                api.call_api("GET", &path, token, None).await
            }
            "user_get" => {
                let userid = arguments.get("userid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let body = json!({ "userid": userid });
                api.call_api("POST", "/cgi-bin/user/get", token, Some(body)).await
            }
            "department_list" => {
                let id = arguments.get("id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1);
                let path = format!("/cgi-bin/department/list?id={}", id);
                api.call_api("GET", &path, token, None).await
            }
            "user_list_by_department" => {
                let department_id = arguments.get("department_id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1);
                let fetch_child = arguments.get("fetch_child")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let path = format!(
                    "/cgi-bin/user/simplelist?department_id={}&fetch_child={}",
                    department_id, if fetch_child { 1 } else { 0 }
                );
                api.call_api("GET", &path, token, None).await
            }
            _ => {
                anyhow::bail!("unknown tool: {}", tool_name)
            }
        }
    }
}

//! Maton.ai MCP server

use crate::api::{Connection, MatonClient, MatonError};
use crate::tools::all_tools;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// MCP JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

/// MCP JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

/// MCP server for Maton.ai connector
#[derive(Debug, Clone)]
pub struct MatonMcpServer {
    client: Arc<RwLock<Option<MatonClient>>>,
}

impl MatonMcpServer {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Configure the Maton.ai API key
    pub async fn configure(&self, api_key: String) {
        let mut client = self.client.write().await;
        *client = Some(MatonClient::new(api_key));
    }

    /// Handle an MCP request
    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let method = request.method.as_str();

        match method {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "omni-connector-maton",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            },
            "tools/list" => {
                let tools: Vec<_> = all_tools()
                    .into_iter()
                    .map(|t| {
                        serde_json::json!({
                            "name": t.name,
                            "description": t.description,
                            "inputSchema": t.input_schema
                        })
                    })
                    .collect();

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({ "tools": tools })),
                    error: None,
                }
            }
            "tools/call" => {
                let client_guard = self.client.read().await;
                let client = client_guard.as_ref();

                match client {
                    Some(client) => {
                        let params = request.params.unwrap_or(serde_json::Value::Null);
                        let result = self.handle_tool_call(client, &params).await;

                        match result {
                            Ok(result) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id,
                                result: Some(result),
                                error: None,
                            },
                            Err(e) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id,
                                result: None,
                                error: Some(JsonRpcError {
                                    code: -32603,
                                    message: e.to_string(),
                                }),
                            },
                        }
                    }
                    None => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32603,
                            message: "Maton.ai not configured. Set MATON_API_KEY.".to_string(),
                        }),
                    },
                }
            }
            "ping" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({ "status": "ok" })),
                error: None,
            },
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {method}"),
                }),
            },
        }
    }

    async fn handle_tool_call(
        &self,
        client: &MatonClient,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, MatonError> {
        let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let arguments = params.get("arguments").cloned().unwrap_or(serde_json::Value::Null);

        match tool_name {
            "maton_list_connections" => {
                let app = arguments.get("app").and_then(|v| v.as_str());
                let status = arguments.get("status").and_then(|v| v.as_str());

                let connections = client.list_connections(app, status).await?;
                let text = format_connections_list(&connections);

                Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": text
                    }]
                }))
            }
            "maton_create_connection" => {
                let app = arguments.get("app").and_then(|v| v.as_str()).unwrap_or("");

                let connection = client.create_connection(app).await?;

                Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": format_connection(&connection)
                    }]
                }))
            }
            "maton_gateway_call" => {
                let app = arguments.get("app").and_then(|v| v.as_str()).unwrap_or("");
                let method = arguments.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
                let path = arguments.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                let body = arguments.get("body").cloned();
                let connection_id = arguments.get("connection_id").and_then(|v| v.as_str());

                let result = client
                    .gateway_call(app, method, path, body, connection_id)
                    .await?;

                Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                    }]
                }))
            }
            _ => Err(MatonError::Api {
                code: -32601,
                message: format!("Unknown tool: {tool_name}"),
            }),
        }
    }
}

fn format_connections_list(connections: &[Connection]) -> String {
    if connections.is_empty() {
        return "No connections found. Use maton_create_connection to add one.".to_string();
    }

    let lines: Vec<String> = connections
        .iter()
        .map(|c| {
            format!(
                "- {} ({}): created={}, updated={}",
                c.app, c.status, c.creation_time, c.last_updated_time
            )
        })
        .collect();

    format!("Connected services:\n{}", lines.join("\n"))
}

fn format_connection(conn: &Connection) -> String {
    let method = conn.method.as_deref().unwrap_or("unknown");
    let status_msg: String = match conn.status.as_str() {
        "ACTIVE" => "Connected and ready".to_string(),
        "PENDING" => {
            if let Some(url) = &conn.url {
                format!("Pending authorization. Open this URL to complete: {}", url)
            } else {
                "Pending authorization".to_string()
            }
        }
        "FAILED" => "Connection failed".to_string(),
        _ => conn.status.clone(),
    };

    format!(
        "Connection: {}\nApp: {}\nMethod: {}\nStatus: {}\nConnection ID: {}",
        status_msg, conn.app, method, conn.status, conn.connection_id
    )
}

impl Default for MatonMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

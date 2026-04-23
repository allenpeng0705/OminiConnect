//! OminiConnect MCP client

use crate::config::Config;
use crate::error::{Result, SdkError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP JSON-RPC request
#[derive(Debug, Clone, Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    method: String,
    params: Value,
    id: u64,
}

/// MCP JSON-RPC response
#[derive(Debug, Clone, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: u64,
}

/// JSON-RPC error
#[derive(Debug, Clone, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

/// MCP tool definition
#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema
    pub input_schema: Value,
}

/// MCP server capabilities
#[derive(Debug, Clone, Deserialize)]
pub struct ServerCapabilities {
    /// Tools capability
    pub tools: Option<ToolsCapability>,
}

/// Tools capability
#[derive(Debug, Clone, Deserialize)]
pub struct ToolsCapability {
    /// Whether tools can be listed
    pub list_tools: Option<Value>,
}

/// Initialize result
#[derive(Debug, Clone, Deserialize)]
pub struct InitializeResult {
    /// Protocol version
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
    /// Server info
    pub server_info: ServerInfo,
}

/// Server info
#[derive(Debug, Clone, Deserialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
}

/// Tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether the call was successful
    pub success: bool,
    /// Result content
    pub content: Vec<ToolContent>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Tool content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    /// Content type (text, image, etc.)
    #[serde(rename = "type")]
    pub content_type: String,
    /// Content value
    pub text: Option<String>,
}

/// Client for connecting to OminiConnect MCP servers
#[derive(Debug, Clone)]
pub struct Client {
    server_url: String,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new client connected to the given server URL
    pub fn new(server_url: impl Into<String>) -> Self {
        Self {
            server_url: server_url.into(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Connect using configuration
    pub async fn connect(_config: Config) -> Result<Self> {
        // In a real implementation, this would:
        // 1. Authenticate with OAuth2 vault
        // 2. Get tokens for each platform
        // 3. Establish connections to MCP servers
        // For now, return a basic client
        Ok(Self {
            server_url: "http://localhost:8080".to_string(),
            http_client: reqwest::Client::new(),
        })
    }

    /// Initialize the connection
    pub async fn initialize(&self, client_info: &str) -> Result<InitializeResult> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            method: "initialize".to_string(),
            params: serde_json::json!({
                "protocolVersion": "2024-11-05",
                "clientInfo": {
                    "name": client_info,
                    "version": "0.1.0"
                },
                "capabilities": {}
            }),
            id: 1,
        };

        let response = self.send_request(request).await?;
        serde_json::from_value(response)
            .map_err(|e| SdkError::Connection(format!("Failed to parse initialize response: {}", e)))
    }

    /// List available tools
    pub async fn list_tools(&self) -> Result<Vec<Tool>> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            method: "tools/list".to_string(),
            params: serde_json::json!({}),
            id: 2,
        };

        let response = self.send_request(request).await?;

        // Parse tools.list response
        let tools: Vec<Tool> = serde_json::from_value(response)
            .map_err(|e| SdkError::Connection(format!("Failed to parse tools list: {}", e)))?;

        Ok(tools)
    }

    /// Call a tool
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<ToolResult> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            method: "tools/call".to_string(),
            params: serde_json::json!({
                "name": name,
                "arguments": arguments
            }),
            id: 3,
        };

        let response = self.send_request(request).await?;

        serde_json::from_value(response)
            .map_err(|e| SdkError::ToolInvocation(format!("Failed to parse tool result: {}", e)))
    }

    /// Send a JSON-RPC request
    async fn send_request(&self, request: JsonRpcRequest) -> Result<Value> {
        let response = self.http_client
            .post(&self.server_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| SdkError::Connection(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SdkError::Connection(format!(
                "Server returned error: {}",
                response.status()
            )));
        }

        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| SdkError::Connection(format!("Failed to parse response: {}", e)))?;

        if let Some(error) = rpc_response.error {
            return Err(SdkError::ToolInvocation(error.message));
        }

        rpc_response.result.ok_or_else(|| {
            SdkError::Connection("No result in response".to_string())
        })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new("http://localhost:8080")
    }
}

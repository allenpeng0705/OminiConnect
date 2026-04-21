//! Maton.ai connector binary
//!
//! MCP server that wraps Maton.ai API gateway.
//! Configure via MATON_API_KEY environment variable.

use omni_connector_maton::MatonMcpServer;
use std::net::SocketAddr;
use axum::{
    Router,
    routing::post,
    extract::State,
    response::Json,
    http::StatusCode,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    server: MatonMcpServer,
}

#[derive(Deserialize)]
struct McpRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    #[serde(default)]
    params: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<serde_json::Value>,
}

async fn handle_mcp(
    State(state): State<AppState>,
    Json(request): Json<McpRequest>,
) -> Result<Json<McpResponse>, StatusCode> {
    let response = state
        .server
        .handle_request(omni_connector_maton::server::JsonRpcRequest {
            jsonrpc: request.jsonrpc,
            id: request.id,
            method: request.method,
            params: request.params,
        })
        .await;

    Ok(Json(McpResponse {
        jsonrpc: response.jsonrpc,
        id: response.id,
        result: response.result,
        error: response.error.map(|e| {
            serde_json::json!({
                "code": e.code,
                "message": e.message
            })
        }),
    }))
}

#[tokio::main]
async fn main() {
    // Load API key from environment
    let api_key = std::env::var("MATON_API_KEY")
        .expect("MATON_API_KEY environment variable must be set");

    println!("Starting Maton.ai connector MCP server");
    println!("Gateway: https://gateway.maton.ai/{{app}}/{{path}}");
    println!("Control: https://ctrl.maton.ai/connections");
    println!("Listening on http://127.0.0.1:8093/mcp");

    let server = MatonMcpServer::new();
    server.configure(api_key).await;

    let app = Router::new()
        .route("/mcp", post(handle_mcp))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(AppState { server });

    let addr: SocketAddr = "127.0.0.1:8093".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

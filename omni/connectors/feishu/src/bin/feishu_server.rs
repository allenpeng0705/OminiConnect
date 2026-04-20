//! Feishu MCP Server binary
//!
//! Run as: cargo run -p omni-connector-feishu --bin feishu_server

use omni_connector_feishu::{FeishuApiClient, FeishuMcpServer, JsonRpcRequest};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};

#[derive(Clone)]
struct AppState {
    server: Arc<RwLock<FeishuMcpServer>>,
}

async fn handle_mcp(
    State(state): State<AppState>,
    Json(req): Json<JsonRpcRequest>,
) -> Json<omni_connector_feishu::JsonRpcResponse> {
    let server = state.server.read().await;
    Json(server.handle_request(req).await)
}

#[tokio::main]
async fn main() {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();

    let api_client = FeishuApiClient::new(
        Arc::new(MockTokenVault),
        app_id,
        app_secret,
    );

    let server = FeishuMcpServer::new(api_client);

    let state = AppState {
        server: Arc::new(RwLock::new(server)),
    };

    let app = Router::new()
        .route("/mcp", post(handle_mcp))
        .with_state(state);

    let port = std::env::args()
        .find(|arg| arg.starts_with("--port="))
        .and_then(|arg| arg.strip_prefix("--port=").map(|s| s.to_string()))
        .and_then(|s| s.parse().ok())
        .unwrap_or(8090);

    println!("Feishu MCP server listening on http://0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Mock token vault for standalone server
struct MockTokenVault;

impl omni_connector_feishu::api::TokenVaultAccess for MockTokenVault {
    fn get_token(&self, _platform: &str, _subject: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send + '_>> {
        Box::pin(async { Ok("mock_token".to_string()) })
    }
}

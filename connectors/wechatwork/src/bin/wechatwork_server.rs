//! WeChat Work MCP Server binary

use axum::{extract::State, routing::post, Json, Router};
use omini_connect_wechatwork::{
    JsonRpcRequest, TokenVaultAccess, WeChatWorkApiClient, WeChatWorkMcpServer,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppState {
    server: Arc<RwLock<WeChatWorkMcpServer>>,
}

async fn handle_mcp(
    State(state): State<AppState>,
    Json(req): Json<JsonRpcRequest>,
) -> Json<omini_connect_wechatwork::JsonRpcResponse> {
    let server = state.server.read().await;
    Json(server.handle_request(req).await)
}

#[tokio::main]
async fn main() {
    let api_client = WeChatWorkApiClient::new();
    let vault = Arc::new(MockTokenVault::new());

    let server = WeChatWorkMcpServer::new(api_client, vault);

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
        .unwrap_or(8092);

    println!(
        "WeChat Work MCP server listening on http://0.0.0.0:{}",
        port
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Mock token vault for standalone server
/// In production, this would be replaced with a real OAuthVault from oauth_vault
struct MockTokenVault {
    token: String,
}

impl MockTokenVault {
    fn new() -> Self {
        Self {
            token: "mock_token".to_string(),
        }
    }
}

impl TokenVaultAccess for MockTokenVault {
    fn get_token(
        &self,
        _platform: &str,
        _subject: &str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send + '_>,
    > {
        Box::pin(async { Ok(self.token.clone()) })
    }
}

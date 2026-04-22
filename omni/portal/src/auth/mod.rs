//! Authentication: login, logout, API key management.

pub mod handlers;
pub mod middleware;
pub mod models;

use std::sync::Arc;

use axum::{Router, routing::get, routing::post, routing::delete};

use crate::app::AppState;

async fn serve_spa() -> axum::response::Html<String> {
    match tokio::fs::read_to_string("/Users/shileipeng/Documents/mygithub/OminiConnect/omni/portal/frontend/dist/index.html").await {
        Ok(html) => axum::response::Html(html),
        Err(e) => {
            tracing::error!("Failed to read SPA HTML: {}", e);
            axum::response::Html("<html><body><h1>Server error</h1></body></html>".to_string())
        }
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(handlers::login).get(serve_spa))
        .route("/logout", post(handlers::logout))
        .route("/apikey", get(handlers::list_api_keys).post(handlers::generate_api_key))
        .route("/apikey/:key_hash", delete(handlers::delete_api_key))
        .route("/me", get(handlers::me))
}

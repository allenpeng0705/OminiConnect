//! Authentication: login, logout, API key management.

pub mod handlers;
pub mod middleware;
pub mod models;

use std::sync::Arc;

use axum::{Router, routing::get, routing::post};

use crate::app::AppState;

const SPA_HTML: &str = std::include_str!("../../frontend/dist/index.html");

async fn serve_spa() -> axum::response::Html<&'static str> {
    axum::response::Html(SPA_HTML)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(handlers::login).get(serve_spa))
        .route("/logout", post(handlers::logout))
        .route("/apikey", post(handlers::generate_api_key))
        .route("/me", get(handlers::me))
}

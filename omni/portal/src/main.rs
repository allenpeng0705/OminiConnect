//! OmniConnect Portal — Operator Connection Management UI
//!
//! Run with: `cargo run -p omni-portal`
//!
//! Then open http://localhost:8090

mod api;
mod app;
mod auth;
mod db;
mod oauth;

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::AppState;

const PORT: u16 = 8090;

const SPA_HTML: &str = std::include_str!("../frontend/dist/index.html");

async fn serve_spa() -> axum::response::Html<&'static str> {
    axum::response::Html(SPA_HTML)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(AppState::new());

    // Seed default admin user
    db::seed_admin_user(&app_state).await;

    let app = Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/auth/login") }))
        .nest("/auth", auth::router())
        .nest("/oauth", oauth::router())
        .nest("/api", api::router())
        .route_service("/*path", ServeDir::new("omni/portal/frontend/dist"))
        .fallback(serve_spa)
        .with_state(app_state);

    let addr = format!("0.0.0.0:{PORT}");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Portal listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

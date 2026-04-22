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

const PORT: u16 = 9000;

const SPA_HTML_PATH: &str = "/Users/shileipeng/Documents/mygithub/OminiConnect/omni/portal/frontend/dist/index.html";

async fn serve_spa() -> axum::response::Html<String> {
    match tokio::fs::read_to_string(SPA_HTML_PATH).await {
        Ok(html) => axum::response::Html(html),
        Err(e) => {
            tracing::error!("Failed to read SPA HTML: {}", e);
            axum::response::Html("<html><body><h1>Server error</h1></body></html>".to_string())
        }
    }
}

/// Register platform handlers with OAuthVault based on configured connectors.
async fn register_platforms(state: &Arc<AppState>) {
    let connectors = match state.connectors.list().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Could not load connectors for platform registration: {}", e);
            return;
        }
    };

    for config in connectors.iter().filter(|c| c.enabled) {
        let platform_config = omni_oauth_vault::PlatformConfig {
            name: config.platform.clone(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            auth_url: String::new(),
            token_url: String::new(),
            revoke_url: None,
            redirect_uri: config.redirect_uri.clone(),
            scopes: config.scopes.clone(),
        };

        use omni_oauth_vault::OAuth2Platform;

        let handler: Box<dyn OAuth2Platform + Send + Sync> = match config.platform.as_str() {
            "feishu" => Box::new(omni_oauth_vault::platforms::FeishuPlatform::new(platform_config)),
            "dingtalk" => Box::new(omni_oauth_vault::platforms::DingTalkPlatform::new(platform_config)),
            "wechatwork" => Box::new(omni_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config)),
            "linkedin" => Box::new(omni_oauth_vault::platforms::LinkedInPlatform::new(platform_config)),
            "facebook" => Box::new(omni_oauth_vault::platforms::FacebookPlatform::new(platform_config)),
            _ => continue,
        };

        state.oauth_vault.register_platform(handler).await;
        tracing::info!("Registered platform handler: {}", config.platform);
    }
}

/// Background task that periodically checks and refreshes tokens.
async fn token_refresh_loop(vault: Arc<omni_oauth_vault::OAuthVault>) {
    let check_interval = std::time::Duration::from_secs(300); // 5 minutes

    loop {
        tokio::time::sleep(check_interval).await;

        tracing::debug!("Checking tokens for refresh...");

        for platform in ["feishu", "dingtalk", "wechatwork", "linkedin", "facebook"] {
            match vault.get_token(platform, "user").await {
                Ok(_) => {
                    tracing::debug!("Token OK for {}", platform);
                }
                Err(omni_oauth_vault::OAuthError::TokenNotFound(_)) => {
                    // No token stored for this platform yet
                }
                Err(e) => {
                    tracing::warn!("Token check failed for {}: {}", platform, e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create DB pool and run migrations
    let pool = db::create_pool().await?;
    db::run_migrations(&pool).await?;

    let app_state = Arc::new(AppState::new(pool).await);

    // Seed default admin user
    db::seed_admin_user(app_state.users.as_ref()).await;

    // Register platform handlers
    register_platforms(&app_state).await;

    // Spawn background token refresh task
    let vault_for_refresh = Arc::clone(&app_state.oauth_vault);
    tokio::spawn(async move {
        token_refresh_loop(vault_for_refresh).await;
    });

    let app = Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/auth/login") }))
        .nest("/auth", auth::router())
        .nest("/oauth", oauth::router())
        .nest("/api", api::router())
        .route_service("/assets/*path", ServeDir::new("/Users/shileipeng/Documents/mygithub/OminiConnect/omni/portal/frontend/dist"))
        .fallback(serve_spa)
        .with_state(app_state);

    let addr = format!("0.0.0.0:{PORT}");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Portal listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

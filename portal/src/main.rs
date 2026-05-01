//! OminiConnect Portal — Operator Connection Management UI
//!
//! Run with: `cargo run -p omini-connect-portal`
//!
//! Listen port: `PORTAL_LISTEN_PORT` (default 9000). Then open http://localhost:9000

mod api;
mod app;
mod auth;
mod connector_engine;
mod connector_scope;
mod db;
mod llm;
mod nango;
mod oauth;
mod panda;
mod portal_env;
mod telemetry;
mod argument_extractor;
mod tools;
mod websocket;

use std::sync::Arc;

use axum::{
    routing::{any, get},
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::AppState;

fn portal_listen_port() -> u16 {
    std::env::var("PORTAL_LISTEN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9000)
}

fn spa_index_html() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/dist/index.html")
}

fn spa_dist_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/dist")
}

async fn serve_spa() -> axum::response::Html<String> {
    match tokio::fs::read_to_string(spa_index_html()).await {
        Ok(html) => axum::response::Html(html),
        Err(e) => {
            tracing::error!("Failed to read SPA HTML: {}", e);
            axum::response::Html("<html><body><h1>Server error</h1></body></html>".to_string())
        }
    }
}

/// Register platform handlers with OAuthVault based on configured connectors.
async fn register_platforms(state: &Arc<AppState>) {
    let connectors = match state.connectors.list_all().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Could not load connectors for platform registration: {}", e);
            return;
        }
    };

    use omini_connect_oauth_vault::OAuth2Platform;

    for config in connectors
        .iter()
        .filter(|c| c.enabled)
        .filter(|c| c.engine != "nango")
    {
        let platform_config = omini_connect_oauth_vault::PlatformConfig {
            name: config.platform.clone(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            auth_url: String::new(),
            token_url: String::new(),
            revoke_url: None,
            redirect_uri: config.redirect_uri.clone(),
            scopes: config.scopes.clone(),
            agent_id: config.agent_id.clone(),
        };

        let storage_key =
            connector_scope::oauth_vault_platform_key(&config.owner_username, &config.platform);

        let inner: Box<dyn OAuth2Platform + Send + Sync> = match config.platform.as_str() {
            "feishu" => Box::new(omini_connect_oauth_vault::platforms::FeishuPlatform::new(
                platform_config,
            )),
            "dingtalk" => Box::new(omini_connect_oauth_vault::platforms::DingTalkPlatform::new(
                platform_config,
            )),
            "wechatwork" => Box::new(
                omini_connect_oauth_vault::platforms::WeChatWorkPlatform::new(platform_config),
            ),
            "linkedin" => Box::new(omini_connect_oauth_vault::platforms::LinkedInPlatform::new(
                platform_config,
            )),
            "facebook" => Box::new(omini_connect_oauth_vault::platforms::FacebookPlatform::new(
                platform_config,
            )),
            "x" => Box::new(omini_connect_oauth_vault::platforms::XPlatform::new(
                platform_config,
            )),
            _ => continue,
        };

        let handler = Box::new(
            crate::oauth::vault_namespaced::VaultNamespacedPlatform::new(
                storage_key.clone(),
                inner,
            ),
        );
        state.oauth_vault.register_platform(handler).await;
        tracing::info!(
            "Registered platform handler: {} ({})",
            config.platform,
            storage_key
        );
    }
}

/// Background task that periodically checks and refreshes tokens.
async fn token_refresh_loop(state: Arc<AppState>) {
    let check_interval = std::time::Duration::from_secs(300); // 5 minutes
    let vault = Arc::clone(&state.oauth_vault);

    loop {
        tokio::time::sleep(check_interval).await;

        tracing::debug!("Checking tokens for refresh...");

        let connectors = match state.connectors.list_all().await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("token_refresh_loop: could not list connectors: {}", e);
                continue;
            }
        };

        for c in connectors
            .iter()
            .filter(|c| c.enabled && c.engine != "nango")
            .filter(|c| {
                matches!(
                    c.platform.as_str(),
                    "feishu" | "dingtalk" | "wechatwork" | "linkedin" | "facebook" | "x"
                )
            })
        {
            let vk = connector_scope::oauth_vault_platform_key(&c.owner_username, &c.platform);
            match vault.get_token(&vk, "user").await {
                Ok(_) => {
                    tracing::debug!("Token OK for {} / {}", c.owner_username, c.platform);
                }
                Err(omini_connect_oauth_vault::OAuthError::TokenNotFound(_)) => {}
                Err(e) => {
                    tracing::warn!(
                        "Token check failed for {} / {}: {}",
                        c.owner_username,
                        c.platform,
                        e
                    );
                }
            }
        }
    }
}

fn load_dotenv() {
    let repo_root_env = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.env");
    if repo_root_env.is_file() {
        let _ = dotenvy::from_path(&repo_root_env);
    }
    let _ = dotenvy::dotenv();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create DB pool and run migrations
    let pool = db::create_pool().await?;
    db::run_migrations(&pool).await?;

                // Load tool registry from YAML files (CARGO_MANIFEST_DIR = portal/ dir)
    let tools_dir =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/registry");
    let tool_registry = match tools::ToolRegistry::load_from_dir(&tools_dir) {
        Ok(reg) => {
            tracing::info!(
                "Loaded {} toolkits from {}",
                reg.toolkits().len(),
                tools_dir.display()
            );
            reg
        }
        Err(e) => {
            tracing::warn!(
                "Failed to load tool registry from {}: {}",
                tools_dir.display(),
                e
            );
            tracing::warn!("Starting without tool registry");
            tools::ToolRegistry::empty()
        }
    };

    let app_state = Arc::new(AppState::new(pool, tool_registry).await);

    // Seed default admin user
    db::seed_admin_user(app_state.users.as_ref()).await;

    // Register platform handlers
    register_platforms(&app_state).await;

    // Spawn background token refresh task
    let state_for_refresh = Arc::clone(&app_state);
    tokio::spawn(async move {
        token_refresh_loop(state_for_refresh).await;
    });

    // Axum rejects `/prefix/{*rest}` (catch-all not at end of pattern); nest + `fallback` like `/__omini/nango-hq`.
    let nango_hq_proxy =
        Router::new().fallback(axum::routing::any(api::nango_hq_proxy::proxy_nango_hq_all));
    let nango_integrations_proxy = Router::new().fallback(axum::routing::any(
        api::nango_hq_proxy::proxy_nango_integrations_nested,
    ));
    let nango_providers_proxy = Router::new().fallback(axum::routing::any(
        api::nango_hq_proxy::proxy_nango_providers_nested,
    ));
    let nango_oauth_connect_proxy = Router::new().fallback(axum::routing::any(
        api::nango_hq_proxy::proxy_nango_oauth_connect_nested,
    ));

    let app = Router::new()
        .route("/", get(serve_spa))
        // Connect UI overwrites URL pathname — must proxy Nango API paths at portal origin.
        .route(
            "/connect/session",
            any(api::nango_hq_proxy::proxy_nango_connect_public),
        )
        .route(
            "/connect/telemetry",
            any(api::nango_hq_proxy::proxy_nango_connect_public),
        )
        .nest("/integrations", nango_integrations_proxy)
        .nest("/providers", nango_providers_proxy)
        // OAuth popup + callback must reach Nango when `apiURL` is the portal origin.
        .nest("/oauth/connect", nango_oauth_connect_proxy)
        .route(
            "/oauth/callback",
            any(api::nango_hq_proxy::proxy_nango_oauth_callback),
        )
        .nest("/__omini/nango-hq", nango_hq_proxy)
        .nest("/auth", auth::router())
        .nest("/oauth", oauth::router())
        .nest("/api", api::router())
        .route_service("/assets/*path", ServeDir::new(spa_dist_dir()))
        .route_service("/images/*path", ServeDir::new(spa_dist_dir()))
        .merge(websocket::websocket_routes())
        .fallback(serve_spa)
        .with_state(app_state);

    let port = portal_listen_port();
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Portal listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

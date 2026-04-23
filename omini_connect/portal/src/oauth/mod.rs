//! OAuth: initiate flows and handle callbacks.

pub mod handlers;
pub mod models;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:platform/nango-finalize", get(handlers::nango_finalize))
        .route("/:platform/callback", get(handlers::oauth_callback))
        .route("/:platform", get(handlers::oauth_init))
}

//! OAuth: initiate flows and handle callbacks.

pub mod handlers;
pub mod models;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:platform", get(handlers::oauth_init))
        .route("/:platform/callback", get(handlers::oauth_callback))
}

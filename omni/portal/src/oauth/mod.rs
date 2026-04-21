//! OAuth: initiate flows and handle callbacks.

pub mod handlers;
pub mod models;

use std::sync::Arc;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{platform}", axum::routing::get(handlers::oauth_init))
        .route("/{platform}/callback", axum::routing::get(handlers::oauth_callback))
}

//! Authentication: login, logout, API key management.

pub mod handlers;
pub mod middleware;
pub mod models;

use std::sync::Arc;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", axum::routing::post(handlers::login))
        .route("/logout", axum::routing::post(handlers::logout))
        .route("/apikey", axum::routing::post(handlers::generate_api_key))
        .route("/me", axum::routing::get(handlers::me))
}

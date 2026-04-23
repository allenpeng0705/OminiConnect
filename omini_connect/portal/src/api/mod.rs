//! REST API routes.

pub mod connectors;
pub mod nango_catalog;
pub mod proxy;
pub mod status;

use std::sync::Arc;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", axum::routing::get(status::health))
        .route("/status/nango", axum::routing::get(status::nango))
        .route("/connectors", axum::routing::get(connectors::list))
        .route("/connectors", axum::routing::post(connectors::upsert))
        .route("/connectors/:platform", axum::routing::delete(connectors::delete))
        .route("/connectors/:platform/status", axum::routing::get(connectors::status))
        .route("/connectors/:platform/test", axum::routing::post(connectors::test))
        .route("/nango/integrations", axum::routing::get(nango_catalog::list_integrations))
        .route("/nango/providers", axum::routing::get(nango_catalog::list_providers))
        .route("/proxy/:platform/*path", axum::routing::get(proxy::forward).post(proxy::forward))
}

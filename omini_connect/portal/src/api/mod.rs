//! REST API routes.

pub mod agents;
pub mod connectors;
pub mod nango_connection;
pub mod nango_hq_proxy;
pub mod nango_catalog;
pub mod proxy;
pub mod status;
pub mod tools;
pub mod mcp;

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
        .route("/nango/connect-session", axum::routing::post(nango_catalog::post_connect_session))
        .route("/nango/connections", axum::routing::post(nango_catalog::post_nango_connection))
        .route("/nango/connections", axum::routing::get(nango_catalog::list_nango_connections))
        .route("/ominiconnect/proxy/:platform/*path", axum::routing::get(proxy::forward).post(proxy::forward))
        .merge(agents::router())
        .merge(tools::router())
        .merge(mcp::router())
}

//! Nango integration + provider catalog for the portal UI.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::middleware::try_auth;

#[derive(Debug, Deserialize, Default)]
pub struct NangoProvidersQuery {
    /// Forwarded to Nango `GET /providers?search=` (provider id filter).
    pub search: Option<String>,
}

/// GET /api/nango/integrations — list Nango environment integrations (requires login).
pub async fn list_integrations(State(state): State<Arc<AppState>>, headers: HeaderMap) -> impl IntoResponse {
    if try_auth(&state, &headers).await.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (StatusCode::OK, Json(Vec::<crate::nango::NangoIntegrationCatalogItem>::new())).into_response();
    };

    match crate::nango::list_integrations_catalog(&base, &secret).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            tracing::error!("Nango catalog: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

/// GET /api/nango/providers — full Nango provider catalog (same set Nango exposes in `providers.yaml`).
pub async fn list_providers(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(q): Query<NangoProvidersQuery>,
) -> impl IntoResponse {
    if try_auth(&state, &headers).await.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response();
    }

    let Some((base, secret)) = crate::nango::nango_credentials() else {
        return (StatusCode::OK, Json(Vec::<serde_json::Value>::new())).into_response();
    };

    let search = q.search.as_deref();
    match crate::nango::list_providers_catalog(&base, &secret, search).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            tracing::error!("Nango providers: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("{e}") })),
            )
                .into_response()
        }
    }
}

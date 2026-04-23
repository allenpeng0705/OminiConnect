//! Health / status API.

use axum::Json;

pub async fn health() -> impl axum::response::IntoResponse {
    Json(serde_json::json!({
        "service": "omini-connect-portal",
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn nango() -> impl axum::response::IntoResponse {
    let base = std::env::var("NANGO_BASE_URL").unwrap_or_default();
    let secret = std::env::var("NANGO_SECRET_KEY").unwrap_or_default();

    Json(serde_json::json!({
        "configured": !base.trim().is_empty(),
        "base_url": base,
        "has_secret_key": !secret.trim().is_empty(),
    }))
}

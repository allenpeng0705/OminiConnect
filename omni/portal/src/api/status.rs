//! Health / status API.

use axum::Json;

pub async fn health() -> impl axum::response::IntoResponse {
    Json(serde_json::json!({
        "service": "omni-portal",
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

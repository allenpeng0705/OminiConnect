//! Public portal URL from environment (OAuth callbacks, API hints).

/// `PORTAL_BASE_URL` with surrounding whitespace and trailing slashes stripped.
pub fn portal_base_url() -> String {
    std::env::var("PORTAL_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string())
        .trim()
        .trim_end_matches('/')
        .to_string()
}

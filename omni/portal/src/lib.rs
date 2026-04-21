//! OmniConnect Portal — library crate
//!
//! Re-exports all public types.

pub mod api;
pub mod app;
pub mod auth;
pub mod db;
pub mod oauth;

pub use app::AppState;

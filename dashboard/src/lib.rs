//! # OminiConnect Monitoring Dashboard
//!
//! Web-based dashboard for monitoring OminiConnect compliance and audit logs.
//!
//! ## Running
//!
//! ```bash
//! cargo run -p omini-connect-dashboard
//! ```
//!
//! Then open http://localhost:8081 in your browser.

mod api;
mod model;

pub use api::*;
pub use model::*;

//! # OmniConnect Monitoring Dashboard
//!
//! Web-based dashboard for monitoring OmniConnect compliance and audit logs.
//!
//! ## Running
//!
//! ```bash
//! cargo run -p omni-dashboard
//! ```
//!
//! Then open http://localhost:8081 in your browser.

mod api;
mod model;

pub use api::*;
pub use model::*;

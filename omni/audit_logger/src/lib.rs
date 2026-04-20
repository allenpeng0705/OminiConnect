//! # OmniConnect Audit Logger
//!
//! Compliance audit logging for PIPL requirements.
//!
//! Tracks:
//! - Policy violations (content moderation, data residency)
//! - Tool invocations
//! - Token access events
//!
//! ## Export Formats
//!
//! - JSON Lines (default)
//! - CSV
//!
//! ## Building
//!
//! ```bash
//! cargo build -p omni-audit_logger
//! ```

mod models;
mod storage;
mod export;

pub use models::*;
pub use storage::*;
pub use export::*;

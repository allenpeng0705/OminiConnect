//! # OminiConnect Audit Logger
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
//! cargo build -p omini-connect-audit-logger
//! ```

mod export;
mod models;
mod storage;

pub use export::*;
pub use models::*;
pub use storage::*;

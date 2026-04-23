//! # OminiConnect Compliance Wasm Policies
//!
//! Combined Wasm policy for Chinese enterprise compliance requirements.
//!
//! This module combines PII scrubbing, keyword filtering, and data residency checks
//! into a single Wasm plugin for Panda's plugin system.
//!
//! ## Standalone Wasm Modules
//!
//! The following are standalone Wasm modules that can be built separately:
//! - `content_moderation.rs` - Content moderation for adult/hate/violence content
//! - `data_residency.rs` - Data residency checker for PIPL compliance
//!
//! ## Building
//!
//! ```bash
//! # Build for host (testing)
//! cargo build -p omini-connect-wasm-policies
//!
//! # Build for Wasm (production)
//! cargo build -p omini-connect-wasm-policies --target wasm32-unknown-unknown --release
//!
//! # Copy to plugins directory
//! cp target/wasm32-unknown-unknown/release/libomini_connect_wasm_policies.wasm plugins/omini_connect_policy.wasm
//! ```

mod combined_policy;

pub use combined_policy::*;

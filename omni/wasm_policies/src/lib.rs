//! # OmniConnect Compliance Wasm Policies
//!
//! Wasm-based compliance policies for Chinese enterprise requirements.
//!
//! ## Policies
//!
//! - **pii_scrubber**: PII detection and redaction for Chinese IDs, phone numbers, emails
//! - **keyword_filter**: Sensitive keyword blocking for outgoing messages
//! - **data_residency**: Data residency checks before API calls
//!
//! ## Building
//!
//! ```bash
//! # Build individual policy
//! cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release
//!
//! # Copy to plugins directory
//! cp target/wasm32-unknown-unknown/release/omni_policy_pii.wasm plugins/
//! ```

pub mod pii_scrubber;
pub mod keyword_filter;
pub mod data_residency;

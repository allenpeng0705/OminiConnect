//! # OmniConnect Compliance Wasm Policies
//!
//! Combined Wasm policy for Chinese enterprise compliance requirements.
//!
//! This module combines PII scrubbing, keyword filtering, and data residency checks
//! into a single Wasm plugin for Panda's plugin system.
//!
//! ## Building
//!
//! ```bash
//! # Build for host (testing)
//! cargo build -p omni-wasm_policies
//!
//! # Build for Wasm (production)
//! cargo build -p omni-wasm_policies --target wasm32-unknown-unknown --release
//!
//! # Copy to plugins directory
//! cp target/wasm32-unknown-unknown/release/libomni_wasm_policies.wasm plugins/omni_policy.wasm
//! ```

mod combined_policy;

pub use combined_policy::*;

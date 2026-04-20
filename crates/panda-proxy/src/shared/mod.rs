//! **Shared** — identity context, budgets (TPM), compliance, console, per-route RPS, TLS,
//! and [`brain`] (HITL, rate-limit fallback, summarization) used from both inbound and outbound paths.
//!
//! See `docs/architecture_two_pillars.md`.

pub mod brain;
pub mod budget_hierarchy;
pub mod compliance_export;
pub mod console_oidc;
pub mod gateway;
pub mod jwks;
pub mod route_rps;
pub mod tls;
pub mod tpm;

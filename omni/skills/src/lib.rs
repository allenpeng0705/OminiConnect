//! # OmniConnect Skill Marketplace
//!
//! Pre-packaged MCP tool sets for enterprise use cases.
//!
//! ## Skills
//!
//! A "Skill" is a pre-defined bundle consisting of:
//! - One or more platform connectors
//! - Specific tools to expose
//! - System prompts
//! - Wasm compliance policies
//!
//! ## Built-in Skills
//!
//! - **Feishu Project Manager** - Calendar, tasks, approvals
//! - **WeChat Customer Service** - Customer management, messaging
//! - **DingTalk Workflow** - Approval workflows, notifications
//!
//! ## Using Skills
//!
//! ```rust,ignore
//! use omni_skills::{Marketplace, SkillRegistry};
//!
//! // List available skills in marketplace
//! let marketplace = Marketplace::new();
//! let skills = marketplace.list_skills();
//!
//! // Install a skill to local registry
//! let mut registry = SkillRegistry::new();
//! registry.add(omni_skills::SkillEntry {
//!     id: "feishu_project_manager".to_string(),
//!     path: "/path/to/skill".to_string(),
//!     enabled: true,
//!     config: serde_json::json!({}),
//! });
//! ```

mod marketplace;
mod registry;

pub use marketplace::Marketplace;
pub use registry::{SkillEntry, SkillRegistry};

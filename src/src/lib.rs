//! # OminiConnect Developer SDK
//!
//! Rust SDK for building applications on OminiConnect.
//!
//! ## Features
//!
//! - **Client** - Connect to OminiConnect MCP servers
//! - **Skill** - Define and package skills for reuse
//! - **Config** - Configuration helpers
//!
//! ## Example
//!
//! ```rust,ignore
//! use omini_connect_sdk::{Client, Config};
//!
//! // Load configuration from file
//! let config = Config::from_file("omini_connect.yaml")?;
//! let client = Client::connect(config).await?;
//! // Use client to call tools...
//! ```
//!
//! ## Building Skills
//!
//! A skill packages connectors, tools, and policies together:
//!
//! ```rust
//! use omini_connect_sdk::skill::Skill;
//!
//! let skill = Skill::builder("feishu_project_manager")
//!     .description("Project management with Feishu")
//!     .add_connector("feishu")
//!         .with_tool("calendar_list")
//!         .with_tool("calendar_create")
//!         .done()
//!     .add_policy("content_moderation")
//!     .build()
//!     .unwrap();
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod skill;

pub use client::Client;
pub use config::Config;
pub use error::SdkError;
pub use skill::Skill;

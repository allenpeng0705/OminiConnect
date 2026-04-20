//! # OmniConnect DingTalk Connector
//!
//! MCP connector for DingTalk (Alibaba) API.
//! Provides tools for Workflow, Messaging, and other DingTalk APIs.
//!
//! ## Running standalone
//!
//! ```bash
//! cargo run -p omni-connector-dingtalk -- serve --port 8091
//! ```
//!
//! Then configure Panda in `panda.yaml`:
//! ```yaml
//! mcp:
//!   servers:
//!     - name: dingtalk
//!       remote_mcp_url: http://localhost:8091/mcp
//! ```

use std::sync::Arc;

pub mod api;
pub mod server;
pub mod tools;

pub use api::{DingTalkApiClient, TokenVaultAccess};
pub use server::{DingTalkMcpServer, JsonRpcRequest, JsonRpcResponse};
pub use tools::DingTalkTool;

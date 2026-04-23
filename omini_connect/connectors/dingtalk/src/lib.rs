//! # OminiConnect DingTalk Connector
//!
//! MCP connector for DingTalk (Alibaba) API.
//! Provides tools for Workflow, Messaging, and other DingTalk APIs.
//!
//! ## Running standalone
//!
//! ```bash
//! cargo run -p omini-connect-dingtalk -- serve --port 8091
//! ```
//!
//! Then configure Panda in `panda.yaml`:
//! ```yaml
//! mcp:
//!   servers:
//!     - name: dingtalk
//!       remote_mcp_url: http://localhost:8091/mcp
//! ```

pub mod api;
pub mod docs;
pub mod server;
pub mod tools;

pub use api::DingTalkApiClient;
pub use docs::{generate_markdown, generate_summary};
pub use server::{DingTalkMcpServer, JsonRpcRequest, JsonRpcResponse, TokenVaultAccess};
pub use tools::DingTalkTool;

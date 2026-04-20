//! # OmniConnect Feishu Connector
//!
//! MCP connector for Feishu (Lark) API.
//! Provides tools for Calendar, Bitable, and Messaging.
//!
//! ## Usage with Panda MCP Runtime
//!
//! This connector can be used in two ways:
//!
//! 1. **As a standalone MCP server** - run this crate as a separate service
//!    and configure Panda to connect via `remote_mcp_url`.
//!
//! 2. **As a library** - import and register with Panda's McpRuntime directly.
//!
//! ## Running standalone
//!
//! ```bash
//! cargo run -p omni-connector-feishu -- serve --port 8090
//! ```
//!
//! Then configure Panda in `panda.yaml`:
//! ```yaml
//! mcp:
//!   servers:
//!     - name: feishu
//!       remote_mcp_url: http://localhost:8090/mcp
//!       remote_mcp_egress_profile: feishu
//! ```

pub mod api;
pub mod server;
pub mod tools;

pub use api::FeishuApiClient;
pub use server::{FeishuMcpServer, JsonRpcRequest, JsonRpcResponse, TokenVaultAccess};
pub use tools::FeishuTool;

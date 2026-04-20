//! # OmniConnect WeChat Work Connector
//!
//! MCP connector for WeChat Work (WeCom) API.
//! Provides tools for Customer Management, Messaging, and other WeChat Work APIs.
//!
//! ## Running standalone
//!
//! ```bash
//! cargo run -p omni-connector-wechatwork -- serve --port 8092
//! ```
//!
//! Then configure Panda in `panda.yaml`:
//! ```yaml
//! mcp:
//!   servers:
//!     - name: wechatwork
//!       remote_mcp_url: http://localhost:8092/mcp
//! ```

use std::sync::Arc;

pub mod api;
pub mod server;
pub mod tools;

pub use api::{WeChatWorkApiClient, TokenVaultAccess};
pub use server::{WeChatWorkMcpServer, JsonRpcRequest, JsonRpcResponse};
pub use tools::WeChatWorkTool;

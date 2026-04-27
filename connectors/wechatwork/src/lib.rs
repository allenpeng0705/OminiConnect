//! # OminiConnect WeChat Work Connector
//!
//! MCP connector for WeChat Work (WeCom) API.
//! Provides tools for Customer Management, Messaging, and other WeChat Work APIs.
//!
//! ## Running standalone
//!
//! ```bash
//! cargo run -p omini-connect-wechatwork -- serve --port 8092
//! ```
//!
//! Then configure Panda in `panda.yaml`:
//! ```yaml
//! mcp:
//!   servers:
//!     - name: wechatwork
//!       remote_mcp_url: http://localhost:8092/mcp
//! ```

pub mod api;
pub mod docs;
pub mod server;
pub mod tools;

pub use api::WeChatWorkApiClient;
pub use docs::{generate_markdown, generate_summary};
pub use server::{JsonRpcRequest, JsonRpcResponse, TokenVaultAccess, WeChatWorkMcpServer};
pub use tools::WeChatWorkTool;

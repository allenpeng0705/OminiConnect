//! **Inbound pillar — MCP gateway (with Panda API gateway all-in-one)**
//!
//! **Panda API gateway** handles **ingress** (in front of MCP) and/or **egress** (behind MCP, toward corporate L7);
//! this crate’s **inbound** modules implement the **MCP** side—tool servers,
//! OpenAI-style tool lists, stdio, **remote MCP over HTTP** (`mcp_http_remote`), **ingress MCP HTTP** (`mcp_http_ingress`),
//! declarative REST tools (`mcp_http_tool`), and **tool discovery, execution, and multi-round** model↔tool flows.
//!
//! See `docs/architecture_two_pillars.md` and **`docs/mcp_gateway_phase1.md`** (Phase 1 scope: minimal MCP vs advanced).

pub mod mcp;
pub mod mcp_http_ingress;
pub mod mcp_http_remote;
pub mod mcp_http_tool;
pub mod mcp_openai;
pub mod mcp_stdio;
pub mod mcp_streamable_http;

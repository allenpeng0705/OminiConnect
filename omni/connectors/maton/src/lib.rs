//! Maton.ai connector library

pub mod api;
pub mod server;
pub mod tools;

pub use api::MatonClient;
pub use server::MatonMcpServer;
pub use tools::all_tools;

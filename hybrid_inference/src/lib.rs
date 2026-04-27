//! # OminiConnect Hybrid Inference
//!
//! Route sensitive AI requests to local LLM for PIPL compliance.
//!
//! ## Features
//!
//! - **Sensitivity Rules Engine** - Configurable routing rules
//! - **Local LLM Support** - Ollama/vLLM integration
//! - **Cloud Fallback** - Fallback to cloud when local unavailable
//!
//! ## Example
//!
//! ```rust,ignore
//! use omini_connect_hybrid_inference::{Config, Router, RequestContext};
//!
//! let config = Config::from_yaml(yaml_str)?;
//! let router = Router::new(config).await?;
//!
//! let ctx = RequestContext {
//!     content: Some("查询员工工资".to_string()),
//!     tools: vec!["hr_get_salary".to_string()],
//!     ..Default::default()
//! };
//!
//! let response = router.route(ctx, messages).await?;
//! ```

pub mod config;
pub mod local_llm;
pub mod rules;
pub mod router;

pub use config::HybridConfig;
pub use local_llm::{ChatMessage, LocalLlmClient};
pub use router::{ResponseContent, RouteTarget, Router, RouterResponse, RouterError};
pub use rules::{PiiType, RequestContext, RoutingDecision, RulesEngine, RuleResult};

/// Maximum sensitivity score
pub const MAX_SENSITIVITY_SCORE: u8 = 100;

/// Default Wasm sensitivity threshold for local routing
pub const DEFAULT_SENSITIVITY_THRESHOLD: u8 = 70;

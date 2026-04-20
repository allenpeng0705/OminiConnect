//! **Outbound pillar — AI gateway**
//!
//! OpenAI-shaped client traffic to upstream LLMs: HTTP client join, SSE, provider adapters
//! (Anthropic native + OpenAI passthrough; more labels in `panda-config::OPENAI_SHAPED_ADAPTER_PROVIDER_LABELS`),
//! semantic cache, semantic upstream routing, and model failover chains.
//!
//! See `docs/architecture_two_pillars.md`, `docs/provider_adapters.md`.

pub mod adapter;
pub mod adapter_stream;
pub mod model_failover;
pub mod semantic_cache;
pub mod semantic_routing;
pub mod sse;
pub mod upstream;

# OmniConnect (Project-X)
### The Universal Execution Layer for AI Agents in China

OmniConnect is a high-performance AI Gateway and Tool Orchestrator designed to bridge the gap between LLMs (Large Language Models) and the Chinese software ecosystem. Inspired by the "Execution Layer" philosophy, it provides a unified interface for agents to interact with local platforms like Feishu, DingTalk, and WeChat Work via MCP (Model Context Protocol).

## 🚀 Vision
To become the "central nervous system" for AI agents, providing secure, authenticated, and standardized "hands" to manipulate existing APIs without the complexity of manual OAuth management.

## 🛠 Core Features
- **Unified Auth Gateway:** Centralized OAuth2/Token management for major Chinese SaaS platforms.
- **MCP-Native:** Built from the ground up to support the Model Context Protocol for seamless tool discovery.
- **Rust-Based Core:** High-concurrency, memory-safe ingress/egress gateway (leveraging experience from Panda architecture).
- **Privacy-First:** Support for local-first deployment and PII (Personally Identifiable Information) filtering.
- **Hybrid Routing:** Intelligently route tasks between local inference (llama.cpp) and cloud APIs.

## 🏗 Tech Stack
- **Gateway:** Rust (Axum/Tokio)
- **Plugin System:** Python/TypeScript (MCP Servers)
- **Database:** PostgreSQL (for token storage) + Redis (for caching)
- **Deployment:** Docker / K8s

## 📂 Project Structure
- `/gateway`: The Rust-based proxy and auth manager.
- `/mcp-servers`: Standardized connectors for Feishu, DingTalk, etc.
- `/dashboard`: Management UI for API keys and tool monitoring.

# Project Roadmap: OmniConnect

## Phase 1: Foundation (Weeks 1-4)
- [ ] **Core Gateway Architecture:** Implement the basic Rust-based proxy layer.
- [ ] **Auth Engine:** Build the centralized OAuth2 handler for Feishu (Lark).
- [ ] **MCP Integration:** Setup the base MCP server structure to register tools.
- [ ] **Basic Security:** Implement API key management for agent access.

## Phase 2: Tool Expansion (Weeks 5-8)
- [ ] **Enterprise Connectors:**
    - [ ] Feishu (Bitable, Calendar, Messaging)
    - [ ] DingTalk (Workflow, Messaging)
    - [ ] WeChat Work (Customer Management)
- [ ] **Schema Registry:** A discovery service for LLMs to understand available tool capabilities.
- [ ] **Documentation:** Generate OpenAPI/MCP specs automatically.

## Phase 3: Advanced Features & Compliance (Weeks 9-12)
- [ ] **Data Localization:** Ensure all token/log storage complies with PIPL regulations.
- [ ] **Content Moderation:** Integrated filtering for upstream/downstream AI content.
- [ ] **Hybrid Inference:** Integration with local LLM providers for sensitive tasks.

## Phase 4: Scaling & Ecosystem (Ongoing)
- [ ] **Developer SDK:** Simplify connecting new agents to OmniConnect.
- [ ] **Skill Marketplace:** Allow community-contributed MCP servers.
- [ ] **Monitoring:** Real-time observability dashboard for API usage and costs.

# OminiConnect Roadmap

## ✅ Completed

### Phase 1: Foundation
- [x] **Core Gateway Architecture**: Axum-based proxy server
- [x] **Auth Engine**: OAuth2 handler for Feishu, DingTalk, WeChat Work, LinkedIn, Facebook
- [x] **API Key Management**: Web UI to create, list, delete API keys
- [x] **Portal UI**: Dashboard, connector config, API key management pages

### Phase 2: Proxy & Token Management
- [x] **Token Persistence**: SQLite/MySQL/PostgreSQL storage via sqlx
- [x] **Auto Token Refresh**: Background task refreshes tokens before expiry
- [x] **Maton-style Proxy**: `/api/proxy/{platform}/{path}` passthrough endpoint
- [x] **OAuth Token Injection**: Proxy automatically injects stored tokens

## 📋 In Progress

- [ ] **LinkedIn Image Upload**: Multi-step asset upload flow
- [ ] **MCP Connectors**: Full MCP server implementations for each platform
- [ ] **Schema Registry**: Discovery service for LLM tool understanding

## 📋 TODO

### Phase 2 (Continued)
- [ ] **Facebook Posting**: Test and document posting API
- [ ] **Feishu Fix**: Custom App support (Enterprise Internal Apps don't support user OAuth)

### Phase 3: Advanced Features
- [ ] **Content Moderation**: Integrated filtering for upstream/downstream AI content
- [ ] **PII Filtering**: Data localization compliance
- [ ] **Hybrid Inference**: Integration with local LLM providers

### Phase 4: Ecosystem
- [ ] **Developer SDK**: Simplify connecting new agents to OminiConnect
- [ ] **Monitoring Dashboard**: Real-time observability for API usage and costs
- [ ] **Skill Marketplace**: Community-contributed MCP servers

## Known Issues

- **Feishu Enterprise Internal Apps**: User OAuth endpoints return 404 - use Custom App type instead
- **LinkedIn Geographic Restriction**: API calls may fail based on server IP location
- **Image Upload**: Requires `w_asset_upload` scope and multi-step binary upload flow

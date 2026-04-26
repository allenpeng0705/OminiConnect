# Tool Registry Tracker

## Coverage Notation

`X/Y covered` means **Y tools implemented** out of **X total in our allowlist**. We don't aim to cover 100% of every API — only the most commonly used ones that make sense for AI agents. Higher priority = more complete coverage.

## Coverage Summary

**886 providers** | **8,883 tools** total

All 886 YAML registry files parse correctly (`python3 -c "import yaml; yaml.safe_load(open('...'))"`). All tools have corresponding documentation in `tools/docs/`.

Average: **10.0 tools per provider**

---

## Provider List

The full alphabetical provider list (886 entries) is available via:

```bash
ls tools/registry/*.yaml | wc -l   # → 886
ls tools/registry/*.yaml | xargs -n1 basename | sed 's/\.yaml$//' | sort
```

Key auth-mode variants included:
- OAuth2 variants (`-oauth`, `-oauth2-cc`) for 100+ providers
- API_KEY variants (`-api-key`, `-pat`) for 80+ providers
- BASIC variants (`-basic`) for 30+ providers
- SCIM variants (`-scim`) for 20+ providers
- Sandbox/staging variants (`-sandbox`, `-staging`) for 40+ providers

---

## Expansion Phases

All phases complete. Total coverage: **886 providers** with **8,883 tools**.

---

## 3rd Party MCP Support

> **Note**: This section is for registering external MCP servers. OminiConnect's own tool registry uses internal proxy routing. 3rd-party MCP support enables LLM agents to call tools from user-registered MCP servers.

| MCP Server | Provider | Status | Config | Notes |
|-----------|-----------|--------|--------|-------|
| (none yet) | - | - | - | - |

**How it would work**:
1. User registers an MCP server (URL + auth token)
2. OminiConnect fetches available tools via `tools/list`
3. MCP tools appear alongside OminiConnect tools for connected platforms
4. `tools/call` routes to the external MCP server

---

## Dynamic Routing Rule

**Only connected services work.** The tool list is dynamically filtered per user — `GET /api/tools` and `MCP tools/list` only return tools for platforms where the user has an enabled connector. An LLM cannot call a GitHub tool unless the user has connected GitHub.

---

## Adding New Providers

1. Create `tools/registry/{provider}.yaml` with tool allowlist
2. Create `tools/docs/{provider}.md` with documentation
3. Update this file with coverage status
4. Verify YAML parses: `python3 -c "import yaml; yaml.safe_load(open('tools/registry/{provider}.yaml'))"`

Tools are defined as allowlists — only listed tools are exposed to LLMs. Each tool specifies:
- `slug`: Unique identifier (e.g., `github_list_repos`)
- `name`: Human-readable name
- `description`: What the tool does
- `provider`: Nango provider key
- `endpoint`: API path with `{path_params}`
- `method`: HTTP method
- `scopes`: Required OAuth scopes
- `input_schema`: JSON Schema for tool arguments

# OminiConnect SDKs

Client libraries for the OminiConnect API. Use these instead of raw HTTP calls.

---

## Python SDK

**Requires:** Python 3.10+

```bash
pip install ominiconnect
```

```python
from ominiconnect import OminiConnect

client = OminiConnect(api_key="sk-...")

# Register an AI agent
agent = client.agents.register(
    name="github-writer",
    description="Writes GitHub PR descriptions"
)
print(agent["api_key"])  # shown once — store securely

# List agents
agents = client.agents.list()

# Browse available tools
tools = client.tools.list(platform="github")
for tool in tools["tools"]:
    print(tool["slug"], tool["scope_satisfied"])

# Execute a tool
result = client.tools.execute(
    "github_list_repos",
    arguments={"sort": "updated"}
)
print(result["body"])

# Async: result POSTed to your callback URL
result = client.tools.execute_async(
    "github_list_repos",
    arguments={},
    callback_url="https://myapp.com/webhook"
)
# Returns immediately with {"call_id": "..."}
```

### From source

```bash
cd sdk/python
pip install -e .
```

---

## JavaScript / TypeScript SDK

**Requires:** Node.js 16+ or any `fetch`-compatible runtime.

```bash
npm install @ominiconnect/sdk
```

```typescript
import { OminiConnectClient } from "@ominiconnect/sdk";

const client = new OminiConnectClient({ apiKey: "sk-..." });

// Register an AI agent
const agent = await client.agents.register("github-writer", {
  description: "Writes GitHub PR descriptions",
});
console.log(agent.api_key); // shown once

// List agents
const { agents } = await client.agents.list();

// Browse available tools
const { tools } = await client.tools.list("github");

// Execute a tool
const result = await client.tools.execute("github_list_repos", { sort: "updated" });

// Async with callback
const asyncResult = await client.tools.execute("github_list_repos", {}, {
  callbackUrl: "https://myapp.com/webhook",
});
```

### From source

```bash
cd sdk/js
npm install
npm run build
```

The SDK ships pre-compiled types — no extra build step needed for consumers.

---

## SDK vs raw HTTP

Both SDKs are thin wrappers around the REST API. The same functionality is available with `curl`:

| SDK method | HTTP |
|---|---|
| `client.agents.register(name, desc)` | `POST /api/agents` |
| `client.agents.list()` | `GET /api/agents` |
| `client.tools.list(platform)` | `GET /api/tools?platform=...` |
| `client.tools.search(q)` | `GET /api/tools/search?q=...` |
| `client.tools.execute(slug, args)` | `POST /api/tools/execute` |
| `client.tools.execute_async(...)` | `POST /api/tools/execute` with `callback_url` |

# MCP Generic Tools

Provider: `mcp-generic` | Engine: `nango` | Auth: MCP_OAUTH2_GENERIC via Nango

## Overview

These tools wrap MCP (Model Context Protocol) servers as generic OAuth2 APIs. They allow AI agents to interact with MCP tools, resources, and prompts. **Requires MCP Server OAuth2 configuration.**

## Authentication

**Nango MCP_OAUTH2_GENERIC**:
- User configures MCP server URL via Nango Connect
- OAuth2 flow handled by Nango
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.mcp_server_url}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mcp_generic_list_tools` | List available tools from MCP server | GET | /tools |
| `mcp_generic_call_tool` | Call a specific tool on MCP server | POST | /tools/{toolName} |
| `mcp_generic_list_resources` | List available resources | GET | /resources |
| `mcp_generic_read_resource` | Read a specific resource | GET | /resources/{uri} |
| `mcp_generic_list_prompts` | List available prompts | GET | /prompts |
| `mcp_generic_get_prompt` | Get a specific prompt | GET | /prompts/{promptName} |
| `mcp_generic_list_servers` | List connected MCP servers | GET | /servers |
| `mcp_generic_get_server` | Get MCP server info | GET | /servers/{serverId} |
| `mcp_generic_list_aliases` | List tool aliases | GET | /aliases |
| `mcp_generic_create_alias` | Create a tool alias | POST | /aliases |

---

## Tool Details

### mcp_generic_list_tools

**What it does**: Lists all available tools from the connected MCP server.

**When to use**: Discover available MCP tools, understand capabilities.

**Arguments**:
- `category` (optional): Filter by tool category

**Example LLM prompt**: "What tools are available on this MCP server"

---

### mcp_generic_call_tool

**What it does**: Calls a specific tool on the MCP server with arguments.

**When to use**: Execute MCP tool functionality.

**Arguments**:
- `toolName` (required): Name of the tool to call
- `arguments` (required): Tool arguments as key-value pairs

**Example LLM prompt**: "Call the github_create_issue tool with title 'Bug found' and body 'Description'"

---

### mcp_generic_list_resources

**What it does**: Lists all available resources on the MCP server.

**When to use**: Discover data sources, understand information available.

**Arguments**: None

**Example LLM prompt**: "What resources are available"

---

### mcp_generic_read_resource

**What it does**: Reads a specific resource from the MCP server.

**When to use**: Access data from MCP resources.

**Arguments**:
- `uri` (required): Resource URI

**Example LLM prompt**: "Read the resource at uri 'file://documents/readme.md'"

---

### mcp_generic_list_prompts

**What it does**: Lists all available prompts on the MCP server.

**When to use**: Discover reusable prompt templates.

**Arguments**: None

**Example LLM prompt**: "What prompts are available"

---

### mcp_generic_get_prompt

**What it does**: Gets a specific prompt with arguments.

**When to use**: Use prompt templates for common tasks.

**Arguments**:
- `promptName` (required): Name of the prompt
- `arguments` (optional): Prompt arguments

**Example LLM prompt**: "Get the code_review prompt with repository 'myrepo'"

---

### mcp_generic_list_servers

**What it does**: Lists all connected MCP servers.

**When to use**: Check available MCP server connections.

**Arguments**: None

**Example LLM prompt**: "List all connected MCP servers"

---

### mcp_generic_get_server

**What it does**: Gets information about a specific MCP server.

**When to use**: Check MCP server status and capabilities.

**Arguments**:
- `serverId` (required): Server ID

**Example LLM prompt**: "Get details for MCP server 12345"

---

### mcp_generic_list_aliases

**What it does**: Lists all tool aliases configured for the MCP server.

**When to use**: Find shortcut names for tools.

**Arguments**: None

**Example LLM prompt**: "List all tool aliases"

---

### mcp_generic_create_alias

**What it does**: Creates an alias for an MCP tool.

**When to use**: Create shortcuts for frequently used tools.

**Arguments**:
- `alias` (required): Alias name
- `tool_name` (required): Original tool name
- `description` (optional): Alias description

**Example LLM prompt**: "Create alias 'gh-issues' for tool 'github_list_issues'"

---

## MCP Generic Notes

- **MCP Server URL**: Required configuration for connection
- **Dynamic tools**: Tool set depends on connected MCP server
- **Resources**: Data sources available via MCP
- **Prompts**: Reusable prompt templates
- **Aliases**: Custom shortcuts for tools

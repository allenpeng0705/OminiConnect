# Slack (MCP) Tools

Provider: `slack-mcp` | Engine: `nango` | Auth: MCP_OAUTH2

## Overview

These tools wrap the Slack (MCP) API. They allow AI agents to interact with Slack (MCP) functionality. **Requires MCP_OAUTH2 authentication.**

## Authentication

**MCP OAuth2 Authentication**:
- Special OAuth2 flow for MCP (Model Context Protocol)
- User authenticates via MCP OAuth2
- Token managed by Nango for MCP server access

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_channels` | List all channels | GET | /conversations.list |
| `get_channel` | Get channel details | GET | /conversations.info |
| `list_messages` | List messages in channel | GET | /conversations.history |
| `post_message` | Post a message to channel | POST | /chat.postMessage |
| `upload_file` | Upload a file | POST | /files.upload |
| `list_users` | List workspace users | GET | /users.list |
| `get_user` | Get user details | GET | /users.info |
| `list_emoji` | List custom emoji | GET | /emoji.list |
| `get_team_info` | Get workspace info | GET | /team.info |
| `search_messages` | Search messages | GET | /search.messages |

---

## Tool Details

### list_channels

**What it does**: List all channels

**When to use**: Use this tool when you need to list all channels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list channels to..."

---

### get_channel

**What it does**: Get channel details

**When to use**: Use this tool when you need to get channel details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get channel to..."

---

### list_messages

**What it does**: List messages in channel

**When to use**: Use this tool when you need to list messages in channel.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list messages to..."

---

### post_message

**What it does**: Post a message to channel

**When to use**: Use this tool when you need to post a message to channel.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use post message to..."

---

### upload_file

**What it does**: Upload a file

**When to use**: Use this tool when you need to upload a file.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload file to..."

---

### list_users

**What it does**: List workspace users

**When to use**: Use this tool when you need to list workspace users.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list users to..."

---

### get_user

**What it does**: Get user details

**When to use**: Use this tool when you need to get user details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_emoji

**What it does**: List custom emoji

**When to use**: Use this tool when you need to list custom emoji.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list emoji to..."

---

### get_team_info

**What it does**: Get workspace info

**When to use**: Use this tool when you need to get workspace info.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get team info to..."

---

### search_messages

**What it does**: Search messages

**When to use**: Use this tool when you need to search messages.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use search messages to..."

---

## Slack (MCP) API Notes

- **Auth mode**: MCP_OAUTH2
- **Base URL**: https://mcp.slack.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits

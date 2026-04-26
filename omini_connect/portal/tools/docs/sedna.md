# Sedna (OAuth) Tools

Provider: `sedna` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the Sedna (OAuth) API. They allow AI agents to interact with Sedna (OAuth) functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_user` | Get current user profile | GET | /users/me |
| `list_messages` | List messages in workspace | GET | /messages |
| `send_message` | Send a message | POST | /messages |
| `list_channels` | List all channels | GET | /channels |
| `create_channel` | Create a new channel | POST | /channels |
| `get_channel` | Get channel details | GET | /channels/{id} |
| `list_members` | List workspace members | GET | /members |
| `list_files` | List files in workspace | GET | /files |
| `upload_file` | Upload a file | POST | /files |
| `get_workspace` | Get workspace details | GET | /workspace |

---

## Tool Details

### get_user

**What it does**: Get current user profile

**When to use**: Use this tool when you need to get current user profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_messages

**What it does**: List messages in workspace

**When to use**: Use this tool when you need to list messages in workspace.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list messages to..."

---

### send_message

**What it does**: Send a message

**When to use**: Use this tool when you need to send a message.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use send message to..."

---

### list_channels

**What it does**: List all channels

**When to use**: Use this tool when you need to list all channels.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list channels to..."

---

### create_channel

**What it does**: Create a new channel

**When to use**: Use this tool when you need to create a new channel.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create channel to..."

---

### get_channel

**What it does**: Get channel details

**When to use**: Use this tool when you need to get channel details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get channel to..."

---

### list_members

**What it does**: List workspace members

**When to use**: Use this tool when you need to list workspace members.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list members to..."

---

### list_files

**What it does**: List files in workspace

**When to use**: Use this tool when you need to list files in workspace.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list files to..."

---

### upload_file

**What it does**: Upload a file

**When to use**: Use this tool when you need to upload a file.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload file to..."

---

### get_workspace

**What it does**: Get workspace details

**When to use**: Use this tool when you need to get workspace details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get workspace to..."

---

## Sedna (OAuth) API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: https://{tenant}.sednanetwork.com/platform
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits

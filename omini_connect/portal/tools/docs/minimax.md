# MiniMax Tools

Provider: `minimax` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the MiniMax API. They allow AI agents to manage chats, AI models, roles, and permissions. **Requires MiniMax API key.**

## Authentication

**Nango API_KEY**:
- User provides MiniMax API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `authorization: Bearer ${apiKey}`, `group-id: ${connectionConfig.groupId}`
- Base URL: `https://api.minimaxi.chat`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `minimax_list_chats` | List all chats | GET | /v1/chat/conversations |
| `minimax_get_chat` | Get chat details | GET | /v1/chat/conversations/{conversationId} |
| `minimax_create_chat` | Create a new chat | POST | /v1/chat/conversations |
| `minimax_send_message` | Send a chat message | POST | /v1/chat/conversations/{conversationId}/messages |
| `minimax_list_models` | List available models | GET | /v1/models |
| `minimax_get_model` | Get model details | GET | /v1/models/{modelId} |
| `minimax_list_roles` | List roles | GET | /v1/roles |
| `minimax_create_role` | Create a role | POST | /v1/roles |
| `minimax_list_permissions` | List permissions | GET | /v1/permissions |
| `minimax_assign_permission` | Assign permission to role | POST | /v1/roles/{roleId}/permissions |

---

## Tool Details

### minimax_list_chats

**What it does**: Lists all conversations in MiniMax.

**When to use**: Browse chat history, find conversations.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all my chats in MiniMax"

---

### minimax_get_chat

**What it does**: Gets details of a specific chat conversation.

**When to use**: Get conversation info, message count.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "Get details for chat CON-12345"

---

### minimax_create_chat

**What it does**: Creates a new chat conversation.

**When to use**: Start new conversations with AI.

**Arguments**:
- `title` (required): Chat title

**Example LLM prompt**: "Create a new chat called 'Project Planning'"

---

### minimax_send_message

**What it does**: Sends a message in a chat conversation.

**When to use**: Interact with AI models.

**Arguments**:
- `conversationId` (required): Conversation ID
- `content` (required): Message content
- `role` (optional): Role (user/assistant) (default user)

**Example LLM prompt**: "Send message 'Hello AI' to chat CON-12345"

---

### minimax_list_models

**What it does**: Lists all available AI models in MiniMax.

**When to use**: Discover available models.

**Arguments**: None

**Example LLM prompt**: "What AI models are available"

---

### minimax_get_model

**What it does**: Gets details of a specific AI model.

**When to use**: Check model capabilities, context length.

**Arguments**:
- `modelId` (required): Model ID

**Example LLM prompt**: "Get details for model MODEL-12345"

---

### minimax_list_roles

**What it does**: Lists all roles in the organization.

**When to use**: Role management, permission overview.

**Arguments**: None

**Example LLM prompt**: "List all roles"

---

### minimax_create_role

**What it does**: Creates a new role in the organization.

**When to use**: Add custom roles for access control.

**Arguments**:
- `name` (required): Role name
- `description` (optional): Role description

**Example LLM prompt**: "Create a role called 'Data Analyst'"

---

### minimax_list_permissions

**What it does**: Lists all available permissions.

**When to use**: Understand available access controls.

**Arguments**: None

**Example LLM prompt**: "What permissions are available"

---

### minimax_assign_permission

**What it does**: Assigns a permission to a role.

**When to use**: Configure role access.

**Arguments**:
- `roleId` (required): Role ID
- `permission` (required): Permission identifier

**Example LLM prompt**: "Assign 'chat:read' permission to role 12345"

---

## MiniMax Notes

- **AI platform**: Chat and AI model access
- **Conversations**: Chat history with AI
- **Models**: Various AI models available
- **Roles**: Permission management
- **Permissions**: Granular access control

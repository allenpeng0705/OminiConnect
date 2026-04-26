# Front Tools

Provider: `front_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Front API. They allow AI agents to manage conversations, channels, teammates, templates, and automation rules in your Front workspace.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation`, `message`, `channel`, `teammate`, `template`, `rule`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `front_more_list_conversations` | List conversations with filtering options | GET | /conversations |
| `front_more_get_conversation` | Get details of a specific conversation | GET | /conversations/{conversation_id} |
| `front_more_create_message` | Send a message or reply to a conversation | POST | /messages |
| `front_more_list_channels` | List all channels/inboxes | GET | /channels |
| `front_more_get_channel` | Get details of a specific channel | GET | /channels/{channel_id} |
| `front_more_list_teammates` | List all teammates | GET | /teammates |
| `front_more_get_teammate` | Get details of a specific teammate | GET | /teammates/{teammate_id} |
| `front_more_list_templates` | List all message templates | GET | /templates |
| `front_more_get_template` | Get details of a specific template | GET | /templates/{template_id} |
| `front_more_list_rules` | List all automation rules | GET | /rules |

---

## Tool Details

### front_more_list_conversations

**What it does**: Returns a paginated list of conversations with optional filtering by status, inbox, or contact.

**When to use**: Browse open tickets, find conversations by customer, or monitor inbox activity.

**Arguments**:
- `q` (optional): Query object with `status`, `inbox_id`, `contact_id`
- `page_token` (optional): Pagination token
- `limit` (optional, max 100): default 25

**Example LLM prompt**: "Show me all open conversations in the support inbox"

---

### front_more_get_conversation

**What it does**: Gets full details of a specific conversation including all messages and metadata.

**When to use**: Read the full conversation history before responding to a customer.

**Arguments**:
- `conversation_id` (required): The conversation ID

**Example LLM prompt**: "Show me conversation conv_123"

---

### front_more_create_message

**What it does**: Sends a new message or reply to a conversation through a specific channel.

**When to use**: Respond to customer inquiries, send follow-ups, or add internal notes.

**Arguments**:
- `conversation_id` (required): Conversation to reply to
- `type` (optional): `reply`, `note`, or `direct` — default `reply`
- `body` (required): Message content (HTML or plain text)
- `author_id` (optional): Teammate sending the message
- `channel_id` (optional): Channel to send through
- `to` (optional): Array of recipients

**Example LLM prompt**: "Send a reply to conversation conv_123 saying we'll look into this"

---

### front_more_list_channels

**What it does**: Lists all channels (inboxes) configured in Front, like email, chat, or SMS.

**When to use**: See available inboxes before assigning or routing conversations.

**Arguments**:
- `page_token` (optional): Pagination token
- `limit` (optional): default 25

**Example LLM prompt**: "What channels are configured in our Front workspace?"

---

### front_more_get_channel

**What it does**: Gets detailed channel info including settings and routing configuration.

**When to use**: Understand how a channel is set up before using it for messaging.

**Arguments**:
- `channel_id` (required): The channel ID

**Example LLM prompt**: "Show me the settings for the support email channel"

---

### front_more_list_teammates

**What it does**: Lists all teammates in the workspace with their admin status and availability.

**When to use**: Find available agents, check team structure, or assign conversations.

**Arguments**:
- `page_token` (optional): Pagination token
- `limit` (optional): default 25

**Example LLM prompt**: "List all teammates in the team"

---

### front_more_get_teammate

**What it does**: Gets detailed teammate info including profile, role, and assignment rules.

**When to use**: Check a teammate's permissions or workload before assigning them a conversation.

**Arguments**:
- `teammate_id` (required): The teammate ID

**Example LLM prompt**: "Show me details for teammate jane@company.com"

---

### front_more_list_templates

**What it does**: Lists all message templates for consistent customer responses.

**When to use**: Find available templates before sending a standard response.

**Arguments**:
- `type` (optional): `text`, `html`, or `email`
- `page_token` (optional): Pagination token
- `limit` (optional): default 25

**Example LLM prompt**: "What email templates do we have?"

---

### front_more_get_template

**What it does**: Gets the full template content including variables that can be filled in.

**When to use**: Read a template before using it or to understand what variables it expects.

**Arguments**:
- `template_id` (required): The template ID

**Example LLM prompt**: "Show me the refund policy template"

---

### front_more_list_rules

**What it does**: Lists all automation rules that handle routing, tagging, or other actions.

**When to use**: See what automatic processes are running before making manual changes.

**Arguments**:
- `page_token` (optional): Pagination token
- `limit` (optional): default 25

**Example LLM prompt**: "What automation rules are active?"

---

## Front API Reference

These tools use the Front API. See official docs for full details:
- https://dev.frontapp.com/
- Rate limits: 500 requests/minute for most plans
- Pagination: Use `page_token` from response
- All IDs follow the format: `type_id` (e.g., `conversations` prefix)

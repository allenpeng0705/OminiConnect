# Crisp Plugin Install Tools

Provider: `crisp-plugin-install` | Engine: `nango` | Auth: Plugin Install via Nango

## Overview

These tools wrap the Crisp Plugin API. They allow AI agents to manage conversations, messages, operators, websites, and contacts. Crisp is a customer messaging platform for sales and support teams.

## Authentication

**Nango Plugin Install**:
- Uses plugin install flow with BASIC auth
- Token stored in Nango, accessed via `connection_ref`
- Website ID extracted from redirect URI metadata

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `crisp_plugin_list_conversations` | List conversations | GET | /v1/plugin/connect/account/conversations |
| `crisp_plugin_get_conversation` | Get conversation details | GET | /v1/plugin/connect/account/conversations/{conversation_id} |
| `crisp_plugin_list_messages` | List messages in conversation | GET | /v1/plugin/connect/account/conversations/{conversation_id}/messages |
| `crisp_plugin_send_message` | Send a message | POST | /v1/plugin/connect/account/conversations/{conversation_id}/messages |
| `crisp_plugin_list_operators` | List operators | GET | /v1/plugin/connect/account/operators |
| `crisp_plugin_get_operator` | Get operator details | GET | /v1/plugin/connect/account/operators/{operator_id} |
| `crisp_plugin_list_websites` | List websites | GET | /v1/plugin/connect/account/websites |
| `crisp_plugin_get_website` | Get website details | GET | /v1/plugin/connect/account/websites/{website_id} |
| `crisp_plugin_list_contacts` | List contacts | GET | /v1/plugin/connect/account/contacts |
| `crisp_plugin_get_contact` | Get contact details | GET | /v1/plugin/connect/account/contacts/{contact_id} |

---

## Tool Details

### crisp_plugin_list_conversations

**What it does**: Lists all conversations with optional status filtering.

**When to use**: Find open support conversations, review pending messages, track conversation volume.

**Arguments**:
- `status` (optional): Filter by open, closed, pending (default: open)
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all open conversations"

---

### crisp_plugin_get_conversation

**What it does**: Gets detailed conversation information including participants and metadata.

**When to use**: Review full conversation context, check assignee and priority.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation abc123"

---

### crisp_plugin_list_messages

**What it does**: Lists all messages in a specific conversation.

**When to use**: Review conversation history, find specific messages, analyze customer issues.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all messages in conversation abc123"

---

### crisp_plugin_send_message

**What it does**: Sends a message in an existing conversation.

**When to use**: Respond to customer inquiries, send proactive messages, automate replies.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `content` (required): Message content
- `type` (optional): Message type - text or email (default: text)

**Example LLM prompt**: "Send a message 'Thank you for reaching out!' in conversation abc123"

---

### crisp_plugin_list_operators

**What it does**: Lists all operators (agents) in the workspace.

**When to use**: View team members, find available agents, check operator roles.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all operators in the workspace"

---

### crisp_plugin_get_operator

**What it does**: Gets detailed operator information including role and status.

**When to use**: Check operator availability, verify permissions, view operator settings.

**Arguments**:
- `operator_id` (required): Operator ID

**Example LLM prompt**: "Get details for operator op-456"

---

### crisp_plugin_list_websites

**What it does**: Lists all websites connected to the Crisp workspace.

**When to use**: View integrated websites, check website settings, verify plugin status.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all websites in Crisp"

---

### crisp_plugin_get_website

**What it does**: Gets detailed website configuration and integration status.

**When to use**: Check website settings, verify Crisp plugin installation, view website metrics.

**Arguments**:
- `website_id` (required): Website ID

**Example LLM prompt**: "Get details for website w-789"

---

### crisp_plugin_list_contacts

**What it does**: Lists all contacts with optional segment filtering.

**When to use**: Browse customer database, filter by segment, find specific customers.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `segment` (optional): Filter by segment name

**Example LLM prompt**: "List all contacts in the premium segment"

---

### crisp_plugin_get_contact

**What it does**: Gets detailed contact information including profile, segments, and activity.

**When to use**: Review customer profile, check interaction history, update contact data.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact c-111"

---

## Crisp Plugin API Notes

- **Conversations**: Customer support conversations, can be open/closed/pending
- **Messages**: Individual messages within conversations
- **Operators**: Support agents/admins in the workspace
- **Websites**: Customer websites with Crisp chat widget installed
- **Contacts**: Customer profiles with email, phone, and custom data
- **Plugin Install**: Uses website plugin install flow, not user OAuth

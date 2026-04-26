# Crisp Tools

Provider: `crisp` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Crisp REST API. They allow AI agents to manage conversations, visitors, contacts, operators, and websites in your Crisp workspace. Crisp is a customer messaging platform that combines live chat, visitor monitoring, and customer management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Crisp
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation_read`, `conversation_write`, `visitor_read`, `contact_read`, `operator_read`, `website_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `crisp_list_conversations` | List conversations | GET | /conversations |
| `crisp_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `crisp_send_message` | Send a message | POST | /conversations/{id}/messages |
| `crisp_list_visitors` | List visitors | GET | /visitors |
| `crisp_get_visitor` | Get a specific visitor | GET | /visitors/{id} |
| `crisp_list_contacts` | List contacts | GET | /contacts |
| `crisp_get_contact` | Get a specific contact | GET | /contacts/{id} |
| `crisp_list_operators` | List operators | GET | /operators |
| `crisp_get_operator` | Get a specific operator | GET | /operators/{id} |
| `crisp_get_website` | Get website settings | GET | /websites/{id} |

---

## Tool Details

### crisp_list_conversations

**What it does**: Returns a paginated list of conversations with their status, participants, and latest message.

**When to use**: Browse open or closed conversations to track customer issues.

**Arguments**:
- `status` (optional): `open`, `closed`, `all` — default `open`
- `limit` (optional): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "Show me all open conversations"

---

### crisp_get_conversation

**What it does**: Gets full conversation details including messages, timeline, and metadata.

**When to use**: Read the complete context of a conversation before responding.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Show me conversation #12345"

---

### crisp_send_message

**What it does**: Sends a message in a conversation. Use this to respond to customers or proactively reach out to contacts.

**When to use**: Reply to a customer message in an active conversation.

**Arguments**:
- `id` (required): Conversation ID
- `message` (required): Message content
- `type` (optional): `text` or `file` — default `text`

**Example LLM prompt**: "Send a message to conversation #12345 saying 'Thank you for reaching out'"

---

### crisp_list_visitors

**What it does**: Lists all visitors on your website. Returns visitor profiles including location, device, and current page.

**When to use**: Monitor who's browsing your website in real-time.

**Arguments**:
- `limit` (optional): default 25
- `offset` (optional): default 0
- `website_id` (optional): Filter by website ID

**Example LLM prompt**: "Show me all currently active visitors"

---

### crisp_get_visitor

**What it does**: Gets visitor details including location, device, browsing history, and conversation history.

**When to use**: Understand visitor behavior before starting a chat.

**Arguments**:
- `id` (required): Visitor ID

**Example LLM prompt**: "Show me visitor #67890"

---

### crisp_list_contacts

**What it does**: Lists all contacts in your Crisp workspace. Returns contact profiles including name, email, company, and activity.

**When to use**: Browse your contact database to find customers.

**Arguments**:
- `limit` (optional): default 25
- `offset` (optional): default 0
- `email` (optional): Filter by email address

**Example LLM prompt**: "List all contacts"

---

### crisp_get_contact

**What it does**: Gets full contact profile including custom attributes, conversation history, and activity.

**When to use**: Review contact profile before responding to their conversation.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Show me contact #78901"

---

### crisp_list_operators

**What it does**: Lists all operators in your Crisp workspace. Returns operator names, roles, and availability status.

**When to use**: See team members available for handling conversations.

**Arguments**:
- `limit` (optional): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "List all operators"

---

### crisp_get_operator

**What it does**: Gets a specific operator's profile including name, email, role, and permissions.

**When to use**: Get details about a team member before assigning conversations.

**Arguments**:
- `id` (required): Operator ID

**Example LLM prompt**: "Show me operator #op_123"

---

### crisp_get_website

**What it does**: Gets website settings for your Crisp workspace. Returns configuration including name, domain, and integration settings.

**When to use**: View website configuration and integration details.

**Arguments**:
- `id` (required): Website ID

**Example LLM prompt**: "Show me website settings"

---

## Crisp API Notes

- **Real-time Visitors**: Crisp tracks website visitors in real-time with browsing activity
- **Conversation Types**: Supports text and file messages
- **Contact Management**: Store customer data with custom attributes
- **Operator Roles**: Operators have different roles and permissions
- **Website Configuration**: Each website has its own chat widget and settings
- **Pagination**: Uses offset-based pagination with limit parameter

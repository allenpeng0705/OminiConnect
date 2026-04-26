# Front Tools

Provider: `front` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Front REST API. They allow AI agents to manage conversations, channels, teammates, inboxes, and automation rules in your Front workspace. Front is a shared inbox platform designed for customer support and team collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Front
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation_read`, `conversation_write`, `channel_read`, `teammate_read`, `inbox_read`, `rule_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `front_list_conversations` | List conversations | GET | /conversations |
| `front_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `front_create_message` | Create a message | POST | /messages |
| `front_list_channels` | List channels | GET | /channels |
| `front_get_channel` | Get a specific channel | GET | /channels/{id} |
| `front_list_teammates` | List teammates | GET | /teammates |
| `front_get_teammate` | Get a specific teammate | GET | /teammates/{id} |
| `front_list_inboxes` | List inboxes | GET | /inboxes |
| `front_get_inbox` | Get a specific inbox | GET | /inboxes/{id} |
| `front_list_rules` | List rules | GET | /rules |

---

## Tool Details

### front_list_conversations

**What it does**: Returns a paginated list of conversations with their status, assignees, and latest message.

**When to use**: Get an overview of all active conversations in the shared inbox.

**Arguments**:
- `inbox_id` (optional): Filter by inbox ID
- `teammate_id` (optional): Filter by assigned teammate
- `status` (optional): `open`, `closed`, `all` — default `open`
- `limit` (optional): default 25
- `page_token` (optional): Pagination token

**Example LLM prompt**: "Show me all open conversations in the support inbox"

---

### front_get_conversation

**What it does**: Gets full conversation details including messages, comments, and metadata.

**When to use**: Read the complete context of a conversation before responding or assigning.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Show me conversation #12345"

---

### front_create_message

**What it does**: Creates a new message or reply in a conversation. Use this to send messages to contacts through your Front inbox.

**When to use**: Send a new reply or outbound message to a customer.

**Arguments**:
- `conversation_id` (optional): Reply to existing conversation
- `channel_id` (optional): Channel ID to send message through (required for new conversations)
- `contact_id` (optional): Recipient contact ID
- `body` (required): Message content in HTML
- `subject` (optional): Subject for email messages

**Example LLM prompt**: "Send a message through channel ch_123 saying 'Thank you for reaching out'"

---

### front_list_channels

**What it does**: Lists all channels in your Front workspace. Returns channel names, types, and settings.

**When to use**: See all available channels before sending messages.

**Arguments**:
- `limit` (optional): default 25
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all connected channels"

---

### front_get_channel

**What it does**: Gets full channel details including settings, rules, and statistics.

**When to use**: Review channel configuration or performance metrics.

**Arguments**:
- `id` (required): Channel ID

**Example LLM prompt**: "Show me channel #ch_789"

---

### front_list_teammates

**What it does**: Lists all teammates in your Front workspace. Returns their names, emails, availability, and assignment rules.

**When to use**: See team members and their current availability.

**Arguments**:
- `limit` (optional): default 25
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all teammates and their availability"

---

### front_get_teammate

**What it does**: Gets a specific teammate's profile including name, email, availability status, and permissions.

**When to use**: Review teammate profile or workload before assigning conversations.

**Arguments**:
- `id` (required): Teammate ID

**Example LLM prompt**: "Show me teammate #tm_111"

---

### front_list_inboxes

**What it does**: Lists all inboxes in your Front workspace. Returns inbox names, member assignments, and channel connections.

**When to use**: See all inboxes and their conversation counts.

**Arguments**:
- `limit` (optional): default 25
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all inboxes"

---

### front_get_inbox

**What it does**: Gets full inbox details including settings, members, and statistics.

**When to use**: Review inbox performance or monitor incoming volume.

**Arguments**:
- `id` (required): Inbox ID

**Example LLM prompt**: "Show me inbox #ib_222"

---

### front_list_rules

**What it does**: Lists all rules in your Front workspace. Returns rule names, conditions, and actions for inbox automation.

**When to use**: Understand existing automations or check rule conditions.

**Arguments**:
- `inbox_id` (optional): Filter by inbox ID
- `limit` (optional): default 25
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all automation rules"

---

## Front API Notes

- **Shared Inbox**: Front centralizes messages from multiple channels into team inboxes
- **Conversation Threads**: Messages are organized into conversations with individual contacts
- **Channel Variety**: Supports email, Twitter, Facebook, SMS, and custom channels
- **Team Collaboration**: Assign conversations to teammates and track response times
- **Automation Rules**: Create rules to automatically assign, tag, or route conversations
- **Pagination**: Uses page_token for pagination rather than offset

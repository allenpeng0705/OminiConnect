# Bliro Tools

Provider: `bliro` | Engine: `nango` | Auth: OAuth2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Bliro API. They allow AI agents to manage messages, contacts, conversations, and teams. Bliro is a business communication and messaging platform.

## Authentication

**Nango OAuth2 CC**:
- Uses Client Credentials flow (machine-to-machine)
- User provides client ID and client secret
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bliro_list_messages` | List messages | GET | /v1/messages |
| `bliro_get_message` | Get message details | GET | /v1/messages/{messageId} |
| `bliro_send_message` | Send a message | POST | /v1/messages |
| `bliro_list_contacts` | List contacts | GET | /v1/contacts |
| `bliro_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `bliro_list_conversations` | List conversations | GET | /v1/conversations |
| `bliro_get_conversation` | Get conversation details | GET | /v1/conversations/{conversationId} |
| `bliro_list_teams` | List teams | GET | /v1/teams |
| `bliro_get_team` | Get team details | GET | /v1/teams/{teamId} |
| `bliro_get_webhooks` | Get webhook logs | GET | /v1/webhooks |

---

## Tool Details

### bliro_list_messages

**What it does**: Lists all messages in Bliro.

**When to use**: View message history, track communications.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Messages per page (default 20)

**Example LLM prompt**: "List all messages"

---

### bliro_get_message

**What it does**: Gets details for a specific message.

**When to use**: View message content, check delivery status.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message M-123"

---

### bliro_send_message

**What it does**: Sends a message through Bliro.

**When to use**: Send notifications, start conversations.

**Arguments**:
- `recipientId` (required): Recipient contact ID
- `content` (required): Message content
- `channel` (optional): Channel to send through

**Example LLM prompt**: "Send message 'Hello' to contact C-456"

---

### bliro_list_contacts

**What it does**: Lists all contacts in Bliro.

**When to use**: Browse contact database.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Contacts per page (default 20)

**Example LLM prompt**: "List all contacts"

---

### bliro_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: View contact information.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact C-456"

---

### bliro_list_conversations

**What it does**: Lists all conversations in Bliro.

**When to use**: View conversation threads.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Conversations per page (default 20)

**Example LLM prompt**: "List all conversations"

---

### bliro_get_conversation

**What it does**: Gets details for a specific conversation.

**When to use**: View conversation history, messages.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "Get conversation CV-789"

---

### bliro_list_teams

**What it does**: Lists all teams in Bliro.

**When to use**: View organizational structure.

**Arguments**: None required

**Example LLM prompt**: "List all teams"

---

### bliro_get_team

**What it does**: Gets details for a specific team.

**When to use**: View team members, settings.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get details for team T-100"

---

### bliro_get_webhooks

**What it does**: Gets webhook event logs.

**When to use**: Debug webhooks, check event history.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Events per page (default 20)

**Example LLM prompt**: "Get recent webhook events"

---

## Bliro API Notes

- **Client Credentials**: Uses OAuth2 CC flow for server-to-server auth
- **Messages**: Can be sent through various channels
- **Contacts**: Stored in Bliro's contact database
- **Conversations**: Threaded message conversations
- **Teams**: Organizational units for managing users

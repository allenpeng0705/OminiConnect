# Missive Tools

Provider: `missive` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Missive API. They allow AI agents to manage conversations, messages, contacts, groups, and comments. **Requires Missive API token.**

## Authentication

**Nango API_KEY**:
- User provides Missive API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `api_token: ${apiKey}`
- Base URL: `https://missiveapp.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `missive_list_conversations` | List conversations | GET | /v1/conversations |
| `missive_get_conversation` | Get conversation details | GET | /v1/conversations/{conversationId} |
| `missive_list_messages` | List messages | GET | /v1/conversations/{conversationId}/messages |
| `missive_get_message` | Get message details | GET | /v1/messages/{messageId} |
| `missive_send_message` | Send a message | POST | /v1/conversations/{conversationId}/messages |
| `missive_list_contacts` | List contacts | GET | /v1/contacts |
| `missive_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `missive_list_groups` | List groups | GET | /v1/groups |
| `missive_get_group` | Get group details | GET | /v1/groups/{groupId} |
| `missive_create_comment` | Create a comment | POST | /v1/conversations/{conversationId}/comments |

---

## Tool Details

### missive_list_conversations

**What it does**: Lists all conversations in Missive.

**When to use**: Browse conversations, find specific chats.

**Arguments**:
- `status` (optional): Filter by status (open, closed, archived)
- `assignee_id` (optional): Filter by assignee ID
- `folder_id` (optional): Filter by folder ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all open conversations"

---

### missive_get_conversation

**What it does**: Gets details of a specific conversation.

**When to use**: Check conversation info, participants.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation CON-12345"

---

### missive_list_messages

**What it does**: Lists messages in a conversation.

**When to use**: Read conversation history.

**Arguments**:
- `conversationId` (required): Conversation ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List messages in conversation CON-12345"

---

### missive_get_message

**What it does**: Gets details of a specific message.

**When to use**: Get message content, attachments.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message MSG-12345"

---

### missive_send_message

**What it does**: Sends a message in a conversation.

**When to use**: Reply in conversations, send notifications.

**Arguments**:
- `conversationId` (required): Conversation ID
- `content` (required): Message content
- `author_id` (optional): Author user ID

**Example LLM prompt**: "Send message 'Hello' to conversation CON-12345"

---

### missive_list_contacts

**What it does**: Lists all contacts in Missive.

**When to use**: Find contacts, browse address book.

**Arguments**:
- `search` (optional): Search query
- `organization_id` (optional): Filter by organization ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "Find contact with name John Doe"

---

### missive_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Get contact information, email, phone.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### missive_list_groups

**What it does**: Lists all groups in Missive.

**When to use**: Browse groups, find team groups.

**Arguments**: None

**Example LLM prompt**: "List all groups"

---

### missive_get_group

**What it does**: Gets details of a specific group.

**When to use**: Check group members, settings.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 12345"

---

### missive_create_comment

**What it does**: Creates a comment on a conversation.

**When to use**: Add notes, internal comments.

**Arguments**:
- `conversationId` (required): Conversation ID
- `content` (required): Comment content
- `author_id` (optional): Author user ID

**Example LLM prompt**: "Add comment 'Needs review' to conversation CON-12345"

---

## Missive Notes

- **Shared inbox**: Team email and chat platform
- **Conversations**: Unified message threads
- **Contacts**: Address book management
- **Groups**: Team groups for routing
- **Comments**: Internal notes on conversations

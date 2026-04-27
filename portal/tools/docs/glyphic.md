# Glyphic Tools

Provider: `glyphic` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Glyphic API. They allow AI agents to manage conversations, messages, threads, contacts, and webhooks. **Requires Glyphic API key.**

## Authentication

**Nango API_KEY**:
- User provides their Glyphic API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.glyphic.ai`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `glyphic_list_conversations` | List conversations | GET | /v1/conversations |
| `glyphic_get_conversation` | Get conversation details | GET | /v1/conversations/{id} |
| `glyphic_send_message` | Send a message | POST | /v1/conversations/{id}/messages |
| `glyphic_list_messages` | List messages | GET | /v1/conversations/{id}/messages |
| `glyphic_create_thread` | Create a thread | POST | /v1/threads |
| `glyphic_list_threads` | List threads | GET | /v1/threads |
| `glyphic_get_thread` | Get thread details | GET | /v1/threads/{id} |
| `glyphic_list_contacts` | List contacts | GET | /v1/contacts |
| `glyphic_get_contact` | Get contact details | GET | /v1/contacts/{id} |
| `glyphic_list_webhooks` | List webhooks | GET | /v1/webhooks |

---

## Tool Details

### glyphic_list_conversations

**What it does**: Lists all conversations in Glyphic.

**When to use**: Browse active conversations.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all conversations"

---

### glyphic_get_conversation

**What it does**: Gets detailed information about a conversation.

**When to use**: Get conversation metadata and participants.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation abc123"

---

### glyphic_send_message

**What it does**: Sends a message in a conversation.

**When to use**: Reply to a conversation.

**Arguments**:
- `id` (required): Conversation ID
- `content` (required): Message content

**Example LLM prompt**: "Send 'Hello!' to conversation abc123"

---

### glyphic_list_messages

**What it does**: Lists messages in a conversation.

**When to use**: Read conversation history.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "List messages in conversation abc123"

---

### glyphic_create_thread

**What it does**: Creates a new thread in Glyphic.

**When to use**: Start a new discussion thread.

**Arguments**:
- `title` (required): Thread title
- `participants` (optional): Participant IDs

**Example LLM prompt**: "Create a new thread titled 'Project Discussion'"

---

### glyphic_list_threads

**What it does**: Lists all threads in Glyphic.

**When to use**: Browse available threads.

**Arguments**: None

**Example LLM prompt**: "List all threads"

---

### glyphic_get_thread

**What it does**: Gets detailed information about a thread.

**When to use**: Get thread details and participants.

**Arguments**:
- `id` (required): Thread ID

**Example LLM prompt**: "Get details for thread abc123"

---

### glyphic_list_contacts

**What it does**: Lists all contacts in Glyphic.

**When to use**: Browse contact list.

**Arguments**: None

**Example LLM prompt**: "List all contacts"

---

### glyphic_get_contact

**What it does**: Gets detailed information about a contact.

**When to use**: Get contact details.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact abc123"

---

### glyphic_list_webhooks

**What it does**: Lists all configured webhooks.

**When to use**: View webhook subscriptions.

**Arguments**: None

**Example LLM prompt**: "List all webhooks"

---

## Glyphic API Notes

- **Conversations**: Chat conversations with participants
- **Threads**: Discussion threads for organized communication
- **Messages**: Individual messages within conversations
- **Webhooks**: Event notifications for new messages

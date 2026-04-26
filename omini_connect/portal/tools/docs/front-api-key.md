# Front API Key Tools

Provider: `front-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Front API. They allow AI agents to manage conversations, contacts, teammates, inboxes, tags, and automation rules. **Requires Front API token.**

## Authentication

**Nango API_KEY**:
- User provides their Front API token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api2.frontapp.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `front_list_conversations` | List conversations | GET | /conversations |
| `front_get_conversation` | Get conversation details | GET | /conversations/{conversation_id} |
| `front_list_comments` | List comments on conversation | GET | /conversations/{conversation_id}/comments |
| `front_list_teammates` | List teammates | GET | /team |
| `front_get_teammate` | Get teammate details | GET | /team/{teammate_id} |
| `front_list_inboxes` | List inboxes | GET | /inboxes |
| `front_get_inbox` | Get inbox details | GET | /inboxes/{inbox_id} |
| `front_list_tags` | List tags | GET | /tags |
| `front_list_rules` | List automation rules | GET | /rules |
| `front_list_contacts` | List contacts | GET | /contacts |

---

## Tool Details

### front_list_conversations

**What it does**: Lists conversations from Front with optional filters.

**When to use**: Browse conversations, filter by inbox or status.

**Arguments**:
- `inbox_id` (optional): Filter by inbox ID
- `status` (optional): Filter by status (open, archived, deleted)
- `page_token` (optional): Pagination token

**Example LLM prompt**: "List all open conversations in the support inbox"

---

### front_get_conversation

**What it does**: Gets detailed information about a specific conversation.

**When to use**: Get conversation messages, participants, and metadata.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation abc123"

---

### front_list_comments

**What it does**: Lists all comments on a specific conversation.

**When to use**: Read the full conversation thread and comments.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "List all comments on conversation abc123"

---

### front_list_teammates

**What it does**: Lists all team members in Front.

**When to use**: View team structure and member details.

**Arguments**: None

**Example LLM prompt**: "List all teammates in Front"

---

### front_get_teammate

**What it does**: Gets detailed information about a specific teammate.

**When to use**: Get teammate's email, assignment rules, and status.

**Arguments**:
- `teammate_id` (required): Teammate ID

**Example LLM prompt**: "Get details for teammate john@company.com"

---

### front_list_inboxes

**What it does**: Lists all inboxes in Front.

**When to use**: View all inboxes and their purposes.

**Arguments**: None

**Example LLM prompt**: "List all inboxes"

---

### front_get_inbox

**What it does**: Gets detailed information about a specific inbox.

**When to use**: Get inbox settings, member list, and stats.

**Arguments**:
- `inbox_id` (required): Inbox ID

**Example LLM prompt**: "Get details for inbox support@company.com"

---

### front_list_tags

**What it does**: Lists all tags used in Front.

**When to use**: View available tags for organizing conversations.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### front_list_rules

**What it does**: Lists all automation rules in Front.

**When to use**: View automated workflows and triggers.

**Arguments**: None

**Example LLM prompt**: "List all automation rules"

---

### front_list_contacts

**What it does**: Lists all contacts in Front with optional search.

**When to use**: Find contacts, search by name or email.

**Arguments**:
- `query` (optional): Search query for contacts
- `page_token` (optional): Pagination token

**Example LLM prompt**: "Search for contacts named John"

---

## Front API Notes

- **Conversation ID format**: Typically starts with `cnv_`
- **Inbox management**: Conversations can be moved between inboxes
- **Tags**: Multiple tags can be applied to conversations
- **Comments**: Both public messages and private notes

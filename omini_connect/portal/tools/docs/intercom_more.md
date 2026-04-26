# Intercom More Tools

Provider: `intercom_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Intercom REST API. They allow AI agents to manage conversations, messages, teammates, and articles in your Intercom workspace.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation_read`, `conversation_write`, `team_read`, `article_read`, `article_write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `intercom_list_conversations` | List conversations | GET | /conversations |
| `intercom_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `intercom_reply_to_conversation` | Reply to a conversation | POST | /conversations/{id}/reply |
| `intercom_list_messages` | List messages in a conversation | GET | /conversations/{id}/messages |
| `intercom_get_message` | Get a specific message | GET | /messages/{id} |
| `intercom_list_teammates` | List teammates | GET | /teammates |
| `intercom_get_teammate` | Get a specific teammate | GET | /teammates/{id} |
| `intercom_list_articles` | List articles | GET | /articles |
| `intercom_get_article` | Get a specific article | GET | /articles/{id} |
| `intercom_create_article` | Create an article | POST | /articles |

---

## Tool Details

### intercom_list_conversations

**What it does**: Returns a paginated list of conversations with their status, participants, and latest message.

**When to use**: Browse open or closed support conversations to track customer issues.

**Arguments**:
- `state` (optional): `open`, `closed`, `all` — default `open`
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "Show me all open conversations"

---

### intercom_get_conversation

**What it does**: Gets full details of a specific conversation including messages, parts, and metadata.

**When to use**: Read the complete context of a customer issue before responding or taking action.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Show me conversation #12345"

---

### intercom_reply_to_conversation

**What it does**: Sends a reply to an existing conversation. Use this to respond to a contact or team member.

**When to use**: Reply to a customer message in an active conversation.

**Arguments**:
- `id` (required): Conversation ID
- `message` (required): Reply message content
- `type` (optional): Reply type — default `admin`

**Example LLM prompt**: "Reply to conversation #12345 with 'Thank you for reaching out'"

---

### intercom_list_messages

**What it does**: Lists all messages in a conversation. Returns the message thread with timestamps and sender information.

**When to use**: See the full conversation history with a customer.

**Arguments**:
- `id` (required): Conversation ID
- `per_page` (optional): default 25

**Example LLM prompt**: "Show me all messages in conversation #12345"

---

### intercom_get_message

**What it does**: Gets a specific message by ID within a conversation. Returns message content, author, and metadata.

**When to use**: Read a specific message in detail.

**Arguments**:
- `id` (required): Message ID

**Example LLM prompt**: "Show me message #67890"

---

### intercom_list_teammates

**What it does**: Lists all teammates in your Intercom workspace with their names, emails, and availability status.

**When to use**: See team members before assigning conversations or asking for help.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all teammates in the workspace"

---

### intercom_get_teammate

**What it does**: Gets a specific teammate's profile including name, email, avatar, and role.

**When to use**: Get details about a team member before handing off a conversation.

**Arguments**:
- `id` (required): Teammate ID

**Example LLM prompt**: "Show me teammate #42"

---

### intercom_list_articles

**What it does**: Lists all articles in your Intercom help center. Returns title, description, state, and author information.

**When to use**: Browse your knowledge base to find relevant articles for customers.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "What articles are available?"

---

### intercom_get_article

**What it does**: Gets full article content including title, body, statistics, and publication state.

**When to use**: Read an article before recommending it to customers.

**Arguments**:
- `id` (required): Article ID

**Example LLM prompt**: "Show me article #123"

---

### intercom_create_article

**What it does**: Creates a new article in your Intercom help center with title and body content.

**When to use**: Add new articles to your knowledge base to help customers self-serve.

**Arguments**:
- `title` (required): Article title
- `body` (required): Article body content in HTML
- `state` (optional): `draft` or `published` — default `draft`
- `author_id` (optional): Author user ID

**Example LLM prompt**: "Create an article titled 'How to reset password' with body explaining the steps"

---

## Intercom API Reference

These tools use the Intercom REST API. See official docs for full details:
- https://developers.intercom.com/intercom-api-reference/
- Rate limits: Based on plan (varies by tier)
- Pagination: Use `per_page` and `page` parameters
- All dates: ISO 8601 format

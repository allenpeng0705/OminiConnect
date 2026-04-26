# Kustomer Tools

Provider: `kustomer` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Kustomer API. They allow AI agents to manage conversations, customers, messages, and support operations. **Requires Kustomer API key.**

## Authentication

**API Key via Nango**:
- User provides their Kustomer API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{extension}.kustomerapp.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `kustomer_list_conversations` | List conversations | GET | /conversations |
| `kustomer_get_conversation` | Get conversation details | GET | /conversations/{conversation_id} |
| `kustomer_list_messages` | List messages | GET | /conversations/{conversation_id}/messages |
| `kustomer_create_message` | Create a message | POST | /conversations/{conversation_id}/messages |
| `kustomer_list_customers` | List customers | GET | /customers |
| `kustomer_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `kustomer_list_tags` | List tags | GET | /tags |
| `kustomer_list_teams` | List teams | GET | /teams |
| `kustomer_list_users` | List users | GET | /users |
| `kustomer_get_user` | Get user details | GET | /users/{user_id} |

---

## Tool Details

### kustomer_list_conversations

**What it does**: Lists all conversations.

**When to use**: View conversations, track support tickets.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `pageSize` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all conversations in Kustomer"

---

### kustomer_get_conversation

**What it does**: Gets details for a specific conversation.

**When to use**: Get conversation information.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation abc123"

---

### kustomer_list_messages

**What it does**: Lists messages in a conversation.

**When to use**: View conversation history.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "List messages for conversation abc123"

---

### kustomer_create_message

**What it does**: Creates a message in a conversation.

**When to use**: Reply to conversations.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `body` (required): Message content

**Example LLM prompt**: "Send a message to conversation abc123"

---

### kustomer_list_customers

**What it does**: Lists all customers.

**When to use**: Find customers, view customer list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `pageSize` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all customers in Kustomer"

---

### kustomer_get_customer

**What it does**: Gets details for a specific customer.

**When to use**: Get customer information.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer xyz789"

---

### kustomer_list_tags

**What it does**: Lists all tags.

**When to use**: View tags, categorize conversations.

**Arguments**: None

**Example LLM prompt**: "List all tags in Kustomer"

---

### kustomer_list_teams

**What it does**: Lists all teams.

**When to use**: View support teams.

**Arguments**: None

**Example LLM prompt**: "List all teams in Kustomer"

---

### kustomer_list_users

**What it does**: Lists all users (agents).

**When to use**: View agents, find support staff.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all users in Kustomer"

---

### kustomer_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u1"

---

## Kustomer API Notes

- **Customer Service**: CRM for support operations
- **Conversations**: Support tickets and customer interactions
- **Messages**: Individual messages in conversations
- **Customers**: Customer profiles and history
- **Tags**: Categorize and organize conversations

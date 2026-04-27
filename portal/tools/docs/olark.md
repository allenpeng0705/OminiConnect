# Olark Tools

Provider: `olark` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Olark Live Chat API. They allow AI agents to manage real-time customer conversations, visitors, operators, chats, and notes. Olark is a popular live chat platform for customer support with features like operator management and chat routing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Olark
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `olark_list_conversations` | List chat conversations | GET | /conversations |
| `olark_get_conversation` | Get conversation details | GET | /conversations/{conversation_id} |
| `olark_send_message` | Send a message in a conversation | POST | /conversations/{conversation_id}/messages |
| `olark_list_visitors` | List all chat visitors | GET | /visitors |
| `olark_get_visitor` | Get visitor details | GET | /visitors/{visitor_id} |
| `olark_list_operators` | List all operators | GET | /operators |
| `olark_get_operator` | Get operator details | GET | /operators/{operator_id} |
| `olark_list_chats` | List all chats | GET | /chats |
| `olark_get_chat` | Get chat details | GET | /chats/{chat_id} |
| `olark_add_note` | Add a note to a conversation | POST | /conversations/{conversation_id}/notes |

---

## Tool Details

### olark_list_conversations

**What it does**: Lists all chat conversations in your Olark account, optionally filtered by status, operator, or time period.

**When to use**: Review recent chats, monitor conversation volume, find specific customer interactions.

**Arguments**:
- `status` (optional): Filter by status (open, closed, all) — default `all`
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "Show me all closed conversations from today"

---

### olark_get_conversation

**What it does**: Gets detailed information about a specific conversation including all messages, participants, and metadata.

**When to use**: Read full conversation history, understand context for follow-ups.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "Get the conversation details for chat-456"

---

### olark_send_message

**What it does**: Sends a message in an active chat conversation. Used by operators to respond to visitors.

**When to use**: Reply to a visitor, send automated responses, update customers.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `body` (required): Message text
- `author_id` (optional): Operator ID sending the message

**Example LLM prompt**: "Send a message to conversation chat-123 saying 'Thanks for reaching out!'"

---

### olark_list_visitors

**What it does**: Lists all chat visitors currently on your website, including active and recent visitors with their browsing context.

**When to use**: See who's currently browsing, monitor visitor activity, identify leads.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all current visitors on the website"

---

### olark_get_visitor

**What it does**: Gets detailed information about a specific visitor including browsing history, location, and chat context.

**When to use**: Understand a visitor's needs before responding, see their history with your team.

**Arguments**:
- `visitor_id` (required): Visitor ID

**Example LLM prompt**: "Get details for visitor abc-123"

---

### olark_list_operators

**What it does**: Lists all operators in your Olark account with their status, role, and availability.

**When to use**: View team availability, check operator roster, assign conversations.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `status` (optional): Filter by status (online, offline, away)

**Example LLM prompt**: "List all operators currently online"

---

### olark_get_operator

**What it does**: Gets detailed information about a specific operator including role, status, and performance metrics.

**When to use**: Check operator profile, review performance, manage team.

**Arguments**:
- `operator_id` (required): Operator ID

**Example LLM prompt**: "Get details for operator john-smith"

---

### olark_list_chats

**What it does**: Lists all chat sessions in your Olark account. A chat is a single conversation session between visitor and operator.

**When to use**: Review chat history, monitor operator performance, find specific chats.

**Arguments**:
- `status` (optional): Filter by status (active, ended, all) — default `all`
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Show me all active chats"

---

### olark_get_chat

**What it does**: Gets detailed information about a specific chat session including transcript, participants, and metadata.

**When to use**: Read full chat transcript, understand customer issue context, review conversation history.

**Arguments**:
- `chat_id` (required): Chat ID

**Example LLM prompt**: "Get the transcript for chat abc-123"

---

### olark_add_note

**What it does**: Adds a note to an existing conversation. Notes are internal annotations for operators.

**When to use**: Add context for other operators, record customer preferences, annotate conversations.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `body` (required): Note content
- `author_id` (optional): Operator ID adding the note

**Example LLM prompt**: "Add a note to conversation chat-456 saying 'Customer prefers email follow-up'"

---

## Olark API Notes

- **Visitor IDs**: Unique identifiers for each website visitor
- **Conversation state**: Use status filter to find open vs closed conversations
- **Operator roles**: Operators can have different roles affecting their permissions
- **Chat vs Conversation**: A conversation may contain multiple chats over time
- **Notes**: Internal annotations visible only to operators, not customers

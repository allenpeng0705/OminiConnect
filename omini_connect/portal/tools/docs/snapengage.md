# Snapengage Tools

Provider: `snapengage` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Snapengage REST API. They allow AI agents to manage conversations, operators, engagements, proactive messages, and chats. Snapengage is a customer chat and engagement platform with features like proactive chat invitations and multi-channel support.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Snapengage
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `snapengage_list_conversations` | List conversations | GET | /conversations |
| `snapengage_get_conversation` | Get conversation details | GET | /conversations/{conversation_id} |
| `snapengage_send_message` | Send a message | POST | /conversations/{conversation_id}/messages |
| `snapengage_list_operators` | List all operators | GET | /operators |
| `snapengage_get_operator` | Get operator details | GET | /operators/{operator_id} |
| `snapengage_list_engagements` | List all engagements | GET | /engagements |
| `snapengage_get_engagement` | Get engagement details | GET | /engagements/{engagement_id} |
| `snapengage_send_proactive` | Send proactive message | POST | /proactive |
| `snapengage_list_chats` | List all chats | GET | /chats |
| `snapengage_get_chat` | Get chat details | GET | /chats/{chat_id} |

---

## Tool Details

### snapengage_list_conversations

**What it does**: Lists all conversations across all channels. Snapengage unifies chats from multiple sources including web, mobile, and social.

**When to use**: View unified customer conversations, track cross-channel interactions.

**Arguments**:
- `status` (optional): Filter by status (open, closed) — default `open`
- `limit` (optional): Max results (default 50)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open conversations"

---

### snapengage_get_conversation

**What it does**: Gets detailed information about a specific conversation including all messages and conversation metadata.

**When to use**: Read full conversation history, understand multi-channel customer journeys.

**Arguments**:
- `conversation_id` (required): Conversation ID

**Example LLM prompt**: "Get conversation details for conv-456"

---

### snapengage_send_message

**What it does**: Sends a message in an active conversation. Used to respond to customers across all integrated channels.

**When to use**: Reply to a customer, send follow-ups, provide support responses.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `body` (required): Message text
- `author_id` (optional): Operator ID sending the message

**Example LLM prompt**: "Send a message to conversation conv-789 saying 'Your request is being processed'"

---

### snapengage_list_operators

**What it does**: Lists all operators in your Snapengage account with their status, role, and availability.

**When to use**: View team availability, check operator roster, manage assignments.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `status` (optional): Filter by status (available, busy, offline)

**Example LLM prompt**: "List all available operators"

---

### snapengage_get_operator

**What it does**: Gets detailed information about a specific operator including role, status, and performance data.

**When to use**: Review operator profiles, check performance metrics, manage team.

**Arguments**:
- `operator_id` (required): Operator ID

**Example LLM prompt**: "Get details for operator john.smith"

---

### snapengage_list_engagements

**What it does**: Lists all engagements in Snapengage. Engagements track proactive chat invitations and automated messages.

**When to use**: Review engagement rules, monitor proactive chat performance, manage automated triggers.

**Arguments**:
- `status` (optional): Filter by status (active, ended, all) — default `all`
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Show me all active engagements"

---

### snapengage_get_engagement

**What it does**: Gets detailed information about a specific engagement showing engagement rules, triggers, and performance metrics.

**When to use**: Review engagement effectiveness, modify engagement rules, analyze proactive chat results.

**Arguments**:
- `engagement_id` (required): Engagement ID

**Example LLM prompt**: "Get details for engagement rule spring-promo"

---

### snapengage_send_proactive

**What it does**: Sends a proactive chat invitation to a visitor. Used to start conversations based on engagement rules.

**When to use**: Initiate contact with potential customers, send targeted promotions, offer help proactively.

**Arguments**:
- `visitor_id` (required): Visitor ID to target
- `message` (required): Proactive message text
- `engagement_id` (optional): Engagement rule ID to use

**Example LLM prompt**: "Send a proactive message to visitor abc-123 offering help with checkout"

---

### snapengage_list_chats

**What it does**: Lists all chats in your Snapengage account with filtering by status, operator, or time period.

**When to use**: Review chat history, monitor operator performance, find specific customer chats.

**Arguments**:
- `status` (optional): Filter by status (active, ended, all) — default `all`
- `limit` (optional): Max results (default 50)
- `operator_id` (optional): Filter by operator ID

**Example LLM prompt**: "Show me all ended chats from today"

---

### snapengage_get_chat

**What it does**: Gets detailed information about a specific chat including transcript, participants, and metadata.

**When to use**: Read full chat transcript, understand customer issue context, review conversation history.

**Arguments**:
- `chat_id` (required): Chat ID

**Example LLM prompt**: "Get the transcript for chat abc-123"

---

## Snapengage API Notes

- **Multi-channel**: Snapengage aggregates chats from web, mobile, Facebook, Twitter, and other sources
- **Engagements**: Automated proactive chat invitations triggered by visitor behavior
- **Operator roles**: Operators can have different roles affecting their permissions and chat routing
- **Chat status**: Active chats are ongoing, ended chats are completed
- **Proactive messages**: Automated invitations sent to visitors before they start a chat

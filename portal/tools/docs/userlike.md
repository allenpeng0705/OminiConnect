# Userlike Tools

Provider: `userlike` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Userlike API. They allow AI agents to manage conversations, visitors, agents (operators), canned responses, and availability in your Userlike workspace.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations_read`, `conversations_write`, `visitors_read`, `operators_read`, `operators_write`, `canned_responses_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `userlike_list_conversations` | List conversations with filtering options | GET | /conversations |
| `userlike_get_conversation` | Get details of a specific conversation | GET | /conversations/{conversation_id} |
| `userlike_send_message` | Send a message to a conversation | POST | /conversations/{conversation_id}/messages |
| `userlike_list_visitors` | List website visitors | GET | /visitors |
| `userlike_get_visitor` | Get details of a specific visitor | GET | /visitors/{visitor_id} |
| `userlike_list_agents` | List operators/agents | GET | /operators |
| `userlike_get_agent` | Get details of a specific agent | GET | /operators/{operator_id} |
| `userlike_list_canned_responses` | List canned responses | GET | /canned-responses |
| `userlike_get_canned_response` | Get details of a specific canned response | GET | /canned-responses/{response_id} |
| `userlike_update_availability` | Update agent availability status | PUT | /operators/{operator_id}/availability |

---

## Tool Details

### userlike_list_conversations

**What it does**: Returns a paginated list of conversations filtered by state, operator, or date range.

**When to use**: Browse active chats, find closed sessions, or review conversations by date.

**Arguments**:
- `state` (optional): `open`, `closed`, `all` — default `open`
- `operator_id` (optional): Filter by operator
- `date_from` (optional): ISO 8601 start date
- `date_to` (optional): ISO 8601 end date
- `page` (optional): default 1
- `limit` (optional, max 100): default 20

**Example LLM prompt**: "Show me all closed conversations from last week"

---

### userlike_get_conversation

**What it does**: Gets full details of a specific conversation including messages and client info.

**When to use**: Read the full chat history before responding or reviewing past interactions.

**Arguments**:
- `conversation_id` (required): The conversation ID

**Example LLM prompt**: "Show me conversation abc123 with all messages"

---

### userlike_send_message

**What it does**: Sends a message to an active conversation on behalf of an operator.

**When to use**: Respond to customer inquiries or send follow-up messages in an ongoing chat.

**Arguments**:
- `conversation_id` (required): The conversation ID
- `text` (required): Message text
- `channel` (optional): `chat`, `messenger`, or `sms`

**Example LLM prompt**: "Send a message to conversation abc123 saying we'll resolve this shortly"

---

### userlike_list_visitors

**What it does**: Lists visitors currently browsing your website or recently active.

**When to use**: Monitor active users, see what pages they're viewing, or track visitor patterns.

**Arguments**:
- `state` (optional): `online`, `offline`, `all` — default `all`
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "Who is currently browsing the website?"

---

### userlike_get_visitor

**What it does**: Gets detailed visitor info including browsing history, location, and contact data.

**When to use**: Learn about a visitor before starting a chat or routing their conversation.

**Arguments**:
- `visitor_id` (required): The visitor ID

**Example LLM prompt**: "Show me the browsing history for visitor xyz"

---

### userlike_list_agents

**What it does**: Lists all operators (agents) in the workspace with their availability status.

**When to use**: See who's online, check team structure, or find available agents for routing.

**Arguments**:
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "List all operators and their current status"

---

### userlike_get_agent

**What it does**: Gets detailed operator info including profile, role, and working hours.

**When to use**: Check an operator's schedule, permissions, or assigned channels.

**Arguments**:
- `operator_id` (required): The operator ID

**Example LLM prompt**: "Show me operator john's profile and working hours"

---

### userlike_list_canned_responses

**What it does**: Lists all canned responses (saved replies) grouped by category.

**When to use**: Find a quick response before typing a standard reply.

**Arguments**:
- `category` (optional): Filter by category name
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "What canned responses do we have for shipping questions?"

---

### userlike_get_canned_response

**What it does**: Gets the full content of a specific canned response with shortcuts.

**When to use**: Read a canned response before using it or to understand variable placeholders.

**Arguments**:
- `response_id` (required): The canned response ID

**Example LLM prompt**: "Show me the refund policy canned response"

---

### userlike_update_availability

**What it does**: Updates an operator's availability status and optionally their working hours.

**When to use**: Set yourself online before starting a shift, go offline, or configure your schedule.

**Arguments**:
- `operator_id` (required): The operator ID
- `status` (required): `online`, `offline`, or `away`
- `working_hours` (optional): Object with `timezone` and `schedule`

**Example LLM prompt**: "Set operator john to online status with 9-5 Berlin time"

---

## Userlike API Reference

These tools use the Userlike API. See official docs for full details:
- https://userlike.github.io/api/
- Rate limits: Varies by plan
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format
- Availability states: `online`, `offline`, `away`

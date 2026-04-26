# Autobot Tools

Provider: `autobot` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Autobot chatbot platform API. They allow AI agents to manage chatbots, conversation flows, subscribers, and messaging campaigns.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `bots:read`, `bots:write`, `flows:read`, `flows:write`, `subscribers:read`, `messages:write`, `broadcasts:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `autobot_list_bots` | Retrieve a list of all chatbots | GET | /bots |
| `autobot_get_bot` | Get details of a specific chatbot | GET | /bots/{bot_id} |
| `autobot_create_bot` | Create a new chatbot | POST | /bots |
| `autobot_list_flows` | Retrieve all conversation flows | GET | /flows |
| `autobot_get_flow` | Get configuration of a specific flow | GET | /flows/{flow_id} |
| `autobot_create_flow` | Create a new conversation flow | POST | /flows |
| `autobot_list_subscribers` | Retrieve all subscribers | GET | /subscribers |
| `autobot_get_subscriber` | Get details about a specific subscriber | GET | /subscribers/{subscriber_id} |
| `autobot_send_message` | Send a direct message to a subscriber | POST | /messages/send |
| `autobot_create_broadcast` | Create and send a broadcast message | POST | /broadcasts |

---

## Tool Details

### autobot_list_bots

**What it does**: Returns a list of all chatbots in your account with optional filtering.

**When to use**: Browse available bots before managing subscribers or sending messages.

**Arguments**:
- `limit` (optional): Maximum number of bots to return (default 50)
- `offset` (optional): Offset for pagination
- `status` (optional): Filter by status (`active`, `inactive`)

---

### autobot_get_bot

**What it does**: Gets detailed information about a specific chatbot.

**When to use**: Check bot configuration, personality, or status before sending messages.

**Arguments**:
- `bot_id` (required): Unique identifier of the bot

---

### autobot_create_bot

**What it does**: Creates a new chatbot with specified configuration.

**When to use**: Set up a new chatbot for a specific use case or platform.

**Arguments**:
- `name` (required): Name of the chatbot
- `description` (optional): Description of the bot's purpose
- `personality` (optional): Bot personality type
- `language` (optional): Primary language code

---

### autobot_list_flows

**What it does**: Retrieves all conversation flows for your bots.

**When to use**: See available flows before triggering or modifying conversations.

**Arguments**:
- `bot_id` (optional): Filter flows by bot ID
- `status` (optional): Filter by status (`draft`, `published`, `archived`)

---

### autobot_get_flow

**What it does**: Gets detailed configuration of a specific flow including nodes and connections.

**When to use**: Understand a flow's logic before triggering or modifying it.

**Arguments**:
- `flow_id` (required): Unique identifier of the flow

---

### autobot_create_flow

**What it does**: Creates a new conversation flow with nodes and connections.

**When to use**: Set up automated conversation sequences.

**Arguments**:
- `name` (required): Name of the flow
- `bot_id` (optional): Bot ID to attach this flow to
- `trigger` (optional): Flow trigger type (`keyword`, `event`, `schedule`)

---

### autobot_list_subscribers

**What it does**: Retrieves all subscribers who have opted into your bots.

**When to use**: Manage your audience before broadcasting messages.

**Arguments**:
- `bot_id` (optional): Filter by bot ID
- `segment` (optional): Filter by segment name

---

### autobot_get_subscriber

**What it does**: Gets detailed information about a specific subscriber.

**When to use**: Check subscriber profile before sending personalized messages.

**Arguments**:
- `subscriber_id` (required): Unique identifier of the subscriber

---

### autobot_send_message

**What it does**: Sends a direct message to a subscriber through their bot.

**When to use**: Send personalized responses or notifications to individual subscribers.

**Arguments**:
- `subscriber_id` (required): Target subscriber ID
- `bot_id` (required): Bot to send from
- `content` (required): Message content
- `quick_replies` (optional): Quick reply options

---

### autobot_create_broadcast

**What it does**: Creates and sends a broadcast message to multiple subscribers.

**When to use**: Send announcements, promotions, or notifications to your subscriber base.

**Arguments**:
- `name` (required): Name of the broadcast
- `bot_id` (required): Bot to broadcast from
- `segment` (optional): Target segment name
- `message` (required): Broadcast message content
- `schedule_time` (optional): ISO 8601 time to send (for scheduled broadcasts)

---

## Autobot API Reference

See official docs for full details on rate limits and pagination.

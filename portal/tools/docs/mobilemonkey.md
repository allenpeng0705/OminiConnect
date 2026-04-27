# Mobilemonkey Tools

Provider: `mobilemonkey` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Mobilemonkey is a chatbot platform that enables businesses to create automated conversations across multiple channels. These tools allow AI agents to manage keywords, flows, subscribers, messages, and broadcasts.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `keywords`, `flows`, `subscribers`, `messages`, `broadcasts`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mobilemonkey_list_keywords` | List all keywords | GET | /keywords |
| `mobilemonkey_get_keyword` | Get a specific keyword | GET | /keywords/{id} |
| `mobilemonkey_create_keyword` | Create a new keyword | POST | /keywords |
| `mobilemonkey_list_flows` | List all flows | GET | /flows |
| `mobilemonkey_get_flow` | Get a specific flow | GET | /flows/{id} |
| `mobilemonkey_create_flow` | Create a new flow | POST | /flows |
| `mobilemonkey_list_subscribers` | List all subscribers | GET | /subscribers |
| `mobilemonkey_get_subscriber` | Get a specific subscriber | GET | /subscribers/{id} |
| `mobilemonkey_send_message` | Send a message to a subscriber | POST | /messages/send |
| `mobilemonkey_create_broadcast` | Create a broadcast campaign | POST | /broadcasts |

---

## Tool Details

### mobilemonkey_list_keywords

**What it does**: Returns all keywords configured in your Mobilemonkey account. Keywords are triggers that initiate automated responses.

**When to use**: Browse available keywords to understand what triggers are set up.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page

**Example LLM prompt**: "Show me all keywords configured in Mobilemonkey"

---

### mobilemonkey_get_keyword

**What it does**: Get details of a specific keyword by ID including its response configuration.

**When to use**: View the details and response configuration of a specific keyword.

**Arguments**:
- `id` (required): Unique identifier of the keyword

**Example LLM prompt**: "Show me the keyword with ID keyword_123"

---

### mobilemonkey_create_keyword

**What it does**: Create a new keyword for automated responses. Keywords trigger specific flows or messages when subscribers send matching text.

**When to use**: Set up new automated triggers for subscriber interactions.

**Arguments**:
- `keyword` (required): The keyword text that triggers the response
- `response` (required): The automated response message
- `flow_id` (optional): Optional flow ID to attach to this keyword

**Example LLM prompt**: "Create a keyword 'HELLO' that responds with 'Hi there! How can I help you?'"

---

### mobilemonkey_list_flows

**What it does**: Retrieve all chat flows in your Mobilemonkey account. Flows define the conversation logic and automated responses.

**When to use**: Browse available flows to understand automation sequences.

**Arguments**:
- `status` (optional): Filter by flow status (active, paused, archived)
- `page` (optional): Page number for pagination

**Example LLM prompt**: "List all active flows in Mobilemonkey"

---

### mobilemonkey_get_flow

**What it does**: Get details of a specific flow including steps and configuration.

**When to use**: Understand the conversation logic of a flow before triggering or modifying.

**Arguments**:
- `id` (required): Unique identifier of the flow

**Example LLM prompt**: "Show me the flow with ID flow_456"

---

### mobilemonkey_create_flow

**What it does**: Create a new chat flow with specified steps and logic. Flows automate conversations based on user interactions.

**When to use**: Set up new automated conversation sequences.

**Arguments**:
- `name` (required): Name of the flow
- `steps` (optional): Array of flow steps
- `trigger` (required): What triggers this flow (keyword, event, etc.)

**Example LLM prompt**: "Create a flow called 'Welcome Flow' triggered by new subscriber"

---

### mobilemonkey_list_subscribers

**What it does**: Retrieve all subscribers in your Mobilemonkey account. Subscribers are users who have opted in to receive messages.

**When to use**: Browse your audience for targeted messaging or analysis.

**Arguments**:
- `status` (optional): Filter by subscriber status (active, unsubscribed)
- `tag` (optional): Filter by subscriber tag
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page

**Example LLM prompt**: "Show me all active subscribers"

---

### mobilemonkey_get_subscriber

**What it does**: Get details of a specific subscriber including conversation history.

**When to use**: Understand subscriber interactions and preferences before sending targeted messages.

**Arguments**:
- `id` (required): Unique identifier of the subscriber

**Example LLM prompt**: "Show me details for subscriber sub_789"

---

### mobilemonkey_send_message

**What it does**: Send a message to a specific subscriber. Use this for direct, one-to-one messaging outside of flows.

**When to use**: Send personalized messages to individual subscribers.

**Arguments**:
- `subscriber_id` (required): ID of the subscriber to send the message to
- `message` (required): The message content to send
- `media_url` (optional): Optional URL of media to attach

**Example LLM prompt**: "Send a message to subscriber sub_789 saying 'Thanks for your purchase!'"

---

### mobilemonkey_create_broadcast

**What it does**: Create a new broadcast message to send to multiple subscribers. Broadcasts allow mass messaging to selected audiences.

**When to use**: Set up marketing campaigns or announcements.

**Arguments**:
- `name` (required): Name for this broadcast campaign
- `message` (required): The broadcast message content
- `recipient_filter` (optional): Filter criteria for recipients
- `scheduled_at` (optional): ISO 8601 timestamp to schedule the broadcast

**Example LLM prompt**: "Create a broadcast campaign called 'Spring Sale' with message 'Check out our spring collection!'"

---

## Mobilemonkey API Reference

These tools use the Mobilemonkey API. See official docs for full details:
- https://docs.mobilemonkey.com
- Rate limits: Check your plan limits
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format

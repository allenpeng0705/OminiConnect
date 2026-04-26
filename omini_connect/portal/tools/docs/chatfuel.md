# ChatFuel Tools

Provider: `chatfuel` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the ChatFuel API. They allow AI agents to manage chatbot flows, subscribers, broadcasts, attributes, and plugin settings in your ChatFuel account.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `flows_read`, `flows_write`, `subscribers_read`, `subscribers_write`, `broadcasts_read`, `broadcasts_write`, `attributes_read`, `plugins_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `chatfuel_list_flows` | List all chatbot flows | GET | /flows |
| `chatfuel_get_flow` | Get details of a specific flow | GET | /flows/{flow_id} |
| `chatfuel_create_flow` | Create a new chatbot flow | POST | /flows |
| `chatfuel_list_subscribers` | List chatbot subscribers | GET | /subscribers |
| `chatfuel_get_subscriber` | Get details of a specific subscriber | GET | /subscribers/{subscriber_id} |
| `chatfuel_update_subscriber_attribute` | Update subscriber attributes/tags | PUT | /subscribers/{subscriber_id}/attributes |
| `chatfuel_create_broadcast` | Create a broadcast campaign | POST | /broadcasts |
| `chatfuel_get_broadcast_stats` | Get broadcast statistics | GET | /broadcasts/{broadcast_id}/stats |
| `chatfuel_list_attributes` | List custom subscriber attributes | GET | /attributes |
| `chatfuel_get_plugin_settings` | Get plugin configuration | GET | /plugins/{plugin_id}/settings |

---

## Tool Details

### chatfuel_list_flows

**What it does**: Returns a paginated list of all chatbot flows in your ChatFuel account.

**When to use**: Browse available flows, find flows by name, or understand your chatbot structure.

**Arguments**:
- `page` (optional): default 1
- `limit` (optional, max 100): default 20

**Example LLM prompt**: "List all flows in the chatbot"

---

### chatfuel_get_flow

**What it does**: Gets full details of a specific flow including blocks, elements, and trigger configuration.

**When to use**: Understand how a flow works before modifying or debugging it.

**Arguments**:
- `flow_id` (required): The flow ID

**Example LLM prompt**: "Show me the onboarding flow configuration"

---

### chatfuel_create_flow

**What it does**: Creates a new chatbot flow with specified triggers and elements.

**When to use**: Build new conversation paths, add welcome messages, or create keyword-triggered responses.

**Arguments**:
- `name` (required): Flow name
- `description` (optional): Flow description
- `trigger` (optional): `keyword`, `exact`, `contains`, `regex`
- `trigger_value` (optional): Trigger keyword or pattern

**Example LLM prompt**: "Create a new flow called 'Product Inquiry' triggered by keyword 'product'"

---

### chatfuel_list_subscribers

**What it does**: Lists all subscribers who have interacted with your chatbot with optional status filtering.

**When to use**: See your user base, find active users, or review unsubscribed contacts.

**Arguments**:
- `status` (optional): `active`, `unsubscribed`, `all` — default `all`
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "List all active subscribers"

---

### chatfuel_get_subscriber

**What it does**: Gets detailed subscriber info including custom attributes, tags, and conversation history.

**When to use**: Understand a user's preferences, see their past interactions, or personalize responses.

**Arguments**:
- `subscriber_id` (required): The subscriber ID

**Example LLM prompt**: "Show me subscriber xyz's attributes and tags"

---

### chatfuel_update_subscriber_attribute

**What it does**: Updates custom attributes or adds/removes tags for a subscriber.

**When to use**: Store user preferences, track behavior, or segment users for targeted broadcasts.

**Arguments**:
- `subscriber_id` (required): The subscriber ID
- `attributes` (optional): Object with key-value attribute pairs
- `tags` (optional): Array of tags to add or update

**Example LLM prompt**: "Update subscriber xyz with attribute 'plan: premium' and tag 'vip'"

---

### chatfuel_create_broadcast

**What it does**: Creates a broadcast message to send to subscribers or specific segments.

**When to use**: Send marketing campaigns, announcements, or targeted promotions.

**Arguments**:
- `name` (required): Broadcast name
- `message` (required): Message object with `type` and `content`
- `segment` (optional): Targeting segment definition
- `schedule` (optional): Scheduling with `type` and `send_at`

**Example LLM prompt**: "Create a broadcast called 'Spring Sale' with a text message to all active subscribers"

---

### chatfuel_get_broadcast_stats

**What it does**: Gets delivery and engagement statistics for a broadcast campaign.

**When to use**: Measure campaign effectiveness, track open rates, or optimize future broadcasts.

**Arguments**:
- `broadcast_id` (required): The broadcast ID

**Example LLM prompt**: "What are the open and click rates for the Spring Sale broadcast?"

---

### chatfuel_list_attributes

**What it does**: Lists all custom attributes configured for storing subscriber data.

**When to use**: See what data fields are available for segmentation or personalization.

**Arguments**:
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "What custom attributes do we have for subscribers?"

---

### chatfuel_get_plugin_settings

**What it does**: Gets configuration settings for a ChatFuel plugin like live chat or customer chat.

**When to use**: Review or modify plugin appearance, behavior, or integration settings.

**Arguments**:
- `plugin_id` (required): The plugin ID

**Example LLM prompt**: "Show me the settings for the live chat plugin"

---

## ChatFuel API Reference

These tools use the ChatFuel API. See official docs for full details:
- https://docs.chatfuel.com/
- Rate limits: Varies by plan
- Pagination: Use `page` and `limit` parameters
- Broadcasts: Support immediate or scheduled delivery
- Segments: Can target by tags or attribute values

# Manychat Tools

Provider: `manychat` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Manychat is a Messenger marketing platform that enables businesses to create chatbot experiences for Facebook Messenger, Instagram, and other channels. These tools allow AI agents to manage subscribers, flows, broadcasts, and tags.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `subscribers`, `flows`, `broadcasts`, `tags`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `manychat_list_subscribers` | List all subscribers | GET | /subscribers |
| `manychat_get_subscriber` | Get a specific subscriber | GET | /subscribers/{id} |
| `manychat_create_subscriber` | Create a new subscriber | POST | /subscribers |
| `manychat_list_flows` | List all flows | GET | /flows |
| `manychat_get_flow` | Get a specific flow | GET | /flows/{id} |
| `manychat_start_flow` | Start a flow for a subscriber | POST | /flows/{id}/start |
| `manychat_create_broadcast` | Create a broadcast campaign | POST | /broadcasts |
| `manychat_send_broadcast` | Send a broadcast immediately | POST | /broadcasts/send |
| `manychat_list_tags` | List all tags | GET | /tags |
| `manychat_add_tag` | Add tags to a subscriber | POST | /subscribers/{id}/tags |

---

## Tool Details

### manychat_list_subscribers

**What it does**: Retrieve all subscribers in your Manychat account. Subscribers are users who have connected with your Facebook Messenger bot.

**When to use**: Browse your audience for targeted messaging or analysis.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page
- `tag_id` (optional): Filter by tag ID
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "Show me all active subscribers in Manychat"

---

### manychat_get_subscriber

**What it does**: Get detailed information about a specific subscriber including contact info, tags, and conversation data.

**When to use**: Understand a subscriber's history and preferences before taking action.

**Arguments**:
- `id` (required): Unique identifier of the subscriber

**Example LLM prompt**: "Show me details for subscriber sub_123"

---

### manychat_create_subscriber

**What it does**: Create a new subscriber in your Manychat account. Used to add users programmatically to your Messenger audience.

**When to use**: Add subscribers from external sources or systems.

**Arguments**:
- `first_name` (required): First name of the subscriber
- `last_name` (optional): Last name of the subscriber
- `phone` (optional): Phone number of the subscriber
- `email` (optional): Email address of the subscriber
- `tags` (optional): Array of tag IDs to apply

**Example LLM prompt**: "Create a new subscriber named John Doe with email john@example.com"

---

### manychat_list_flows

**What it does**: Retrieve all flows in your Manychat account. Flows define automated conversation sequences.

**When to use**: Browse available automation sequences.

**Arguments**:
- `status` (optional): Filter by flow status
- `page` (optional): Page number for pagination

**Example LLM prompt**: "List all flows in Manychat"

---

### manychat_get_flow

**What it does**: Get detailed information about a specific flow. Use this to understand the flow logic and steps.

**When to use**: Review automation logic before triggering or modifying flows.

**Arguments**:
- `id` (required): Unique identifier of the flow

**Example LLM prompt**: "Show me the flow with ID flow_456"

---

### manychat_start_flow

**What it does**: Trigger a flow for a specific subscriber. Use this to programmatically start an automated sequence.

**When to use**: Initiate an automation sequence for a specific subscriber.

**Arguments**:
- `flow_id` (required): ID of the flow to start
- `subscriber_id` (required): ID of the subscriber to start the flow for
- `tags` (optional): Optional tags to apply when starting flow

**Example LLM prompt**: "Start the welcome flow for subscriber sub_789"

---

### manychat_create_broadcast

**What it does**: Create a new broadcast campaign. Broadcasts can be scheduled or sent immediately to filtered audiences.

**When to use**: Set up marketing campaigns or announcements.

**Arguments**:
- `title` (required): Title of the broadcast campaign
- `message` (required): Message content to broadcast
- `filter` (optional): Filter criteria for recipients
- `scheduled_at` (optional): ISO 8601 timestamp for scheduled sending

**Example LLM prompt**: "Create a broadcast campaign called 'Weekly Update'"

---

### manychat_send_broadcast

**What it does**: Immediately send a broadcast to specified recipients. Use this for time-sensitive mass messaging.

**When to use**: Send urgent announcements or time-sensitive offers.

**Arguments**:
- `message` (required): Message content to broadcast
- `subscriber_ids` (required): Array of subscriber IDs to send to

**Example LLM prompt**: "Send a broadcast to subscribers sub_1, sub_2, and sub_3"

---

### manychat_list_tags

**What it does**: Retrieve all tags in your Manychat account. Tags are used to categorize and segment subscribers.

**When to use**: View available tags for segmentation or filtering.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page

**Example LLM prompt**: "Show me all tags in Manychat"

---

### manychat_add_tag

**What it does**: Add one or more tags to a subscriber. Use tags to segment and categorize your audience.

**When to use**: Label subscribers for future targeting or segmentation.

**Arguments**:
- `subscriber_id` (required): ID of the subscriber to add tags to
- `tags` (required): Array of tag IDs to add

**Example LLM prompt**: "Add tag premium_customer to subscriber sub_123"

---

## Manychat API Reference

These tools use the Manychat API. See official docs for full details:
- https://api.manychat.com
- Rate limits: Check your plan limits
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format

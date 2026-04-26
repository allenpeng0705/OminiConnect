# Braze Tools

Provider: `braze` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Braze REST API. They allow AI agents to manage users, campaigns, content cards, segments, and track events for the authenticated account.

## Authentication

**Nango (API Key)**:
- User provides Braze API key and instance URL
- Token stored in Nango, accessed via `connection_ref`
- Instance URL format: `iad-02.braze.com` (REST API URL: `https://rest.{instance}`)
- Headers: `Authorization: Bearer {apiKey}` set automatically

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `braze_list_users` | List users in Braze | GET | /users/list |
| `braze_get_user` | Get details of a specific user | GET | /users/{user_id} |
| `braze_track_event` | Track a custom event for a user | POST | /events/track |
| `braze_list_campaigns` | List campaigns in Braze | GET | /campaigns/list |
| `braze_get_campaign` | Get details of a specific campaign | GET | /campaigns/{campaign_id} |
| `braze_create_campaign` | Create a new campaign | POST | /campaigns |
| `braze_list_content_cards` | List content cards | GET | /content_cards/list |
| `braze_get_content_card` | Get details of a specific content card | GET | /content_cards/{card_id} |
| `braze_list_segments` | List segments in Braze | GET | /segments/list |
| `braze_get_segment` | Get details of a specific segment | GET | /segments/{segment_id} |

---

## Tool Details

### braze_list_users

**What it does**: List users in Braze with external ID, email, and device information.

**When to use**: Find users, search for specific profiles, or review user base.

**Arguments**:
- `limit` (optional, max 100): default 50
- `offset` (optional): default 0

**Example LLM prompt**: "List users in my Braze account"

---

### braze_get_user

**What it does**: Get details of a specific user including attributes, events, and purchase history.

**When to use**: Check user profile, view engagement history, or verify user data.

**Arguments**:
- `user_id` (required): User ID (external_id or braze_id)

**Example LLM prompt**: "Get details for user user123@example.com"

---

### braze_track_event

**What it does**: Track a custom event for a user for segmentation and analytics.

**When to use**: Record user actions like purchases, page views, or custom behaviors.

**Arguments**:
- `external_id` (required): User external ID
- `event_name` (required): Name of the event
- `time` (optional): Event timestamp (ISO 8601)
- `properties` (optional): Event properties

**Example LLM prompt**: "Track a 'purchase_complete' event for user user123@example.com"

---

### braze_list_campaigns

**What it does**: List all campaigns with name, status, and performance metrics.

**When to use**: Review campaign performance or find campaigns to modify.

**Arguments**:
- `limit` (optional, max 100): default 100
- `offset` (optional): default 0

**Example LLM prompt**: "List all campaigns in my Braze account"

---

### braze_get_campaign

**What it does**: Get details of a specific campaign including settings, audience, and statistics.

**When to use**: Check campaign details, view performance, or verify settings.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign camp_abc123"

---

### braze_create_campaign

**What it does**: Create a new campaign with audience, content, and delivery schedule.

**When to use**: Launch new marketing campaigns, newsletters, or promotional messages.

**Arguments**:
- `name` (required): Campaign name
- `description` (optional): Campaign description
- `channel` (required): Channel: email, push, in_app_message
- `segment_id` (optional): Target segment ID
- `content` (optional): Message content

**Example LLM prompt**: "Create an email campaign called 'Spring Sale' for segment seg_xyz"

---

### braze_list_content_cards

**What it does**: List all content cards with title, message, and status.

**When to use**: Review available content cards or find cards to update.

**Arguments**:
- `limit` (optional, max 100): default 100
- `offset` (optional): default 0

**Example LLM prompt**: "List all content cards"

---

### braze_get_content_card

**What it does**: Get details of a specific content card including message, images, and performance data.

**When to use**: Check card details or view engagement metrics.

**Arguments**:
- `card_id` (required): Content card ID

**Example LLM prompt**: "Get details for content card card_abc123"

---

### braze_list_segments

**What it does**: List all segments with name, size, and filter criteria.

**When to use**: Find audience segments for campaigns or review segment sizes.

**Arguments**:
- `limit` (optional, max 100): default 100
- `offset` (optional): default 0

**Example LLM prompt**: "List all segments in my Braze account"

---

### braze_get_segment

**What it does**: Get details of a specific segment including name, user count, and filters.

**When to use**: Check segment definition, verify audience size, or review filter criteria.

**Arguments**:
- `segment_id` (required): Segment ID

**Example LLM prompt**: "Get details for segment seg_xyz789"

---

## Braze API Reference

These tools use the Braze REST API. See official docs for full details:
- https://www.braze.com/docs/api/basics
- Rate limits: Vary by plan
- Pagination: Use `limit` and `offset` parameters
- All dates: ISO 8601 format (UTC)

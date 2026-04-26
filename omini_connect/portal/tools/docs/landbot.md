# Landbot Tools

Provider: `landbot` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Landbot is a chatbot builder that enables businesses to create conversational experiences across web, WhatsApp, and other channels. These tools allow AI agents to manage conversations, leads, settings, and analytics.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations`, `leads`, `settings`, `analytics`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `landbot_list_conversations` | List all conversations | GET | /conversations |
| `landbot_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `landbot_create_conversation` | Create a new conversation | POST | /conversations |
| `landbot_list_leads` | List all leads | GET | /leads |
| `landbot_get_lead` | Get a specific lead | GET | /leads/{id} |
| `landbot_export_leads` | Export leads data | POST | /leads/export |
| `landbot_list_settings` | List all settings | GET | /settings |
| `landbot_get_setting` | Get a specific setting | GET | /settings/{key} |
| `landbot_update_setting` | Update a setting | PUT | /settings/{key} |
| `landbot_get_analytics` | Get analytics data | GET | /analytics |

---

## Tool Details

### landbot_list_conversations

**What it does**: Retrieve all conversations in your Landbot account. Conversations are interactions between users and your chatbots.

**When to use**: Browse conversation history for analysis or follow-up.

**Arguments**:
- `status` (optional): Filter by status (open, closed, archived)
- `bot_id` (optional): Filter by bot ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page

**Example LLM prompt**: "Show me all open conversations"

---

### landbot_get_conversation

**What it does**: Get details of a specific conversation including messages. Use this to review the full conversation history.

**When to use**: Review what was discussed in a specific conversation.

**Arguments**:
- `id` (required): Unique identifier of the conversation

**Example LLM prompt**: "Show me the conversation with ID conv_123"

---

### landbot_create_conversation

**What it does**: Create a new conversation manually. Useful for initiating conversations programmatically.

**When to use**: Start a new conversation from an external trigger or system.

**Arguments**:
- `bot_id` (required): ID of the bot to associate with
- `customer_id` (required): ID of the customer
- `channel` (optional): Channel type (web, whatsapp, etc.)

**Example LLM prompt**: "Create a new conversation for customer cust_456 on whatsapp"

---

### landbot_list_leads

**What it does**: Retrieve all leads collected by your Landbot bots. Leads are user information captured through chatbot interactions.

**When to use**: Browse and analyze leads captured through your chatbots.

**Arguments**:
- `bot_id` (optional): Filter by bot ID
- `start_date` (optional): Filter by start date (ISO 8601)
- `end_date` (optional): Filter by end date (ISO 8601)
- `page` (optional): Page number for pagination

**Example LLM prompt**: "Show me all leads from the last week"

---

### landbot_get_lead

**What it does**: Get details of a specific lead. Includes contact information and interaction data.

**When to use**: Review individual lead information for sales follow-up.

**Arguments**:
- `id` (required): Unique identifier of the lead

**Example LLM prompt**: "Show me the lead with ID lead_789"

---

### landbot_export_leads

**What it does**: Export leads data in CSV or JSON format. Use this to download lead data for external analysis.

**When to use**: Export leads for CRM integration or external analysis.

**Arguments**:
- `bot_id` (required): ID of the bot to export leads from
- `format` (required): Export format (csv, json)
- `date_from` (optional): Start date for export filter
- `date_to` (optional): End date for export filter

**Example LLM prompt**: "Export all leads from bot bot_123 as CSV"

---

### landbot_list_settings

**What it does**: Retrieve all settings for your Landbot bots. Settings control bot behavior and configuration.

**When to use**: View bot configuration before making changes.

**Arguments**:
- `bot_id` (optional): Filter by bot ID

**Example LLM prompt**: "Show me all settings for bot bot_123"

---

### landbot_get_setting

**What it does**: Get a specific setting value. Use this to retrieve individual configuration values.

**When to use**: Check a specific setting before modifying it.

**Arguments**:
- `key` (required): Setting key name
- `bot_id` (optional): Bot ID to get setting for

**Example LLM prompt**: "Get the greeting_message setting for bot bot_123"

---

### landbot_update_setting

**What it does**: Update a specific setting value. Use this to modify bot configuration programmatically.

**When to use**: Change bot settings without accessing the dashboard.

**Arguments**:
- `key` (required): Setting key name
- `value` (required): New value for the setting
- `bot_id` (optional): Bot ID to update setting for

**Example LLM prompt**: "Update the greeting_message setting to 'Hello! How can I help?'"

---

### landbot_get_analytics

**What it does**: Retrieve analytics data for your Landbot bots. Analytics include engagement metrics, conversion rates, and user behavior.

**When to use**: Analyze bot performance and user engagement.

**Arguments**:
- `bot_id` (required): Bot ID to get analytics for
- `start_date` (required): Start date (ISO 8601)
- `end_date` (required): End date (ISO 8601)
- `metrics` (optional): Specific metrics to retrieve

**Example LLM prompt**: "Show me analytics for bot bot_123 for the last 30 days"

---

## Landbot API Reference

These tools use the Landbot API. See official docs for full details:
- https://developers.landbot.io
- Rate limits: Check your plan limits
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format

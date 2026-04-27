# Drift Tools

Provider: `drift` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Drift REST API. They allow AI agents to manage conversations, contacts, bots, campaigns, and reports in your Drift workspace. Drift is a conversational marketing platform that helps businesses connect with leads through chat, bots, and targeted campaigns.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Drift
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation_read`, `conversation_write`, `contact_read`, `bot_read`, `campaign_read`, `analytics_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `drift_list_conversations` | List conversations | GET | /conversations |
| `drift_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `drift_send_message` | Send a message | POST | /messages |
| `drift_list_contacts` | List contacts | GET | /contacts |
| `drift_get_contact` | Get a specific contact | GET | /contacts/{id} |
| `drift_list_bots` | List bots | GET | /bots |
| `drift_get_bot` | Get a specific bot | GET | /bots/{id} |
| `drift_list_campaigns` | List campaigns | GET | /campaigns |
| `drift_get_campaign` | Get a specific campaign | GET | /campaigns/{id} |
| `drift_get_report` | Get a report | GET | /reports/{type} |

---

## Tool Details

### drift_list_conversations

**What it does**: Returns a paginated list of conversations with their status, participants, and latest message.

**When to use**: Monitor active sales conversations or review closed deals.

**Arguments**:
- `status` (optional): `open`, `closed`, `all` — default `open`
- `limit` (optional): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "Show me all open conversations"

---

### drift_get_conversation

**What it does**: Gets full details of a specific conversation including messages, participants, and history.

**When to use**: Review conversation history before following up with a lead.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Show me conversation #12345"

---

### drift_send_message

**What it does**: Sends a message in a conversation or starts a new conversation with a contact. Use this to proactively reach out to leads or respond to incoming messages.

**When to use**: Reply to a lead or start a new conversation with a contact.

**Arguments**:
- `conversation_id` (optional): Conversation ID to send message to
- `contact_id` (optional): Contact ID to start new conversation with
- `message` (required): Message content
- `type` (optional): `chat` or `email` — default `chat`

**Example LLM prompt**: "Send a message to conversation #12345 saying 'Thanks for your interest'"

---

### drift_list_contacts

**What it does**: Lists all contacts in your Drift workspace. Returns contact profiles including name, email, company, and activity.

**When to use**: Get a list of all leads and customers in the system.

**Arguments**:
- `limit` (optional): default 25
- `offset` (optional): default 0
- `email` (optional): Filter by email address

**Example LLM prompt**: "List all contacts"

---

### drift_get_contact

**What it does**: Gets full contact profile including custom attributes, conversation history, and activity.

**When to use**: Review lead profile before reaching out or during a conversation.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Show me contact #67890"

---

### drift_list_bots

**What it does**: Lists all bots in your Drift workspace. Returns bot names, statuses, and configuration details.

**When to use**: See available bots for routing or check bot configurations.

**Arguments**:
- `limit` (optional): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "List all bots in the workspace"

---

### drift_get_bot

**What it does**: Gets full bot configuration including settings, triggers, and conversation flows.

**When to use**: Review bot configuration or understand how it qualifies leads.

**Arguments**:
- `id` (required): Bot ID

**Example LLM prompt**: "Show me bot #42"

---

### drift_list_campaigns

**What it does**: Lists all campaigns in your Drift workspace. Returns campaign names, statuses, and targeting rules.

**When to use**: See active campaigns or check campaign performance.

**Arguments**:
- `status` (optional): `active`, `paused`, `all` — default `all`
- `limit` (optional): default 25
- `offset` (optional): default 0

**Example LLM prompt**: "List all active campaigns"

---

### drift_get_campaign

**What it does**: Gets full campaign details including targeting, messaging, and scheduling.

**When to use**: Review campaign configuration or performance metrics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Show me campaign #789"

---

### drift_get_report

**What it does**: Gets analytics report for Drift conversations and campaigns. Returns metrics including conversation volume, response times, and conversion rates.

**When to use**: Analyze sales performance or campaign effectiveness.

**Arguments**:
- `type` (required): `conversations`, `campaigns`, or `overview`
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get conversation report for the last 30 days"

---

## Drift API Notes

- **Sales Focus**: Drift is designed for sales teams to convert leads through conversations
- **Bot Automation**: Bots qualify leads and route conversations to the right reps
- **Campaign Sequences**: Outbound messaging campaigns automate follow-ups
- **Contact Organization**: Contacts can be grouped by organization for account-based sales
- **Pagination**: Default limit is 25, use offset for pagination

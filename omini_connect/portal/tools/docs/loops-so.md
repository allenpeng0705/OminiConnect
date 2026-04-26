# Loops.so Tools

Provider: `loops-so` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Loops.so API. They allow AI agents to manage contacts, campaigns, and send transactional emails. **Requires Loops.so API key.**

## Authentication

**Nango API_KEY**:
- User provides Loops.so API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `authorization: Bearer ${apiKey}`
- Base URL: `https://app.loops.so/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `loops_list_contacts` | List all contacts | GET | /v1/contacts |
| `loops_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `loops_create_contact` | Create a contact | POST | /v1/contacts |
| `loops_update_contact` | Update a contact | PUT | /v1/contacts/{contactId} |
| `loops_delete_contact` | Delete a contact | DELETE | /v1/contacts/{contactId} |
| `loops_list_campaigns` | List all campaigns | GET | /v1/campaigns |
| `loops_get_campaign` | Get campaign details | GET | /v1/campaigns/{campaignId} |
| `loops_send_transactional` | Send transactional email | POST | /v1/send |
| `loops_list_events` | List events | GET | /v1/events |
| `loops_create_event` | Create an event | POST | /v1/events |

---

## Tool Details

### loops_list_contacts

**What it does**: Lists all contacts in Loops.so with optional filters.

**When to use**: Find contacts, filter by segment or tags.

**Arguments**:
- `segment` (optional): Filter by segment
- `tags` (optional): Filter by tags (comma-separated)
- `query` (optional): Search query
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all contacts in the vip segment"

---

### loops_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, check contact properties.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact CTR-12345"

---

### loops_create_contact

**What it does**: Creates a new contact in Loops.so.

**When to use**: Add new subscribers, sync contacts from other systems.

**Arguments**:
- `email` (required): Contact email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `tags` (optional): Tags to assign
- `subscribed` (optional): Whether contact is subscribed (default true)

**Example LLM prompt**: "Create a contact for john@example.com with name John Doe"

---

### loops_update_contact

**What it does**: Updates an existing contact.

**When to use**: Update contact properties, change subscription status.

**Arguments**:
- `contactId` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `tags` (optional): Tags to assign
- `subscribed` (optional): Whether contact is subscribed

**Example LLM prompt**: "Update contact CTR-12345 to add the premium tag"

---

### loops_delete_contact

**What it does**: Deletes a contact from Loops.so.

**When to use**: Remove unsubscribed contacts, GDPR requests.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Delete contact CTR-12345"

---

### loops_list_campaigns

**What it does**: Lists all email campaigns in Loops.so.

**When to use**: View campaigns, check campaign status.

**Arguments**:
- `status` (optional): Filter by status (draft, scheduled, sending, sent)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all sent campaigns"

---

### loops_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign performance, get campaign content.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign CMP-12345"

---

### loops_send_transactional

**What it does**: Sends a transactional email to a contact.

**When to use**: Send password resets, order confirmations, notifications.

**Arguments**:
- `email` (required): Recipient email
- `transactional_id` (required): Transactional email ID
- `data_variables` (optional): Variables to replace in template

**Example LLM prompt**: "Send password reset email to john@example.com"

---

### loops_list_events

**What it does**: Lists events for contacts (opens, clicks, etc.).

**When to use**: Track email engagement, analyze customer behavior.

**Arguments**:
- `contact_id` (optional): Contact ID
- `type` (optional): Event type (opened, clicked, bounced, etc.)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all events for contact CTR-12345"

---

### loops_create_event

**What it does**: Tracks a custom event for a contact.

**When to use**: Track website actions, CRM events, behavioral data.

**Arguments**:
- `email` (required): Contact email
- `event_name` (required): Event name
- `created_at` (optional): Event timestamp (ISO 8601)

**Example LLM prompt**: "Track that john@example.com completed onboarding"

---

## Loops.so Notes

- **Transactional email**: Requires pre-configured transactional email template
- **Events**: Custom events for segmentation and automation
- **Tags**: Organize contacts into segments
- **Rate limits**: Check API limits for your plan

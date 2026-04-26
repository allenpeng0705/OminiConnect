# Emarsys Tools

Provider: `emarsys` | Engine: `nango` | Auth: SIGNATURE (WSSE) via Nango

## Overview

These tools wrap the Emarsys API. They allow AI agents to manage contacts, send campaigns, trigger events, and retrieve marketing analytics. Emarsys is a marketing automation platform for enterprise B2C brands.

## Authentication

**Nango WSSE**:
- User provides Emarsys API credentials (ID + Secret) via Nango Connect
- Nango generates WSSE token for authentication
- Token expires every 5 minutes (300000ms)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `emarsys_get_settings` | Get account settings | GET | /api/v2/settings |
| `emarsys_list_contacts` | List contacts | GET | /api/v2/contacts |
| `emarsys_get_contact` | Get contact details | GET | /api/v2/contacts/{id} |
| `emarsys_create_contact` | Create a new contact | POST | /api/v2/contacts |
| `emarsys_update_contact` | Update contact details | PUT | /api/v2/contacts/{id} |
| `emarsys_list_campaigns` | List campaigns | GET | /api/v2/campaigns |
| `emarsys_get_campaign` | Get campaign details | GET | /api/v2/campaigns/{id} |
| `emarsys_trigger_event` | Trigger a contact event | POST | /api/v2/events |
| `emarsys_get_aggregate_report` | Get campaign aggregate report | GET | /api/v2/reports/campaigns/{id}/aggregate |
| `emarsys_list_segments` | List segments | GET | /api/v2/segments |

---

## Tool Details

### emarsys_get_settings

**What it does**: Retrieves account settings and configuration.

**When to use**: Verify account configuration, check API access.

**Arguments**: None

**Example LLM prompt**: "Get my Emarsys account settings"

---

### emarsys_list_contacts

**What it does**: Lists contacts in the account with pagination.

**When to use**: Browse contacts, find specific contacts.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List contacts in my Emarsys account"

---

### emarsys_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, check contact data.

**Arguments**:
- `id` (required): Contact email or ID

**Example LLM prompt**: "Get details for contact john@example.com"

---

### emarsys_create_contact

**What it does**: Creates a new contact in the account.

**When to use**: Add new customers to marketing list.

**Arguments**:
- `email` (required): Contact email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a new contact for john@example.com"

---

### emarsys_update_contact

**What it does**: Updates an existing contact's details.

**When to use**: Modify contact information, update preferences.

**Arguments**:
- `id` (required): Contact email or ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Update contact john@example.com with new phone number"

---

### emarsys_list_campaigns

**What it does**: Lists all campaigns in the account.

**When to use**: Browse campaigns, find campaign IDs.

**Arguments**:
- `limit` (optional): Max results

**Example LLM prompt**: "List all my Emarsys campaigns"

---

### emarsys_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign status, get campaign configuration.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 123"

---

### emarsys_trigger_event

**What it does**: Triggers an event for a contact in automation.

**When to use**: Fire marketing automations, record user actions.

**Arguments**:
- `event_id` (required): Event ID
- `contact` (required): Contact email or ID
- `data` (optional): Event payload data

**Example LLM prompt**: "Trigger the 'purchase_complete' event for john@example.com"

---

### emarsys_get_aggregate_report

**What it does**: Gets aggregate report for a campaign.

**When to use**: Get campaign performance metrics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get the aggregate report for campaign 123"

---

### emarsys_list_segments

**What it does**: Lists all segments in the account.

**When to use**: Find segments for targeting, manage audience lists.

**Arguments**:
- `limit` (optional): Max results

**Example LLM prompt**: "List all Emarsys segments"

---

## Emarsys API Notes

- **Contact IDs**: Can be email address or numeric ID
- **Segments**: Used for targeted marketing campaigns
- **Events**: Trigger marketing automations based on user actions
- **Reports**: Campaign performance metrics available via reports endpoint
- **Rate Limits**: Check x-ratelimit-reset header for rate limit info

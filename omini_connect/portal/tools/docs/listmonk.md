# Listmonk Tools

Provider: `listmonk` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Listmonk API. They allow AI agents to manage lists, subscribers, campaigns, and templates. **Requires Listmonk credentials (API user + token).**

## Authentication

**Basic Auth via Nango**:
- User provides API user and token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{domain}/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `listmonk_list_lists` | List lists | GET | /api/lists |
| `listmonk_get_list` | Get list details | GET | /api/lists/{list_id} |
| `listmonk_list_subscribers` | List subscribers | GET | /api/subscribers |
| `listmonk_get_subscriber` | Get subscriber details | GET | /api/subscribers/{subscriber_id} |
| `listmonk_add_subscriber` | Add a subscriber | POST | /api/subscribers |
| `listmonk_list_campaigns` | List campaigns | GET | /api/campaigns |
| `listmonk_get_campaign` | Get campaign details | GET | /api/campaigns/{campaign_id} |
| `listmonk_list_templates` | List templates | GET | /api/templates |
| `listmonk_get_template` | Get template details | GET | /api/templates/{template_id} |
| `listmonk_list_forms` | List forms | GET | /api/forms |

---

## Tool Details

### listmonk_list_lists

**What it does**: Lists all lists.

**When to use**: View lists, find subscriber groups.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all lists in Listmonk"

---

### listmonk_get_list

**What it does**: Gets details for a specific list.

**When to use**: Get list information.

**Arguments**:
- `list_id` (required): List ID

**Example LLM prompt**: "Get details for list 12345"

---

### listmonk_list_subscribers

**What it does**: Lists all subscribers.

**When to use**: View subscribers, find contacts.

**Arguments**:
- `list_id` (optional): Filter by list ID
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all subscribers in Listmonk"

---

### listmonk_get_subscriber

**What it does**: Gets details for a specific subscriber.

**When to use**: Get subscriber information.

**Arguments**:
- `subscriber_id` (required): Subscriber ID

**Example LLM prompt**: "Get details for subscriber 67890"

---

### listmonk_add_subscriber

**What it does**: Adds a new subscriber.

**When to use**: Add contacts to lists.

**Arguments**:
- `email` (required): Email address
- `name` (optional): Subscriber name
- `list_ids` (optional): List IDs to subscribe to

**Example LLM prompt**: "Add subscriber john@example.com to list 12345"

---

### listmonk_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View campaigns, track emails.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all campaigns in Listmonk"

---

### listmonk_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign c1"

---

### listmonk_list_templates

**What it does**: Lists all templates.

**When to use**: View email templates.

**Arguments**: None

**Example LLM prompt**: "List all templates in Listmonk"

---

### listmonk_get_template

**What it does**: Gets details for a specific template.

**When to use**: Get template information.

**Arguments**:
- `template_id` (required): Template ID

**Example LLM prompt**: "Get details for template t1"

---

### listmonk_list_forms

**What it does**: Lists all forms.

**When to use**: View subscription forms.

**Arguments**: None

**Example LLM prompt**: "List all forms in Listmonk"

---

## Listmonk API Notes

- **Email Marketing**: Newsletter and marketing platform
- **Lists**: Subscriber lists
- **Subscribers**: Email contacts
- **Campaigns**: Email campaigns
- **Templates**: Email templates

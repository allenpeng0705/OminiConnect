# Mailjet Tools

Provider: `mailjet` | Engine: `nango` | Auth: BASIC via Nango (API Key as username)

## Overview

These tools wrap the Mailjet API. They allow AI agents to manage contacts, campaigns, and email statistics. **Requires Mailjet API key and secret.**

## Authentication

**Nango BASIC**:
- User provides Mailjet API Key as username via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.mailjet.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mailjet_list_contacts` | List all contacts | GET | /v3/REST/contact |
| `mailjet_get_contact` | Get contact details | GET | /v3/REST/contact/{email} |
| `mailjet_create_contact` | Create a contact | POST | /v3/REST/contact |
| `mailjet_update_contact` | Update a contact | PUT | /v3/REST/contact/{email} |
| `mailjet_list_campaigns` | List all campaigns | GET | /v3/REST/campaign |
| `mailjet_get_campaign` | Get campaign details | GET | /v3/REST/campaign/{id} |
| `mailjet_send_campaign` | Send a campaign | POST | /v3/send |
| `mailjet_list_lists` | List contact lists | GET | /v3/REST/contactslist |
| `mailjet_get_list` | Get list details | GET | /v3/REST/contactslist/{id} |
| `mailjet_list_stats` | Get statistics | GET | /v3/REST/message |

---

## Tool Details

### mailjet_list_contacts

**What it does**: Lists all contacts in Mailjet.

**When to use**: Find contacts, manage subscriber list.

**Arguments**:
- `limit` (optional): Max results (default 50, max 1000)
- `offset` (optional): Offset for pagination (default 0)
- `contact_slugs` (optional): Filter by contact slugs

**Example LLM prompt**: "List all contacts in Mailjet"

---

### mailjet_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, check subscription status.

**Arguments**:
- `email` (required): Contact email

**Example LLM prompt**: "Get details for john@example.com"

---

### mailjet_create_contact

**What it does**: Creates a new contact in Mailjet.

**When to use**: Add new subscribers to Mailjet.

**Arguments**:
- `email` (required): Contact email
- `name` (optional): Contact name
- `properties` (optional): Custom properties

**Example LLM prompt**: "Create a contact for john@example.com"

---

### mailjet_update_contact

**What it does**: Updates an existing contact.

**When to use**: Update contact properties, change names.

**Arguments**:
- `email` (required): Contact email
- `name` (optional): Contact name
- `properties` (optional): Custom properties

**Example LLM prompt**: "Update contact john@example.com with new properties"

---

### mailjet_list_campaigns

**What it does**: Lists all email campaigns in Mailjet.

**When to use**: View campaigns, check campaign status.

**Arguments**:
- `status` (optional): Filter by status (1=draft, 2=scheduled, 3=sent)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all sent campaigns"

---

### mailjet_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign details and settings.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign CMP-12345"

---

### mailjet_send_campaign

**What it does**: Sends a campaign to its recipients.

**When to use**: Send a prepared campaign.

**Arguments**:
- `campaign_id` (required): Campaign ID to send

**Example LLM prompt**: "Send campaign CMP-12345"

---

### mailjet_list_lists

**What it do**: Lists all contact lists in Mailjet.

**When to use**: Manage contact lists, find list IDs.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all contact lists"

---

### mailjet_get_list

**What it does**: Gets details of a specific contact list.

**When to use**: Check list details, subscriber count.

**Arguments**:
- `id` (required): List ID

**Example LLM prompt**: "Get details for list 12345"

---

### mailjet_list_stats

**What it does**: Gets email statistics from Mailjet.

**When to use**: Analyze email performance, track delivery.

**Arguments**:
- `from_ts` (optional): From timestamp
- `to_ts` (optional): To timestamp
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Get email statistics for the last 30 days"

---

## Mailjet Notes

- **API Key format**: 32-character hexadecimal string
- **Campaign status**: 1=draft, 2=scheduled, 3=sent
- **Contacts**: Global address book contacts
- **Lists**: Static lists for targeted campaigns
- **Rate limits**: Check your plan limits

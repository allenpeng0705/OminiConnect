# Listrak Tools

Provider: `listrak` | Engine: `nango` | Auth: OAUTH2_CC via Nango

## Overview

These tools wrap the Listrak API. They allow AI agents to manage contacts, segments, schemas, and email campaigns. **Requires Listrak OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials via Nango**:
- User provides Client ID and Client Secret
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.listrak.com/email`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `listrak_list_contacts` | List contacts | GET | /email/contacts |
| `listrak_get_contact` | Get contact details | GET | /email/contacts/{contact_id} |
| `listrak_add_contact` | Add a contact | POST | /email/contacts |
| `listrak_update_contact` | Update a contact | PUT | /email/contacts/{contact_id} |
| `listrak_list_schemas` | List schemas | GET | /email/schemas |
| `listrak_get_schema` | Get schema details | GET | /email/schemas/{schema_id} |
| `listrak_list_segments` | List segments | GET | /email/segments |
| `listrak_get_segment` | Get segment details | GET | /email/segments/{segment_id} |
| `listrak_list_email_campaigns` | List email campaigns | GET | /email/campaigns |
| `listrak_get_email_campaign` | Get email campaign details | GET | /email/campaigns/{campaign_id} |

---

## Tool Details

### listrak_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find contacts, view subscriber list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all contacts in Listrak"

---

### listrak_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### listrak_add_contact

**What it does**: Adds a new contact.

**When to use**: Add subscribers.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Add contact john@example.com to Listrak"

---

### listrak_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact data.

**Arguments**:
- `contact_id` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update contact 12345 with new name"

---

### listrak_list_schemas

**What it does**: Lists all schemas.

**When to use**: View contact data structure.

**Arguments**: None

**Example LLM prompt**: "List all schemas in Listrak"

---

### listrak_get_schema

**What it does**: Gets details for a specific schema.

**When to use**: Get schema information.

**Arguments**:
- `schema_id` (required): Schema ID

**Example LLM prompt**: "Get details for schema s1"

---

### listrak_list_segments

**What it does**: Lists all segments.

**When to use**: View segments, organize contacts.

**Arguments**: None

**Example LLM prompt**: "List all segments in Listrak"

---

### listrak_get_segment

**What it does**: Gets details for a specific segment.

**When to use**: Get segment information.

**Arguments**:
- `segment_id` (required): Segment ID

**Example LLM prompt**: "Get details for segment seg1"

---

### listrak_list_email_campaigns

**What it does**: Lists all email campaigns.

**When to use**: View campaigns, track emails.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all email campaigns in Listrak"

---

### listrak_get_email_campaign

**What it does**: Gets details for a specific email campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign c1"

---

## Listrak API Notes

- **Email Marketing**: Marketing automation platform
- **Contacts**: Subscriber and customer data
- **Schemas**: Contact data structures
- **Segments**: Grouped contacts for targeting
- **Email Campaigns**: Marketing email campaigns

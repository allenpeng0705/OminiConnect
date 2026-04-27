# Cyberimpact Tools

Provider: `cyberimpact` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Cyberimpact Email Marketing API. They allow AI agents to manage email campaigns, contacts, and templates. Cyberimpact is an email marketing platform for sending newsletters and promotional emails.

## Authentication

**Nango API_KEY**:
- User provides their Cyberimpact API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Format: JWT-like token with three parts separated by dots

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cyberimpact_list_campaigns` | List email campaigns | GET | /campaigns |
| `cyberimpact_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `cyberimpact_create_campaign` | Create a new campaign | POST | /campaigns |
| `cyberimpact_list_contacts` | List contacts | GET | /contacts |
| `cyberimpact_get_contact` | Get contact details | GET | /contacts/{id} |
| `cyberimpact_add_contact` | Add a new contact | POST | /contacts |
| `cyberimpact_list_lists` | List contact lists | GET | /lists |
| `cyberimpact_get_list` | Get list details | GET | /lists/{id} |
| `cyberimpact_list_templates` | List email templates | GET | /templates |
| `cyberimpact_get_template` | Get template details | GET | /templates/{id} |

---

## Tool Details

### cyberimpact_list_campaigns

**What it does**: Lists all email campaigns with optional status filtering.

**When to use**: Browse campaign history, find campaigns by status, track campaign performance.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by draft, scheduled, or sent

**Example LLM prompt**: "List all sent campaigns"

---

### cyberimpact_get_campaign

**What it does**: Gets detailed campaign information including stats and recipients.

**When to use**: Review campaign details, check send status, analyze campaign performance.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 123"

---

### cyberimpact_create_campaign

**What it does**: Creates a new email campaign with subject and template.

**When to use**: Create new marketing emails, schedule newsletters, set up promotional campaigns.

**Arguments**:
- `name` (required): Campaign name
- `subject` (required): Email subject line
- `template_id` (optional): Template ID to use
- `list_id` (optional): Contact list ID to send to

**Example LLM prompt**: "Create a campaign named 'Summer Sale' with subject 'Check out our deals'"

---

### cyberimpact_list_contacts

**What it does**: Lists all contacts with optional list filtering.

**When to use**: Browse contact database, find contacts in specific lists, manage subscribers.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `list_id` (optional): Filter by list ID

**Example LLM prompt**: "List all contacts"

---

### cyberimpact_get_contact

**What it does**: Gets detailed contact information including profile and activity.

**When to use**: Review contact details, check subscription status, view contact history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 456"

---

### cyberimpact_add_contact

**What it does**: Adds a new contact to Cyberimpact.

**When to use**: Subscribe new users, add leads to database, import contacts.

**Arguments**:
- `email` (required): Contact email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `list_id` (optional): List ID to add to

**Example LLM prompt**: "Add a new contact with email user@example.com"

---

### cyberimpact_list_lists

**What it does**: Lists all contact lists.

**When to use**: View available lists, manage segment groups, organize contacts.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all contact lists"

---

### cyberimpact_get_list

**What it does**: Gets detailed list information including member count.

**When to use**: Check list size, review list settings, verify list contents.

**Arguments**:
- `id` (required): List ID

**Example LLM prompt**: "Get details for list 789"

---

### cyberimpact_list_templates

**What it does**: Lists all available email templates.

**When to use**: Browse template library, find templates for campaigns, preview designs.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all email templates"

---

### cyberimpact_get_template

**What it does**: Gets detailed template information including content.

**When to use**: Preview template design, get template HTML, select template for campaign.

**Arguments**:
- `id` (required): Template ID

**Example LLM prompt**: "Get details for template 101"

---

## Cyberimpact API Notes

- **Email Marketing**: Platform for sending newsletters and promotional emails
- **Campaigns**: Individual email sends with tracking and analytics
- **Contacts**: Subscriber database with email and profile information
- **Lists**: Contact groups/segments for targeted sending
- **Templates**: Pre-designed email layouts for campaigns
- **Verification**: POST /ping confirms API key validity

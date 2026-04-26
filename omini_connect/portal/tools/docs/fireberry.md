# Fireberry Tools

Provider: `fireberry` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Fireberry marketing platform API. They allow AI agents to manage campaigns, leads, contacts, email delivery, and marketing analytics.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `campaigns:read`, `campaigns:write`, `leads:read`, `leads:write`, `contacts:read`, `emails:write`, `reports:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fireberry_list_campaigns` | Retrieve all marketing campaigns | GET | /campaigns |
| `fireberry_get_campaign` | Get details of a specific campaign | GET | /campaigns/{campaign_id} |
| `fireberry_create_campaign` | Create a new marketing campaign | POST | /campaigns |
| `fireberry_list_leads` | Retrieve all leads in pipeline | GET | /leads |
| `fireberry_get_lead` | Get details of a specific lead | GET | /leads/{lead_id} |
| `fireberry_create_lead` | Create a new lead | POST | /leads |
| `fireberry_list_contacts` | Retrieve all contacts | GET | /contacts |
| `fireberry_get_contact` | Get details of a specific contact | GET | /contacts/{contact_id} |
| `fireberry_send_email` | Send an email to contacts | POST | /emails/send |
| `fireberry_get_campaign_report` | Get campaign performance metrics | GET | /campaigns/{campaign_id}/report |

---

## Tool Details

### fireberry_list_campaigns

**What it does**: Returns all marketing campaigns in your account with optional filtering.

**When to use**: Browse campaigns before checking reports or sending communications.

**Arguments**:
- `status` (optional): Filter by status (`draft`, `active`, `paused`, `completed`)
- `type` (optional): Filter by campaign type (`email`, `sms`, `social`)
- `limit` (optional): Maximum number of campaigns to return

---

### fireberry_get_campaign

**What it does**: Gets detailed information about a specific campaign including targeting and content.

**When to use**: Review campaign configuration before sending or modifying.

**Arguments**:
- `campaign_id` (required): Unique identifier of the campaign

---

### fireberry_create_campaign

**What it does**: Creates a new marketing campaign with targeting and content settings.

**When to use**: Set up a new marketing initiative across channels.

**Arguments**:
- `name` (required): Campaign name
- `type` (required): Campaign type (`email`, `sms`, `social`)
- `target_segment` (optional): Target segment identifier
- `start_date` (optional): Campaign start date (ISO 8601)

---

### fireberry_list_leads

**What it does**: Retrieves all leads in your pipeline with optional filtering.

**When to use**: Review pipeline before prioritizing follow-ups.

**Arguments**:
- `status` (optional): Filter by lead status (`new`, `contacted`, `qualified`, `lost`)
- `source` (optional): Filter by lead source
- `limit` (optional): Maximum number of leads to return

---

### fireberry_get_lead

**What it does**: Gets detailed information about a specific lead including scoring and source.

**When to use**: Review lead details before follow-up or conversion.

**Arguments**:
- `lead_id` (required): Unique identifier of the lead

---

### fireberry_create_lead

**What it does**: Creates a new lead in the marketing pipeline.

**When to use**: Capture new leads from inquiries or campaigns.

**Arguments**:
- `name` (required): Lead name
- `email` (required): Lead email address
- `phone` (optional): Lead phone number
- `source` (optional): Lead source (`web`, `referral`, `event`)
- `score` (optional): Lead score (1-100)

---

### fireberry_list_contacts

**What it does**: Retrieves all contacts in your marketing database.

**When to use**: Browse contacts before sending emails or managing segments.

**Arguments**:
- `segment` (optional): Filter by segment name
- `tag` (optional): Filter by tag
- `limit` (optional): Maximum number of contacts to return

---

### fireberry_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Review contact details before personalized outreach.

**Arguments**:
- `contact_id` (required): Unique identifier of the contact

---

### fireberry_send_email

**What it does**: Sends a transactional or marketing email to specified recipients.

**When to use**: Send newsletters, promotions, or automated responses.

**Arguments**:
- `to` (required): Array of recipient email addresses
- `subject` (required): Email subject line
- `body` (required): Email body content (HTML supported)
- `campaign_id` (optional): Associated campaign ID

---

### fireberry_get_campaign_report

**What it does**: Gets performance metrics and analytics for a campaign.

**When to use**: Measure campaign effectiveness, open rates, and conversions.

**Arguments**:
- `campaign_id` (required): Campaign identifier
- `date_from` (optional): Start date for report period
- `date_to` (optional): End date for report period

---

## Fireberry API Reference

See official docs for full details on rate limits and pagination.

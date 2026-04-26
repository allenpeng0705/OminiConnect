# Icypeas Tools

Provider: `icypeas` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Icypeas API. They allow AI agents to search for emails, manage campaigns, enrich leads, and view statistics. Icypeas is a lead generation and email finding platform.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://app.icypeas.com/api

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `icypeas_email_search` | Email search | POST | /email-search |
| `icypeas_get_profile` | Get profile details | GET | /profile |
| `icypeas_list_campaigns` | List campaigns | GET | /campaigns |
| `icypeas_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `icypeas_list_enrichments` | List enrichments | GET | /enrichments |
| `icypeas_get_enrichment` | Get enrichment details | GET | /enrichments/{id} |
| `icypeas_list_leads` | List leads | GET | /leads |
| `icypeas_get_lead` | Get lead details | GET | /leads/{id} |
| `icypeas_list_stats` | List statistics | GET | /stats |
| `icypeas_list_credits` | List credits | GET | /credits |

---

## Tool Details

### icypeas_email_search

**What it does**: Searches for email addresses using Icypeas.

**When to use**: Find professional email addresses.

**Arguments**:
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `domain` (required): Company domain
- `company_name` (optional): Company name

**Example LLM prompt**: "Find email for John Smith at example.com"

---

### icypeas_get_profile

**What it does**: Gets detailed profile information from Icypeas.

**When to use**: Get professional profile data.

**Arguments**:
- `email` (required): Email address

**Example LLM prompt**: "Get profile for john@example.com"

---

### icypeas_list_campaigns

**What it does**: Lists all campaigns in Icypeas.

**When to use**: Browse outreach campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### icypeas_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: View campaign performance metrics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign with ID abc123"

---

### icypeas_list_enrichments

**What it does**: Lists all enrichments in Icypeas.

**When to use**: View enrichment status.

**Arguments**: None

**Example LLM prompt**: "List all enrichments"

---

### icypeas_get_enrichment

**What it does**: Gets detailed information about a specific enrichment.

**When to use**: View enrichment data.

**Arguments**:
- `id` (required): Enrichment ID

**Example LLM prompt**: "Get enrichment with ID enc456"

---

### icypeas_list_leads

**What it does**: Lists all leads in Icypeas.

**When to use**: Browse lead database.

**Arguments**: None

**Example LLM prompt**: "List all leads"

---

### icypeas_get_lead

**What it does**: Gets detailed information about a specific lead.

**When to use**: View lead profile and data.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get lead with ID led789"

---

### icypeas_list_stats

**What it does**: Lists usage statistics from Icypeas.

**When to use**: View API usage and quota.

**Arguments**: None

**Example LLM prompt**: "List API statistics"

---

### icypeas_list_credits

**What it does**: Lists credit balance from Icypeas.

**When to use**: Check available credits.

**Arguments**: None

**Example LLM prompt**: "List credit balance"

---

## Icypeas API Notes

- **API Base URL**: https://app.icypeas.com/api
- **Auth Mode**: API Key in Authorization header
- **Email Search**: Find professional email addresses
- **Profiles**: Detailed professional profiles
- **Campaigns**: Outreach campaign management
- **Enrichments**: Data enrichment requests
- **Leads**: Lead database
- **Stats**: API usage statistics
- **Credits**: Credit balance and usage

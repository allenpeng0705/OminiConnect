# Firefish Tools

Provider: `firefish` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Firefish API. They allow AI agents to manage contacts, deals, companies, and sales pipelines. Firefish is a CRM platform for small businesses.

## Authentication

**Nango OAuth2 CC**:
- Uses OAuth2 Client Credentials flow
- Client ID and Secret stored in Nango
- Token automatically refreshed by Nango
- Suitable for server-to-server integration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `firefish_list_contacts` | List contacts | GET | /api/v1/contacts |
| `firefish_get_contact` | Get contact details | GET | /api/v1/contacts/{id} |
| `firefish_create_contact` | Create a contact | POST | /api/v1/contacts |
| `firefish_update_contact` | Update contact details | PUT | /api/v1/contacts/{id} |
| `firefish_list_deals` | List deals | GET | /api/v1/deals |
| `firefish_get_deal` | Get deal details | GET | /api/v1/deals/{id} |
| `firefish_create_deal` | Create a deal | POST | /api/v1/deals |
| `firefish_list_companies` | List companies | GET | /api/v1/companies |
| `firefish_get_company` | Get company details | GET | /api/v1/companies/{id} |
| `firefish_get_stats` | Get CRM stats | GET | /api/v1/stats |

---

## Tool Details

### firefish_list_contacts

**What it does**: Lists all contacts in the CRM.

**When to use**: Browse contact database.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### firefish_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact abc123"

---

### firefish_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new leads to the CRM.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Create a contact for john@company.com"

---

### firefish_update_contact

**What it does**: Updates contact details.

**When to use**: Modify contact information.

**Arguments**:
- `id` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update contact abc123"

---

### firefish_list_deals

**What it does**: Lists all deals in the pipeline.

**When to use**: View sales pipeline, track deals.

**Arguments**:
- `stage` (optional): Filter by pipeline stage

**Example LLM prompt**: "List all deals"

---

### firefish_get_deal

**What it does**: Gets details of a specific deal.

**When to use**: View deal information, value.

**Arguments**:
- `id` (required): Deal ID

**Example LLM prompt**: "Get details for deal xyz789"

---

### firefish_create_deal

**What it does**: Creates a new deal.

**When to use**: Add new opportunities to pipeline.

**Arguments**:
- `title` (required): Deal title
- `value` (optional): Deal value
- `stage` (optional): Pipeline stage

**Example LLM prompt**: "Create a deal called 'Enterprise Sale' worth 50000"

---

### firefish_list_companies

**What it does**: Lists all companies.

**When to use**: Browse company database.

**Arguments**: None

**Example LLM prompt**: "List all companies"

---

### firefish_get_company

**What it does**: Gets details of a specific company.

**When to use**: View company information.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company def456"

---

### firefish_get_stats

**What it does**: Gets CRM statistics.

**When to use**: View pipeline metrics, sales performance.

**Arguments**: None

**Example LLM prompt**: "Get CRM stats"

---

## Firefish API Notes

- **OAuth2 CC**: Client credentials flow for server auth
- **Contacts**: Individual people in the CRM
- **Companies**: Organizations linked to contacts
- **Deals**: Sales opportunities in the pipeline
- **Stages**: Pipeline stages for deal progression
- **Stats**: Pipeline and sales metrics

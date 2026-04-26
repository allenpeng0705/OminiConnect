# nocrm Tools

Provider: `nocrm` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the nocrm.io CRM API. They allow AI agents to manage leads, companies, activities, and view the sales pipeline. **Requires nocrm.io API key authentication.**

## Authentication

**API Key**:
- User provides nocrm.io API key
- Key passed via `X-API-Key` header
- Base URL: `https://{subdomain}.nocrm.io`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nocrm_list_leads` | List leads/opportunities | GET | /api/v2/leads |
| `nocrm_get_lead` | Get lead details | GET | /api/v2/leads/{id} |
| `nocrm_create_lead` | Create new lead | POST | /api/v2/leads |
| `nocrm_update_lead` | Update lead | PUT | /api/v2/leads/{id} |
| `nocrm_list_companies` | List companies | GET | /api/v2/companies |
| `nocrm_get_company` | Get company details | GET | /api/v2/companies/{id} |
| `nocrm_list_activities` | List activities | GET | /api/v2/activities |
| `nocrm_create_activity` | Create activity | POST | /api/v2/activities |
| `nocrm_list_tags` | List tags | GET | /api/v2/tags |
| `nocrm_get_pipeline` | Get sales pipeline | GET | /api/v2/pipelines |

---

## Tool Details

### nocrm_list_leads

**What it does**: Lists all leads and opportunities in nocrm.

**When to use**: Browse leads, find opportunities, track sales pipeline.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Items per page (default 20)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all new leads this week"

---

### nocrm_get_lead

**What it does**: Gets detailed information for a specific lead.

**When to use**: View lead details, contact information, activity history.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get details for lead ID 12345"

---

### nocrm_create_lead

**What it does**: Creates a new lead in nocrm.

**When to use**: Add new prospects, create opportunities.

**Arguments**:
- `name` (required): Lead name
- `email` (optional): Lead email
- `company_id` (optional): Company ID
- `pipeline_id` (optional): Pipeline ID
- `step_id` (optional): Step ID

**Example LLM prompt**: "Create a new lead for John Doe at Acme Corp"

---

### nocrm_update_lead

**What it does**: Updates an existing lead.

**When to use**: Change lead status, update information.

**Arguments**:
- `id` (required): Lead ID
- `name` (optional): Lead name
- `status` (optional): Lead status

**Example LLM prompt**: "Update lead 12345 status to won"

---

### nocrm_list_companies

**What it does**: Lists all companies in nocrm.

**When to use**: Browse company database.

**Arguments**:
- `page` (optional): Page number

**Example LLM prompt**: "List all companies"

---

### nocrm_get_company

**What it does**: Gets detailed information for a specific company.

**When to use**: View company details, associated leads.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company ID 67890"

---

### nocrm_list_activities

**What it does**: Lists all activities (calls, emails, meetings).

**When to use**: View activity history, track interactions.

**Arguments**:
- `lead_id` (optional): Filter by lead ID
- `type` (optional): Filter by activity type

**Example LLM prompt**: "List all calls with lead 12345"

---

### nocrm_create_activity

**What it does**: Logs a new activity for a lead.

**When to use**: Record calls, meetings, emails.

**Arguments**:
- `lead_id` (required): Lead ID
- `type` (required): Activity type (call, email, meeting)
- `content` (required): Activity description

**Example LLM prompt**: "Log a call with lead 12345 about pricing"

---

### nocrm_list_tags

**What it does**: Lists all tags used in nocrm.

**When to use**: View tagging categories, segmentation.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### nocrm_get_pipeline

**What it does**: Gets the sales pipeline configuration.

**When to use**: View pipeline stages, understand sales process.

**Arguments**: None

**Example LLM prompt**: "Get the sales pipeline stages"

---

## nocrm Notes

- **Subdomain**: Your nocrm.io account subdomain
- **API Key format**: 48 character hexadecimal string
- **Pagination**: Use page and per_page for large result sets
- **Activity types**: call, email, meeting, note

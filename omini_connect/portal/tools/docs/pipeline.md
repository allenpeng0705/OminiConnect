# Pipeline Tools

Provider: `pipeline` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Pipeline CRM API. They allow AI agents to manage deals, contacts, activities, and pipelines. Pipeline is a sales-focused CRM with visual pipeline management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Pipeline
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pipeline_list_deals` | List deals | GET | /deals |
| `pipeline_get_deal` | Get deal details | GET | /deals/{id} |
| `pipeline_create_deal` | Create a new deal | POST | /deals |
| `pipeline_list_contacts` | List contacts | GET | /contacts |
| `pipeline_get_contact` | Get contact details | GET | /contacts/{id} |
| `pipeline_list_activities` | List activities | GET | /activities |
| `pipeline_get_activity` | Get activity details | GET | /activities/{id} |
| `pipeline_list_pipelines` | List pipelines | GET | /pipelines |
| `pipeline_get_pipeline` | Get pipeline details | GET | /pipelines/{id} |
| `pipeline_get_report` | Get report data | GET | /reports/{type} |

---

## Tool Details

### pipeline_list_deals

**What it does**: Lists all deals in Pipeline CRM with optional filters.

**When to use**: Find deals by pipeline, stage, or status.

**Arguments**:
- `pipeline_id` (optional): Filter by pipeline ID
- `stage_id` (optional): Filter by stage ID
- `status` (optional): Filter by status (`open`, `won`, `lost`)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all open deals in the sales pipeline"

---

### pipeline_get_deal

**What it does**: Gets detailed information about a specific deal.

**When to use**: Read full deal details before updating stage or closing.

**Arguments**:
- `id` (required): Deal ID

**Example LLM prompt**: "Get details for deal 123"

---

### pipeline_create_deal

**What it does**: Creates a new deal in Pipeline CRM.

**When to use**: Add new opportunities to the sales pipeline.

**Arguments**:
- `name` (required): Deal name
- `value` (optional): Deal value (amount)
- `pipeline_id` (optional): Pipeline ID
- `stage_id` (optional): Stage ID
- `contact_id` (optional): Associated contact ID

**Example LLM prompt**: "Create a new deal called Acme Sale worth $50000"

---

### pipeline_list_contacts

**What it does**: Lists all contacts in Pipeline CRM.

**When to use**: Find contacts, search the contact database.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all contacts"

---

### pipeline_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Read full contact details including associated deals.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 456"

---

### pipeline_list_activities

**What it does**: Lists all activities in Pipeline CRM with optional filters.

**When to use**: Find recent calls, meetings, emails by deal or date.

**Arguments**:
- `type` (optional): Activity type (`call`, `meeting`, `email`, `note`)
- `deal_id` (optional): Filter by deal ID
- `date_after` (optional): Activities after date (ISO 8601)
- `date_before` (optional): Activities before date (ISO 8601)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all calls from last week"

---

### pipeline_get_activity

**What it does**: Gets detailed information about a specific activity.

**When to use**: Read full activity details including notes.

**Arguments**:
- `id` (required): Activity ID

**Example LLM prompt**: "Get details for activity 789"

---

### pipeline_list_pipelines

**What it does**: Lists all sales pipelines in Pipeline CRM.

**When to use**: View available pipelines and their structure.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all pipelines"

---

### pipeline_get_pipeline

**What it does**: Gets detailed information about a specific pipeline including stages.

**When to use**: Understand pipeline stages for deal organization.

**Arguments**:
- `id` (required): Pipeline ID

**Example LLM prompt**: "Get details for pipeline 1 including all stages"

---

### pipeline_get_report

**What it does**: Gets report data from Pipeline CRM.

**When to use**: Retrieve sales metrics, activity summaries, pipeline performance.

**Arguments**:
- `type` (required): Report type (`sales`, `activity`, `pipeline`)
- `date_from` (optional): Start date (ISO 8601)
- `date_to` (optional): End date (ISO 8601)

**Example LLM prompt**: "Get sales report for Q1 2024"

---

## Pipeline API Notes

- **Integer IDs**: Pipeline uses integer IDs for all entities
- **Visual pipelines**: Pipeline emphasizes visual pipeline management
- **Activity tracking**: Activities include calls, meetings, emails, and notes
- **Reporting**: Built-in reports for sales, activity, and pipeline metrics
- **Pagination**: Default limit is 50, adjust as needed
- **ISO 8601 dates**: All dates should be in ISO 8601 format (e.g., `2024-12-31T23:59:59Z`)

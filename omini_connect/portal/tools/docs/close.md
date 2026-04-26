# Close Tools

Provider: `close` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Close CRM API. They allow AI agents to manage leads, opportunities, activities, and tasks. Close is a sales-focused CRM built for outbound prospecting and pipeline management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Close
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `lead:read`, `lead:write`, `opportunity:read`, `opportunity:write`, `activity:read`, `task:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `close_list_leads` | List leads | GET | /api/v1/lead |
| `close_get_lead` | Get lead details | GET | /api/v1/lead/{id} |
| `close_create_lead` | Create a new lead | POST | /api/v1/lead |
| `close_list_opportunities` | List opportunities | GET | /api/v1/opportunity |
| `close_get_opportunity` | Get opportunity details | GET | /api/v1/opportunity/{id} |
| `close_create_opportunity` | Create a new opportunity | POST | /api/v1/opportunity |
| `close_list_activities` | List activities | GET | /api/v1/activity |
| `close_get_activity` | Get activity details | GET | /api/v1/activity/{id} |
| `close_list_tasks` | List tasks | GET | /api/v1/task |
| `close_get_task` | Get task details | GET | /api/v1/task/{id} |

---

## Tool Details

### close_list_leads

**What it does**: Lists all leads with optional filtering by status, owner, or source.

**When to use**: Browse pipeline, find leads at a certain stage, see all leads for a rep.

**Arguments**:
- `status` (optional): Lead status (`active`, `lost`, `junk`)
- `owner_id` (optional): Owner user ID
- `query` (optional): Search query
- `limit` (optional): Max results (default 100)
- `skip` (optional): Pagination offset

**Example LLM prompt**: "List all active leads assigned to me"

---

### close_get_lead

**What it does**: Gets detailed information about a specific lead including name, status, contacts, and opportunities.

**When to use**: Read lead details before updating or adding activities.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get details for lead abc-123"

---

### close_create_lead

**What it does**: Creates a new lead with name, status, owner, and source.

**When to use**: Add a new prospect to the CRM.

**Arguments**:
- `name` (required): Lead name
- `status` (optional): Status (`active`, `lost`, `junk`)
- `owner_id` (optional): Owner user ID
- `source` (optional): Lead source
- `contacts` (optional): Initial contacts

**Example LLM prompt**: "Create a new lead named Acme Corp from the website"

---

### close_list_opportunities

**What it does**: Lists all opportunities (deals) with optional filtering by lead, stage, or status.

**When to use**: View pipeline, find deals at a certain stage, track win/loss.

**Arguments**:
- `lead_id` (optional): Filter by lead ID
- `stage_id` (optional): Filter by stage ID
- `status` (optional): Status (`open`, `won`, `lost`)
- `limit` (optional): Max results (default 100)
- `skip` (optional): Pagination offset

**Example LLM prompt**: "List all open opportunities in the sales pipeline"

---

### close_get_opportunity

**What it does**: Gets detailed information about a specific opportunity including value, stage, contacts, and tasks.

**When to use**: Read opportunity details before updating or adding tasks.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity def-789"

---

### close_create_opportunity

**What it does**: Creates a new opportunity with name, value, lead association, and stage.

**When to use**: Add a new deal to the pipeline.

**Arguments**:
- `name` (required): Opportunity name
- `lead_id` (optional): Associated lead ID
- `value` (optional): Opportunity monetary value
- `stage_id` (optional): Pipeline stage ID
- `status` (optional): Status (`open`, `won`, `lost`)
- `probability` (optional): Win probability percentage

**Example LLM prompt**: "Create an opportunity named Enterprise Deal for lead abc-123 worth $50,000"

---

### close_list_activities

**What it does**: Lists all activities with optional filtering by lead, user, type, or date range.

**When to use**: See call logs, emails, notes, and meetings for a lead or user.

**Arguments**:
- `lead_id` (optional): Filter by lead ID
- `user_id` (optional): Filter by user ID
- `type` (optional): Activity type (`call`, `email`, `note`, `meeting`)
- `limit` (optional): Max results (default 100)
- `skip` (optional): Pagination offset

**Example LLM prompt**: "List all calls with lead abc-123 this week"

---

### close_get_activity

**What it does**: Gets detailed information about a specific activity including type, date, and participants.

**When to use**: Read activity details after the fact.

**Arguments**:
- `id` (required): Activity ID

**Example LLM prompt**: "Get details for activity xyz-456"

---

### close_list_tasks

**What it does**: Lists all tasks with optional filtering by lead, user, status, or date range.

**When to use**: Find open tasks, see all tasks for a rep, check overdue items.

**Arguments**:
- `lead_id` (optional): Filter by lead ID
- `user_id` (optional): Filter by assigned user ID
- `status` (optional): Status (`open`, `completed`)
- `limit` (optional): Max results (default 100)
- `skip` (optional): Pagination offset

**Example LLM prompt**: "List all open tasks assigned to me"

---

### close_get_task

**What it does**: Gets detailed information about a specific task including title, due date, and assignments.

**When to use**: Get full task details before marking complete or updating.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task task-789"

---

## Close API Notes

- **Lead status**: Leads can be `active`, `lost`, or `junk`. Lost and junk leads are typically hidden by default.
- **Opportunities vs Leads**: In Close, a Lead contains Contacts and Opportunities. Opportunities represent potential deals.
- **Activities**: Activity types include calls, emails, notes, and meetings. Activities are logged against leads.
- **Tasks**: Tasks can be linked to leads, contacts, or opportunities for context.
- **Source tracking**: Close tracks lead sources (referral, website, cold outreach, etc.) for attribution.
- **Pagination**: Use `skip` and `limit` for pagination through large result sets.

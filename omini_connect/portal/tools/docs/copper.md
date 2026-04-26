# Copper Tools

Provider: `copper` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Copper CRM API. They allow AI agents to manage companies, contacts, opportunities, and tasks. Copper is a popular CRM for Gmail and Google Workspace users.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Copper
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `copper_list_companies` | List companies | GET | /companies |
| `copper_get_company` | Get company details | GET | /companies/{company_id} |
| `copper_create_company` | Create a new company | POST | /companies |
| `copper_list_contacts` | List contacts | GET | /contacts |
| `copper_get_contact` | Get contact details | GET | /contacts/{contact_id} |
| `copper_create_contact` | Create a new contact | POST | /contacts |
| `copper_list_opportunities` | List opportunities | GET | /opportunities |
| `copper_get_opportunity` | Get opportunity details | GET | /opportunities/{opportunity_id} |
| `copper_list_tasks` | List tasks | GET | /tasks |
| `copper_get_task` | Get task details | GET | /tasks/{task_id} |

---

## Tool Details

### copper_list_companies

**What it does**: Lists all companies in Copper CRM with optional filters.

**When to use**: Find companies by location, industry, or search by name.

**Arguments**:
- `city` (optional): Filter by city
- `state` (optional): Filter by state
- `country` (optional): Filter by country
- `industry` (optional): Filter by industry
- `page_size` (optional): Number of results (default 20, max 200)

**Example LLM prompt**: "List all companies in California"

---

### copper_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: Read full company details before updating or linking to opportunities.

**Arguments**:
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for company 12345"

---

### copper_create_company

**What it does**: Creates a new company in Copper CRM.

**When to use**: Add new companies to the CRM when discovered.

**Arguments**:
- `name` (required): Company name
- `city` (optional): City
- `state` (optional): State
- `country` (optional): Country
- `industry` (optional): Industry
- `website` (optional): Website URL
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a new company called Acme Corp in San Francisco"

---

### copper_list_contacts

**What it does**: Lists all contacts in Copper CRM with optional filters.

**When to use**: Find contacts by company, email, or name.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `email` (optional): Filter by email address
- `name` (optional): Filter by name
- `page_size` (optional): Number of results (default 20, max 200)

**Example LLM prompt**: "List all contacts at Acme Corp"

---

### copper_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Read full contact details before calling or emailing.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 67890"

---

### copper_create_contact

**What it does**: Creates a new contact in Copper CRM.

**When to use**: Add new leads or contacts to the CRM.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Associated company ID
- `title` (optional): Job title

**Example LLM prompt**: "Create a new contact John Doe with email john@example.com"

---

### copper_list_opportunities

**What it does**: Lists all opportunities (deals) in Copper CRM with optional filters.

**When to use**: Find deals by pipeline, stage, or owner.

**Arguments**:
- `status` (optional): Filter by status (`open`, `won`, `lost`)
- `owner_id` (optional): Filter by owner user ID
- `pipeline_id` (optional): Filter by pipeline ID
- `page_size` (optional): Number of results (default 20, max 200)

**Example LLM prompt**: "List all open opportunities in the sales pipeline"

---

### copper_get_opportunity

**What it does**: Gets detailed information about a specific opportunity.

**When to use**: Read full opportunity details before updating stage or closing.

**Arguments**:
- `opportunity_id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity 11111"

---

### copper_list_tasks

**What it does**: Lists all tasks in Copper CRM with optional filters.

**When to use**: Find tasks to complete, see tasks by assignee, filter by due date.

**Arguments**:
- `status` (optional): Filter by `open` or `completed`
- `assignee_id` (optional): Filter by user ID
- `related_id` (optional): Filter by related entity ID
- `due_before` (optional): Due before date
- `due_after` (optional): Due after date
- `page_size` (optional): Number of results (default 20, max 200)

**Example LLM prompt**: "List all open tasks due this week"

---

### copper_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: Read full task details before updating or completing.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Get details for task 12345"

---

## Copper API Notes

- **Related entities**: Tasks can be linked to leads, persons, companies, or opportunities
- **User IDs**: Copper user IDs are numeric strings
- **Pagination**: Default page_size is 20, max is 200
- **ISO 8601 dates**: All dates should be in ISO 8601 format (e.g., `2024-12-31T23:59:59Z`)

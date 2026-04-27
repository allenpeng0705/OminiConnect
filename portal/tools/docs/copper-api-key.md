# Copper Tools

Provider: `copper-api-key` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Copper CRM API. They allow AI agents to manage accounts, contacts, opportunities, and tasks. Copper is a CRM platform focused on relationship management and sales pipeline tracking.

## Authentication

**Nango API_KEY**:
- User provides their Copper API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Headers include: `x-pw-accesstoken`, `x-pw-application`, `x-pw-useremail`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `copper_api_key_list_accounts` | List all accounts | GET | /v1/accounts |
| `copper_api_key_get_account` | Get a specific account | GET | /v1/accounts/{id} |
| `copper_api_key_list_contacts` | List all contacts | GET | /v1/contacts |
| `copper_api_key_get_contact` | Get a specific contact | GET | /v1/contacts/{id} |
| `copper_api_key_create_contact` | Create a new contact | POST | /v1/contacts |
| `copper_api_key_list_opportunities` | List all opportunities | GET | /v1/opportunities |
| `copper_api_key_get_opportunity` | Get a specific opportunity | GET | /v1/opportunities/{id} |
| `copper_api_key_create_opportunity` | Create a new opportunity | POST | /v1/opportunities |
| `copper_api_key_list_tasks` | List all tasks | GET | /v1/tasks |
| `copper_api_key_get_task` | Get a specific task | GET | /v1/tasks/{id} |

---

## Tool Details

### copper_api_key_list_accounts

**What it does**: Lists all accounts (companies) in Copper CRM.

**When to use**: Browse companies, find account details, or select an account for further operations.

**Arguments**:
- `page_size` (optional): Results per page (default 20, max 200)
- `page_number` (optional): Page number (default 1)

**Example LLM prompt**: "List all accounts in Copper"

---

### copper_api_key_get_account

**What it does**: Gets detailed account information including contact info, owner, and related opportunities.

**When to use**: Get account details before creating contacts or opportunities.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get details for account ID 12345"

---

### copper_api_key_list_contacts

**What it does**: Lists all contacts with optional sorting and pagination.

**When to use**: Find contacts, browse the contact database, or locate a specific person.

**Arguments**:
- `page_size` (optional): Results per page (default 20, max 200)
- `page_number` (optional): Page number (default 1)
- `sort_by` (optional): Sort field - name, email_address, date_modified

**Example LLM prompt**: "List all contacts sorted by name"

---

### copper_api_key_get_contact

**What it does**: Gets detailed contact information including email, phone, and associated account.

**When to use**: Get contact details before updating or creating related records.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact ID 67890"

---

### copper_api_key_create_contact

**What it does**: Creates a new contact with name and optional email, phone, and company association.

**When to use**: Add new leads, create contact records for new customers.

**Arguments**:
- `name` (required): Contact name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Associated account ID

**Example LLM prompt**: "Create a new contact named John Doe with email john@example.com"

---

### copper_api_key_list_opportunities

**What it does**: Lists all opportunities (deals) with optional status filtering.

**When to use**: View sales pipeline, check deal status, find opportunities to update.

**Arguments**:
- `page_size` (optional): Results per page (default 20, max 200)
- `page_number` (optional): Page number (default 1)
- `status` (optional): Filter by open, won, or lost

**Example LLM prompt**: "List all open opportunities"

---

### copper_api_key_get_opportunity

**What it does**: Gets detailed opportunity information including pipeline stage, value, and owner.

**When to use**: Check deal details before updating or moving through pipeline stages.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity ID 11111"

---

### copper_api_key_create_opportunity

**What it does**: Creates a new opportunity in the sales pipeline.

**When to use**: Log new deals, track new sales prospects.

**Arguments**:
- `name` (required): Opportunity name
- `pipeline_id` (required): Pipeline ID for this opportunity
- `assigned_to_user_id` (optional): Owner user ID
- `monetary_value` (optional): Deal value
- `customer_source` (optional): Lead source

**Example LLM prompt**: "Create a new opportunity named 'Big Sale' in pipeline 1 with value $50000"

---

### copper_api_key_list_tasks

**What it does**: Lists all tasks with optional status and pagination.

**When to use**: View pending tasks, check completed items, find overdue tasks.

**Arguments**:
- `page_size` (optional): Results per page (default 20, max 200)
- `page_number` (optional): Page number (default 1)
- `status` (optional): Filter by open or completed

**Example LLM prompt**: "List all open tasks"

---

### copper_api_key_get_task

**What it does**: Gets detailed task information including assignee, due date, and description.

**When to use**: Get task context before marking complete or updating.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task ID 22222"

---

## Copper API Notes

- **Account vs Contact**: Accounts are companies/organizations; Contacts are individuals within those accounts
- **Opportunities**: Sales deals that move through pipelines; have monetary value and pipeline stages
- **Tasks**: Activity reminders and to-do items associated with accounts, contacts, or opportunities
- **API Key format**: 32-character hexadecimal string
- **Pagination**: Use page_size and page_number for navigating large result sets

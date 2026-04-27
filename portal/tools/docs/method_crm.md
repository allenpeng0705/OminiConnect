# Method CRM Tools

Provider: `method_crm` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Method CRM API. They allow AI agents to manage people, companies, deals, tasks, and activities for professional service firms. Method CRM is designed for accounting and bookkeeping professionals with a focus on people and company management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Method CRM
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `people:r`, `people:w`, `companies:r`, `companies:w`, `deals:r`, `deals:w`, `tasks:r`, `tasks:w`, `activities:r`, `activities:w`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `method_crm_list_people` | List all people | GET | /people |
| `method_crm_get_person` | Get person details | GET | /people/{id} |
| `method_crm_create_person` | Create a new person | POST | /people |
| `method_crm_list_companies` | List all companies | GET | /companies |
| `method_crm_get_company` | Get company details | GET | /companies/{id} |
| `method_crm_list_deals` | List all deals | GET | /deals |
| `method_crm_get_deal` | Get deal details | GET | /deals/{id} |
| `method_crm_list_tasks` | List all tasks | GET | /tasks |
| `method_crm_get_task` | Get task details | GET | /tasks/{id} |
| `method_crm_list_activities` | List all activities | GET | /activities |

---

## Tool Details

### method_crm_list_people

**What it does**: Retrieves all people from Method CRM with optional filtering by name or email.

**When to use**: Get an overview of all contacts and clients in the CRM.

**Arguments**:
- `search` (optional): Search by name or email
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all my contacts at Method CRM"

---

### method_crm_get_person

**What it does**: Gets detailed information about a specific person including contact info and company association.

**When to use**: Get full details about a specific client or contact.

**Arguments**:
- `id` (required): Person ID

**Example LLM prompt**: "Get details for person ID 12345"

---

### method_crm_create_person

**What it does**: Creates a new person in Method CRM with name, contact info, and company details.

**When to use**: Add a new client or contact to the CRM.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Company ID
- `title` (optional): Job title

**Example LLM prompt**: "Add a new person for Jane Doe at ABC Services with email jane@abcservices.com"

---

### method_crm_list_companies

**What it does**: Retrieves all companies from Method CRM with optional filtering by name or industry.

**When to use**: Browse and search company records in the CRM.

**Arguments**:
- `search` (optional): Search by name or industry
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all companies in the tech industry"

---

### method_crm_get_company

**What it does**: Gets detailed information about a specific company including address and contacts.

**When to use**: Get full company details before meetings or business development.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company ID 67890"

---

### method_crm_list_deals

**What it does**: Retrieves all deals with optional filtering by status, stage, or assignee.

**When to use**: Track sales pipeline and deal progress.

**Arguments**:
- `status` (optional): Filter by status (open, won, lost)
- `stage` (optional): Filter by deal stage
- `assigned_to` (optional): Filter by assigned person ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all open deals in the proposal stage"

---

### method_crm_get_deal

**What it does**: Gets detailed information about a specific deal including value and stage history.

**When to use**: Review deal details before client conversations or status updates.

**Arguments**:
- `id` (required): Deal ID

**Example LLM prompt**: "Get details for deal ID 11111"

---

### method_crm_list_tasks

**What it does**: Retrieves all tasks with optional filtering by status, assignee, or due date.

**When to use**: Track pending action items and deadlines.

**Arguments**:
- `status` (optional): Filter by status (pending, in-progress, completed)
- `assignee_id` (optional): Filter by assignee ID
- `due_date_from` (optional): Due date from (YYYY-MM-DD)
- `due_date_to` (optional): Due date to (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all tasks due this week"

---

### method_crm_get_task

**What it does**: Gets detailed information about a specific task including description and assignee.

**When to use**: Review task details before working on it or updating status.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task ID 22222"

---

### method_crm_list_activities

**What it does**: Retrieves all activities with optional filtering by type, person, or date range.

**When to use**: Review recent interactions and touchpoints with clients.

**Arguments**:
- `type` (optional): Filter by activity type (call, email, meeting, note)
- `person_id` (optional): Filter by person ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all meetings from this month"

---

## Method CRM API Notes

- **People vs Companies**: Method CRM separates people (individuals) from companies, allowing detailed tracking of contacts within organizations
- **Deal Pipeline**: Track deals through stages from initial contact to closed/won/lost
- **Activity Logging**: Record all client interactions including calls, emails, meetings, and notes
- **Task Management**: Create and assign tasks with priorities and due dates
- **Company Associations**: Link people to companies for organizational context
- **Pagination**: Default per_page is 50, adjust based on your needs
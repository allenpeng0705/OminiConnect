# Nutshell Tools

Provider: `nutshell` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Nutshell is a sales CRM platform that helps teams manage people, companies, deals, and tasks. These tools wrap the Nutshell REST API, enabling AI agents to access and manage CRM data — perfect for automating sales workflows, updating deal statuses, and retrieving customer information.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Nutshell
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write` (based on permission level)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nutshell_list_people` | List people in CRM | GET | /api/people |
| `nutshell_get_person` | Get person details | GET | /api/people/{id} |
| `nutshell_create_person` | Create a new person | POST | /api/people |
| `nutshell_list_companies` | List companies | GET | /api/companies |
| `nutshell_get_company` | Get company details | GET | /api/companies/{id} |
| `nutshell_list_deals` | List deals | GET | /api/deals |
| `nutshell_get_deal` | Get deal details | GET | /api/deals/{id} |
| `nutshell_create_deal` | Create a new deal | POST | /api/deals |
| `nutshell_list_tasks` | List tasks | GET | /api/tasks |
| `nutshell_get_task` | Get task details | GET | /api/tasks/{id} |

---

## Tool Details

### nutshell_list_people

**What it does**: Lists all people in the Nutshell CRM account with optional filtering.

**When to use**: Browse contacts, find specific people, get an overview of the CRM database.

**Arguments**:
- `company` (optional): Filter by company ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all people in the CRM"

---

### nutshell_get_person

**What it does**: Gets detailed information about a specific person including contact info and associated company.

**When to use**: Get full details about a contact before calling or emailing.

**Arguments**:
- `id` (required): Person ID

**Example LLM prompt**: "Get details for person 12345"

---

### nutshell_create_person

**What it does**: Creates a new person record in Nutshell CRM.

**When to use**: Add new leads or contacts captured from conversations or other sources.

**Arguments**:
- `name` (required): Person name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Associated company ID

**Example LLM prompt**: "Add a new person named John Smith with email john@example.com"

---

### nutshell_list_companies

**What it does**: Lists all companies in the Nutshell CRM account.

**When to use**: Browse companies, find accounts to associate deals or contacts with.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all companies in the CRM"

---

### nutshell_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: Review company details, associated contacts, and deal history.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company 67890"

---

### nutshell_list_deals

**What it does**: Lists all deals in the CRM with optional status filtering.

**When to use**: Review pipeline, check deal progress, find deals needing attention.

**Arguments**:
- `status` (optional): Filter by deal status (open, won, lost)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all open deals in the pipeline"

---

### nutshell_get_deal

**What it does**: Gets detailed information about a specific deal including value, parties, and status.

**When to use**: Review deal details before follow-up calls or status updates.

**Arguments**:
- `id` (required): Deal ID

**Example LLM prompt**: "Get details for deal 11111"

---

### nutshell_create_deal

**What it does**: Creates a new deal in Nutshell CRM.

**When to use**: Log new sales opportunities, track new business conversations.

**Arguments**:
- `name` (required): Deal name
- `value` (optional): Deal value (amount)
- `company_id` (optional): Associated company ID
- `contact_id` (optional): Associated contact ID

**Example LLM prompt**: "Create a new deal called 'Q2 Enterprise Deal' worth $50000"

---

### nutshell_list_tasks

**What it does**: Lists all tasks in the CRM with optional status filtering.

**When to use**: Review pending tasks, find overdue items, plan daily workflow.

**Arguments**:
- `status` (optional): Filter by status (active, completed)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active tasks"

---

### nutshell_get_task

**What it does**: Gets detailed information about a specific task including due date and assignee.

**When to use**: Review task details before completion or follow-up.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 99999"

---

## Nutshell API Notes

- **IDs**: Nutshell uses integer IDs for all objects
- **People**: Individual contacts who may be associated with companies
- **Companies**: Business accounts that can have multiple people and deals
- **Deals**: Sales opportunities with values, statuses, and associations to people/companies
- **Tasks**: Action items that can be linked to people, companies, or deals

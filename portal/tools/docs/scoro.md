# Scoro Tools

Provider: `scoro` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Scoro REST API. They allow AI agents to manage projects, tasks, contacts, invoices, and quotes — Scoro is a business management platform designed for professional services, creative agencies, and consulting firms.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Scoro
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projects_read`, `projects_write`, `tasks_read`, `tasks_write`, `contacts_read`, `contacts_write`, `invoices_read`, `invoices_write`, `quotes_read`, `quotes_write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `scoro_list_projects` | List projects | GET | /api/v1/projects |
| `scoro_get_project` | Get project details | GET | /api/v1/projects/{id} |
| `scoro_create_project` | Create a project | POST | /api/v1/projects |
| `scoro_list_tasks` | List tasks | GET | /api/v1/tasks |
| `scoro_get_task` | Get task details | GET | /api/v1/tasks/{id} |
| `scoro_list_contacts` | List contacts | GET | /api/v1/contacts |
| `scoro_get_contact` | Get contact details | GET | /api/v1/contacts/{id} |
| `scoro_list_invoices` | List invoices | GET | /api/v1/invoices |
| `scoro_get_invoice` | Get invoice details | GET | /api/v1/invoices/{id} |
| `scoro_list_quotes` | List quotes | GET | /api/v1/quotes |

---

## Tool Details

### scoro_list_projects

**What it does**: Lists all projects with optional status filtering.

**When to use**: Browse available projects, find active projects for task or quote association.

**Arguments**:
- `status` (optional): Filter by status (Active, Archived, Draft)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active projects in Scoro"

---

### scoro_get_project

**What it does**: Gets detailed project info — name, status, budget, team members, dates.

**When to use**: Understand project scope, budget, and timeline before creating tasks or quotes.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 12345"

---

### scoro_create_project

**What it does**: Creates a new project with name, description, and dates.

**When to use**: Set up new projects for clients or internal initiatives.

**Arguments**:
- `name` (required): Project name
- `description` (optional): Project description
- `startDate` (optional): Start date (YYYY-MM-DD)
- `deadline` (optional): Deadline date (YYYY-MM-DD)

**Example LLM prompt**: "Create a new project called 'Marketing Campaign' with deadline end of April"

---

### scoro_list_tasks

**What it does**: Lists tasks with optional project, assignee, and status filters.

**When to use**: Find tasks across projects, filter by assignee or status.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `assignee_id` (optional): Filter by assignee ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all tasks assigned to me in project 12345"

---

### scoro_get_task

**What it does**: Gets detailed task info — title, description, assignee, due date, time logged.

**When to use**: Read full task context before updating or logging time.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task details for task 67890"

---

### scoro_list_contacts

**What it does**: Lists all contacts with optional type and search filters.

**When to use**: Find contacts for quoting or invoicing, search by name or email.

**Arguments**:
- `type` (optional): Filter by type (Person, Company)
- `search` (optional): Search by name or email

**Example LLM prompt**: "Search for company contacts named 'Acme'"

---

### scoro_get_contact

**What it does**: Gets detailed contact info — name, email, phone, company, address, tax ID.

**When to use**: Get full contact details for quoting, invoicing, or project assignment.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact details for contact 12345"

---

### scoro_list_invoices

**What it does**: Lists invoices with optional status filtering.

**When to use**: Find invoices by status, see overdue or unpaid invoices.

**Arguments**:
- `status` (optional): Filter by status (Draft, Sent, Paid, Overdue)

**Example LLM prompt**: "List all overdue invoices"

---

### scoro_get_invoice

**What it does**: Gets detailed invoice info — number, amount, line items, status, payment details.

**When to use**: Check invoice status, review line items before payment or follow-up.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice details for invoice 12345"

---

### scoro_list_quotes

**What it does**: Lists quotes with optional status filtering.

**When to use**: Find quotes by status, see pending quotes awaiting approval.

**Arguments**:
- `status` (optional): Filter by status (Draft, Sent, Accepted, Declined)

**Example LLM prompt**: "List all sent quotes awaiting response"

---

## Scoro API Notes

- **IDs**: Scoro uses integer IDs for all objects
- **Projects**: Top-level containers for tasks, time, quotes, and invoices
- **Tasks**: Work items within projects; can have assignees, due dates, estimated time
- **Quotes**: Sales documents that can be sent to clients and converted to invoices
- **Invoices**: Billing documents linked to projects; track payment status
- **Contacts**: People and companies that can be associated with projects, quotes, and invoices
- **API Version**: Uses `/api/v1/` prefix for all endpoints
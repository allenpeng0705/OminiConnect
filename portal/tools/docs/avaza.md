# Avaza Tools

Provider: `avaza` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Avaza REST API. They allow AI agents to manage projects, tasks, time entries, invoices, and expenses — Avaza is an all-in-one project management, time tracking, and invoicing platform popular with agencies and professional services teams.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Avaza
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projectread`, `projectwrite`, `taskread`, `taskwrite`, `timeread`, `invoiceread`, `invoicewrite`, `expanseread`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `avaza_list_projects` | List projects | GET | /api/projects |
| `avaza_get_project` | Get project details | GET | /api/projects/{projectGuid} |
| `avaza_create_project` | Create a project | POST | /api/projects |
| `avaza_list_tasks` | List tasks | GET | /api/tasks |
| `avaza_get_task` | Get task details | GET | /api/tasks/{taskGuid} |
| `avaza_list_time_entries` | List time entries | GET | /api/timeentries |
| `avaza_get_time_entry` | Get time entry details | GET | /api/timeentries/{timeEntryGuid} |
| `avaza_list_invoices` | List invoices | GET | /api/invoices |
| `avaza_get_invoice` | Get invoice details | GET | /api/invoices/{invoiceGuid} |
| `avaza_list_expenses` | List expenses | GET | /api/expenses |

---

## Tool Details

### avaza_list_projects

**What it does**: Lists all projects with optional status filtering.

**When to use**: Browse available projects, find active projects for task or invoice association.

**Arguments**:
- `status` (optional): Filter by status (Active, Draft, Closed)
- `pageSize` (optional): Number of results per page (default 20)
- `pageNumber` (optional): Page number (default 1)

**Example LLM prompt**: "List all active projects in Avaza"

---

### avaza_get_project

**What it does**: Gets detailed project info — name, status, dates, team members, budget.

**When to use**: Understand project scope, budget, and timeline before creating tasks or invoices.

**Arguments**:
- `projectGuid` (required): Project GUID

**Example LLM prompt**: "Get details for project abc-123-def"

---

### avaza_create_project

**What it does**: Creates a new project with name, description, and dates.

**When to use**: Set up new projects for clients or internal initiatives.

**Arguments**:
- `name` (required): Project name
- `description` (optional): Project description
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Create a new project called 'Website Redesign' starting next week"

---

### avaza_list_tasks

**What it does**: Lists tasks with optional project and assignee filters.

**When to use**: Find tasks across projects, filter by assignee or completion status.

**Arguments**:
- `projectGuid` (optional): Filter by project GUID
- `assigneeGuid` (optional): Filter by assignee GUID
- `isCompleted` (optional): Filter by completion status

**Example LLM prompt**: "List all incomplete tasks assigned to me"

---

### avaza_get_task

**What it does**: Gets detailed task info — title, description, assignee, due date, time logged.

**When to use**: Read full task context before updating or adding time entries.

**Arguments**:
- `taskGuid` (required): Task GUID

**Example LLM prompt**: "Get task details for abc-456-def"

---

### avaza_list_time_entries

**What it does**: Lists time entries with optional project, user, and date filters.

**When to use**: Review work hours logged, find time entries for billing or project tracking.

**Arguments**:
- `projectGuid` (optional): Filter by project GUID
- `userGuid` (optional): Filter by user GUID
- `startDate` (optional): Start date filter (YYYY-MM-DD)
- `endDate` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "List all time entries for project abc-123 this month"

---

### avaza_get_time_entry

**What it does**: Gets detailed time entry info — project, user, hours, date, notes.

**When to use**: Review specific time entry details for billing or verification.

**Arguments**:
- `timeEntryGuid` (required): Time entry GUID

**Example LLM prompt**: "Get details for time entry abc-789-def"

---

### avaza_list_invoices

**What it does**: Lists invoices with optional status and project filters.

**When to use**: Find invoices by status, see invoices for a specific project.

**Arguments**:
- `status` (optional): Filter by status (Draft, Sent, Paid, Overdue)
- `projectGuid` (optional): Filter by project GUID

**Example LLM prompt**: "List all overdue invoices"

---

### avaza_get_invoice

**What it does**: Gets detailed invoice info — number, amount, line items, status, payment details.

**When to use**: Check invoice status, review line items before payment.

**Arguments**:
- `invoiceGuid` (required): Invoice GUID

**Example LLM prompt**: "Get invoice details for abc-789-def"

---

### avaza_list_expenses

**What it does**: Lists expenses with optional project, category, and date filters.

**When to use**: Review project expenses, categorize spending, prepare for invoicing.

**Arguments**:
- `projectGuid` (optional): Filter by project GUID
- `category` (optional): Filter by expense category
- `startDate` (optional): Start date filter (YYYY-MM-DD)
- `endDate` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "List all expenses for project abc-123 in Q1 2026"

---

## Avaza API Notes

- **GUIDs**: Avaza uses GUIDs (strings like `abc-123-def-456`) for all object IDs
- **Projects**: Top-level containers for tasks, time entries, invoices, and expenses
- **Tasks**: Work items within projects; can have assignees, due dates, time estimates
- **Time Entries**: Logged work hours that can be billed via invoices or tracked for project成本
- **Invoices**: Billable documents linked to projects; can include time entries and expenses
- **Expenses**: Project-related costs that can be added to invoices
- **Date Filtering**: Most list endpoints support date range filtering for time-based queries
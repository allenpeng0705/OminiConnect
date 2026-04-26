# Dubsado Tools

Provider: `dubsado` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Dubsado CRM API. They allow AI agents to manage clients, projects, tasks, invoices, and time tracking for creative agencies and professional service firms. Dubsado is designed for agencies managing multiple clients and projects with integrated time tracking.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dubsado
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `clients:r`, `clients:w`, `projects:r`, `projects:w`, `tasks:r`, `tasks:w`, `invoices:r`, `invoices:w`, `time_entries:r`, `time_entries:w`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dubsado_list_clients` | List all clients | GET | /clients |
| `dubsado_get_client` | Get client details | GET | /clients/{id} |
| `dubsado_create_client` | Create a new client | POST | /clients |
| `dubsado_list_projects` | List all projects | GET | /projects |
| `dubsado_get_project` | Get project details | GET | /projects/{id} |
| `dubsado_list_tasks` | List all tasks | GET | /tasks |
| `dubsado_get_task` | Get task details | GET | /tasks/{id} |
| `dubsado_list_invoices` | List all invoices | GET | /invoices |
| `dubsado_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `dubsado_list_time_entries` | List all time entries | GET | /time_entries |

---

## Tool Details

### dubsado_list_clients

**What it does**: Retrieves all clients from Dubsado with optional filtering by name or tags.

**When to use**: Get an overview of all active clients and their status.

**Arguments**:
- `search` (optional): Search by name or email
- `tag_ids` (optional): Filter by tag IDs
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all my active clients"

---

### dubsado_get_client

**What it does**: Gets detailed information about a specific client including contact info and address.

**When to use**: Get full client details before meetings or project planning.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get details for client ID 12345"

---

### dubsado_create_client

**What it does**: Creates a new client in Dubsado with full contact and address information.

**When to use**: Add a new client to the system for project work.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company` (optional): Company name
- `address` (optional): Street address
- `city` (optional): City
- `state` (optional): State/Province
- `postal_code` (optional): Postal code
- `country` (optional): Country

**Example LLM prompt**: "Add a new client for Sarah Johnson at Creative Studio with email sarah@creativestudio.com"

---

### dubsado_list_projects

**What it does**: Retrieves all projects with optional filtering by client, status, or type.

**When to use**: Track active projects across all clients.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `status` (optional): Filter by status (active, completed, on-hold)
- `project_type` (optional): Filter by project type
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all active projects this month"

---

### dubsado_get_project

**What it does**: Gets detailed information about a specific project including timeline and deliverables.

**When to use**: Review project status and scope before client meetings.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project ID 67890"

---

### dubsado_list_tasks

**What it does**: Retrieves all tasks with optional filtering by project, status, or assignee.

**When to use**: Track pending action items and deadlines across projects.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `status` (optional): Filter by status (pending, in-progress, completed)
- `assignee_id` (optional): Filter by assignee ID
- `due_date_from` (optional): Due date from (YYYY-MM-DD)
- `due_date_to` (optional): Due date to (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all tasks due this week"

---

### dubsado_get_task

**What it does**: Gets detailed information about a specific task including description and project association.

**When to use**: Review task details before working on it or updating status.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task ID 22222"

---

### dubsado_list_invoices

**What it does**: Retrieves all invoices with optional filtering by client, status, or date.

**When to use**: Track outstanding payments and invoice history.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `status` (optional): Filter by status (draft, sent, paid, overdue)
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all overdue invoices"

---

### dubsado_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items and payment status.

**When to use**: Review invoice details before sending or following up on payment.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice ID 11111"

---

### dubsado_list_time_entries

**What it does**: Retrieves all time entries with optional filtering by project, task, or date range.

**When to use**: Review time tracked across projects and tasks for billing or productivity analysis.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `task_id` (optional): Filter by task ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all time entries from this week"

---

## Dubsado API Notes

- **Agency Focus**: Dubsado is designed for creative agencies managing multiple clients and projects
- **Client Management**: Clients can be individuals or companies with full contact details
- **Project Types**: Projects can be filtered by type (branding, web design, marketing, etc.)
- **Time Tracking**: Built-in time tracking allows monitoring of billable hours across projects and tasks
- **Invoice Workflow**: Track invoices from draft through sent to paid
- **Tags**: Use tags to categorize clients and projects by service type or status
- **Pagination**: Default per_page is 50, adjust based on your needs
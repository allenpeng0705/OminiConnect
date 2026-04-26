# 17hats Tools

Provider: `17hats` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the 17hats CRM API. They allow AI agents to manage clients, projects, tasks, and invoices for service-based businesses. 17hats is designed for solo entrepreneurs and small businesses to manage their client workflow.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with 17hats
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `clients:r`, `clients:w`, `projects:r`, `projects:w`, `tasks:r`, `tasks:w`, `invoices:r`, `invoices:w`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `17hats_list_clients` | List all clients | GET | /clients |
| `17hats_get_client` | Get client details | GET | /clients/{id} |
| `17hats_create_client` | Create a new client | POST | /clients |
| `17hats_list_projects` | List all projects | GET | /projects |
| `17hats_get_project` | Get project details | GET | /projects/{id} |
| `17hats_create_project` | Create a new project | POST | /projects |
| `17hats_list_tasks` | List all tasks | GET | /tasks |
| `17hats_get_task` | Get task details | GET | /tasks/{id} |
| `17hats_list_invoices` | List all invoices | GET | /invoices |
| `17hats_get_invoice` | Get invoice details | GET | /invoices/{id} |

---

## Tool Details

### 17hats_list_clients

**What it does**: Retrieves all clients from 17hats with optional filtering by name or email.

**When to use**: Get an overview of all active clients in the CRM.

**Arguments**:
- `search` (optional): Search by name or email
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all my clients"

---

### 17hats_get_client

**What it does**: Gets detailed information about a specific client including contact info and company details.

**When to use**: Get full client details before meetings or project planning.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get details for client ID 12345"

---

### 17hats_create_client

**What it does**: Creates a new client in 17hats with name, contact info, and company details.

**When to use**: Add a new client for a project or service engagement.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company` (optional): Company name

**Example LLM prompt**: "Add a new client for Sarah Johnson at Creative Studio with email sarah@creativestudio.com"

---

### 17hats_list_projects

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

### 17hats_get_project

**What it does**: Gets detailed information about a specific project including timeline and deliverables.

**When to use**: Review project status and scope before client meetings.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project ID 67890"

---

### 17hats_create_project

**What it does**: Creates a new project with title, client, description, and timeline.

**When to use**: Start a new engagement or project for a client.

**Arguments**:
- `name` (required): Project name
- `client_id` (required): Client ID
- `description` (optional): Project description
- `status` (optional): Project status
- `start_date` (optional): Start date (YYYY-MM-DD)
- `due_date` (optional): Due date (YYYY-MM-DD)

**Example LLM prompt**: "Create a new project for client 12345 for brand identity design due in 4 weeks"

---

### 17hats_list_tasks

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

### 17hats_get_task

**What it does**: Gets detailed information about a specific task including description and project association.

**When to use**: Review task details before working on it or updating status.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task ID 22222"

---

### 17hats_list_invoices

**What it does**: Retrieves all invoices with optional filtering by client, status, or date range.

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

### 17hats_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items and payment status.

**When to use**: Review invoice details before sending or following up on payment.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice ID 11111"

---

## 17hats API Notes

- **Solo Business Focus**: 17hats is designed for solo entrepreneurs, so the workflow is streamlined for individual use
- **Client Management**: Clients are the primary entity; projects and tasks are associated with clients
- **Project Workflow**: Track projects from initiation through completion
- **Task Tracking**: Tasks can be standalone or associated with projects
- **Invoice Status**: Track invoices through draft, sent, paid, and overdue stages
- **Pagination**: Default per_page is 50, adjust based on your needs
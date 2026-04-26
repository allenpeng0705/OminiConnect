# Harvest Tools

Provider: `harvest` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Harvest API. They allow AI agents to track time, manage projects and clients, and handle tasks. Harvest is a time tracking and invoicing platform popular with agencies and freelancers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Harvest
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read` (for reading data), `write` (for creating/updating data)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `harvest_list_time_entries` | List time entries | GET | /time_entries |
| `harvest_get_time_entry` | Get time entry details | GET | /time_entries/{id} |
| `harvest_create_time_entry` | Create a new time entry | POST | /time_entries |
| `harvest_update_time_entry` | Update a time entry | PATCH | /time_entries/{id} |
| `harvest_list_projects` | List projects | GET | /projects |
| `harvest_get_project` | Get project details | GET | /projects/{id} |
| `harvest_list_clients` | List clients | GET | /clients |
| `harvest_get_client` | Get client details | GET | /clients/{id} |
| `harvest_list_tasks` | List tasks | GET | /tasks |
| `harvest_get_task` | Get task details | GET | /tasks/{id} |

---

## Tool Details

### harvest_list_time_entries

**What it does**: Lists time entries with optional filtering by user, project, or date range.

**When to use**: Review tracked time, generate reports, or verify time logs.

**Arguments**:
- `userId` (optional): User ID
- `projectId` (optional): Project ID
- `from` (optional): From date (YYYY-MM-DD)
- `to` (optional): To date (YYYY-MM-DD)

**Example LLM prompt**: "Show me all time entries for project 123 this month"

---

### harvest_get_time_entry

**What it does**: Gets detailed information about a specific time entry.

**When to use**: View details of a particular tracked time block.

**Arguments**:
- `id` (required): Time entry ID

**Example LLM prompt**: "Get details for time entry 456"

---

### harvest_create_time_entry

**What it does**: Creates a new time entry for a project and task.

**When to use**: Log time worked on a task or project.

**Arguments**:
- `projectId` (required): Project ID
- `taskId` (required): Task ID
- `spentDate` (required): Date (YYYY-MM-DD)
- `hours` (optional): Hours worked
- `notes` (optional): Notes

**Example LLM prompt**: "Log 3 hours on project 123 for today with notes 'API integration'"

---

### harvest_update_time_entry

**What it does**: Updates an existing time entry.

**When to use**: Correct time entry details, change project, or adjust hours.

**Arguments**:
- `id` (required): Time entry ID
- `projectId` (optional): New project ID
- `taskId` (optional): New task ID
- `spentDate` (optional): New date (YYYY-MM-DD)
- `hours` (optional): New hours
- `notes` (optional): New notes

**Example LLM prompt**: "Update time entry 456 to 4 hours and add notes 'Follow-up call'"

---

### harvest_list_projects

**What it does**: Lists all projects with optional filtering by client or status.

**When to use**: View available projects for time tracking.

**Arguments**:
- `clientId` (optional): Client ID
- `isActive` (optional): Filter by active status

**Example LLM prompt**: "List all active projects for client 789"

---

### harvest_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details including budget, tasks, and team.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 123"

---

### harvest_list_clients

**What it does**: Lists all clients.

**When to use**: View available clients for project assignment.

**Arguments**:
- `isActive` (optional): Filter by active status

**Example LLM prompt**: "List all active clients"

---

### harvest_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: View client details including contact info and associated projects.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get details for client 456"

---

### harvest_list_tasks

**What it does**: Lists all tasks.

**When to use**: View available tasks for time entry creation.

**Arguments**:
- `isActive` (optional): Filter by active status

**Example LLM prompt**: "List all tasks"

---

### harvest_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: View task details including default hours and billing rates.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 789"

---

## Harvest API Notes

- **Date format**: Harvest uses YYYY-MM-DD format for dates
- **Hours format**: Hours are specified as decimal numbers (e.g., 1.5 for 1.5 hours)
- **Projects**: Contain tasks and belong to clients
- **Tasks**: Define the type of work that can be tracked against projects
- **Time entries**: Link a user to a project and task with date and hours
- **Active status**: Both projects and tasks can be active or archived

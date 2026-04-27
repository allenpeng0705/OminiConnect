# Toggl Tools

Provider: `toggl` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Toggl Track API. They allow AI agents to track time, manage projects and clients, and generate reports. Toggl Track is a popular time tracking tool known for its simplicity and powerful reporting capabilities.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Toggl Track
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `time_entries:read`, `time_entries:write`, `projects:read`, `clients:read`, `reports:read`, `workspaces:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `toggl_list_time_entries` | List time entries | GET | /me/time-entries |
| `toggl_get_time_entry` | Get time entry details | GET | /time-entries/{id} |
| `toggl_create_time_entry` | Create a new time entry | POST | /time-entries |
| `toggl_update_time_entry` | Update a time entry | PUT | /time-entries/{id} |
| `toggl_list_projects` | List projects | GET | /workspaces/{workspace_id}/projects |
| `toggl_get_project` | Get project details | GET | /projects/{id} |
| `toggl_list_clients` | List clients | GET | /workspaces/{workspace_id}/clients |
| `toggl_get_client` | Get client details | GET | /clients/{id} |
| `toggl_get_weekly_report` | Get weekly report | GET | /reports/weekly |
| `toggl_list_workspaces` | List workspaces | GET | /workspaces |

---

## Tool Details

### toggl_list_time_entries

**What it does**: Lists time entries with optional filtering by date range, project, or workspace.

**When to use**: Review tracked time, generate reports, or verify time logs.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)
- `workspace_id` (optional): Workspace ID
- `project_id` (optional): Project ID

**Example LLM prompt**: "Show me all time entries from this week"

---

### toggl_get_time_entry

**What it does**: Gets detailed information about a specific time entry.

**When to use**: View details of a particular tracked time block.

**Arguments**:
- `id` (required): Time entry ID

**Example LLM prompt**: "Get details for time entry 12345"

---

### toggl_create_time_entry

**What it does**: Creates a new time entry with description, start time, duration, and project.

**When to use**: Log time worked on a task or project.

**Arguments**:
- `description` (required): Time entry description
- `start` (required): Start time (ISO 8601)
- `duration` (optional): Duration in seconds
- `project_id` (optional): Project ID

**Example LLM prompt**: "Create a time entry for 2 hours on project 456"

---

### toggl_update_time_entry

**What it does**: Updates an existing time entry.

**When to use**: Correct time entry details, change project, or adjust duration.

**Arguments**:
- `id` (required): Time entry ID
- `description` (optional): New description
- `start` (optional): New start time (ISO 8601)
- `duration` (optional): New duration in seconds
- `project_id` (optional): New project ID

**Example LLM prompt**: "Update time entry 12345 to add description 'Final review'"

---

### toggl_list_projects

**What it does**: Lists all projects in a workspace.

**When to use**: View available projects for time tracking.

**Arguments**:
- `workspace_id` (required): Workspace ID

**Example LLM prompt**: "List all projects in workspace 123"

---

### toggl_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details including client, color, and billing rates.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 456"

---

### toggl_list_clients

**What it does**: Lists all clients in a workspace.

**When to use**: View available clients for project assignment.

**Arguments**:
- `workspace_id` (required): Workspace ID

**Example LLM prompt**: "List all clients in workspace 123"

---

### toggl_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: View client details including associated projects.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get details for client 789"

---

### toggl_get_weekly_report

**What it does**: Gets a weekly time report for a workspace.

**When to use**: Generate weekly summaries of time tracked.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `start_date` (optional): Start date for the report

**Example LLM prompt**: "Get weekly report for workspace 123"

---

### toggl_list_workspaces

**What it does**: Lists all workspaces for the user.

**When to use**: Find available workspaces and get workspace IDs.

**Arguments**: None

**Example LLM prompt**: "List all my workspaces"

---

## Toggl API Notes

- **Duration format**: Toggl uses seconds for duration. 2 hours = 7200 seconds
- **Time format**: ISO 8601 format for all time values
- **Workspace**: The workspace is the top-level organizational unit in Toggl
- **Projects**: Belong to a workspace and optionally to a client
- **Time entries**: Can be manual (with duration) or running (with start time but no duration)
- **Billable**: Projects and time entries can be marked as billable

# Clockify Tools

Provider: `clockify` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Clockify API. They allow AI agents to track time, manage workspaces, projects, and users. Clockify is a popular free time tracking tool with robust reporting and team management features.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Clockify
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read` (for reading data), `write` (for creating/updating data)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clockify_list_time_entries` | List time entries | GET | /workspaces/{workspace_id}/time-entries |
| `clockify_get_time_entry` | Get time entry details | GET | /workspaces/{workspace_id}/time-entries/{id} |
| `clockify_create_time_entry` | Create a new time entry | POST | /workspaces/{workspace_id}/time-entries |
| `clockify_update_time_entry` | Update a time entry | PUT | /workspaces/{workspace_id}/time-entries/{id} |
| `clockify_list_projects` | List projects | GET | /workspaces/{workspace_id}/projects |
| `clockify_get_project` | Get project details | GET | /workspaces/{workspace_id}/projects/{id} |
| `clockify_list_workspaces` | List workspaces | GET | /workspaces |
| `clockify_get_workspace` | Get workspace details | GET | /workspaces/{id} |
| `clockify_get_time_report` | Get time report | POST | /workspaces/{workspace_id}/reports/summary |
| `clockify_list_users` | List users | GET | /workspaces/{workspace_id}/users |

---

## Tool Details

### clockify_list_time_entries

**What it does**: Lists time entries for a workspace with optional filtering by user, project, or date range.

**When to use**: Review tracked time, generate reports, or verify time logs.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `user_id` (optional): User ID
- `project_id` (optional): Project ID
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show me all time entries for workspace 123 this week"

---

### clockify_get_time_entry

**What it does**: Gets detailed information about a specific time entry.

**When to use**: View details of a particular tracked time block.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `id` (required): Time entry ID

**Example LLM prompt**: "Get details for time entry 456 in workspace 123"

---

### clockify_create_time_entry

**What it does**: Creates a new time entry with project, description, and time range.

**When to use**: Log time worked on a task or project.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `project_id` (required): Project ID
- `description` (optional): Time entry description
- `start` (optional): Start time (ISO 8601)
- `end` (optional): End time (ISO 8601)

**Example LLM prompt**: "Create a time entry for 2 hours on project 456 in workspace 123"

---

### clockify_update_time_entry

**What it does**: Updates an existing time entry.

**When to use**: Correct time entry details, change project, or adjust time range.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `id` (required): Time entry ID
- `project_id` (optional): New project ID
- `description` (optional): New description
- `start` (optional): New start time (ISO 8601)
- `end` (optional): New end time (ISO 8601)

**Example LLM prompt**: "Update time entry 456 in workspace 123 to change description to 'Final review'"

---

### clockify_list_projects

**What it does**: Lists all projects in a workspace.

**When to use**: View available projects for time tracking.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `archived` (optional): Include archived projects

**Example LLM prompt**: "List all projects in workspace 123"

---

### clockify_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details including tasks, time tracked, and team.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 456 in workspace 123"

---

### clockify_list_workspaces

**What it does**: Lists all workspaces for the authenticated user.

**When to use**: Find available workspaces and get workspace IDs.

**Arguments**: None

**Example LLM prompt**: "List all my workspaces"

---

### clockify_get_workspace

**What it does**: Gets detailed information about a specific workspace.

**When to use**: View workspace settings, admin info, and member list.

**Arguments**:
- `id` (required): Workspace ID

**Example LLM prompt**: "Get details for workspace 123"

---

### clockify_get_time_report

**What it does**: Gets a detailed time report for a workspace with grouping options.

**When to use**: Generate time reports by project, user, or other dimensions.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `start_date` (required): Start date for the report
- `end_date` (required): End date for the report
- `group_by` (optional): Grouping option (e.g., project, user)

**Example LLM prompt**: "Get time report for workspace 123 grouped by project"

---

### clockify_list_users

**What it does**: Lists all users in a workspace.

**When to use**: View team members or find user IDs for time entry filtering.

**Arguments**:
- `workspace_id` (required): Workspace ID

**Example LLM prompt**: "List all users in workspace 123"

---

## Clockify API Notes

- **Workspace-centric**: Clockify organizes everything around workspaces
- **Time format**: ISO 8601 format for all time values
- **Time entries**: Can have start/end times or be duration-based
- **Projects**: Belong to workspaces and can be organized with tasks
- **Users**: Can be members of multiple workspaces with different roles
- **Archived projects**: Projects can be archived but still retain historical data

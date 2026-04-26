# Timeular Tools

Provider: `timeular` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Timeular API. They allow AI agents to track time, manage activities and projects, and generate reports. Timeular is a time tracking platform known for its physical device and intuitive interface.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Timeular
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `time_entries:read`, `time_entries:write`, `activities:read`, `projects:read`, `reports:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `timeular_list_time_entries` | List time entries | GET | /time-entries |
| `timeular_get_time_entry` | Get time entry details | GET | /time-entries/{id} |
| `timeular_create_time_entry` | Create a new time entry | POST | /time-entries |
| `timeular_stop_time_entry` | Stop tracking time | POST | /time-entries/{id}/stop |
| `timeular_list_activities` | List activities | GET | /activities |
| `timeular_get_activity` | Get activity details | GET | /activities/{id} |
| `timeular_list_projects` | List projects | GET | /projects |
| `timeular_get_project` | Get project details | GET | /projects/{id} |
| `timeular_get_daily_summary` | Get daily summary | GET | /reports/daily-summary |
| `timeular_list_users` | List users | GET | /users |

---

## Tool Details

### timeular_list_time_entries

**What it does**: Retrieves time entries with optional date range filtering.

**When to use**: Review tracked time, generate reports, or verify time logs.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show me all time entries from this week"

---

### timeular_get_time_entry

**What it does**: Gets detailed information about a specific time entry.

**When to use**: View details of a particular tracked time block.

**Arguments**:
- `id` (required): Time entry ID

**Example LLM prompt**: "Get details for time entry 12345"

---

### timeular_create_time_entry

**What it does**: Creates a new time entry manually with activity, time range, and optional note.

**When to use**: Log time worked on a task or project.

**Arguments**:
- `activity_id` (required): Activity ID to track
- `start_time` (required): Start time in ISO 8601 format
- `end_time` (required): End time in ISO 8601 format
- `note` (optional): Optional note for the entry

**Example LLM prompt**: "Create a time entry for activity 5 from 9am to 11am"

---

### timeular_stop_time_entry

**What it does**: Stops the currently running time tracking entry.

**When to use**: End a time tracking session.

**Arguments**:
- `id` (required): Time entry ID to stop

**Example LLM prompt**: "Stop time entry 12345"

---

### timeular_list_activities

**What it does**: Retrieves all available activities for time tracking.

**When to use**: View available activities before starting time tracking.

**Arguments**: None

**Example LLM prompt**: "Show me all my activities"

---

### timeular_get_activity

**What it does**: Gets detailed information about a specific activity.

**When to use**: View activity details before tracking time.

**Arguments**:
- `id` (required): Activity ID

**Example LLM prompt**: "Get details for activity 5"

---

### timeular_list_projects

**What it does**: Retrieves all projects for time tracking.

**When to use**: View available projects for time tracking.

**Arguments**: None

**Example LLM prompt**: "Show me all projects"

---

### timeular_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details and time tracked.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 123"

---

### timeular_get_daily_summary

**What it does**: Gets a summary of time tracked for a specific day.

**When to use**: Review daily time totals by activity.

**Arguments**:
- `date` (required): Date to get summary for (YYYY-MM-DD)

**Example LLM prompt**: "Get daily summary for 2026-04-26"

---

### timeular_list_users

**What it does**: Retrieves all users in the Timeular workspace.

**When to use**: View team members or find user IDs.

**Arguments**: None

**Example LLM prompt**: "List all users in the workspace"

---

## Timeular API Notes

- **Time format**: ISO 8601 format for all time values
- **Activities**: Activities are the categories used to track time (e.g., "Client Work", "Meetings")
- **Projects**: Projects organize time tracking entries
- **Time entries**: Records of time tracked with start/end times and associated activities
- **Daily summary**: Provides aggregated time totals per activity for a given day

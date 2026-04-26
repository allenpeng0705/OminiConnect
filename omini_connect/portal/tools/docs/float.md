# Float Tools

Provider: `float` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Float API. They allow AI agents to manage projects, tasks, team members, and resource scheduling. Float is a resource management and project planning platform.

## Authentication

**Nango API_KEY**:
- User provides their Float API key via Nango Connect
- Key is passed in the Authorization header as Bearer token
- User-Agent header includes app details
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `float_list_projects` | List projects | GET | /api/v3/projects |
| `float_get_project` | Get project details | GET | /api/v3/projects/{id} |
| `float_list_tasks` | List tasks | GET | /api/v3/tasks |
| `float_get_task` | Get task details | GET | /api/v3/tasks/{id} |
| `float_list_people` | List team members | GET | /api/v3/people |
| `float_get_person` | Get person details | GET | /api/v3/people/{id} |
| `float_list_clients` | List clients | GET | /api/v3/clients |
| `float_get_client` | Get client details | GET | /api/v3/clients/{id} |
| `float_list_holidays` | List holidays | GET | /api/v3/holidays |
| `float_get_schedule` | Get project schedule | GET | /api/v3/schedule |

---

## Tool Details

### float_list_projects

**What it does**: Lists all projects.

**When to use**: Browse project list, find project IDs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all projects"

---

### float_get_project

**What it does**: Gets details of a specific project.

**When to use**: View project information, status.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project abc123"

---

### float_list_tasks

**What it does**: Lists all tasks.

**When to use**: Browse tasks, find task IDs.

**Arguments**:
- `project_id` (optional): Filter by project

**Example LLM prompt**: "List tasks for project abc123"

---

### float_get_task

**What it does**: Gets details of a specific task.

**When to use**: View task details, assignment.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task xyz789"

---

### float_list_people

**What it does**: Lists all team members.

**When to use**: View team roster.

**Arguments**: None

**Example LLM prompt**: "List all team members"

---

### float_get_person

**What it does**: Gets details of a specific person.

**When to use**: View person info, availability.

**Arguments**:
- `id` (required): Person ID

**Example LLM prompt**: "Get person def456"

---

### float_list_clients

**What it does**: Lists all clients.

**When to use**: Browse client list.

**Arguments**: None

**Example LLM prompt**: "List all clients"

---

### float_get_client

**What it does**: Gets details of a specific client.

**When to use**: View client information.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get client ghi789"

---

### float_list_holidays

**What it does**: Lists all holidays.

**When to use**: View holiday calendar.

**Arguments**:
- `year` (optional): Year

**Example LLM prompt**: "List holidays for this year"

---

### float_get_schedule

**What it does**: Gets project schedule with allocations.

**When to use**: View resource allocation, project timeline.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get schedule for the next month"

---

## Float API Notes

- **Projects**: Client projects with tasks and timelines
- **Tasks**: Work items within projects
- **People**: Team members with availability
- **Clients**: Customer companies
- **Holidays**: Company holiday calendar
- **Schedule**: Resource allocation across projects

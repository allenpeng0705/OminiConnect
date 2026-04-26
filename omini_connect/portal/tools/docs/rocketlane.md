# Rocketlane Tools

Provider: `rocketlane` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Rocketlane is a project management and client portal platform for agencies. These tools allow AI agents to manage projects, tasks, clients, and time entries.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Rocketlane
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projects:read`, `projects:write`, `tasks:read`, `tasks:write`, `clients:read`, `clients:write`, `time-entries:read`, `time-entries:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rocketlane_list_projects` | List all projects | GET | /v1/projects |
| `rocketlane_get_project` | Get project details | GET | /v1/projects/{projectId} |
| `rocketlane_create_project` | Create a project | POST | /v1/projects |
| `rocketlane_list_tasks` | List tasks in a project | GET | /v1/projects/{projectId}/tasks |
| `rocketlane_create_task` | Create a task | POST | /v1/tasks |
| `rocketlane_list_clients` | List all clients | GET | /v1/clients |
| `rocketlane_get_client` | Get client details | GET | /v1/clients/{clientId} |
| `rocketlane_create_client` | Create a client | POST | /v1/clients |
| `rocketlane_list_time_entries` | List time entries | GET | /v1/time-entries |
| `rocketlane_create_time_entry` | Log time entry | POST | /v1/time-entries |

---

## Tool Details

### rocketlane_list_projects

**What it does**: Returns a list of all projects.

**When to use**: View active projects, track client work.

**Arguments**:
- `limit` (optional): Number of projects (default 50)
- `status` (optional): Filter by status (active, completed, on-hold)

**Example LLM prompt**: "List all active projects"

---

### rocketlane_get_project

**What it does**: Gets details of a specific project.

**When to use**: Get project status, timeline, and team.

**Arguments**:
- `projectId` (required): The project ID

**Example LLM prompt**: "Get details for project prj_abc123"

---

### rocketlane_create_project

**What it does**: Creates a new project.

**When to use**: Start a new client engagement.

**Arguments**:
- `name` (required): Project name
- `clientId` (required): Client ID
- `status` (optional): Project status (default active)
- `startDate` (optional): Start date

**Example LLM prompt**: "Create a project called 'Website Redesign' for client cli_456"

---

### rocketlane_list_tasks

**What it does**: Returns tasks for a project.

**When to use**: View project tasks and progress.

**Arguments**:
- `projectId` (required): The project ID
- `limit` (optional): Number of tasks (default 50)

**Example LLM prompt**: "List all tasks for project prj_abc123"

---

### rocketlane_create_task

**What it does**: Creates a new task in a project.

**When to use**: Add work items to a project.

**Arguments**:
- `projectId` (required): Project ID
- `title` (required): Task title
- `assigneeId` (optional): Assignee user ID
- `dueDate` (optional): Due date (ISO 8601)

**Example LLM prompt**: "Create a task 'Design homepage' in project prj_abc123"

---

### rocketlane_list_clients

**What it does**: Returns a list of all clients.

**When to use**: View client roster.

**Arguments**:
- `limit` (optional): Number of clients (default 50)

**Example LLM prompt**: "List all clients"

---

### rocketlane_get_client

**What it does**: Gets details of a specific client.

**When to use**: Get client information and projects.

**Arguments**:
- `clientId` (required): The client ID

**Example LLM prompt**: "Get details for client cli_xyz789"

---

### rocketlane_create_client

**What it does**: Creates a new client.

**When to use**: Add a new client to your roster.

**Arguments**:
- `name` (required): Client name
- `email` (optional): Client email

**Example LLM prompt**: "Create a client called 'Acme Corp'"

---

### rocketlane_list_time_entries

**What it does**: Returns time entries.

**When to use**: Track time spent on projects.

**Arguments**:
- `projectId` (optional): Filter by project
- `limit` (optional): Number of entries (default 50)

**Example LLM prompt**: "List time entries for project prj_abc123"

---

### rocketlane_create_time_entry

**What it does**: Logs a time entry.

**When to use**: Track work on tasks.

**Arguments**:
- `taskId` (required): Task ID
- `duration` (required): Duration in minutes
- `projectId` (optional): Project ID
- `description` (optional): Entry description

**Example LLM prompt**: "Log 60 minutes on task tsk_123"

---

## Rocketlane Notes

- Projects belong to clients and contain tasks
- Tasks can be assigned to team members
- Time entries track work duration
- Client portal provides visibility to clients

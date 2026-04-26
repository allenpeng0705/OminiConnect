# Ordinal Tools

Provider: `ordinal` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Ordinal productivity API. They allow AI agents to manage workspaces, projects, tasks, and comments. **Requires Ordinal API key authentication.**

## Authentication

**API Key**:
- User provides Ordinal API key
- Key passed via `Authorization: Bearer` header
- Base URL: `https://app.tryordinal.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ordinal_list_workspaces` | List workspaces | GET | /v1/workspace |
| `ordinal_get_workspace` | Get workspace details | GET | /v1/workspace/{id} |
| `ordinal_list_projects` | List projects | GET | /v1/projects |
| `ordinal_get_project` | Get project details | GET | /v1/projects/{id} |
| `ordinal_create_task` | Create task | POST | /v1/tasks |
| `ordinal_list_tasks` | List tasks | GET | /v1/tasks |
| `ordinal_update_task` | Update task | PUT | /v1/tasks/{id} |
| `ordinal_get_comments` | Get comments | GET | /v1/comments |
| `ordinal_create_comment` | Create comment | POST | /v1/comments |
| `ordinal_search` | Search | GET | /v1/search |

---

## Tool Details

### ordinal_list_workspaces

**What it does**: Lists all workspaces in Ordinal.

**When to use**: Browse workspaces, switch contexts.

**Arguments**: None

**Example LLM prompt**: "List all my workspaces"

---

### ordinal_get_workspace

**What it does**: Gets detailed information for a specific workspace.

**When to use**: View workspace settings, details.

**Arguments**:
- `id` (required): Workspace ID

**Example LLM prompt**: "Get details for workspace ABC123"

---

### ordinal_list_projects

**What it does**: Lists all projects in Ordinal.

**When to use**: Browse projects, find work areas.

**Arguments**:
- `workspace_id` (optional): Workspace ID

**Example LLM prompt**: "List all projects in workspace ABC123"

---

### ordinal_get_project

**What it does**: Gets detailed information for a specific project.

**When to use**: View project details, progress.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 12345"

---

### ordinal_create_task

**What it does**: Creates a new task in Ordinal.

**When to use**: Create work items, add tasks.

**Arguments**:
- `title` (required): Task title
- `project_id` (required): Project ID
- `description` (optional): Task description

**Example LLM prompt**: "Create a task called 'Review PR' in project 12345"

---

### ordinal_list_tasks

**What it does**: Lists all tasks in Ordinal.

**When to use**: Browse tasks, track work.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all open tasks in project 12345"

---

### ordinal_update_task

**What it does**: Updates an existing task.

**When to use**: Modify task, change status.

**Arguments**:
- `id` (required): Task ID
- `title` (optional): Task title
- `status` (optional): Task status

**Example LLM prompt**: "Update task 67890 status to completed"

---

### ordinal_get_comments

**What it does**: Gets comments for a task or project.

**When to use**: View discussions, history.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Get comments for task 67890"

---

### ordinal_create_comment

**What it does**: Creates a comment on a task.

**When to use**: Add feedback, start discussions.

**Arguments**:
- `task_id` (required): Task ID
- `content` (required): Comment content

**Example LLM prompt**: "Add comment 'Looks good!' to task 67890"

---

### ordinal_search

**What it does**: Searches for tasks, projects, and other records.

**When to use**: Find specific items, locate content.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for tasks containing 'review'"

---

## Ordinal Notes

- **API Key format**: Starts with `ord_` prefix
- **Workspaces**: Top-level organizational units
- **Projects**: Contain related tasks
- **Tasks**: Work items within projects
- **Comments**: Collaborative discussions on tasks

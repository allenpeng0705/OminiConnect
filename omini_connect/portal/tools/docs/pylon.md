# Pylon Tools

Provider: `pylon` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Pylon API. They allow AI agents to manage tasks, projects, comments, and teams. Pylon is a task management platform. **Requires Pylon API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.usepylon.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pylon_list_tasks` | List tasks | GET | /me |
| `pylon_get_task` | Get task details | GET | /me |
| `pylon_create_task` | Create a task | POST | /me |
| `pylon_update_task` | Update a task | PUT | /me |
| `pylon_list_projects` | List projects | GET | /me |
| `pylon_get_project` | Get project details | GET | /me |
| `pylon_list_comments` | List comments | GET | /me |
| `pylon_get_user` | Get user info | GET | /me |
| `pylon_list_teams` | List teams | GET | /me |
| `pylon_get_task_stats` | Get task stats | GET | /me |

---

## Tool Details

### pylon_list_tasks

**What it does**: Lists all tasks.

**When to use**: Browse task list.

**Arguments**:
- `projectId` (optional): Filter by project
- `status` (optional): Filter by status

**Example LLM prompt**: "List all tasks"

---

### pylon_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: Get task details.

**Arguments**:
- `taskId` (required): Task ID

**Example LLM prompt**: "Get details for task 12345"

---

### pylon_create_task

**What it does**: Creates a new task.

**When to use**: Add new work item.

**Arguments**:
- `name` (required): Task name
- `projectId` (optional): Project ID

**Example LLM prompt**: "Create a task called 'Review PR'"

---

### pylon_update_task

**What it does**: Updates an existing task.

**When to use**: Modify task.

**Arguments**:
- `taskId` (required): Task ID
- `name` (optional): Task name
- `status` (optional): Task status

**Example LLM prompt**: "Update task 12345 to completed"

---

### pylon_list_projects

**What it does**: Lists all projects.

**When to use**: Browse projects.

**Arguments**: None

**Example LLM prompt**: "List all projects"

---

### pylon_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: Get project details.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project 67890"

---

### pylon_list_comments

**What it does**: Lists all comments.

**When to use**: Browse comments.

**Arguments**:
- `taskId` (optional): Filter by task

**Example LLM prompt**: "List comments for task 12345"

---

### pylon_get_user

**What it does**: Gets current user information.

**When to use**: Get user profile.

**Arguments**: None

**Example LLM prompt**: "Get my user information"

---

### pylon_list_teams

**What it does**: Lists all teams.

**When to use**: Browse teams.

**Arguments**: None

**Example LLM prompt**: "List all teams"

---

### pylon_get_task_stats

**What it does**: Gets task statistics.

**When to use**: Analytics overview.

**Arguments**:
- `projectId` (optional): Filter by project

**Example LLM prompt**: "Get task statistics for project 67890"

---

## Pylon API Notes

- **API Key**: Uses Bearer token for authentication
- **Task Management**: Projects, tasks, comments
- **Rate limits**: Apply to API calls

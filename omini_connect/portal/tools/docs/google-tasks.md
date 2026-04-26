# Google Tasks Tools

Provider: `google-tasks` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Tasks API. They allow AI agents to manage task lists and tasks. **Requires Google OAuth2 with Tasks permissions.**

## Authentication

**Nango OAUTH2 (Google Tasks)**:
- User authenticates via OAuth2 with Tasks scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://tasks.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_tasks_list_tasklists` | List task lists | GET | /tasks/v1/users/@me/lists |
| `google_tasks_get_tasklist` | Get task list details | GET | /tasks/v1/users/@me/lists/{tasklist} |
| `google_tasks_create_tasklist` | Create task list | POST | /tasks/v1/users/@me/lists |
| `google_tasks_update_tasklist` | Update task list | PUT | /tasks/v1/users/@me/lists/{tasklist} |
| `google_tasks_delete_tasklist` | Delete task list | DELETE | /tasks/v1/users/@me/lists/{tasklist} |
| `google_tasks_list_tasks` | List tasks | GET | /tasks/v1/lists/{tasklist}/tasks |
| `google_tasks_get_task` | Get task details | GET | /tasks/v1/lists/{tasklist}/tasks/{task} |
| `google_tasks_create_task` | Create task | POST | /tasks/v1/lists/{tasklist}/tasks |
| `google_tasks_update_task` | Update task | PUT | /tasks/v1/lists/{tasklist}/tasks/{task} |
| `google_tasks_delete_task` | Delete task | DELETE | /tasks/v1/lists/{tasklist}/tasks/{task} |

---

## Tool Details

### google_tasks_list_tasklists

**What it does**: Lists all task lists.

**When to use**: Browse available task lists.

**Arguments**: None

**Example LLM prompt**: "List all my task lists"

---

### google_tasks_get_tasklist

**What it does**: Gets task list details.

**When to use**: Get task list info.

**Arguments**:
- `tasklist` (required): Task list ID

**Example LLM prompt**: "Get task list abc123"

---

### google_tasks_create_tasklist

**What it does**: Creates a new task list.

**When to use**: Create new todo lists.

**Arguments**:
- `title` (required): Task list title

**Example LLM prompt**: "Create a task list called 'Shopping'"

---

### google_tasks_update_tasklist

**What it does**: Updates a task list.

**When to use**: Rename task lists.

**Arguments**:
- `tasklist` (required): Task list ID
- `title` (optional): New title

**Example LLM prompt**: "Rename task list abc123 to 'Errands'"

---

### google_tasks_delete_tasklist

**What it does**: Deletes a task list.

**When to use**: Remove unwanted lists.

**Arguments**:
- `tasklist` (required): Task list ID

**Example LLM prompt**: "Delete task list abc123"

---

### google_tasks_list_tasks

**What it does**: Lists tasks in a task list.

**When to use**: View tasks.

**Arguments**:
- `tasklist` (optional): Task list ID (default: @default)
- `show_completed` (optional): Show completed tasks (default: true)

**Example LLM prompt**: "List tasks in my default list"

---

### google_tasks_get_task

**What it does**: Gets task details.

**When to use**: Get task info.

**Arguments**:
- `tasklist` (optional): Task list ID (default: @default)
- `task` (required): Task ID

**Example LLM prompt**: "Get task xyz789"

---

### google_tasks_create_task

**What it does**: Creates a new task.

**When to use**: Add new todos.

**Arguments**:
- `tasklist` (optional): Task list ID (default: @default)
- `title` (required): Task title
- `notes` (optional): Task notes
- `due` (optional): Due date (YYYY-MM-DD)

**Example LLM prompt**: "Create a task 'Buy milk' due tomorrow"

---

### google_tasks_update_task

**What it does**: Updates a task.

**When to use**: Modify or complete tasks.

**Arguments**:
- `tasklist` (optional): Task list ID (default: @default)
- `task` (required): Task ID
- `title` (optional): New title
- `status` (optional): Status (needsAction, completed)

**Example LLM prompt**: "Mark task xyz789 as completed"

---

### google_tasks_delete_task

**What it does**: Deletes a task.

**When to use**: Remove tasks.

**Arguments**:
- `tasklist` (optional): Task list ID (default: @default)
- `task` (required): Task ID

**Example LLM prompt**: "Delete task xyz789"

---

## Google Tasks API Notes

- **Task list ID**: Unique identifier for task lists
- **@default**: Refers to the default task list
- **Task status**: needsAction (incomplete) or completed
- **Due dates**: YYYY-MM-DD format
- **Hierarchy**: Tasks can have subtasks

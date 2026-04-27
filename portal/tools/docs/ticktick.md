# TickTick Tools

Provider: `ticktick` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

TickTick is a task and project management application. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TickTick
- Token stored in Nango, accessed via `connection_ref`
- Scopes: tasks:read, tasks:write, projects:read, projects:write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ticktick_list_tasks` | List all tasks | GET | /open/v1/task |
| `ticktick_get_task` | Get task details | GET | /open/v1/task/{task_id} |
| `ticktick_create_task` | Create a new task | POST | /open/v1/task |
| `ticktick_update_task` | Update an existing task | POST | /open/v1/task/{task_id} |
| `ticktick_delete_task` | Delete a task | DELETE | /open/v1/task/{task_id} |
| `ticktick_list_projects` | List all projects | GET | /open/v1/project |
| `ticktick_get_project` | Get project details | GET | /open/v1/project/{project_id} |
| `ticktick_create_project` | Create a new project | POST | /open/v1/project |
| `ticktick_list_sections` | List sections in a project | GET | /open/v1/project/{project_id}/section |
| `ticktick_list_tags` | List all tags | GET | /open/v1/tag |

---

## Tool Details

### ticktick_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_get_task

**What it does**: Get task details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_create_task

**What it does**: Create a new task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_update_task

**What it does**: Update an existing task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_delete_task

**What it does**: Delete a task

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_list_projects

**What it does**: List all projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_get_project

**What it does**: Get project details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_create_project

**What it does**: Create a new project

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_list_sections

**What it does**: List sections in a project

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ticktick_list_tags

**What it does**: List all tags

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.ticktick.com`
- Docs: https://nango.dev/docs/integrations/all/ticktick

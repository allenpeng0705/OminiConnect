# Devin Tools

Provider: `devin` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Devin AI API. They allow AI agents to manage sessions, tasks, files, and view usage statistics. Devin is an AI coding assistant platform.

## Authentication

**Nango API_KEY**:
- User provides their Devin API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Format: `apk_user_` followed by base64-encoded credentials

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `devin_list_sessions` | List sessions | GET | /v1/sessions |
| `devin_get_session` | Get session details | GET | /v1/sessions/{session_id} |
| `devin_create_session` | Create a new session | POST | /v1/sessions |
| `devin_list_tasks` | List tasks | GET | /v1/tasks |
| `devin_get_task` | Get task details | GET | /v1/tasks/{task_id} |
| `devin_create_task` | Create a task | POST | /v1/tasks |
| `devin_list_files` | List files | GET | /v1/files |
| `devin_get_file` | Get file content | GET | /v1/files/{file_id} |
| `devin_list_models` | List available models | GET | /v1/models |
| `devin_get_usage` | Get usage statistics | GET | /v1/usage |

---

## Tool Details

### devin_list_sessions

**What it does**: Lists all Devin AI sessions.

**When to use**: View session history, find active sessions, track AI interactions.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by active, completed, or cancelled

**Example LLM prompt**: "List all active sessions"

---

### devin_get_session

**What it does**: Gets detailed session information.

**When to use**: Review session details, check session state, continue previous work.

**Arguments**:
- `session_id` (required): Session ID

**Example LLM prompt**: "Get details for session s-123"

---

### devin_create_session

**What it does**: Creates a new Devin AI session.

**When to use**: Start new AI tasks, create coding sessions, initiate AI workflows.

**Arguments**:
- `model` (optional): Model to use
- `prompt` (required): Initial prompt or task description

**Example LLM prompt**: "Create a new session with prompt 'Write a Python function to calculate fibonacci'"

---

### devin_list_tasks

**What it does**: Lists all tasks in Devin.

**When to use**: View project tasks, track progress, manage work items.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all tasks"

---

### devin_get_task

**What it does**: Gets detailed task information.

**When to use**: Review task details, check task status, update task progress.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Get details for task t-456"

---

### devin_create_task

**What it does**: Creates a new task.

**When to use**: Create work items, add project tasks, track deliverables.

**Arguments**:
- `name` (required): Task name
- `description` (optional): Task description

**Example LLM prompt**: "Create a task named 'Implement login feature' with description 'Add OAuth2 login to the app'"

---

### devin_list_files

**What it does**: Lists all files in the Devin workspace.

**When to use**: Browse workspace files, find source files, navigate project structure.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all files in workspace"

---

### devin_get_file

**What it does**: Gets the content of a specific file.

**When to use**: Read source code, review file contents, get file for editing.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Get content of file f-789"

---

### devin_list_models

**What it does**: Lists all available AI models.

**When to use**: View available models, select appropriate model, compare capabilities.

**Arguments**: None

**Example LLM prompt**: "List all available models"

---

### devin_get_usage

**What it does**: Gets usage statistics and quota information.

**When to use**: Check usage limits, track billing, monitor API consumption.

**Arguments**: None

**Example LLM prompt**: "Get my usage statistics"

---

## Devin API Notes

- **Sessions**: AI interaction sessions with specific tasks
- **Tasks**: Work items and project management
- **Files**: Source code and documents in workspace
- **Models**: Available AI models for different tasks
- **Usage**: API usage tracking and quotas
- **API Key Format**: `apk_user_` followed by base64-encoded credentials
- **Verification**: GET /v1/sessions confirms API key validity

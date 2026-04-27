# Circleback MCP Tools

Provider: `circleback-mcp` | Engine: `nango` | Auth: MCP OAuth2 via Nango

## Overview

These tools wrap the Circleback MCP API. Circleback is an AI-powered meeting notes and action tracking platform. **Requires Circleback MCP OAuth access.**

## Authentication

**Nango MCP_OAUTH2**:
- User authenticates via Nango Connect with Circleback MCP
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://app.circleback.ai/api/`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `circleback_list_actions` | List actions | GET | /api/v1/actions |
| `circleback_get_action` | Get action details | GET | /api/v1/actions/{id} |
| `circleback_create_action` | Create an action | POST | /api/v1/actions |
| `circleback_update_action` | Update an action | PUT | /api/v1/actions/{id} |
| `circleback_list_workflows` | List workflows | GET | /api/v1/workflows |
| `circleback_get_workflow` | Get workflow details | GET | /api/v1/workflows/{id} |
| `circleback_list_tasks` | List tasks | GET | /api/v1/tasks |
| `circleback_get_task` | Get task details | GET | /api/v1/tasks/{id} |
| `circleback_create_task` | Create a task | POST | /api/v1/tasks |
| `circleback_list_team_members` | List team members | GET | /api/v1/team/members |

---

## Tool Details

### circleback_list_actions

**What it does**: Lists all actions in the workspace.

**When to use**: View all tracked action items.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Circleback actions"

---

### circleback_get_action

**What it does**: Gets details of a specific action.

**When to use**: View action details and status.

**Arguments**:
- `id` (required): Action ID

**Example LLM prompt**: "Get action 123 details"

---

### circleback_create_action

**What it does**: Creates a new action.

**When to use**: Track a new action item from a meeting.

**Arguments**:
- `title` (required): Action title
- `description` (optional): Action description
- `due_date` (optional): Due date

**Example LLM prompt**: "Create an action to follow up with the client"

---

### circleback_update_action

**What it does**: Updates an existing action.

**When to use**: Mark action complete or update details.

**Arguments**:
- `id` (required): Action ID
- `title` (optional): New title
- `completed` (optional): Mark as completed

**Example LLM prompt**: "Mark action 123 as completed"

---

### circleback_list_workflows

**What it does**: Lists all workflows in the workspace.

**When to use**: View automated workflows.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Circleback workflows"

---

### circleback_get_workflow

**What it does**: Gets details of a specific workflow.

**When to use**: View workflow configuration.

**Arguments**:
- `id` (required): Workflow ID

**Example LLM prompt**: "Get workflow 456 details"

---

### circleback_list_tasks

**What it does**: Lists all tasks.

**When to use**: View task list.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all pending tasks"

---

### circleback_get_task

**What it does**: Gets details of a specific task.

**When to use**: View task details.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task 789 details"

---

### circleback_create_task

**What it does**: Creates a new task.

**When to use**: Create a new task item.

**Arguments**:
- `title` (required): Task title
- `assignee` (optional): Assignee ID

**Example LLM prompt**: "Create a task to prepare the report"

---

### circleback_list_team_members

**What it does**: Lists all team members.

**When to use**: View team members for assignment.

**Arguments**: None

**Example LLM prompt**: "List all team members"

---

## Circleback MCP Notes

- **MCP OAuth2**: Uses Model Context Protocol OAuth2 flow
- **Actions**: Tracked items from meetings
- **Workflows**: Automated action pipelines
- **Tasks**: General task management

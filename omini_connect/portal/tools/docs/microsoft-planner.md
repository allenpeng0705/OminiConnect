# Microsoft Planner Tools

Provider: `microsoft-planner` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Planner API. They allow AI agents to manage plans, tasks, buckets, and assignments in Microsoft 365 Planner. **Requires Microsoft Planner OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Planner)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `planner_list_plans` | List all plans | GET | /v1.0/me/planner/plans |
| `planner_get_plan` | Get plan details | GET | /v1.0/planner/plans/{planId} |
| `planner_list_tasks` | List tasks in a plan | GET | /v1.0/me/planner/plans/{planId}/tasks |
| `planner_get_task` | Get task details | GET | /v1.0/planner/tasks/{taskId} |
| `planner_create_task` | Create a task | POST | /v1.0/planner/tasks |
| `planner_update_task` | Update a task | PATCH | /v1.0/planner/tasks/{taskId} |
| `planner_delete_task` | Delete a task | DELETE | /v1.0/planner/tasks/{taskId} |
| `planner_list_buckets` | List buckets in a plan | GET | /v1.0/me/planner/plans/{planId}/buckets |
| `planner_get_bucket` | Get bucket details | GET | /v1.0/planner/buckets/{bucketId} |
| `planner_list_my_tasks` | List my tasks | GET | /v1.0/me/planner/tasks |

---

## Tool Details

### planner_list_plans

**What it does**: Lists all Planner plans accessible to the user.

**When to use**: Browse plans, find plan IDs.

**Arguments**:
- `$filter` (optional): OData filter expression

**Example LLM prompt**: "List all my Planner plans"

---

### planner_get_plan

**What it does**: Gets details of a specific Planner plan.

**When to use**: Check plan settings, understand plan structure.

**Arguments**:
- `planId` (required): Plan ID

**Example LLM prompt**: "Get details for plan 12345"

---

### planner_list_tasks

**What it does**: Lists all tasks in a Planner plan.

**When to use**: View tasks, track progress.

**Arguments**:
- `planId` (required): Plan ID
- `$filter` (optional): OData filter expression

**Example LLM prompt**: "List all tasks in plan 12345"

---

### planner_get_task

**What it does**: Gets details of a specific Planner task.

**When to use**: Check task details, view assignments.

**Arguments**:
- `taskId` (required): Task ID

**Example LLM prompt**: "Get details for task 12345"

---

### planner_create_task

**What it does**: Creates a new task in a Planner plan.

**When to use**: Create new tasks, assign work.

**Arguments**:
- `planId` (required): Plan ID
- `bucketId` (optional): Bucket ID
- `title` (required): Task title
- `assignments` (optional): Assignee assignments
- `dueDateTime` (optional): Due date (ISO 8601)
- `startDateTime` (optional): Start date (ISO 8601)

**Example LLM prompt**: "Create a task called 'Review PR' in plan 12345"

---

### planner_update_task

**What it does**: Updates an existing Planner task.

**When to use**: Modify task, update progress.

**Arguments**:
- `taskId` (required): Task ID
- `title` (optional): Task title
- `percentComplete` (optional): Completion percentage (0-100)
- `dueDateTime` (optional): Due date (ISO 8601)
- `bucketId` (optional): Bucket ID

**Example LLM prompt**: "Mark task 12345 as 50% complete"

---

### planner_delete_task

**What it does**: Deletes a Planner task.

**When to use**: Remove completed or unwanted tasks.

**Arguments**:
- `taskId` (required): Task ID

**Example LLM prompt**: "Delete task 12345"

---

### planner_list_buckets

**What it does**: Lists all buckets in a Planner plan.

**When to use**: View columns, organize tasks.

**Arguments**:
- `planId` (required): Plan ID

**Example LLM prompt**: "List all buckets in plan 12345"

---

### planner_get_bucket

**What it does**: Gets details of a specific bucket.

**When to use**: Check bucket settings, task count.

**Arguments**:
- `bucketId` (required): Bucket ID

**Example LLM prompt**: "Get details for bucket 12345"

---

### planner_list_my_tasks

**What it does**: Lists all tasks assigned to the current user.

**When to use**: Personal task management, my work.

**Arguments**:
- `$filter` (optional): OData filter expression

**Example LLM prompt**: "List all tasks assigned to me"

---

## Planner Notes

- **Plans**: Containers for tasks (like projects)
- **Buckets**: Columns within plans (like Kanban columns)
- **Tasks**: Work items with assignments
- **Assignments**: Task assignees
- **Microsoft 365 Groups**: Plans are linked to M365 groups

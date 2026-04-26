# Perdoo Tools

Provider: `perdoo` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Perdoo API. They allow AI agents to manage goals, key results, projects, tasks, and team members. Perdoo is an OKR and project management platform. **Requires Perdoo API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://eu.perdoo.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `perdoo_list_goals` | List goals | GET | /goals |
| `perdoo_get_goal` | Get goal details | GET | /goals/{goalId} |
| `perdoo_list_key_results` | List key results | GET | /key-results |
| `perdoo_get_key_result` | Get key result details | GET | /key-results/{keyResultId} |
| `perdoo_list_projects` | List projects | GET | /projects |
| `perdoo_get_project` | Get project details | GET | /projects/{projectId} |
| `perdoo_list_tasks` | List tasks | GET | /tasks |
| `perdoo_get_task` | Get task details | GET | /tasks/{taskId} |
| `perdoo_list_users` | List team members | GET | /users |
| `perdoo_get_user` | Get user details | GET | /users/{userId} |

---

## Tool Details

### perdoo_list_goals

**What it does**: Lists all goals in the organization.

**When to use**: Browse organizational goals, track OKRs.

**Arguments**:
- `status` (optional): Filter by status (active, completed)

**Example LLM prompt**: "List all active goals"

---

### perdoo_get_goal

**What it does**: Gets detailed information about a specific goal.

**When to use**: Get goal progress, details.

**Arguments**:
- `goalId` (required): Goal ID

**Example LLM prompt**: "Get details for goal G-12345"

---

### perdoo_list_key_results

**What it does**: Lists all key results.

**When to use**: Browse key results, track progress.

**Arguments**:
- `goalId` (optional): Filter by goal ID

**Example LLM prompt**: "List all key results for goal G-12345"

---

### perdoo_get_key_result

**What it does**: Gets detailed information about a specific key result.

**When to use**: Get key result progress, details.

**Arguments**:
- `keyResultId` (required): Key result ID

**Example LLM prompt**: "Get details for key result KR-67890"

---

### perdoo_list_projects

**What it does**: Lists all projects.

**When to use**: Browse projects, find work items.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active projects"

---

### perdoo_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: Get project details, progress.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project P-11111"

---

### perdoo_list_tasks

**What it does**: Lists all tasks.

**When to use**: Browse tasks, find work items.

**Arguments**:
- `projectId` (optional): Filter by project ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all tasks in project P-11111"

---

### perdoo_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: Get task details, status.

**Arguments**:
- `taskId` (required): Task ID

**Example LLM prompt**: "Get details for task T-22222"

---

### perdoo_list_users

**What it does**: Lists all team members.

**When to use**: Browse team directory.

**Arguments**: None

**Example LLM prompt**: "List all team members"

---

### perdoo_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, assignments.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user U-33333"

---

## Perdoo API Notes

- **API Key**: Uses Bearer token for authentication
- **OKR Structure**: Goals contain Key Results
- **Projects/Tasks**: Work items within Perdoo

# Asana Tools

Provider: `asana` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Asana REST API. They allow AI agents to interact with tasks, projects, teams, and workspaces on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `task:read`, `task:write`, `project:read`, `project:write`, `team:read`, `workspace:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `asana_list_tasks` | List tasks with filters | GET | /api/1.0/tasks |
| `asana_get_task` | Get task details | GET | /api/1.0/tasks/{task_gid} |
| `asana_create_task` | Create a new task | POST | /api/1.0/tasks |
| `asana_update_task` | Update a task | PUT | /api/1.0/tasks/{task_gid} |
| `asana_list_projects` | List projects in workspace | GET | /api/1.0/projects |
| `asana_get_project` | Get project details | GET | /api/1.0/projects/{project_gid} |
| `asana_list_teams` | List all teams | GET | /api/1.0/teams |
| `asana_get_team` | Get team details | GET | /api/1.0/teams/{team_gid} |
| `asana_list_portfolios` | List portfolios | GET | /api/1.0/portfolios |
| `asana_get_portfolio` | Get portfolio details | GET | /api/1.0/portfolios/{portfolio_gid} |

---

## Tool Details

### asana_list_tasks

**What it does**: Returns a paginated list of tasks. Can be filtered by project, assignee, completed status, or due date.

**When to use**: Find tasks assigned to you, search for incomplete work, or browse tasks in a project.

**Arguments**:
- `project` (optional): Project GID to filter by
- `assignee` (optional): Assignee user GID
- `completed` (optional): Filter by completed status (true/false)
- `due_on` (optional): Due date filter (YYYY-MM-DD)
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "Show me all my incomplete tasks due this week"

---

### asana_get_task

**What it does**: Returns detailed information about a specific task including all fields.

**When to use**: Get full task context before editing, marking complete, or adding comments.

**Arguments**:
- `task_gid` (required): Task GID

**Example LLM prompt**: "Show me details of task 123456789"

---

### asana_create_task

**What it does**: Creates a new task with the given name, optional description, assignee, due date, and project associations.

**When to use**: Create bugs, tasks, features, or any work item from conversation.

**Arguments**:
- `name` (required): Task name
- `projects` (optional): Array of project GIDs to add task to
- `assignee` (optional): Assignee user GID
- `notes` (optional): Task description
- `due_on` (optional): Due date (YYYY-MM-DD)

**Example LLM prompt**: "Create a task in the Mobile App project titled 'Fix login crash' due Friday"

---

### asana_update_task

**What it does**: Updates an existing task. Can modify name, description, assignee, due date, or completion status.

**When to use**: Change task details, reassign work, update due dates, or mark tasks complete.

**Arguments**:
- `task_gid` (required): Task GID
- `name` (optional): New name
- `notes` (optional): New description
- `assignee` (optional): New assignee GID
- `completed` (optional): Mark as completed (true/false)
- `due_on` (optional): New due date (YYYY-MM-DD)

**Example LLM prompt**: "Mark task 123456789 as completed and assign it to john"

---

### asana_list_portfolios

**What it does**: Returns a list of portfolios in a workspace. Can be filtered by color or owner.

**When to use**: Browse portfolios to track progress across multiple projects.

**Arguments**:
- `workspace` (optional): Workspace GID
- `color` (optional): Filter by color
- `owner` (optional): Filter by owner user GID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "Show me all portfolios in the Engineering workspace"

---

### asana_get_portfolio

**What it does**: Returns detailed information about a portfolio including its projects and metrics.

**When to use**: Get portfolio details, view project list, or check portfolio health.

**Arguments**:
- `portfolio_gid` (required): Portfolio GID

**Example LLM prompt**: "Show me details of portfolio 123456789"

---

### asana_list_projects

**What it does**: Returns a list of projects in a workspace. Can be filtered by team or archived status.

**When to use**: Browse projects in a workspace before creating tasks or searching for project-specific work.

**Arguments**:
- `workspace` (required): Workspace GID
- `team` (optional): Team GID to filter by
- `archived` (optional): Include archived projects
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "What projects exist in the Engineering workspace?"

---

### asana_get_project

**What it does**: Returns detailed information about a project including description, status, team, and members.

**When to use**: Get project context before creating tasks or viewing project-wide progress.

**Arguments**:
- `project_gid` (required): Project GID

**Example LLM prompt**: "Show me details of project 123456789"

---

### asana_list_teams

**What it does**: Returns a list of all teams in the organization.

**When to use**: Browse available teams before creating projects or searching for team-specific work.

**Arguments**:
- `organization` (required): Organization GID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "What teams do we have?"

---

### asana_get_team

**What it does**: Returns detailed information about a team including members and projects.

**When to use**: Get team context, view member list, or understand team structure.

**Arguments**:
- `team_gid` (required): Team GID

**Example LLM prompt**: "Show me details of the Engineering team"

---

## Asana API Reference

These tools use the Asana REST API. See official docs for full details:
- https://developers.asana.com/docs/using-the-api
- Rate limits: 1,500 requests/minute for most plans
- Pagination: Use `limit` and `offset` parameters
- All dates: ISO 8601 format (UTC)
- GIDs are strings, not integers

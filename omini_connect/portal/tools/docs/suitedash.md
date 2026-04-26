# SuiteDash Tools

Provider: `suitedash` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the SuiteDash API. They allow AI agents to manage users, teams, projects, tasks, and invoices. SuiteDash is a business management platform for client portals and project collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SuiteDash
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `users:read`, `users:write`, `teams:read`, `projects:read`, `tasks:read`, `invoices:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `suitedash_list_users` | List all users | GET | /users |
| `suitedash_get_user` | Get user details | GET | /users/{id} |
| `suitedash_create_user` | Create a new user | POST | /users |
| `suitedash_list_teams` | List all teams | GET | /teams |
| `suitedash_get_team` | Get team details | GET | /teams/{id} |
| `suitedash_list_projects` | List all projects | GET | /projects |
| `suitedash_get_project` | Get project details | GET | /projects/{id} |
| `suitedash_list_tasks` | List all tasks | GET | /tasks |
| `suitedash_get_task` | Get task details | GET | /tasks/{id} |
| `suitedash_list_invoices` | List all invoices | GET | /invoices |

---

## Tool Details

### suitedash_list_users

**What it does**: Lists all users in SuiteDash with optional filtering by role, status, or team.

**When to use**: View team members, find users by role or department.

**Arguments**:
- `role` (optional): Filter by role
- `status` (optional): Filter by status (active, inactive)
- `team_id` (optional): Filter by team ID
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active admins in the company"

---

### suitedash_get_user

**What it does**: Gets detailed information about a specific user including profile, role, and permissions.

**When to use**: Get user details before assigning tasks or projects.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user 123"

---

### suitedash_create_user

**What it does**: Creates a new user with name, email, role, and team assignment.

**When to use**: Add a new team member to the platform.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `role` (optional): User role (admin, member, guest)
- `team_ids` (optional): Team IDs to assign

**Example LLM prompt**: "Create a new user named Jane Doe with email jane@acme.com as a member"

---

### suitedash_list_teams

**What it does**: Lists all teams in SuiteDash with their names, members, and settings.

**When to use**: View team structure, find teams for project assignment.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all teams in the organization"

---

### suitedash_get_team

**What it does**: Gets detailed information about a specific team including members and settings.

**When to use**: Understand team composition before assigning work.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Get details for team engineering"

---

### suitedash_list_projects

**What it does**: Lists all projects in SuiteDash with optional filtering by status, team, or client.

**When to use**: View active projects, find projects by team or status.

**Arguments**:
- `status` (optional): Filter by status (active, completed, on-hold)
- `team_id` (optional): Filter by team ID
- `client_id` (optional): Filter by client ID
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active projects for the engineering team"

---

### suitedash_get_project

**What it does**: Gets detailed information about a specific project including tasks, team, and timeline.

**When to use**: Get project overview before assigning tasks or updating status.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 456"

---

### suitedash_list_tasks

**What it does**: Lists all tasks in SuiteDash with optional filtering by project, assignee, or status.

**When to use**: View task list, find tasks by status or project.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `assignee_id` (optional): Filter by assignee user ID
- `status` (optional): Filter by status (todo, in-progress, completed)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all todo tasks for project 456"

---

### suitedash_get_task

**What it does**: Gets detailed information about a specific task including description, assignee, due date, and comments.

**When to use**: Get task details before updating or completing.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 789"

---

### suitedash_list_invoices

**What it does**: Lists all invoices in SuiteDash with optional filtering by status, client, or date range.

**When to use**: View invoice history, find invoices by status or client.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, paid, overdue)
- `client_id` (optional): Filter by client ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all overdue invoices"

---

## SuiteDash API Notes

- **Role-based access**: Users have roles (admin, member, guest) that determine permissions.
- **Team structure**: Users can belong to multiple teams; teams can have multiple members.
- **Project hierarchy**: Projects contain tasks; tasks can be assigned to team members.
- **Invoice workflow**: Invoices go through statuses: draft, sent, paid, overdue.

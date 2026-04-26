# Linear Tools

Provider: `linear` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Linear GraphQL API. They allow AI agents to interact with issues, projects, teams, cycles, and users on behalf of the authenticated organization.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `issues:read`, `issues:create`, `issues:update`, `issues:delete`, `teams:read`, `projects:read`, `cycles:read`, `users:read`, `comments:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `linear_list_issues` | List issues with filters | POST | /graphql |
| `linear_get_issue` | Get issue details | POST | /graphql |
| `linear_create_issue` | Create a new issue | POST | /graphql |
| `linear_update_issue` | Update an issue | POST | /graphql |
| `linear_list_projects` | List all projects | POST | /graphql |
| `linear_get_project` | Get project details | POST | /graphql |
| `linear_list_teams` | List all teams | POST | /graphql |
| `linear_get_team` | Get team details | POST | /graphql |
| `linear_list_cycles` | List all cycles | POST | /graphql |
| `linear_get_cycle` | Get cycle details | POST | /graphql |

---

## Tool Details

### linear_list_issues

**What it does**: Returns a paginated list of issues with optional filters for team, status, assignee, priority, label, and cycle.

**When to use**: Search for issues across projects, filter by status or assignee, or find issues in a specific cycle.

**Arguments**:
- `team` (optional): Team slug or ID
- `status` (optional): Status filter (`Backlog`, `Todo`, `In Progress`, `Done`)
- `assignee` (optional): Assignee user ID
- `priority` (optional): Priority (0=No priority, 1=Urgent, 2=High, 3=Normal, 4=Low)
- `cycleId` (optional): Filter by cycle ID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "Show me all open high-priority issues in the Engineering team"

---

### linear_get_issue

**What it does**: Returns detailed information about a specific issue including title, description, state, priority, assignees, labels, and cycle.

**When to use**: Get the full context of an issue before editing, commenting, or updating its state.

**Arguments**:
- `issueId` (required): Issue ID

**Example LLM prompt**: "Show me details of issue ABC-123"

---

### linear_create_issue

**What it does**: Creates a new issue in a Linear team with the given title, description, priority, and optional fields.

**When to use**: Create bugs, tasks, features, or any work item from conversation.

**Arguments**:
- `teamId` (required): Team ID
- `title` (required): Issue title
- `description` (optional): Issue description (Markdown)
- `priority` (optional): Priority (0=No priority, 1=Urgent, 2=High, 3=Normal, 4=Low)
- `assigneeId` (optional): Assignee user ID
- `labelIds` (optional): Array of label IDs
- `cycleId` (optional): Cycle ID to add issue to

**Example LLM prompt**: "Create a high-priority bug in Engineering titled 'API timeout under load'"

---

### linear_update_issue

**What it does**: Updates an existing issue. Can modify title, description, state, priority, assignee, and cycle.

**When to use**: Change issue status, reassign work, update priorities, or move issues between cycles.

**Arguments**:
- `issueId` (required): Issue ID
- `title` (optional): New title
- `description` (optional): New description
- `status` (optional): New status (`Backlog`, `Todo`, `In Progress`, `Done`, `Canceled`)
- `priority` (optional): New priority
- `assigneeId` (optional): New assignee ID
- `cycleId` (optional): Cycle ID to move issue to

**Example LLM prompt**: "Move issue ABC-123 to the current sprint and mark it as In Progress"

---

### linear_list_projects

**What it does**: Returns a list of all projects in the Linear organization.

**When to use**: Browse available projects before creating issues or searching for project-specific work.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "What projects exist in our organization?"

---

### linear_get_project

**What it does**: Returns detailed information about a specific project including description, status, teams, and milestones.

**When to use**: Get project context before creating issues or viewing project-wide progress.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Show me details of the Mobile App project"

---

### linear_list_teams

**What it does**: Returns a list of all teams in the Linear organization.

**When to use**: Browse available teams before creating issues or searching for team-specific work.

**Arguments**: None

**Example LLM prompt**: "What teams do we have?"

---

### linear_get_team

**What it does**: Returns detailed information about a specific team including members, projects, and issue statistics.

**When to use**: Get team context, view member list, or understand team workload.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Show me details of the Engineering team"

---

### linear_list_cycles

**What it does**: Returns a list of all cycles. Can be filtered by team or status.

**When to use**: Find active sprints, view upcoming cycles, or see completed cycle history.

**Arguments**:
- `teamId` (optional): Filter by team ID
- `status` (optional): Cycle status (`upcoming`, `active`, `completed`)

**Example LLM prompt**: "What cycles are currently active?"

---

### linear_get_cycle

**What it does**: Returns detailed information about a specific cycle including its start/end dates, progress, and associated issues.

**When to use**: Get cycle details, see all issues in the cycle, or track sprint progress.

**Arguments**:
- `cycleId` (required): Cycle ID

**Example LLM prompt**: "Show me details of cycle Q1-Sprint-1"

---

## Linear API Reference

These tools use the Linear GraphQL API. See official docs for full details:
- https://developers.linear.app/docs/graphql/working-with-the-graphql-api
- Rate limits: Varies by plan
- Pagination: Use `first`/`after` cursor-based pagination
- All dates: ISO 8601 format (UTC)

# ProjectManager Tools

Provider: `projectmanager` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the ProjectManager REST API. They allow AI agents to manage projects, tasks, teams, timesheets, and milestones — ProjectManager is a cloud-based project management platform designed for teams to plan, track, and deliver projects.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with ProjectManager
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projects:read`, `projects:write`, `tasks:read`, `tasks:write`, `teams:read`, `timesheets:read`, `milestones:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `projectmanager_list_projects` | List projects | GET | /api/projects |
| `projectmanager_get_project` | Get project details | GET | /api/projects/{projectId} |
| `projectmanager_create_project` | Create a project | POST | /api/projects |
| `projectmanager_list_tasks` | List tasks | GET | /api/tasks |
| `projectmanager_get_task` | Get task details | GET | /api/tasks/{taskId} |
| `projectmanager_list_teams` | List teams | GET | /api/teams |
| `projectmanager_get_team` | Get team details | GET | /api/teams/{teamId} |
| `projectmanager_list_timesheets` | List timesheets | GET | /api/timesheets |
| `projectmanager_get_timesheet` | Get timesheet details | GET | /api/timesheets/{timesheetId} |
| `projectmanager_list_milestones` | List milestones | GET | /api/milestones |

---

## Tool Details

### projectmanager_list_projects

**What it does**: Lists all projects with optional status filtering.

**When to use**: Browse available projects, find active projects for task or team association.

**Arguments**:
- `status` (optional): Filter by status (Active, Archived, Draft)

**Example LLM prompt**: "List all active projects"

---

### projectmanager_get_project

**What it does**: Gets detailed project info — name, status, dates, team, budget, milestones.

**When to use**: Understand project scope and timeline before adding tasks or viewing files.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project abc-123"

---

### projectmanager_create_project

**What it does**: Creates a new project with name, description, and dates.

**When to use**: Set up new projects for clients or internal initiatives.

**Arguments**:
- `name` (required): Project name
- `description` (optional): Project description
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Create a new project called 'Website Redesign' starting next week"

---

### projectmanager_list_tasks

**What it does**: Lists tasks with optional project and status filters.

**When to use**: Find tasks across projects, see what needs to be done.

**Arguments**:
- `projectId` (optional): Filter by project ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all tasks in project abc-123"

---

### projectmanager_get_task

**What it does**: Gets detailed task info — name, description, assignees, due date, priority, time tracked.

**When to use**: Read full task context before updating or adding time entries.

**Arguments**:
- `taskId` (required): Task ID

**Example LLM prompt**: "Get task details for task xyz-456"

---

### projectmanager_list_teams

**What it does**: Lists all teams in the account.

**When to use**: See all available teams, find team members for task assignment.

**Arguments**: None

**Example LLM prompt**: "What teams are available in our account"

---

### projectmanager_get_team

**What it does**: Gets detailed team info — name, description, member list.

**When to use**: Understand team structure before assigning tasks to team members.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get details for team abc-team-1"

---

### projectmanager_list_timesheets

**What it does**: Lists timesheets with optional user, project, and date filters.

**When to use**: Review work hours logged, find timesheets for billing or project tracking.

**Arguments**:
- `userId` (optional): Filter by user ID
- `projectId` (optional): Filter by project ID
- `startDate` (optional): Start date filter (YYYY-MM-DD)
- `endDate` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "List all timesheets for user abc-user this month"

---

### projectmanager_get_timesheet

**What it does**: Gets detailed timesheet info — user, project, hours, dates, status.

**When to use**: Review specific timesheet details for approval or billing.

**Arguments**:
- `timesheetId` (required): Timesheet ID

**Example LLM prompt**: "Get timesheet details for timesheet def-789"

---

### projectmanager_list_milestones

**What it does**: Lists milestones with optional project and status filters.

**When to use**: Track project progress, find upcoming deadlines.

**Arguments**:
- `projectId` (optional): Filter by project ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all milestones for project abc-123"

---

## ProjectManager API Notes

- **IDs**: ProjectManager uses string IDs for all objects
- **Projects**: Top-level containers for tasks, timesheets, and team collaboration
- **Tasks**: Work items within projects; have assignees, due dates, priority, status
- **Teams**: Groups of users that can be assigned to projects and tasks
- **Timesheets**: Records of work hours logged by team members
- **Milestones**: Important project checkpoints with target dates
- **REST API**: Uses `/api/` prefix for all endpoints
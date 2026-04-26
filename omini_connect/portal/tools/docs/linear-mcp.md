# Linear MCP Tools

Provider: `linear-mcp` | Engine: `nango` | Auth: MCP_OAUTH2 via Nango

## Overview

These tools wrap the Linear MCP API. They allow AI agents to manage issues, projects, teams, and workflows. **Requires Linear MCP OAuth2 authentication.**

## Authentication

**MCP OAuth2 via Nango**:
- User authenticates via Nango Connect with Linear
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.linear.app`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `linear_mcp_list_issues` | List issues | GET | /issues |
| `linear_mcp_get_issue` | Get issue details | GET | /issues/{issue_id} |
| `linear_mcp_create_issue` | Create an issue | POST | /issues |
| `linear_mcp_update_issue` | Update an issue | PUT | /issues/{issue_id} |
| `linear_mcp_list_teams` | List teams | GET | /teams |
| `linear_mcp_get_team` | Get team details | GET | /teams/{team_id} |
| `linear_mcp_list_projects` | List projects | GET | /projects |
| `linear_mcp_get_project` | Get project details | GET | /projects/{project_id} |
| `linear_mcp_list_users` | List users | GET | /users |
| `linear_mcp_get_user` | Get user details | GET | /users/{user_id} |

---

## Tool Details

### linear_mcp_list_issues

**What it does**: Lists all issues.

**When to use**: Find issues, view project tasks.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all issues in Linear"

---

### linear_mcp_get_issue

**What it does**: Gets details for a specific issue.

**When to use**: Get issue information.

**Arguments**:
- `issue_id` (required): Issue ID

**Example LLM prompt**: "Get details for issue abc123"

---

### linear_mcp_create_issue

**What it does**: Creates a new issue.

**When to use**: Create tasks, bug reports.

**Arguments**:
- `team_id` (required): Team ID
- `title` (required): Issue title
- `description` (optional): Issue description

**Example LLM prompt**: "Create an issue titled 'Fix login bug' in team t1"

---

### linear_mcp_update_issue

**What it does**: Updates an existing issue.

**When to use**: Modify issues, change status.

**Arguments**:
- `issue_id` (required): Issue ID
- `title` (optional): Issue title
- `description` (optional): Issue description

**Example LLM prompt**: "Update issue abc123 with new title"

---

### linear_mcp_list_teams

**What it does**: Lists all teams.

**When to use**: View teams, organize work.

**Arguments**: None

**Example LLM prompt**: "List all teams in Linear"

---

### linear_mcp_get_team

**What it does**: Gets details for a specific team.

**When to use**: Get team information.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team xyz789"

---

### linear_mcp_list_projects

**What it does**: Lists all projects.

**When to use**: View projects, track progress.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all projects in Linear"

---

### linear_mcp_get_project

**What it does**: Gets details for a specific project.

**When to use**: Get project information.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project p1"

---

### linear_mcp_list_users

**What it does**: Lists all users.

**When to use**: View team members.

**Arguments**: None

**Example LLM prompt**: "List all users in Linear"

---

### linear_mcp_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u1"

---

## Linear MCP API Notes

- **Issue Tracking**: Project management and issue tracking
- **Teams**: Groups of members working on projects
- **Projects**: Collections of related issues
- **Issues**: Tasks, bugs, and features
- **MCP**: Model Context Protocol integration

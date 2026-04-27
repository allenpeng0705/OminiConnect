# Jira Tools

Provider: `jira` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Jira Cloud REST API. They allow AI agents to interact with projects, issues, sprints, and worklogs on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read:jira-work`, `create:jira-work`, `edit:jira-work`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jira_list_issues` | Search and list issues using JQL | GET | /rest/api/3/search |
| `jira_get_issue` | Get details of a specific issue | GET | /rest/api/3/issue/{issueIdOrKey} |
| `jira_create_issue` | Create a new issue | POST | /rest/api/3/issue |
| `jira_update_issue` | Update an existing issue | PUT | /rest/api/3/issue/{issueIdOrKey} |
| `jira_list_projects` | List projects the user has access to | GET | /rest/api/3/project |
| `jira_get_project` | Get details of a specific project | GET | /rest/api/3/project/{projectIdOrKey} |
| `jira_list_sprints` | List sprints in a project | GET | /rest/agile/1.0/board/{boardId}/sprint |
| `jira_get_sprint` | Get details of a specific sprint | GET | /rest/agile/1.0/sprint/{sprintId} |
| `jira_add_worklog` | Add a worklog entry to an issue | POST | /rest/api/3/issue/{issueIdOrKey}/worklog |
| `jira_get_worklog` | Get worklog entries for an issue | GET | /rest/api/3/issue/{issueIdOrKey}/worklog |

---

## Tool Details

### jira_list_issues

**What it does**: Returns a paginated list of issues matching a JQL (Jira Query Language) query. This is the primary way to search and filter issues.

**When to use**: Search for issues by project, status, assignee, sprint, labels, or any combination of criteria.

**Arguments**:
- `jql` (optional): JQL query string (e.g., `project = MYPROJ AND status = Open`)
- `maxResults` (optional): Maximum results (default 50, max 100)
- `startAt` (optional): Starting index for pagination (default 0)
- `fields` (optional): Array of fields to include in response
- `expand` (optional): Additional expand options: `renderedFields`, `names`, `schema`, `transitions`, `operations`

**Example LLM prompt**: "Show me all open bugs in the MyProject project assigned to me"

---

### jira_get_issue

**What it does**: Returns detailed information about a specific issue including description, status, assignee, and comments.

**When to use**: Get the full details of an issue before editing, commenting, or closing it.

**Arguments**:
- `issueIdOrKey` (required): Issue ID or key (e.g., `MYPROJ-123`)
- `fields` (optional): Array of fields to include (default includes most fields)
- `expand` (optional): Additional expand options: `renderedFields`, `names`, `schema`, `transitions`, `operations`, `comments`

**Example LLM prompt**: "Show me the details of issue PROJ-456"

---

### jira_create_issue

**What it does**: Creates a new issue in a specified Jira project with the given summary, type, and optional fields.

**When to use**: File bugs, create tasks, log feature requests, or any work item from conversation.

**Arguments**:
- `projectId` (optional): Project ID (required if projectKey not used)
- `projectKey` (optional): Project key (e.g., `MYPROJ`)
- `issuetype` (required): Issue type object (name: `Bug`, `Story`, `Task`, etc.)
- `summary` (required): Issue summary/title
- `description` (optional): Issue description (Atlassian Document Format)
- `assignee` (optional): Assignee object
- `labels` (optional): Array of label strings

**Example LLM prompt**: "Create a bug in MYPROJ titled 'Login fails with special characters'"

---

### jira_update_issue

**What it does**: Updates an existing issue. Can modify summary, description, assignee, labels, and more.

**When to use**: Close issues, reassign work, change priorities, or edit descriptions after gathering context.

**Arguments**:
- `issueIdOrKey` (required): Issue ID or key
- `summary` (optional): New summary
- `description` (optional): New description (Atlassian Document Format)
- `assignee` (optional): New assignee object
- `labels` (optional): New labels array (replaces existing)

**Example LLM prompt**: "Update issue PROJ-789 to change the summary to 'Fixed: Login issue'"

---

### jira_list_projects

**What it does**: Returns a list of all Jira projects the authenticated user has access to.

**When to use**: Browse available projects before creating issues or searching within a project.

**Arguments**:
- `expand` (optional): Additional expand: `description`, `issueTypes`, `lead`, `projectKeys`
- `maxResults` (optional): Maximum results (default 50)
- `recent` (optional): Number of recent projects to return (default 0)

**Example LLM prompt**: "What projects do I have access to?"

---

### jira_get_project

**What it does**: Returns detailed information about a specific project including description, lead, issue types, and versions.

**When to use**: Get project metadata, issue types, or configuration before creating issues.

**Arguments**:
- `projectIdOrKey` (required): Project ID or key
- `expand` (optional): Additional expand: `description`, `issueTypes`, `lead`, `projectKeys`, `versions`

**Example LLM prompt**: "Show me details of the MyProject project"

---

### jira_list_sprints

**What it does**: Returns a list of sprints for a board. Can be filtered by state (active, closed, future).

**When to use**: Find available sprints for planning, see sprint progress, or assign issues to sprints.

**Arguments**:
- `boardId` (required): Board ID
- `state` (optional): Filter by state (`active`, `closed`, `future`)
- `maxResults` (optional): Maximum results (default 50)
- `startAt` (optional): Starting index (default 0)

**Example LLM prompt**: "What sprints are active on board 123?"

---

### jira_get_sprint

**What it does**: Returns detailed information about a specific sprint including name, state, start/end dates, and issues.

**When to use**: Get sprint details, see issues in sprint, or track sprint progress.

**Arguments**:
- `sprintId` (required): Sprint ID
- `expand` (optional): Expand options: `issues`, `actions`, `estimatedBM`, `linkedPages`

**Example LLM prompt**: "Show me sprint 456 details including all issues"

---

### jira_add_worklog

**What it does**: Adds a worklog entry to an issue to track time spent. Include time spent and optional start date and comment.

**When to use**: Log time worked on an issue, track time for billing, or update progress.

**Arguments**:
- `issueIdOrKey` (required): Issue ID or key
- `timeSpent` (required): Time spent (e.g., `3h 30m`)
- `started` (optional): Start time (ISO 8601 format)
- `comment` (optional): Worklog comment/description

**Example LLM prompt**: "Add a worklog to PROJ-789 for 2 hours with comment 'Fixed the login bug'"

---

### jira_get_worklog

**What it does**: Returns worklog entries for an issue. Shows time tracking data and comments.

**When to use**: View time spent on an issue, check worklog history, or verify logged hours.

**Arguments**:
- `issueIdOrKey` (required): Issue ID or key
- `maxResults` (optional): Maximum results (default 100)
- `startAt` (optional): Starting index (default 0)

**Example LLM prompt**: "Get the worklog for issue PROJ-789 to see time spent"

---

## Jira API Reference

These tools use the Jira Cloud REST API. See official docs for full details:
- https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro/
- Rate limits: Varies by plan
- Pagination: Use `maxResults` and `startAt` parameters
- All dates: ISO 8601 format (UTC)
- Issue keys: Format is `PROJECTKEY-123` (e.g., `MYPROJ-456`)

# Jira Data Center Tools

Provider: `jira-data-center` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Jira Data Center API. They allow AI agents to manage projects, issues, and comments. **Requires Jira Data Center OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Jira Data Center
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Configured instance URL

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jira_data_center_get_current_user` | Get current user | GET | /rest/api/3/myself |
| `jira_data_center_list_projects` | List projects | GET | /rest/api/3/project |
| `jira_data_center_get_project` | Get project details | GET | /rest/api/3/project/{project_id_or_key} |
| `jira_data_center_list_issues` | List issues | GET | /rest/api/3/search |
| `jira_data_center_get_issue` | Get issue details | GET | /rest/api/3/issue/{issue_id_or_key} |
| `jira_data_center_create_issue` | Create an issue | POST | /rest/api/3/issue |
| `jira_data_center_update_issue` | Update an issue | PUT | /rest/api/3/issue/{issue_id_or_key} |
| `jira_data_center_list_comments` | List issue comments | GET | /rest/api/3/issue/{issue_id_or_key}/comment |
| `jira_data_center_add_comment` | Add a comment | POST | /rest/api/3/issue/{issue_id_or_key}/comment |
| `jira_data_center_search_issues` | Search issues | GET | /rest/api/3/search |

---

## Tool Details

### jira_data_center_get_current_user

**What it does**: Gets the current authenticated user.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Who is the current Jira user?"

---

### jira_data_center_list_projects

**What it does**: Lists all projects the user has access to.

**When to use**: Find projects, list available workspaces.

**Arguments**: None

**Example LLM prompt**: "List all Jira projects"

---

### jira_data_center_get_project

**What it does**: Gets details for a specific project.

**When to use**: Get project info, view project settings.

**Arguments**:
- `project_id_or_key` (required): Project ID or key (e.g., "PROJ" or "10000")

**Example LLM prompt**: "Get details for project PROJ"

---

### jira_data_center_list_issues

**What it does**: Lists issues, optionally filtered by JQL.

**When to use**: Browse issues, filter by project or status.

**Arguments**:
- `jql` (optional): JQL query string
- `maxResults` (optional): Max results (default 50)
- `startAt` (optional): Start at index (default 0)

**Example LLM prompt**: "List issues in project PROJ with status Open"

---

### jira_data_center_get_issue

**What it does**: Gets details for a specific issue.

**When to use**: Get issue info, view issue details.

**Arguments**:
- `issue_id_or_key` (required): Issue ID or key (e.g., "PROJ-123")

**Example LLM prompt**: "Get details for issue PROJ-123"

---

### jira_data_center_create_issue

**What it does**: Creates a new issue.

**When to use**: Create tasks, bugs, stories.

**Arguments**:
- `project_id` (required): Project ID or key
- `issue_type` (required): Issue type (Bug, Task, Story, etc.)
- `summary` (required): Issue summary
- `description` (optional): Issue description
- `priority` (optional): Priority (Low, Medium, High, Highest)

**Example LLM prompt**: "Create a bug in project PROJ with summary 'Login button not working'"

---

### jira_data_center_update_issue

**What it does**: Updates an existing issue.

**When to use**: Modify issues, change status, update details.

**Arguments**:
- `issue_id_or_key` (required): Issue ID or key
- `summary` (optional): New summary
- `description` (optional): New description
- `priority` (optional): New priority

**Example LLM prompt**: "Update issue PROJ-123 priority to High"

---

### jira_data_center_list_comments

**What it does**: Lists comments on an issue.

**When to use**: View discussion, read comments.

**Arguments**:
- `issue_id_or_key` (required): Issue ID or key

**Example LLM prompt**: "List comments on issue PROJ-123"

---

### jira_data_center_add_comment

**What it does**: Adds a comment to an issue.

**When to use**: Add notes, respond to issues.

**Arguments**:
- `issue_id_or_key` (required): Issue ID or key
- `body` (required): Comment content

**Example LLM prompt**: "Add a comment to PROJ-123 saying 'Looking into this'"

---

### jira_data_center_search_issues

**What it does**: Searches for issues using JQL.

**When to use**: Find specific issues, complex searches.

**Arguments**:
- `jql` (required): JQL query
- `maxResults` (optional): Max results (default 50)
- `fields` (optional): Fields to return

**Example LLM prompt**: "Search for all open bugs assigned to me"

---

## Jira Data Center API Notes

- **JQL**: Jira Query Language for filtering issues (e.g., `project = PROJ AND status = Open`)
- **Issue keys**: Format is PROJECTKEY-123 (e.g., PROJ-1)
- **Self-hosted**: This is for Jira Data Center (self-hosted), not Jira Cloud
- **API Rate Limits**: May vary based on instance configuration

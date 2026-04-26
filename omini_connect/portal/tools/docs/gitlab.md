# GitLab Tools

Provider: `gitlab` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the GitLab REST API. They allow AI agents to manage projects, issues, merge requests, and branches. GitLab is a complete DevOps platform offering version control, CI/CD, and project management — often used as a self-hosted GitHub alternative.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with GitLab
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `api` (full read/write access)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gitlab_list_projects` | List projects for the authenticated user | GET | /api/v4/projects |
| `gitlab_get_project` | Get details of a specific project | GET | /api/v4/projects/{id} |
| `gitlab_list_issues` | List issues in a project | GET | /api/v4/projects/{id}/issues |
| `gitlab_get_issue` | Get a specific issue | GET | /api/v4/projects/{id}/issues/{issue_iid} |
| `gitlab_create_issue` | Create a new issue | POST | /api/v4/projects/{id}/issues |
| `gitlab_list_merge_requests` | List merge requests in a project | GET | /api/v4/projects/{id}/merge_requests |
| `gitlab_get_merge_request` | Get a specific merge request | GET | /api/v4/projects/{id}/merge_requests/{mr_iid} |
| `gitlab_list_branches` | List branches in a project | GET | /api/v4/projects/{id}/repository/branches |
| `gitlab_get_branch` | Get a specific branch | GET | /api/v4/projects/{id}/repository/branches/{branch} |
| `gitlab_list_commits` | List commits in a project | GET | /api/v4/projects/{id}/repository/commits |

---

## Tool Details

### gitlab_list_projects

**What it does**: Lists all projects the authenticated user has access to, including owned, member, and starred projects.

**When to use**: Find projects, browse available repositories, select a project for issue or MR listing.

**Arguments**:
- `membership` (optional): Limit to projects where user is a member (default: true)
- `owned` (optional): Limit to projects owned by user (default: false)
- `per_page` (optional): Number of results (default 20, max 100)
- `page` (optional): Page number (default 1)
- `order_by` (optional): `id`, `name`, `created_at`, `updated_at` (default: created_at)
- `sort` (optional): `desc` or `asc` (default: desc)

**Example LLM prompt**: "List projects I have access to"

---

### gitlab_get_project

**What it does**: Gets detailed project information — name, description, visibility, web URL, default branch, stats.

**When to use**: Understand project structure and settings before creating issues or MRs.

**Arguments**:
- `id` (required): Project ID, path, or namespace/project format

**Example LLM prompt**: "Get details for project mygroup/myproject"

---

### gitlab_list_issues

**What it does**: Lists issues in a project with optional filtering by state, labels, or assignees.

**When to use**: Find open bugs, see issues by label or assignee, track project progress.

**Arguments**:
- `id` (required): Project ID or path
- `state` (optional): `opened`, `closed`, or `all` (default: opened)
- `labels` (optional): Comma-separated label names to filter by
- `assignee_id` (optional): Filter by assignee user ID
- `per_page` (optional): Number of results (default 20, max 100)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open issues with label 'bug' in project mygroup/myproject"

---

### gitlab_get_issue

**What it does**: Gets detailed issue information — title, description, labels, assignees, weight, milestone, due date.

**When to use**: Read full issue context before updating or commenting.

**Arguments**:
- `id` (required): Project ID or path
- `issue_iid` (required): Issue internal ID (IID, not global issue ID)

**Example LLM prompt**: "Get issue IID 42 from project mygroup/myproject"

---

### gitlab_create_issue

**What it does**: Creates a new issue in a project.

**When to use**: Log bugs, create tasks, capture feature requests.

**Arguments**:
- `id` (required): Project ID or path
- `title` (required): Issue title
- `description` (optional): Issue description
- `labels` (optional): Comma-separated label names
- `assignee_ids` (optional): Array of user IDs to assign

**Example LLM prompt**: "Create an issue titled 'Fix login bug' in project mygroup/myproject with label 'bug'"

---

### gitlab_list_merge_requests

**What it does**: Lists merge requests in a project with state filtering, source/target branch filtering.

**When to use**: Review open MRs, check pipeline status, see who is requesting reviews.

**Arguments**:
- `id` (required): Project ID or path
- `state` (optional): `opened`, `closed`, `merged`, or `all` (default: opened)
- `source_branch` (optional): Filter by source branch
- `target_branch` (optional): Filter by target branch
- `per_page` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open merge requests in project mygroup/myproject"

---

### gitlab_get_merge_request

**What it does**: Gets detailed MR information — title, description, source/target branch, reviewers, pipeline status, discussions.

**When to use**: Review MR details before approving or requesting changes.

**Arguments**:
- `id` (required): Project ID or path
- `mr_iid` (required): MR internal ID (IID)

**Example LLM prompt**: "Get MR IID 15 from project mygroup/myproject"

---

### gitlab_list_branches

**What it does**: Lists all branches in a project with their latest commit info.

**When to use**: See available branches, find feature branches, check for stale branches.

**Arguments**:
- `id` (required): Project ID or path
- `per_page` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all branches in project mygroup/myproject"

---

### gitlab_get_branch

**What it does**: Get details of a specific branch including the commit SHA, protected status, and branch name.

**When to use**: Check branch details, find the latest commit, or verify branch protection rules.

**Arguments**:
- `id` (required): Project ID or path
- `branch` (required): Branch name

**Example LLM prompt**: "Get details for the main branch in project mygroup/myproject"

---

### gitlab_list_commits

**What it does**: Lists commits in a project, optionally filtered by branch, author, or date range.

**When to use**: View change history, find when code was changed, or trace the introduction of a bug.

**Arguments**:
- `id` (required): Project ID or path
- `ref_name` (optional): Branch name or tag to list commits from
- `since` (optional): ISO 8601 date — only commits after this
- `until` (optional): ISO 8601 date — only commits before this
- `per_page` (optional): Number of results (default 20, max 100)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List commits from the last week in project mygroup/myproject"

---

## GitLab API Notes

- **IIDs**: GitLab uses internal IDs (IIDs) for issues and MRs — scoped to project, stable across all instances
- **Project IDs**: Can be numeric (`123`), path-based (`owner/project`), or encoded (`123+project-name`)
- **Labels**: Can be project-level or group-level; color-coded in UI
- **Merge Requests**: Equivalent to GitHub Pull Requests; have source and target branches
- **Branches**: List and get branch details including protected status and commit information
- **Commits**: View commit history with optional filtering by date range and branch

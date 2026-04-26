# GitLab PAT Tools

Provider: `gitlab-pat` | Engine: `nango` | Auth: API_KEY (PAT) via Nango

## Overview

These tools wrap the GitLab API using Personal Access Token authentication. They allow AI agents to manage projects, issues, merge requests, members, groups, branches, and tags. **Requires GitLab Personal Access Token.**

## Authentication

**Nango API_KEY (Personal Access Token)**:
- User provides GitLab Personal Access Token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Configured GitLab hostname (default: gitlab.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gitlab_pat_list_projects` | List projects | GET | /api/v4/projects |
| `gitlab_pat_get_project` | Get project details | GET | /api/v4/projects/{id} |
| `gitlab_pat_list_issues` | List issues | GET | /api/v4/issues |
| `gitlab_pat_get_issue` | Get issue details | GET | /api/v4/projects/{project_id}/issues/{issue_iid} |
| `gitlab_pat_list_mrs` | List merge requests | GET | /api/v4/projects/{id}/merge_requests |
| `gitlab_pat_get_mr` | Get merge request details | GET | /api/v4/projects/{id}/merge_requests/{mr_iid} |
| `gitlab_pat_list_members` | List project members | GET | /api/v4/projects/{id}/members |
| `gitlab_pat_list_groups` | List groups | GET | /api/v4/groups |
| `gitlab_pat_list_branches` | List branches | GET | /api/v4/projects/{id}/repository/branches |
| `gitlab_pat_list_tags` | List tags | GET | /api/v4/projects/{id}/repository/tags |

---

## Tool Details

### gitlab_pat_list_projects

**What it does**: Lists GitLab projects the authenticated user has access to.

**When to use**: Browse user's projects.

**Arguments**:
- `membership` (optional): Only show projects user is a member (default: true)
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all my GitLab projects"

---

### gitlab_pat_get_project

**What it does**: Gets detailed information about a project.

**When to use**: Get project description, default branch, settings.

**Arguments**:
- `id` (required): Project ID or path (URL-encoded)

**Example LLM prompt**: "Get details for project mygroup/myproject"

---

### gitlab_pat_list_issues

**What it does**: Lists issues across projects or within a project.

**When to use**: Browse issues, filter by project or state.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `state` (optional): Filter by state (opened, closed, all)

**Example LLM prompt**: "List all my assigned open issues"

---

### gitlab_pat_get_issue

**What it does**: Gets detailed information about a specific issue.

**When to use**: Get issue details, labels, assignees.

**Arguments**:
- `project_id` (required): Project ID or path
- `issue_iid` (required): Issue IID (internal ID)

**Example LLM prompt**: "Get issue #123 in project mygroup/myproject"

---

### gitlab_pat_list_mrs

**What it does**: Lists merge requests in a project.

**When to use**: Browse open merge requests.

**Arguments**:
- `id` (required): Project ID or path
- `state` (optional): Filter by state (opened, closed, merged, all)

**Example LLM prompt**: "List all open merge requests in myproject"

---

### gitlab_pat_get_mr

**What it does**: Gets detailed information about a merge request.

**When to use**: Get MR details, reviews, pipelines.

**Arguments**:
- `id` (required): Project ID or path
- `mr_iid` (required): Merge request IID

**Example LLM prompt**: "Get MR !456 in project myproject"

---

### gitlab_pat_list_members

**What it does**: Lists members of a project.

**When to use**: See who has access to a project.

**Arguments**:
- `id` (required): Project ID or path

**Example LLM prompt**: "List members of project myproject"

---

### gitlab_pat_list_groups

**What it does**: Lists GitLab groups the user has access to.

**When to use**: Browse user's groups.

**Arguments**:
- `min_access_level` (optional): Minimum access level

**Example LLM prompt**: "List all my groups"

---

### gitlab_pat_list_branches

**What it does**: Lists branches in a project.

**When to use**: See all branches in a repo.

**Arguments**:
- `id` (required): Project ID or path

**Example LLM prompt**: "List all branches in myproject"

---

### gitlab_pat_list_tags

**What it does**: Lists tags in a project.

**When to use**: See release tags.

**Arguments**:
- `id` (required): Project ID or path

**Example LLM prompt**: "List tags in myproject"

---

## GitLab PAT API Notes

- **IID**: Internal ID used in URLs (not global issue ID)
- **Project ID**: Can be numeric ID or path (URL-encoded)
- **Rate limits**: Subject to GitLab's rate limiting rules
- **Self-hosted**: Works with self-hosted GitLab instances

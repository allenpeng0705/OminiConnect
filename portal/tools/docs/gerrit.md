# Gerrit Tools

Provider: `gerrit` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Gerrit Code Review API. They allow AI agents to manage code changes, projects, branches, groups, and reviews. **Requires Gerrit username/password authentication.**

## Authentication

**Nango BASIC**:
- User provides Gerrit username and password
- Credentials stored in Nango, accessed via `connection_ref`
- Base URL: Configured Gerrit host

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gerrit_list_changes` | List changes | GET | /changes/ |
| `gerrit_get_change` | Get change details | GET | /changes/{change_id} |
| `gerrit_list_comments` | List comments on change | GET | /changes/{change_id}/comments |
| `gerrit_list_files` | List files in change | GET | /changes/{change_id}/revisions/{revision_id}/files |
| `gerrit_get_commit` | Get commit details | GET | /changes/{change_id}/revisions/{revision_id}/commit |
| `gerrit_list_projects` | List projects | GET | /projects/ |
| `gerrit_get_project` | Get project details | GET | /projects/{project_name} |
| `gerrit_list_branches` | List branches | GET | /projects/{project_name}/branches |
| `gerrit_list_groups` | List groups | GET | /groups/ |
| `gerrit_get_server_version` | Get server version | GET | /config/server/version |

---

## Tool Details

### gerrit_list_changes

**What it does**: Lists changes (code reviews) in Gerrit.

**When to use**: Browse open reviews, filter by project or status.

**Arguments**:
- `status` (optional): Filter by status (open, merged, abandoned)
- `project` (optional): Filter by project name

**Example LLM prompt**: "List all open changes in the myproject repo"

---

### gerrit_get_change

**What it does**: Gets detailed information about a specific change.

**When to use**: Get change status, owner, reviewers.

**Arguments**:
- `change_id` (required): Change ID

**Example LLM prompt**: "Get details for change I1234567890abcdef"

---

### gerrit_list_comments

**What it does**: Lists all comments on a specific change.

**When to use**: Read code review comments.

**Arguments**:
- `change_id` (required): Change ID

**Example LLM prompt**: "List all comments on change I123456"

---

### gerrit_list_files

**What it does**: Lists all files modified in a change.

**When to use**: See what files were changed in a review.

**Arguments**:
- `change_id` (required): Change ID
- `revision_id` (optional): Revision ID (default: current)

**Example LLM prompt**: "List files changed in revision abc123"

---

### gerrit_get_commit

**What it does**: Gets commit details for a change revision.

**When to use**: Get commit message, author, commit ID.

**Arguments**:
- `change_id` (required): Change ID
- `revision_id` (required): Revision ID

**Example LLM prompt**: "Get commit info for revision abc123"

---

### gerrit_list_projects

**What it does**: Lists all projects in Gerrit.

**When to use**: Browse available repositories.

**Arguments**:
- `prefix` (optional): Filter by project name prefix

**Example LLM prompt**: "List all projects starting with myorg-"

---

### gerrit_get_project

**What it does**: Gets detailed information about a project.

**When to use**: Get project description, parent, branches.

**Arguments**:
- `project_name` (required): Project name

**Example LLM prompt**: "Get info for project myorg/myproject"

---

### gerrit_list_branches

**What it does**: Lists all branches in a project.

**When to use**: See all branches in a repo.

**Arguments**:
- `project_name` (required): Project name

**Example LLM prompt**: "List all branches in myproject"

---

### gerrit_list_groups

**What it does**: Lists all groups in Gerrit.

**When to use**: Browse access control groups.

**Arguments**: None

**Example LLM prompt**: "List all groups"

---

### gerrit_get_server_version

**What it does**: Gets Gerrit server version information.

**When to use**: Check server status and version.

**Arguments**: None

**Example LLM prompt**: "Get Gerrit server version"

---

## Gerrit API Notes

- **Change ID format**: Typically starts with I followed by a SHA-1 hash
- **Revision ID**: Each patchset has a revision ID
- **Status workflow**: New -> Review -> Verified -> Submit (merged)
- **Projects**: Repositories in Gerrit
- **Groups**: Used for access control and notifications

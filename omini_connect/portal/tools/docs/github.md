# GitHub Tools

Provider: `github` | Engine: `nango` | Auth: OAuth (via Nango) or PAT (native)

## Overview

These tools wrap the GitHub REST API. They allow AI agents to interact with repositories, issues, pull requests, and branches on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `repo`, `user`, `workflow` etc.

**Native PAT (engine=omini_connect_native)**:
- PAT stored in `client_secret` field
- Direct Bearer token passthrough to GitHub API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `github_list_repos` | List repositories for the authenticated user | GET | /user/repos |
| `github_get_repo` | Get details of a specific repository | GET | /repos/{owner}/{repo} |
| `github_create_repo` | Create a new repository | POST | /user/repos |
| `github_list_issues` | List issues in a repository | GET | /repos/{owner}/{repo}/issues |
| `github_get_issue` | Get a specific issue | GET | /repos/{owner}/{repo}/issues/{issue_number} |
| `github_create_issue` | Create a new issue | POST | /repos/{owner}/{repo}/issues |
| `github_list_pull_requests` | List pull requests in a repository | GET | /repos/{owner}/{repo}/pulls |
| `github_get_pull_request` | Get a specific pull request | GET | /repos/{owner}/{repo}/pulls/{pull_number} |
| `github_list_branches` | List branches in a repository | GET | /repos/{owner}/{repo}/branches |
| `github_get_branch` | Get a specific branch | GET | /repos/{owner}/{repo}/branches/{branch} |

---

## Tool Details

### github_list_repos

**What it does**: Returns a paginated list of repositories the authenticated user owns or has access to (including organization repos).

**When to use**: Before creating issues, PRs, or searching, an agent should list repos to know what exists.

**Arguments**:
- `sort` (optional): `created` | `updated` | `pushed` | `full_name` — default `full_name`
- `direction` (optional): `asc` | `desc` — default `desc`
- `per_page` (optional, max 100): default 30
- `page` (optional): default 1

**Example LLM prompt**: "List my repositories to find one about AI"

---

### github_get_repo

**What it does**: Get details of a specific repository including description, visibility, default branch, and stats.

**When to use**: Understand repository configuration, check settings, or get context before making changes.

**Arguments**:
- `owner` (required): Repository owner username or organization
- `repo` (required): Repository name

**Example LLM prompt**: "Get details for the ai-attestation repository"

---

### github_create_repo

**What it does**: Creates a new repository under the authenticated user's account or an organization.

**When to use**: Initialize a new project or create an org repo programmatically.

**Arguments**:
- `name` (required): Repository name (must be unique to the owner)
- `description` (optional): Repository description
- `private` (optional): Make private (default: true)
- `auto_init` (optional): Initialize with README (default: false)
- `org` (optional): Organization name (if creating under an org)

**Example LLM prompt**: "Create a new private repo called 'my-new-project'"

---

### github_list_issues

**What it does**: Lists issues in a repository. Filter by state (open/closed), labels, or assignees.

**When to use**: Track open bugs, feature requests, or tasks in a repository.

**Arguments**:
- `owner` (required): Repository owner username
- `repo` (required): Repository name
- `state` (optional): `open` | `closed` | `all` — default `open`
- `sort` (optional): `created` | `updated` | `comments` — default `created`
- `direction` (optional): `asc` | `desc`
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "Show me all open issues in allenpeng0705/ai-attestation"

---

### github_get_issue

**What it does**: Gets details of a specific issue including title, body, labels, assignees, and state.

**When to use**: Read the full content of an issue before responding, editing, or closing it.

**Arguments**:
- `owner` (required): Repository owner username
- `repo` (required): Repository name
- `issue_number` (required): Issue number

**Example LLM prompt**: "Show me issue #42 in allenpeng0705/ai-attestation"

---

### github_create_issue

**What it does**: Creates a new issue in a specified repository.

**When to use**: File bugs, log feature requests, or create task items from conversation.

**Arguments**:
- `owner` (required): Repository owner username
- `repo` (required): Repository name
- `title` (required): Issue title
- `body` (optional): Issue description (markdown supported)
- `labels` (optional): Array of label names
- `assignees` (optional): Array of usernames to assign

**Example LLM prompt**: "Create an issue in allenpeng0705/ai-attestation titled 'Fix memory leak in loader'"

---

### github_list_pull_requests

**What it does**: Lists pull requests in a repository, optionally filtered by state.

**When to use**: Track open PRs, review pending changes, or monitor merge status.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `state` (optional): `open` | `closed` | `all` — default `open`
- `sort` (optional): `created` | `updated` | `popularity`
- `direction` (optional): `asc` | `desc`
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "Show me all open pull requests in this repository"

---

### github_get_pull_request

**What it does**: Gets details of a specific pull request — title, body, state, reviews, and merge status.

**When to use**: Before merging, reviewing, or responding to a PR. Check if it's mergeable.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `pull_number` (required): Pull request number

**Example LLM prompt**: "Show me PR #15 — can it be merged?"

---

### github_list_branches

**What it does**: Lists all branches in a repository including protected branch info.

**When to use**: See what branches exist before creating a PR, creating a branch, or checking CI status.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `per_page` (optional): default 30
- `page` (optional): default 1

**Example LLM prompt**: "List all branches in this repository"

---

### github_get_branch

**What it does**: Get details of a specific branch including the commit SHA, protected status, and branch name.

**When to use**: Check branch details, find the latest commit, or verify branch protection rules.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `branch` (required): Branch name

**Example LLM prompt**: "Get details for the main branch"

---

## GitHub API Reference

These tools use the GitHub REST API. See official docs for full details:
- https://docs.github.com/rest
- Rate limits: 5,000 requests/hour for authenticated users
- Pagination: Use `per_page` and `page` parameters
- All dates: ISO 8601 format (UTC)

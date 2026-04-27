# GitHub App OAuth Tools

Provider: `github-app-oauth` | Engine: `nango` | Auth: CUSTOM (OAuth2 + App) via Nango

## Overview

These tools wrap the GitHub App OAuth API. They allow AI agents to manage repositories, issues, pull requests, comments, labels, and milestones using OAuth2 with GitHub App capabilities. **Requires GitHub OAuth with App permissions.**

## Authentication

**Nango CUSTOM (OAuth2 + GitHub App)**:
- Supports both OAuth2 user tokens and GitHub App installation tokens
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.github.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `github_app_oauth_list_repos` | List repositories | GET | /installation/repositories |
| `github_app_oauth_get_repo` | Get repository details | GET | /repos/{owner}/{repo} |
| `github_app_oauth_list_issues` | List issues | GET | /issues |
| `github_app_oauth_get_issue` | Get issue details | GET | /repos/{owner}/{repo}/issues/{issue_number} |
| `github_app_oauth_list_pulls` | List pull requests | GET | /repos/{owner}/{repo}/pulls |
| `github_app_oauth_get_pull` | Get pull request details | GET | /repos/{owner}/{repo}/pulls/{pull_number} |
| `github_app_oauth_list_comments` | List issue comments | GET | /repos/{owner}/{repo}/issues/{issue_number}/comments |
| `github_app_oauth_create_comment` | Create issue comment | POST | /repos/{owner}/{repo}/issues/{issue_number}/comments |
| `github_app_oauth_list_labels` | List labels | GET | /repos/{owner}/{repo}/labels |
| `github_app_oauth_list_milestones` | List milestones | GET | /repos/{owner}/{repo}/milestones |

---

## Tool Details

### github_app_oauth_list_repos

**What it does**: Lists repositories the authenticated user has access to.

**When to use**: Browse user's repositories and org repos.

**Arguments**:
- `per_page` (optional): Results per page (default 30)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all my repositories"

---

### github_app_oauth_get_repo

**What it does**: Gets detailed information about a repository.

**When to use**: Get repo description, default branch, settings.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name

**Example LLM prompt**: "Get details for owner/repo"

---

### github_app_oauth_list_issues

**What it does**: Lists issues in repositories the user has access to.

**When to use**: Browse issues across repositories.

**Arguments**:
- `state` (optional): Filter by state (open, closed, all)
- `repo` (optional): Filter by repository (owner/repo)

**Example LLM prompt**: "List all my assigned open issues"

---

### github_app_oauth_get_issue

**What it does**: Gets detailed information about a specific issue.

**When to use**: Get issue details, labels, assignees.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `issue_number` (required): Issue number

**Example LLM prompt**: "Get issue #123 in owner/repo"

---

### github_app_oauth_list_pulls

**What it does**: Lists pull requests in a repository.

**When to use**: Browse open pull requests.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `state` (optional): Filter by state (open, closed, all)

**Example LLM prompt**: "List all open pull requests"

---

### github_app_oauth_get_pull

**What it does**: Gets detailed information about a pull request.

**When to use**: Get PR details, reviews, status checks.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `pull_number` (required): Pull request number

**Example LLM prompt**: "Get PR #456 in owner/repo"

---

### github_app_oauth_list_comments

**What it does**: Lists comments on an issue or pull request.

**When to use**: Read discussion on issues/PRs.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `issue_number` (required): Issue or PR number

**Example LLM prompt**: "List comments on issue #123"

---

### github_app_oauth_create_comment

**What it does**: Creates a comment on an issue or pull request.

**When to use**: Respond to issues or PRs.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `issue_number` (required): Issue or PR number
- `body` (required): Comment text

**Example LLM prompt**: "Add a comment to issue #123"

---

### github_app_oauth_list_labels

**What it does**: Lists labels in a repository.

**When to use**: See available labels for tagging issues.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name

**Example LLM prompt**: "List labels in owner/repo"

---

### github_app_oauth_list_milestones

**What it does**: Lists milestones in a repository.

**When to use**: Track project progress.

**Arguments**:
- `owner` (required): Repository owner
- `repo` (required): Repository name
- `state` (optional): Filter by state (open, closed, all)

**Example LLM prompt**: "List open milestones in owner/repo"

---

## GitHub App OAuth API Notes

- **Dual auth mode**: Can use OAuth2 user token or App installation token
- **User context**: OAuth2 tokens have user permissions
- **App context**: App tokens have installation permissions
- **Rate limits**: Subject to GitHub's rate limiting rules

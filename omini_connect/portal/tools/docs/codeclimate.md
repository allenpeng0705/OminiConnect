# Code Climate Tools

Provider: `codeclimate` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Code Climate API. Code Climate is a code quality and security analysis platform. **Requires Code Climate API key.**

## Authentication

**Nango API_KEY**:
- User provides their Code Climate API token
- Token passed via `Authorization: Token token=${apiKey}` header
- Base URL: `https://${connectionConfig.domain}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `codeclimate_list_repos` | List repositories | GET | /repos |
| `codeclimate_get_repo` | Get repository details | GET | /repos/{id} |
| `codeclimate_get_user` | Get user details | GET | /user |
| `codeclimate_list_issues` | List issues | GET | /repos/{repoId}/issues |
| `codeclimate_get_issue` | Get issue details | GET | /issues/{id} |
| `codeclimate_get_snapshot` | Get snapshot details | GET | /snapshots/{id} |
| `codeclimate_list_comments` | List comments | GET | /issues/{issueId}/comments |
| `codeclimate_create_comment` | Create a comment | POST | /issues/{issueId}/comments |
| `codeclimate_list_badges` | List badges | GET | /repos/{repoId}/badges |
| `codeclimate_get_analysis` | Get analysis results | GET | /repos/{repoId}/analysis |

---

## Tool Details

### codeclimate_list_repos

**What it does**: Lists all repositories in your organization.

**When to use**: Find repos to analyze.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Code Climate repos"

---

### codeclimate_get_repo

**What it does**: Gets details of a specific repository.

**When to use**: View repo analysis summary.

**Arguments**:
- `id` (required): Repository ID

**Example LLM prompt**: "Get repo 123 details"

---

### codeclimate_get_user

**What it does**: Gets details of the authenticated user.

**When to use**: Verify identity and organization.

**Arguments**: None

**Example LLM prompt**: "Get my Code Climate user info"

---

### codeclimate_list_issues

**What it does**: Lists all issues for a repository.

**When to use**: View code quality issues.

**Arguments**:
- `repoId` (required): Repository ID
- `filter` (optional): Filter by issue type

**Example LLM prompt**: "List issues for repo 123"

---

### codeclimate_get_issue

**What it does**: Gets details of a specific issue.

**When to use**: View issue details and location.

**Arguments**:
- `id` (required): Issue ID

**Example LLM prompt**: "Get issue 456 details"

---

### codeclimate_get_snapshot

**What it does**: Gets details of a specific snapshot.

**When to use**: View analysis snapshot data.

**Arguments**:
- `id` (required): Snapshot ID

**Example LLM prompt**: "Get snapshot 789 details"

---

### codeclimate_list_comments

**What it does**: Lists all comments for an issue.

**When to use**: View issue discussions.

**Arguments**:
- `issueId` (required): Issue ID

**Example LLM prompt**: "List comments for issue 456"

---

### codeclimate_create_comment

**What it does**: Creates a new comment on an issue.

**When to use**: Add discussion to an issue.

**Arguments**:
- `issueId` (required): Issue ID
- `body` (required): Comment body

**Example LLM prompt**: "Add a comment to issue 456"

---

### codeclimate_list_badges

**What it does**: Lists all badges for a repository.

**When to use**: Get badge URLs for README.

**Arguments**:
- `repoId` (required): Repository ID

**Example LLM prompt**: "List badges for repo 123"

---

### codeclimate_get_analysis

**What it does**: Gets the latest analysis results.

**When to use**: View current code quality score.

**Arguments**:
- `repoId` (required): Repository ID

**Example LLM prompt**: "Get analysis for repo 123"

---

## Code Climate API Notes

- **Domain**: Required configuration for API access (e.g., codeclimate.com)
- **Issues**: Code quality problems found in analysis
- **Snapshots**: Point-in-time analysis results
- **Badges**: Code quality badges for repos

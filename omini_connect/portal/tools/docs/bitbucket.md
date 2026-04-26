# Bitbucket Tools

Provider: `bitbucket` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Bitbucket REST API. They allow AI agents to manage repositories, branches, commits, pull requests, and snippets. Bitbucket is a Git repository hosting service by Atlassian, commonly used for code collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Bitbucket
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `repository`, `snippet`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bitbucket_list_repositories` | List repositories for a workspace | GET | /repositories/{workspace} |
| `bitbucket_get_repository` | Get details of a specific repository | GET | /repositories/{workspace}/{repo_slug} |
| `bitbucket_create_repository` | Create a new repository | POST | /repositories/{workspace} |
| `bitbucket_list_branches` | List branches in a repository | GET | /repositories/{workspace}/{repo_slug}/refs/branches |
| `bitbucket_get_branch` | Get a specific branch | GET | /repositories/{workspace}/{repo_slug}/refs/branches/{branch} |
| `bitbucket_list_commits` | List commits in a repository | GET | /repositories/{workspace}/{repo_slug}/commits |
| `bitbucket_get_commit` | Get a specific commit | GET | /repositories/{workspace}/{repo_slug}/commit/{revision} |
| `bitbucket_list_pullrequests` | List pull requests in a repository | GET | /repositories/{workspace}/{repo_slug}/pullrequests |
| `bitbucket_get_pullrequest` | Get a specific pull request | GET | /repositories/{workspace}/{repo_slug}/pullrequests/{pull_request_id} |
| `bitbucket_list_snippets` | List snippets in a workspace | GET | /snippets/{workspace} |

---

## Tool Details

### bitbucket_list_repositories

**What it does**: Lists repositories for a workspace with filtering by role and pagination.

**When to use**: Find repositories, browse available projects, check your repository access across a workspace.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `role` (optional): Filter by role (`owner`, `collaborator`, `member`)
- `per_page` (optional): Results per page (default 10, max 100)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all repositories in the mycompany workspace where I'm an owner"

---

### bitbucket_get_repository

**What it does**: Gets detailed information about a specific repository including description, visibility, and main branch.

**When to use**: Understand repository configuration, check settings, review project details before making changes.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)

**Example LLM prompt**: "Get details for the backend-api repository in mycompany workspace"

---

### bitbucket_create_repository

**What it does**: Creates a new repository in a workspace.

**When to use**: Initialize a new project, create a repo for a new codebase, set up repository structure.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `name` (required): Repository name
- `description` (optional): Repository description
- `is_private` (optional): Make private (default: true)
- `scm` (optional): SCM type: `git` (default)

**Example LLM prompt**: "Create a new private repository called 'ml-pipeline' in the mycompany workspace"

---

### bitbucket_list_branches

**What it does**: Lists all branches in a repository including protected branch info.

**When to use**: Find branches, see feature branches, check branch naming patterns, identify stale branches.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `sort` (optional): Sort by (`-timestamp`, `alphabetical`) (default: `-timestamp`)
- `per_page` (optional): Results per page (default 10)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all branches in the frontend-app repository"

---

### bitbucket_get_branch

**What it does**: Gets detailed information about a specific branch including the target, head commit, and metadata.

**When to use**: Check branch status, find branch head commit, understand branch protection rules.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `branch` (required): Branch name

**Example LLM prompt**: "Get details for the feature/new-checkout branch"

---

### bitbucket_list_commits

**What it does**: Lists commits in a repository, optionally filtered by branch. Useful for viewing change history and author information.

**When to use**: View commit history, find recent changes, check branch activity, trace when code was introduced.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `branch` (optional): Branch name to list commits from
- `per_page` (optional): Results per page (default 10, max 100)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List commits on the main branch from the last week"

---

### bitbucket_get_commit

**What it does**: Gets detailed information about a specific commit including author, message, changed files, and parent commits.

**When to use**: Understand changes, review code diff, find commit details, see what files were modified.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `revision` (required): Commit hash or branch/tag name

**Example LLM prompt**: "Get details for commit abc123def to see what files were changed"

---

### bitbucket_list_pullrequests

**What it does**: Lists pull requests in a repository. Pull requests are proposals to merge changes from one branch into another.

**When to use**: Find open PRs, review pending changes, track code review progress, see recently merged PRs.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `state` (optional): Filter by state (`OPEN`, `CLOSED`, `MERGED`, `SUPERSEDED`) (default: `OPEN`)
- `per_page` (optional): Results per page (default 10)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open pull requests in the backend-api repository"

---

### bitbucket_get_pullrequest

**What it does**: Gets detailed information about a specific pull request including title, description, source and target branches, reviewers, and current status.

**When to use**: Review PR details, check reviewer status, understand changes, see if it can be merged.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `repo_slug` (required): Repository name (slug)
- `pull_request_id` (required): Pull request ID

**Example LLM prompt**: "Get details for pull request #42 to see if it's ready to merge"

---

### bitbucket_list_snippets

**What it does**: Lists code snippets for the authenticated user. Snippets are small pieces of code that can be shared publicly or kept private.

**When to use**: Browse your code snippets, find shared utilities, manage saved code fragments.

**Arguments**:
- `workspace` (required): Workspace ID or slug
- `per_page` (optional): Results per page (default 10)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all my code snippets in the mycompany workspace"

---

## Bitbucket API Notes

- **Workspace slugs**: Lowercase, hyphenated names (e.g., `my-company`)
- **Repository slugs**: Lowercase, hyphenated names (e.g., `backend-api`)
- **Pull request IDs**: Numeric identifiers (e.g., `#42`)
- **Commit hashes**: Full SHA-1 hashes (40 characters) or shorter references
- **Branch names**: Can include slashes like `feature/login` but are URL-encoded in API paths
- **Rate Limits**: Bitbucket API has rate limits; use pagination for large result sets
- **Snippets**: Individual files within a snippet are accessed separately via the snippets API

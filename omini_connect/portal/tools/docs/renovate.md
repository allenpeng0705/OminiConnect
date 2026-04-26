# Renovate Tools Documentation

Renovate is an automated dependency update tool that creates pull requests to keep dependencies up-to-date. It supports multiple package ecosystems and provides extensive configuration options.

## Available Tools

### Repositories

#### `renovate_list_repositories`
Retrieve all repositories configured with Renovate in the organization.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org | string | Yes | Organization name |
| include_pull_requests | boolean | No | Include active pull requests (default: true) |

#### `renovate_get_repository`
Retrieve detailed configuration and status for a Renovate repository.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |

### Pull Requests

#### `renovate_list_pull_requests`
Retrieve all Renovate-managed pull requests for repositories.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| status | string | No | Filter by status (open, closed, merged) |
| limit | integer | No | Maximum number of results (default: 100) |

#### `renovate_get_pull_request`
Retrieve details about a specific Renovate pull request.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| pr_number | integer | Yes | Pull request number |

### Configuration

#### `renovate_get_config`
Retrieve the Renovate configuration for a repository.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |

#### `renovate_update_config`
Update the Renovate configuration for a repository.

**Scopes:** `renovate_write`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| config | object | Yes | Renovate configuration object |

### Branches

#### `renovate_list_branches`
Retrieve all branches managed by Renovate for a repository.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| limit | integer | No | Maximum number of results |

#### `renovate_get_branch`
Retrieve details about a specific Renovate-managed branch.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| branch_name | string | Yes | Branch name |

#### `renovate_create_branch`
Create a new Renovate-managed branch for dependency updates.

**Scopes:** `renovate_write`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |
| package_rules | array | No | Rules for dependency updates |
| branch_name | string | No | Desired branch name |
| commit_message | string | No | Custom commit message |

### Status

#### `renovate_get_repository_status`
Retrieve the overall Renovate status for a repository including dashboard state.

**Scopes:** `renovate_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| repo_name | string | Yes | Full repository name (org/repo) |

## Authentication

Renovate typically uses a bot account or GitHub App authentication. Configure your Renovate bot token or app installation in your integration settings.

## Rate Limits

Renovate API rate limits depend on your self-hosted or Mend (formerly WhiteSource) hosted plan configuration.

## Scopes

| Scope | Description |
|-------|-------------|
| renovate_read | Read access to Renovate repositories and configuration |
| renovate_write | Create and manage branches, pull requests, and configuration updates |
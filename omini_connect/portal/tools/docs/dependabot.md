# Dependabot Tools Documentation

Dependabot is GitHub's automated dependency update service that keeps your repositories secure and up-to-date by creating pull requests for outdated dependencies.

## Available Tools

### Repositories

#### `dependabot_list_repositories`
Retrieve all repositories enabled for Dependabot in the organization.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org | string | Yes | GitHub organization login |
| per_page | integer | No | Results per page (default: 100) |
| page | integer | No | Page number for pagination |

#### `dependabot_get_repository`
Retrieve Dependabot configuration and status for a specific repository.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |

### Dependencies

#### `dependabot_list_dependencies`
Retrieve all dependencies tracked by Dependabot for a repository.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| manifest_path | string | No | Filter by manifest file path |

#### `dependabot_get_dependency`
Retrieve details about a specific dependency tracked by Dependabot.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| dependency_id | string | Yes | The dependency identifier (package ecosystem:name:version) |

#### `dependabot_update_dependency`
Trigger an immediate update for a specific dependency via Dependabot.

**Scopes:** `dependabot_write`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| update_id | string | Yes | The update configuration ID |
| version | string | Yes | Target version for the update |

### Pull Requests

#### `dependabot_list_pull_requests`
Retrieve all Dependabot pull requests for repositories in the organization.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org | string | Yes | GitHub organization login |
| repo | string | No | Repository name (optional, for single repo) |
| state | string | No | Filter by state (open, closed, all) |
| package_ecosystem | string | No | Filter by package ecosystem (npm, pip, gems) |

#### `dependabot_get_pull_request`
Retrieve details about a specific Dependabot pull request.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| pull_number | integer | Yes | Pull request number |

#### `dependabot_create_pull_request`
Create a Dependabot pull request to update a dependency.

**Scopes:** `dependabot_write`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| dependency | object | Yes | Dependency to update |
| version_updates | array | No | Version update specifications |

### Security Updates

#### `dependabot_list_security_updates`
Retrieve all security updates configured for repositories.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org | string | Yes | GitHub organization login |
| repository | string | No | Specific repository name |

#### `dependabot_get_security_update`
Retrieve details about a specific security update.

**Scopes:** `dependabot_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| owner | string | Yes | Repository owner login |
| repo | string | Yes | Repository name |
| update_id | string | Yes | The security update ID |

## Authentication

Dependabot uses GitHub OAuth or token-based authentication. Configure your GitHub App or personal access token in your integration settings.

## Rate Limits

GitHub API rate limits apply (5000 requests/hour for authenticated users). Use conditional requests to minimize API calls.

## Scopes

| Scope | Description |
|-------|-------------|
| dependabot_read | Read access to Dependabot data |
| dependabot_write | Create and manage Dependabot pull requests and updates |
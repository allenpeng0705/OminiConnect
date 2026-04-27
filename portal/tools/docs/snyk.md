# Snyk Tools Documentation

Snyk is a developer-first security platform that helps find and fix vulnerabilities in dependencies, containers, and infrastructure as code.

## Available Tools

### Projects

#### `snyk_list_projects`
Retrieve all projects associated with the Snyk account or organization.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| version | string | No | API version (default: 2024-01-29) |

#### `snyk_get_project`
Retrieve detailed information about a specific Snyk project.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| project_id | string | Yes | The Snyk project ID |

### Issues

#### `snyk_list_issues`
Retrieve all issues (vulnerabilities and license issues) for a project.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| project_id | string | Yes | The Snyk project ID |
| version | string | No | API version (default: 2024-01-29) |
| include_vulnerabilities | boolean | No | Include vulnerability issues (default: true) |
| include_license_issues | boolean | No | Include license issues (default: true) |

#### `snyk_get_issue`
Retrieve detailed information about a specific issue in a project.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| issue_id | string | Yes | The issue ID (e.g., SNYK-JS-LODASH-450201) |
| version | string | No | API version (default: 2024-01-29) |

### Vulnerabilities

#### `snyk_list_vulnerabilities`
Retrieve all vulnerabilities across projects in an organization.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| version | string | No | API version (default: 2024-01-29) |
| pagination | string | No | Pagination cursor for next page |

#### `snyk_get_vulnerability`
Retrieve detailed information about a specific vulnerability by ID.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| vulnerability_id | string | Yes | The vulnerability identifier (e.g., SNYK-JS-LODASH-450201) |
| version | string | No | API version (default: 2024-01-29) |

### Dependencies

#### `snyk_list_dependencies`
Retrieve all dependencies for a specific project.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| project_id | string | Yes | The Snyk project ID |
| version | string | No | API version (default: 2024-01-29) |

#### `snyk_get_dependency`
Retrieve detailed information about a specific dependency.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| project_id | string | Yes | The Snyk project ID |
| dependency_id | string | Yes | The dependency ID |

### Reports

#### `snyk_get_test_report`
Retrieve a test report for a specific project showing vulnerability findings.

**Scopes:** `snyk_orgs_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| project_id | string | Yes | The Snyk project ID |
| report_type | string | No | Type of report (full, summary) |

### Tokens

#### `snyk_list_api_tokens`
Retrieve all API tokens for a service account or user in the organization.

**Scopes:** `snyk_orgs_admin`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_id | string | Yes | The Snyk organization ID |
| member_id | string | Yes | The member ID |

## Authentication

Snyk uses API token authentication. Obtain your API token from the Snyk dashboard and configure it in your integration settings.

## Rate Limits

Snyk API has rate limits that vary by plan. The API returns 429 status when limits are exceeded with a Retry-After header.

## Scopes

| Scope | Description |
|-------|-------------|
| snyk_orgs_read | Read access to organization data |
| snyk_orgs_admin | Admin access to organization management |
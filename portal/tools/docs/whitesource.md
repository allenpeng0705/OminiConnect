# WhiteSource Tools Documentation

WhiteSource (now part of Mend) is a software composition analysis platform that helps organizations manage open source license compliance and security vulnerabilities.

## Available Tools

### Projects

#### `whitesource_list_projects`
Retrieve all projects in the WhiteSource organization.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| product_ids | array | No | Filter by product IDs |
| project_ids | array | No | Filter by project IDs |

#### `whitesource_get_project`
Retrieve detailed information about a specific WhiteSource project.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| project_token | string | Yes | The unique project token |

### Licenses

#### `whitesource_list_licenses`
Retrieve all licenses identified across projects in the organization.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| project_tokens | array | No | Filter by project tokens |

#### `whitesource_get_license`
Retrieve detailed information about a specific license.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| license_id | string | Yes | The license identifier |

### Vulnerabilities

#### `whitesource_list_vulnerabilities`
Retrieve all vulnerabilities detected across projects.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| project_token | string | No | Filter by project token |
| severity | string | No | Filter by severity (HIGH, MEDIUM, LOW) |

#### `whitesource_get_vulnerability`
Retrieve detailed information about a specific vulnerability.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| vulnerability_id | string | Yes | The vulnerability identifier |

### Reports

#### `whitesource_get_license_report`
Generate and retrieve a license compliance report.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| project_tokens | array | No | Include specific projects |
| report_format | string | No | Report format (JSON, CSV, PDF) |

#### `whitesource_get_vulnerability_report`
Generate and retrieve a vulnerability report for the organization.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| project_token | string | No | Specific project token |
| severity_filter | array | No | Filter by severities |

### Alerts

#### `whitesource_list_alerts`
Retrieve all active alerts for the organization.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| alert_type | string | No | Filter by alert type (SECURITY, LICENSE, OUTDATED) |
| project_token | string | No | Filter by project token |

#### `whitesource_get_alert`
Retrieve detailed information about a specific alert.

**Scopes:** `whitesource_read`

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| org_token | string | Yes | The WhiteSource organization API token |
| alert_id | string | Yes | The alert identifier |

## Authentication

WhiteSource uses API token authentication via the `org_token` parameter. Obtain your API token from the WhiteSource/Mend dashboard.

## Rate Limits

API rate limits vary by plan. The API returns 429 status when limits are exceeded.

## Scopes

| Scope | Description |
|-------|-------------|
| whitesource_read | Read access to organization data |
| whitesource_write | Write access for managing configurations |
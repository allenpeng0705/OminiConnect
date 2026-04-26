# Vercel Provider Documentation

## Overview

Vercel is a cloud platform for frontend frameworks and static sites, providing deployment, serverless functions, and edge computing capabilities for modern web applications.

## Authentication

Vercel uses OAuth 2.0 or API tokens for authentication. The following scopes are available:

- `read` - Read access to deployments, projects, domains, and secrets
- `write` - Create and modify deployments, projects, domains, and secrets

## Base URL

```
https://api.vercel.com
```

## Rate Limits

- Default: 300 requests per minute for most endpoints
- Higher limits available for Enterprise plans
- Some endpoints have specific rate limits noted in their descriptions

## Available Tools

### Deployments

| Tool | Description |
|------|-------------|
| `vercel_list_deployments` | Retrieve a list of all deployments for the authenticated user or team |
| `vercel_get_deployment` | Retrieve details of a specific deployment including build logs and status |
| `vercel_create_deployment` | Create a new deployment from a Git repository or local files |

### Projects

| Tool | Description |
|------|-------------|
| `vercel_list_projects` | Retrieve all projects for the authenticated user or team |
| `vercel_get_project` | Retrieve details and configuration of a specific project |

### Domains

| Tool | Description |
|------|-------------|
| `vercel_list_domains` | Retrieve all domains configured for a project or account |
| `vercel_get_domain` | Retrieve details of a specific domain including DNS records and verification status |
| `vercel_add_domain` | Add a new domain to a project or account |

### Secrets

| Tool | Description |
|------|-------------|
| `vercel_list_secrets` | Retrieve all environment secrets for a team or user |
| `vercel_get_secret` | Retrieve details of a specific environment secret |

## Tool Details

### vercel_list_deployments

List all deployments for the authenticated user or team.

**Endpoint:** `GET /v6/deployments`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | integer | No | Maximum number of deployments to return (default: 20, max: 100) |
| `until` | integer | No | Unix timestamp to list deployments until |
| `from` | integer | No | Unix timestamp to list deployments from |
| `projectId` | string | No | Filter deployments by project ID |
| `state` | string | No | Filter by state: BUILDING, ERROR, READY, QUEUED, CANCELED |

**Example Request:**
```bash
curl -H "Authorization: Bearer {token}" \
     "https://api.vercel.com/v6/deployments?limit=20&state=READY"
```

**Example Response:**
```json
{
  "deployments": [
    {
      "uid": "dpl_abc123xyz",
      "name": "my-project",
      "url": "my-project.vercel.app",
      "state": "READY",
      "createdAt": 1710934800000,
      "readyAt": 1710934850000,
      "source": "git",
      "meta": {}
    }
  ],
  "pagination": {
    "count": 20,
    "next": "eyJpZCI6..."
  }
}
```

---

### vercel_get_deployment

Retrieve details of a specific deployment including build logs and status.

**Endpoint:** `GET /v13/deployments/{deploymentId}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `deploymentId` | string | Yes | The unique identifier of the deployment (URL-safe ID or URL) |

---

### vercel_create_deployment

Create a new deployment from a Git repository or local files.

**Endpoint:** `POST /v13/deployments`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Project name for the deployment |
| `gitSource` | object | No | Git repository source |
| `project` | string | No | Project ID to deploy to |
| `environmentVariables` | array | No | Environment variables to set |

**Example Request:**
```bash
curl -X POST -H "Authorization: Bearer {token}" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "my-new-project",
       "gitSource": {
         "type": "github",
         "repo": "myorg/myrepo",
         "ref": "main"
       },
       "environmentVariables": [
         {"key": "NODE_ENV", "value": "production", "target": ["production"]}
       ]
     }' \
     "https://api.vercel.com/v13/deployments"
```

---

### vercel_list_projects

Retrieve all projects for the authenticated user or team.

**Endpoint:** `GET /v9/projects`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | integer | No | Maximum number of projects to return (default: 20, max: 100) |
| `since` | integer | No | Unix timestamp to filter projects from |
| `teamId` | string | No | Team ID to list projects for |

---

### vercel_get_project

Retrieve details and configuration of a specific project.

**Endpoint:** `GET /v9/projects/{projectId}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | string | Yes | The unique identifier of the project |
| `teamId` | string | No | Team ID the project belongs to |

---

### vercel_list_domains

Retrieve all domains configured for a project or account.

**Endpoint:** `GET /v6/domains`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | string | No | Filter domains by project ID |
| `teamId` | string | No | Team ID to list domains for |
| `limit` | integer | No | Maximum number of domains to return |

---

### vercel_get_domain

Retrieve details of a specific domain including DNS records and verification status.

**Endpoint:** `GET /v6/domains/{domain}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `domain` | string | Yes | The domain name to retrieve |
| `teamId` | string | No | Team ID the domain belongs to |

---

### vercel_add_domain

Add a new domain to a project or account.

**Endpoint:** `POST /v6/domains`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Domain name to add |
| `projectId` | string | No | Project ID to attach the domain to |
| `teamId` | string | No | Team ID to add domain to |
| `gitFork` | boolean | No | Auto-configure DNS for git fork repos |
| `redirect` | string | No | Redirect domain to another domain |
| `redirectStatusCode` | integer | No | HTTP status code for redirect (307, 308) |

---

### vercel_list_secrets

Retrieve all environment secrets for a team or user.

**Endpoint:** `GET /v9/secrets`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `teamId` | string | No | Team ID to list secrets for |
| `limit` | integer | No | Maximum number of secrets to return |
| `since` | integer | No | Unix timestamp to filter from |

---

### vercel_get_secret

Retrieve details of a specific environment secret.

**Endpoint:** `GET /v9/secrets/{secretName}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `secretName` | string | Yes | Name of the secret to retrieve |
| `teamId` | string | No | Team ID the secret belongs to |

---

## Common Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Rate Limit Exceeded |
| 500 | Internal Server Error |

## Additional Resources

- [Vercel API Documentation](https://vercel.com/docs/rest-api)
- [API Authentication](https://vercel.com/docs/rest-api#authentication)
- [Rate Limiting](https://vercel.com/docs/rest-api#rate-limiting)

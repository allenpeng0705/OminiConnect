# Netlify Provider Documentation

## Overview

Netlify is a cloud platform for modern web projects, providing hosting, serverless functions, continuous deployment, and edge computing capabilities.

## Authentication

Netlify uses OAuth 2.0 or personal access tokens for authentication. The following scopes are available:

- `read` - Read access to sites, deploys, functions, and domains
- `write` - Create and modify sites, deploys, and functions

## Base URL

```
https://api.netlify.com
```

## Rate Limits

- Default: 100 requests per minute per token
- Higher limits available for paid plans
- Some endpoints have specific rate limits

## Available Tools

### Sites

| Tool | Description |
|------|-------------|
| `netlify_list_sites` | Retrieve all sites associated with the authenticated user or team |
| `netlify_get_site` | Retrieve details of a specific site including build settings and deploys |
| `netlify_create_site` | Create a new site in Netlify |
| `netlify_delete_site` | Permanently delete a site and all its deploys |

### Deploys

| Tool | Description |
|------|-------------|
| `netlify_list_deploys` | Retrieve all deploys for a specific site |
| `netlify_get_deploy` | Retrieve details of a specific deploy including build logs and files |
| `netlify_create_deploy` | Create a new deploy for a site |

### Functions

| Tool | Description |
|------|-------------|
| `netlify_list_functions` | Retrieve all serverless functions for a site or account |
| `netlify_get_function` | Retrieve details and code of a specific serverless function |

### Domains

| Tool | Description |
|------|-------------|
| `netlify_list_domains` | Retrieve all domains configured for a site |

## Tool Details

### netlify_list_sites

Retrieve all sites associated with the authenticated user or team.

**Endpoint:** `GET /api/v1/sites`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page (default: 20, max: 100) |
| `filter` | string | No | Filter by state: all, buildable, static |
| `team_id` | string | No | Filter by team ID |

**Example Request:**
```bash
curl -H "Authorization: Bearer {token}" \
     "https://api.netlify.com/api/v1/sites?per_page=20&filter=buildable"
```

**Example Response:**
```json
{
  "sites": [
    {
      "id": "abc123-def456",
      "name": "my-awesome-site",
      "url": "https://my-awesome-site.netlify.app",
      "state": "ready",
      "build_settings": {
        "repo_url": "https://github.com/myorg/myrepo",
        "branch": "main",
        "dir": "dist"
      },
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-03-20T14:45:00Z"
    }
  ],
  "total_count": 42
}
```

---

### netlify_get_site

Retrieve details of a specific site including build settings and deploys.

**Endpoint:** `GET /api/v1/sites/{siteId}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site (UUID or site name slug) |

---

### netlify_create_site

Create a new site in Netlify.

**Endpoint:** `POST /api/v1/sites`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | No | Custom site name (slug format: lowercase, alphanumeric, hyphens) |
| `body` | object | No | Site configuration object |
| `team_id` | string | No | Team ID to create site under |

**Example Request:**
```bash
curl -X POST -H "Authorization: Bearer {token}" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "my-new-site",
       "body": {
         "custom_domain": "mynewsite.com",
         "build_settings": {
           "dir": "dist",
           "build_command": "npm run build"
         }
       }
     }' \
     "https://api.netlify.com/api/v1/sites"
```

---

### netlify_delete_site

Permanently delete a site and all its deploys.

**Endpoint:** `DELETE /api/v1/sites/{siteId}`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |
| `archive` | boolean | No | Archive the site instead of deleting (default: false) |

---

### netlify_list_deploys

Retrieve all deploys for a specific site.

**Endpoint:** `GET /api/v1/sites/{siteId}/deploys`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |

---

### netlify_get_deploy

Retrieve details of a specific deploy including build logs and files.

**Endpoint:** `GET /api/v1/deploys/{deployId}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `deployId` | string | Yes | The unique identifier of the deploy |

---

### netlify_create_deploy

Create a new deploy for a site.

**Endpoint:** `POST /api/v1/sites/{siteId}/deploys`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |
| `files` | object | No | File manifest (path to SHA1 hash) for the deploy |
| `title` | string | No | Deploy title or version identifier |
| `context` | string | No | Deploy context: production, deploy-preview, branch-deploy |
| `branch` | string | No | Git branch for the deploy |

---

### netlify_list_functions

Retrieve all serverless functions for a site or account.

**Endpoint:** `GET /api/v1/sites/{siteId}/functions`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |
| `account_id` | string | No | Account ID to list functions for |

---

### netlify_get_function

Retrieve details and code of a specific serverless function.

**Endpoint:** `GET /api/v1/sites/{siteId}/functions/{functionName}`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |
| `functionName` | string | Yes | Name of the function to retrieve |

---

### netlify_list_domains

Retrieve all domains configured for a site.

**Endpoint:** `GET /api/v1/sites/{siteId}/domains`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `siteId` | string | Yes | The unique identifier of the site |

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

- [Netlify API Documentation](https://docs.netlify.com/api/)
- [Authentication](https://docs.netlify.com/api/get-started/#authentication)
- [Rate Limiting](https://docs.netlify.com/api/get-started/#rate-limits)

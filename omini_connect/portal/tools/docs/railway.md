# Railway

Railway is an infrastructure platform that makes it easy to deploy applications, databases, and services with minimal configuration.

## Authentication

Railway uses personal API tokens for authentication. You can generate one from the Railway dashboard under Account Settings > API Tokens.

## Rate Limits

- Default rate limit: 1000 requests per minute
- List endpoints: 500 requests per minute
- Write operations: 200 requests per minute

## Base URL

```
https://graphql.railway.app/v1
```

Note: Railway primarily uses GraphQL. See their GraphQL API documentation for queries and mutations.

## Tools

### Projects

| Tool | Description |
|------|-------------|
| `railway_list_projects` | List all projects |
| `railway_get_project` | Get details for a specific project |
| `railway_create_project` | Create a new project |

### Deployments

| Tool | Description |
|------|-------------|
| `railway_list_deployments` | List all deployments for a project |
| `railway_get_deployment` | Get details for a specific deployment |
| `railway_get_deployment_logs` | Get logs for a deployment |

### Volumes

| Tool | Description |
|------|-------------|
| `railway_list_volumes` | List all persistent volumes |
| `railway_get_volume` | Get details for a specific volume |
| `railway_create_volume` | Create a new persistent volume |

### Environments

| Tool | Description |
|------|-------------|
| `railway_list_environments` | List all environments in a project |

## Scopes

| Scope | Description |
|-------|-------------|
| `projects:read` | Read access to projects |
| `projects:write` | Create and update projects |
| `deployments:read` | Read access to deployments |
| `volumes:read` | Read access to volumes |
| `volumes:write` | Create and manage volumes |
| `environments:read` | Read access to environments |

## GraphQL Example Queries

### List Projects

```graphql
query {
  projects {
    id
    name
    description
    createdAt
  }
}
```

### Get Deployment Logs

```json
{
  "deploymentId": "xxxxx",
  "stream": false
}
```

## Volume Configuration

| Property | Description |
|----------|-------------|
| `name` | Volume display name |
| `sizeGb` | Size in gigabytes |
| `filesystem` | Filesystem type (ext4, xfs) |

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad request - invalid parameters |
| 401 | Unauthorized - invalid API token |
| 403 | Forbidden - insufficient permissions |
| 404 | Not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

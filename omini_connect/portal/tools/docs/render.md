# Render

Render is a cloud hosting platform that makes it easy to deploy and manage web services, background workers, cron jobs, and persistent disks.

## Authentication

Render uses API keys for authentication. You can generate an API key from the Render dashboard under Account Settings > API Keys.

## Rate Limits

- Default rate limit: 100 requests per minute
- List endpoints: 1000 requests per hour
- Write operations: 100 requests per minute

## Base URL

```
https://api.render.com/v1
```

## Tools

### Services

| Tool | Description |
|------|-------------|
| `render_list_services` | List all services (web services, background workers, cron jobs) |
| `render_get_service` | Get details for a specific service |
| `render_create_service` | Create a new service |
| `render_update_service` | Update an existing service |
| `render_delete_service` | Delete a service and all resources |

### Deployments

| Tool | Description |
|------|-------------|
| `render_list_deploys` | List all deployments for a service |
| `render_get_deploy` | Get details for a specific deployment |
| `render_create_deploy` | Trigger a manual deployment |

### Disks

| Tool | Description |
|------|-------------|
| `render_list_disks` | List all persistent disks |
| `render_get_disk` | Get details for a specific disk |

## Scopes

| Scope | Description |
|-------|-------------|
| `services:read` | Read access to services |
| `services:write` | Write access to services |
| `deploys:read` | Read access to deployments |
| `deploys:write` | Trigger deployments |
| `disks:read` | Read access to disks |

## Service Types

- `web_service` - HTTP server with automatic TLS
- `background_worker` - Long-running processes
- `cron_job` - Scheduled tasks
- `private_service` - Internal services without public TLS

## Regions

- `oregon` - US West
- `virginia` - US East
- `frankfurt` - EU Central
- `singapore` - Asia Pacific

## Example Usage

### List all services

```json
{
  "limit": 100,
  "cursor": null
}
```

### Create a web service

```json
{
  "name": "my-web-service",
  "type": "web_service",
  "region": "oregon",
  "plan": "starter",
  "repo": "https://github.com/myuser/myrepo",
  "branch": "main",
  "buildCommand": "npm run build",
  "startCommand": "npm start"
}
```

### Trigger a deployment

```json
{
  "serviceId": "srv-xxxxx",
  "clearCache": false,
  "sync": true
}
```

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad request - invalid parameters |
| 401 | Unauthorized - invalid API key |
| 403 | Forbidden - insufficient permissions |
| 404 | Not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

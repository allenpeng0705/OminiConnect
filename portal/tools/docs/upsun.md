# Upsun

Upsun is a cloud platform built on Platform.sh that provides infrastructure for applications, databases, and services with Git-push deployments.

## Authentication

Upsun uses API tokens for authentication. Generate one from the Upsun dashboard under My Profile > API Tokens.

## Rate Limits

- Default rate limit: 100 requests per minute
- List endpoints: 500 requests per minute
- Write operations: 50 requests per minute

## Base URL

```
https://api.upsun.com
```

## Tools

### Projects

| Tool | Description |
|------|-------------|
| `upsun_list_projects` | List all projects |
| `upsun_get_project` | Get details for a specific project |
| `upsun_create_project` | Create a new project |

### Applications

| Tool | Description |
|------|-------------|
| `upsun_list_apps` | List all applications in a project |
| `upsun_get_app` | Get details for a specific application |

### Services

| Tool | Description |
|------|-------------|
| `upsun_list_services` | List all services in a project |
| `upsun_get_service` | Get details for a specific service |

### Deployments

| Tool | Description |
|------|-------------|
| `upsun_list_deployments` | List all deployments for an environment |
| `upsun_get_deployment` | Get details for a specific deployment |

### Domains

| Tool | Description |
|------|-------------|
| `upsun_list_domains` | List all domains for a project |

## Scopes

| Scope | Description |
|-------|-------------|
| `projects:read` | Read access to projects |
| `projects:write` | Create and update projects |
| `apps:read` | Read access to applications |
| `services:read` | Read access to services |
| `deployments:read` | Read access to deployments |
| `domains:read` | Read access to domains |

## Regions

| Region | Location |
|--------|----------|
| `us-2.platform.sh` | US East (Virginia) |
| `us-3.platform.sh` | US West (Oregon) |
| `us-4.platform.sh` | US Central (Texas) |
| `eu-2.platform.sh` | Europe West (Ireland) |
| `eu-3.platform.sh` | Europe Central (Frankfurt) |
| `eu-4.platform.sh` | Europe North (Sweden) |
| `au-1.platform.sh` | Australia (Sydney) |

## Plan Tiers

| Plan | Description |
|------|-------------|
| `development` | For development and testing |
| `standard` | For small production workloads |
| `performance` | For high-traffic production apps |

## Service Types

Upsun supports managed services including:
- Databases: MySQL, PostgreSQL, MariaDB, Oracle
- Cache: Redis, Memcached
- Search: Elasticsearch, OpenSearch
- Storage: S3-compatible object storage
- Message queues: RabbitMQ

## Example Usage

### Create a project

```json
{
  "region": "us-2.platform.sh",
  "title": "My Project",
  "repository": "https://github.com/myuser/myrepo",
  "plan": "development"
}
```

### List apps

```json
{
  "projectId": "xxxxx"
}
```

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad request - invalid parameters |
| 401 | Unauthorized - invalid API token |
| 403 | Forbidden - insufficient permissions |
| 404 | Not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

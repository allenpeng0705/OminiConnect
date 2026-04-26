# Fly.io

Fly.io is an edge computing platform that lets you run applications close to users worldwide using Firecracker VMs called Machines.

## Authentication

Fly.io uses API tokens for authentication. Generate one with `fly auth token` or from the Fly dashboard under Account > API Tokens.

## Rate Limits

- Default rate limit: 100 requests per minute
- List operations: 500 requests per minute
- Write operations: 50 requests per minute

## Base URL

```
https://api.fly.io/v1
```

## Tools

### Applications

| Tool | Description |
|------|-------------|
| `fly_list_apps` | List all applications |
| `fly_get_app` | Get details for a specific application |
| `fly_create_app` | Create a new application |

### Machines

| Tool | Description |
|------|-------------|
| `fly_list_machines` | List all Machines for an app |
| `fly_get_machine` | Get details for a specific Machine |

### Volumes

| Tool | Description |
|------|-------------|
| `fly_list_volumes` | List all volumes |
| `fly_get_volume` | Get details for a specific volume |

### Certificates

| Tool | Description |
|------|-------------|
| `fly_list_certificates` | List all certificates for an app |
| `fly_get_certificate` | Get details for a specific certificate |

### Secrets

| Tool | Description |
|------|-------------|
| `fly_list_secrets` | List all secrets for an app |

## Scopes

| Scope | Description |
|-------|-------------|
| `apps:read` | Read access to applications |
| `apps:write` | Create and update applications |
| `machines:read` | Read access to Machines |
| `volumes:read` | Read access to volumes |
| `certificates:read` | Read access to certificates |
| `secrets:read` | Read access to secrets |

## Regions

Fly.io has regions worldwide. Common ones include:
- `iad` - Washington, DC (US East)
- `ord` - Chicago (US Central)
- `lax` - Los Angeles (US West)
- `fra` - Frankfurt (EU Central)
- `lhr` - London (EU West)
- `nrt` - Tokyo (Asia Pacific)
- `sin` - Singapore (Asia Pacific)

## Machine Configuration

Machines are the core compute unit on Fly.io. Each machine runs a single process with its own filesystem.

## Volume Attributes

| Attribute | Description |
|-----------|-------------|
| `name` | Volume name |
| `sizeGb` | Size in GB |
| `region` | Primary region |
| `encrypted` | Whether volume is encrypted |

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad request - invalid parameters |
| 401 | Unauthorized - invalid API token |
| 403 | Forbidden - insufficient permissions |
| 404 | Not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

## CLI Reference

Common commands:
- `fly apps list` - List apps
- `fly machines list -a <app>` - List machines
- `fly volumes list` - List volumes
- `fly secrets list -a <app>` - List secrets

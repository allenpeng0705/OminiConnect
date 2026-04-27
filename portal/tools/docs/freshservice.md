# Freshservice Integration

Freshservice is an IT service management (ITSM) platform that helps organizations manage tickets, assets, agents, problems, and changes.

## Authentication

Freshservice uses API key authentication. The API key can be found in your Freshservice admin panel under Settings > API Keys.

## Base URL

```
https://your-domain.freshservice.com/api/v2
```

## Rate Limits

- Default rate limit: 500 requests per minute
- Rate limit headers are included in each response

## Available Tools

### Tickets

| Tool | Description |
|------|-------------|
| `freshservice_list_tickets` | Retrieve a list of all tickets |
| `freshservice_get_ticket` | Get details of a specific ticket |
| `freshservice_create_ticket` | Create a new ticket |

### Agents

| Tool | Description |
|------|-------------|
| `freshservice_list_agents` | Retrieve a list of all agents |
| `freshservice_get_agent` | Get details of a specific agent |

### Assets

| Tool | Description |
|------|-------------|
| `freshservice_list_assets` | Retrieve a list of all assets |
| `freshservice_get_asset` | Get details of a specific asset |

### Problems

| Tool | Description |
|------|-------------|
| `freshservice_list_problems` | Retrieve a list of all problems |
| `freshservice_get_problem` | Get details of a specific problem |

### Changes

| Tool | Description |
|------|-------------|
| `freshservice_list_changes` | Retrieve a list of all changes |
| `freshservice_get_change` | Get details of a specific change |

## Ticket Status Values

| Value | Status |
|-------|--------|
| 2 | Open |
| 3 | Pending |
| 4 | Resolved |
| 5 | Closed |

## Ticket Priority Values

| Value | Priority |
|-------|----------|
| 1 | Low |
| 2 | Medium |
| 3 | High |
| 4 | Urgent |

## Common Use Cases

- **Ticket Management**: Create and track support tickets
- **Asset Tracking**: Monitor IT infrastructure assets
- **Agent Management**: View and manage support team members
- **Problem Management**: Track underlying issues causing incidents
- **Change Management**: Manage change requests in the IT lifecycle

## Notes

- All timestamps are in ISO 8601 format
- Pagination returns a maximum of 100 items per page
- Some endpoints require specific user permissions

# Freshdesk Integration

## Overview

Freshdesk is a cloud-based customer support platform that offers help desk ticketing, knowledge base, and customer self-service features. This integration enables LLMs and AI agents to manage tickets, contacts, agents, and forums.

## Authentication

Freshdesk uses API key authentication. The API key is combined with a password for Basic Auth.

- **Auth Type**: Basic Auth (API Key as username)
- **Base URL**: `https://{subdomain}.freshdesk.com/api/v2`

## Rate Limits

- Default: 100 requests per minute (varies by plan)
- Rate limit information included in response headers

## Tool Categories

### Tickets

| Tool | Description |
|------|-------------|
| `freshdesk_list_tickets` | Retrieve a list of support tickets |
| `freshdesk_get_ticket` | Get a single ticket by ID |
| `freshdesk_create_ticket` | Create a new support ticket |
| `freshdesk_update_ticket` | Update an existing ticket |

### Contacts

| Tool | Description |
|------|-------------|
| `freshdesk_list_contacts` | List all contacts |
| `freshdesk_get_contact` | Get a contact by ID |

### Agents

| Tool | Description |
|------|-------------|
| `freshdesk_list_agents` | List all agents |
| `freshdesk_get_agent` | Get an agent by ID |

### Forums

| Tool | Description |
|------|-------------|
| `freshdesk_list_forums` | List all forums |
| `freshdesk_get_forum` | Get a forum by ID |

## Usage Examples

### List Open Tickets

```json
{
  "tool": "freshdesk_list_tickets",
  "parameters": {
    "status": 2,
    "per_page": 10
  }
}
```

### Create a Ticket

```json
{
  "tool": "freshdesk_create_ticket",
  "parameters": {
    "subject": "Password reset needed",
    "description": "I forgot my password and need help resetting it.",
    "email": "customer@example.com",
    "priority": 2,
    "status": 2
  }
}
```

### Get Contact

```json
{
  "tool": "freshdesk_get_contact",
  "parameters": {
    "id": 12345
  }
}
```

## Ticket Status Reference

| Status ID | Name |
|-----------|------|
| 2 | Open |
| 3 | Pending |
| 4 | Resolved |
| 5 | Closed |

## Priority Reference

| Priority ID | Name |
|-------------|------|
| 1 | Low |
| 2 | Medium |
| 3 | High |
| 4 | Urgent |

## Data Schema

### Ticket Object

```json
{
  "id": 123,
  "subject": "Ticket subject",
  "description": "Ticket description",
  "status": 2,
  "priority": 2,
  "email": "customer@example.com",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T11:45:00Z"
}
```

### Contact Object

```json
{
  "id": 12345,
  "name": "John Doe",
  "email": "customer@example.com",
  "phone": "+1234567890",
  "state": "active",
  "created_at": "2023-06-20T08:00:00Z"
}
```

## Error Handling

| Error Code | Description |
|------------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid API key |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |

# Zendesk Integration

## Overview

Zendesk is a customer service platform providing ticketing, help center, and call center solutions. This integration enables LLMs and AI agents to manage tickets, users, views, and macros.

## Authentication

Zendesk supports API token authentication combined with email and password.

- **Auth Type**: Basic Auth with API Token
- **Base URL**: `https://{subdomain}.zendesk.com/api/v2`

## Rate Limits

- Default: 700 requests per minute
- Rate limit headers included in responses

## Tool Categories

### Tickets

| Tool | Description |
|------|-------------|
| `zendesk_list_tickets` | Retrieve a paginated list of tickets |
| `zendesk_get_ticket` | Get a single ticket by ID |
| `zendesk_create_ticket` | Create a new ticket |
| `zendesk_update_ticket` | Update an existing ticket |

### Users

| Tool | Description |
|------|-------------|
| `zendesk_list_users` | List all users |
| `zendesk_get_user` | Get a user by ID |

### Views

| Tool | Description |
|------|-------------|
| `zendesk_list_views` | List all views |
| `zendesk_get_view` | Get a view by ID |

### Macros

| Tool | Description |
|------|-------------|
| `zendesk_list_macros` | List all macros |
| `zendesk_get_macro` | Get a macro by ID |

## Usage Examples

### List Open Tickets

```json
{
  "tool": "zendesk_list_tickets",
  "parameters": {
    "status": "open",
    "per_page": 20,
    "sort_by": "created_at",
    "sort_order": "desc"
  }
}
```

### Create a Ticket

```json
{
  "tool": "zendesk_create_ticket",
  "parameters": {
    "subject": "Billing inquiry",
    "comment": {
      "body": "I was charged twice for my subscription this month."
    },
    "priority": "high",
    "tags": ["billing", "subscription"]
  }
}
```

### Get User Details

```json
{
  "tool": "zendesk_get_user",
  "parameters": {
    "id": 12345
  }
}
```

## Ticket Status Reference

| Status | Description |
|--------|-------------|
| new | New ticket |
| open | Ticket is open |
| pending | Awaiting customer response |
| hold | On hold |
| solved | Resolved |
| closed | Closed |

## Priority Reference

| Priority | Description |
|----------|-------------|
| low | Low priority |
| normal | Normal priority |
| high | High priority |
| urgent | Urgent priority |

## Data Schema

### Ticket Object

```json
{
  "id": 123,
  "subject": "Ticket subject",
  "description": "Ticket description",
  "status": "open",
  "priority": "high",
  "requester_id": 67890,
  "tags": ["billing", "subscription"],
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T11:45:00Z"
}
```

### User Object

```json
{
  "id": 12345,
  "name": "John Doe",
  "email": "user@example.com",
  "role": "end-user",
  "created_at": "2023-06-20T08:00:00Z"
}
```

## Error Handling

| Error Code | Description |
|------------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid credentials |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 422 | Unprocessable Entity - Validation failed |
| 429 | Too Many Requests - Rate limit exceeded |

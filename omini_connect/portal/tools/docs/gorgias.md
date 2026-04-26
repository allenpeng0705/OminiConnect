# Gorgias Integration

## Overview

Gorgias is a help desk platform that centralizes customer support across multiple channels including email, chat, and social media. This integration enables LLMs and AI agents to manage support tickets, customers, macros, and tags.

## Authentication

Gorgias uses API key authentication. The API key is configured during the Nango connection setup.

- **Auth Type**: API Key
- **Base URL**: `https://{subdomain}.gorgias.com/api`

## Rate Limits

- Default: 180 requests per minute
- Rate limit headers are included in responses

## Tool Categories

### Tickets

| Tool | Description |
|------|-------------|
| `gorgias_list_tickets` | Retrieve a paginated list of support tickets |
| `gorgias_get_ticket` | Get a single ticket by ID |
| `gorgias_create_ticket` | Create a new support ticket |
| `gorgias_update_ticket` | Update an existing ticket |

### Customers

| Tool | Description |
|------|-------------|
| `gorgias_list_customers` | List all customers |
| `gorgias_get_customer` | Get a customer by ID |

### Macros

| Tool | Description |
|------|-------------|
| `gorgias_list_macros` | List all canned response macros |
| `gorgias_get_macro` | Get a macro by ID |

### Tags

| Tool | Description |
|------|-------------|
| `gorgias_list_tags` | List all tags |
| `gorgias_get_tag` | Get a tag by ID |

## Usage Examples

### List Open Tickets

```json
{
  "tool": "gorgias_list_tickets",
  "parameters": {
    "state": "open",
    "per_page": 10
  }
}
```

### Create a Ticket

```json
{
  "tool": "gorgias_create_ticket",
  "parameters": {
    "subject": "Cannot access my account",
    "body": "I've been locked out of my account since yesterday.",
    "customer_email": "customer@example.com",
    "priority": "high"
  }
}
```

### Get Customer Details

```json
{
  "tool": "gorgias_get_customer",
  "parameters": {
    "id": 12345
  }
}
```

## Data Schema

### Ticket Object

```json
{
  "id": 123,
  "subject": "Ticket subject",
  "body": "Ticket description",
  "state": "open",
  "priority": "high",
  "customer_email": "customer@example.com",
  "created_datetime": "2024-01-15T10:30:00Z",
  "updated_datetime": "2024-01-15T11:45:00Z"
}
```

### Customer Object

```json
{
  "id": 12345,
  "email": "customer@example.com",
  "name": "John Doe",
  "created_datetime": "2023-06-20T08:00:00Z"
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

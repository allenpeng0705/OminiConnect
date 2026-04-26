# HubSpot Integration

## Overview

HubSpot is a CRM platform offering marketing, sales, and customer service tools. This integration enables LLMs and AI agents to manage contacts, companies, deals, tickets, and conversations.

## Authentication

HubSpot uses OAuth 2.0 or Private App tokens for authentication.

- **Auth Type**: OAuth 2.0 / Private App Token
- **Base URL**: `https://api.hubapi.com`

## Rate Limits

- Default: 100 requests per 10 seconds (1000 per day for some endpoints)
- Rate limit headers included in responses
- Burst limits apply for certain operations

## Tool Categories

### Contacts

| Tool | Description |
|------|-------------|
| `hubspot_list_contacts` | Retrieve a paginated list of contacts |
| `hubspot_get_contact` | Get a single contact by ID |
| `hubspot_create_contact` | Create a new contact |

### Companies

| Tool | Description |
|------|-------------|
| `hubspot_list_companies` | List all companies |
| `hubspot_get_company` | Get a company by ID |

### Deals

| Tool | Description |
|------|-------------|
| `hubspot_list_deals` | List all deals |
| `hubspot_get_deal` | Get a deal by ID |

### Tickets

| Tool | Description |
|------|-------------|
| `hubspot_list_tickets` | List all support tickets |
| `hubspot_get_ticket` | Get a ticket by ID |

### Conversations

| Tool | Description |
|------|-------------|
| `hubspot_list_conversations` | List all conversations |

## Usage Examples

### List Contacts

```json
{
  "tool": "hubspot_list_contacts",
  "parameters": {
    "limit": 10,
    "properties": ["firstname", "lastname", "email"]
  }
}
```

### Create a Contact

```json
{
  "tool": "hubspot_create_contact",
  "parameters": {
    "properties": {
      "email": "newcontact@example.com",
      "firstname": "John",
      "lastname": "Doe"
    }
  }
}
```

### Get Deal Details

```json
{
  "tool": "hubspot_get_deal",
  "parameters": {
    "id": "deal12345",
    "properties": ["dealname", "amount", "dealstage"]
  }
}
```

### List Open Tickets

```json
{
  "tool": "hubspot_list_tickets",
  "parameters": {
    "limit": 20,
    "pipeline": "default"
  }
}
```

## Common Properties

### Contact Properties

| Property | Type | Description |
|----------|------|-------------|
| email | string | Contact email |
| firstname | string | First name |
| lastname | string | Last name |
| phone | string | Phone number |
| company | string | Company name |

### Deal Properties

| Property | Type | Description |
|----------|------|-------------|
| dealname | string | Deal name |
| amount | number | Deal value |
| dealstage | string | Current pipeline stage |
| closedate | date | Expected close date |

### Ticket Properties

| Property | Type | Description |
|----------|------|-------------|
| subject | string | Ticket subject |
| content | string | Ticket content |
| hs_ticket_priority | string | Priority (HIGH, MEDIUM, LOW) |
| hs_pipeline | string | Pipeline name |

## Data Schema

### Contact Object

```json
{
  "id": "12345",
  "properties": {
    "email": "contact@example.com",
    "firstname": "John",
    "lastname": "Doe",
    "createdate": "2023-06-20T08:00:00Z"
  }
}
```

### Deal Object

```json
{
  "id": "deal12345",
  "properties": {
    "dealname": "Enterprise License",
    "amount": "50000",
    "dealstage": "closedwon",
    "closedate": "2024-03-15"
  }
}
```

## Error Handling

| Error Code | Description |
|------------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid token |
| 403 | Forbidden - Insufficient scopes |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |

## Scopes Required

| Scope | Description |
|-------|-------------|
| crm.objects.contacts.read | Read contacts |
| crm.objects.contacts.write | Write contacts |
| crm.objects.companies.read | Read companies |
| crm.objects.deals.read | Read deals |
| crm.objects.tickets.read | Read tickets |
| conversations | Read conversations |

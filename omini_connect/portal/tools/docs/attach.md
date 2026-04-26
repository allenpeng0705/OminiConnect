# Attach Integration

Attach is a sales engagement platform that helps teams manage deals, contacts, activities, and email tracking.

## Authentication

Attach uses OAuth 2.0 for authentication. Configure the following scopes in your Attach OAuth application:

- `deals:read` - Read access to deals
- `deals:write` - Write access to deals
- `contacts:read` - Read access to contacts
- `contacts:write` - Write access to contacts
- `activities:read` - Read access to activities
- `emails:read` - Read access to email tracking
- `emails:write` - Write access to emails

## Base URL

```
https://api.attach.io/v1
```

## Rate Limits

- Standard rate limit: 500 requests per minute
- Write operations: 100 requests per minute

## Tools

### Deals

| Tool | Description |
|------|-------------|
| `attach_list_deals` | Retrieve a list of all deals in the sales pipeline |
| `attach_get_deal` | Retrieve details of a specific deal by ID |
| `attach_create_deal` | Create a new deal in the sales pipeline |
| `attach_add_contact_to_deal` | Associate a contact with a deal |

### Contacts

| Tool | Description |
|------|-------------|
| `attach_list_contacts` | Retrieve a list of all contacts |
| `attach_get_contact` | Retrieve details of a specific contact by ID |

### Activities

| Tool | Description |
|------|-------------|
| `attach_list_activities` | Retrieve a list of all sales activities |
| `attach_get_activity` | Retrieve details of a specific activity by ID |

### Email

| Tool | Description |
|------|-------------|
| `attach_send_email` | Send an email through the sales engagement platform |
| `attach_track_email_opened` | Track when an email was opened by the recipient |

## Tool Details

### attach_list_deals

Retrieve a list of all deals in the sales pipeline.

**Endpoint:** `GET /deals`

**Scopes:** `deals:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter deals by status (e.g., open, won, lost) |
| `owner_id` | string | No | Filter deals by owner ID |

### attach_get_deal

Retrieve details of a specific deal by ID.

**Endpoint:** `GET /deals/{deal_id}`

**Scopes:** `deals:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `deal_id` | string | Yes | The unique identifier of the deal |

### attach_create_deal

Create a new deal in the sales pipeline.

**Endpoint:** `POST /deals`

**Scopes:** `deals:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the deal |
| `value` | number | No | The monetary value of the deal |
| `stage` | string | No | The sales stage of the deal |
| `owner_id` | string | No | The ID of the deal owner |

### attach_list_contacts

Retrieve a list of all contacts.

**Endpoint:** `GET /contacts`

**Scopes:** `contacts:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `company_id` | string | No | Filter contacts by company ID |
| `tags` | string | No | Filter contacts by tags |

### attach_get_contact

Retrieve details of a specific contact by ID.

**Endpoint:** `GET /contacts/{contact_id}`

**Scopes:** `contacts:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contact_id` | string | Yes | The unique identifier of the contact |

### attach_add_contact_to_deal

Associate a contact with a deal.

**Endpoint:** `POST /deals/{deal_id}/contacts`

**Scopes:** `deals:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `deal_id` | string | Yes | The unique identifier of the deal |
| `contact_id` | string | Yes | The unique identifier of the contact |

### attach_list_activities

Retrieve a list of all sales activities.

**Endpoint:** `GET /activities`

**Scopes:** `activities:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `type` | string | No | Filter by activity type (e.g., call, email, meeting) |
| `deal_id` | string | No | Filter activities by deal ID |

### attach_get_activity

Retrieve details of a specific activity by ID.

**Endpoint:** `GET /activities/{activity_id}`

**Scopes:** `activities:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `activity_id` | string | Yes | The unique identifier of the activity |

### attach_send_email

Send an email through the sales engagement platform.

**Endpoint:** `POST /emails`

**Scopes:** `emails:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `to` | string | Yes | The recipient email address |
| `subject` | string | Yes | The email subject |
| `body` | string | Yes | The email body content |
| `deal_id` | string | No | Associate email with a deal ID |

### attach_track_email_opened

Track when an email was opened by the recipient.

**Endpoint:** `POST /emails/{email_id}/opened`

**Scopes:** `emails:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `email_id` | string | Yes | The unique identifier of the sent email |
| `opened_at` | string | No | Timestamp when the email was opened |

# Engage Customer Engagement Platform

Engage is a customer engagement platform providing customer management, conversation tracking, campaign management, and email marketing.

## Authentication

Engage uses OAuth 2.0 authentication. Obtain an access token and include it in the `Authorization` header as `Bearer {access_token}`.

## Base URL

```
https://api.engage.com/v2
```

## Rate Limits

- Default: 200 requests per minute
- Burst: 25 requests per second

## Available Tools

### Customers

| Tool | Description |
|------|-------------|
| `engage_list_customers` | Retrieve a paginated list of all customers |
| `engage_get_customer` | Get detailed customer information by ID |
| `engage_create_customer` | Create a new customer with name and email |

### Conversations

| Tool | Description |
|------|-------------|
| `engage_list_conversations` | List all customer conversations |
| `engage_get_conversation` | Get detailed conversation information by ID |
| `engage_create_conversation` | Create a new conversation for a customer |

### Campaigns

| Tool | Description |
|------|-------------|
| `engage_list_campaigns` | List all customer engagement campaigns |
| `engage_get_campaign` | Get detailed campaign information by ID |

### Emails

| Tool | Description |
|------|-------------|
| `engage_send_email` | Send an email to a customer using a template |
| `engage_get_email_status` | Get the delivery status of a sent email |

## Tool Details

### engage_list_customers

**Endpoint:** `GET /customers`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `limit` (integer, optional): Number of records per page

---

### engage_get_customer

**Endpoint:** `GET /customers/{id}`

**Parameters:**
- `id` (string, required): Customer ID

---

### engage_create_customer

**Endpoint:** `POST /customers`

**Parameters:**
- `name` (string, required): Name of the customer
- `email` (string, required): Email address
- `phone` (string, optional): Phone number

---

### engage_list_conversations

**Endpoint:** `GET /conversations`

**Parameters:**
- `customer_id` (string, optional): Filter by customer ID
- `status` (string, optional): Filter by status (open, closed)
- `page` (integer, optional): Page number for pagination

---

### engage_get_conversation

**Endpoint:** `GET /conversations/{id}`

**Parameters:**
- `id` (string, required): Conversation ID

---

### engage_create_conversation

**Endpoint:** `POST /conversations`

**Parameters:**
- `customer_id` (string, required): ID of the customer
- `subject` (string, required): Subject of the conversation
- `message` (string, required): Initial message content

---

### engage_list_campaigns

**Endpoint:** `GET /campaigns`

**Parameters:**
- `status` (string, optional): Filter by status (active, paused, completed)
- `page` (integer, optional): Page number for pagination

---

### engage_get_campaign

**Endpoint:** `GET /campaigns/{id}`

**Parameters:**
- `id` (string, required): Campaign ID

---

### engage_send_email

**Endpoint:** `POST /emails/send`

**Parameters:**
- `customer_id` (string, required): ID of the customer to send to
- `template_id` (string, required): ID of the email template
- `subject` (string, optional): Email subject line
- `body` (string, optional): Email body content

---

### engage_get_email_status

**Endpoint:** `GET /emails/{id}/status`

**Parameters:**
- `id` (string, required): Email message ID

---

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or expired access token |
| 404 | Not Found - Resource does not exist |
| 422 | Unprocessable Entity - Validation error |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

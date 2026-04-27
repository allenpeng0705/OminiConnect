# Charcle Integration

Charcle is a customer engagement platform that helps teams manage customers, segments, campaigns, and messages.

## Authentication

Charcle uses OAuth 2.0 for authentication. Configure the following scopes in your Charcle OAuth application:

- `customers:read` - Read access to customers
- `customers:write` - Write access to customers
- `segments:read` - Read access to segments
- `segments:write` - Write access to segments
- `campaigns:read` - Read access to campaigns
- `messages:read` - Read access to messages
- `messages:write` - Write access to messages

## Base URL

```
https://api.charcle.com/v1
```

## Rate Limits

- Standard rate limit: 500 requests per minute
- Write operations: 100 requests per minute

## Tools

### Customers

| Tool | Description |
|------|-------------|
| `charcle_list_customers` | Retrieve a list of all customers in the engagement platform |
| `charcle_get_customer` | Retrieve details of a specific customer by ID |
| `charcle_create_customer` | Create a new customer in the engagement platform |

### Segments

| Tool | Description |
|------|-------------|
| `charcle_list_segments` | Retrieve a list of all customer segments |
| `charcle_get_segment` | Retrieve details of a specific segment by ID |
| `charcle_create_segment` | Create a new customer segment |

### Campaigns

| Tool | Description |
|------|-------------|
| `charcle_list_campaigns` | Retrieve a list of all marketing campaigns |
| `charcle_get_campaign` | Retrieve details of a specific campaign by ID |

### Messages

| Tool | Description |
|------|-------------|
| `charcle_send_message` | Send a message to customers through a campaign |
| `charcle_get_message_status` | Retrieve the delivery status of a sent message |

## Tool Details

### charcle_list_customers

Retrieve a list of all customers in the engagement platform.

**Endpoint:** `GET /customers`

**Scopes:** `customers:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `segment_id` | string | No | Filter customers by segment ID |
| `status` | string | No | Filter customers by status (e.g., active, inactive) |

### charcle_get_customer

Retrieve details of a specific customer by ID.

**Endpoint:** `GET /customers/{customer_id}`

**Scopes:** `customers:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `customer_id` | string | Yes | The unique identifier of the customer |

### charcle_create_customer

Create a new customer in the engagement platform.

**Endpoint:** `POST /customers`

**Scopes:** `customers:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the customer |
| `email` | string | Yes | The email address of the customer |
| `phone` | string | No | The phone number of the customer |
| `segment_id` | string | No | The segment ID to assign the customer to |

### charcle_list_segments

Retrieve a list of all customer segments.

**Endpoint:** `GET /segments`

**Scopes:** `segments:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter segments by status |

### charcle_get_segment

Retrieve details of a specific segment by ID.

**Endpoint:** `GET /segments/{segment_id}`

**Scopes:** `segments:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `segment_id` | string | Yes | The unique identifier of the segment |

### charcle_create_segment

Create a new customer segment.

**Endpoint:** `POST /segments`

**Scopes:** `segments:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | The name of the segment |
| `description` | string | No | The description of the segment |
| `criteria` | object | No | The segment criteria/filter rules |

### charcle_list_campaigns

Retrieve a list of all marketing campaigns.

**Endpoint:** `GET /campaigns`

**Scopes:** `campaigns:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `status` | string | No | Filter campaigns by status |
| `type` | string | No | Filter campaigns by type (e.g., email, sms) |

### charcle_get_campaign

Retrieve details of a specific campaign by ID.

**Endpoint:** `GET /campaigns/{campaign_id}`

**Scopes:** `campaigns:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `campaign_id` | string | Yes | The unique identifier of the campaign |

### charcle_send_message

Send a message to customers through a campaign.

**Endpoint:** `POST /messages`

**Scopes:** `messages:write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `campaign_id` | string | Yes | The campaign ID to send the message through |
| `customer_ids` | array | Yes | Array of customer IDs to send the message to |
| `content` | string | Yes | The message content |
| `channel` | string | No | The channel to send through (e.g., email, sms) |

### charcle_get_message_status

Retrieve the delivery status of a sent message.

**Endpoint:** `GET /messages/{message_id}`

**Scopes:** `messages:read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `message_id` | string | Yes | The unique identifier of the message |

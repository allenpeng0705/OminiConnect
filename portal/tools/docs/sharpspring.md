# SharpSpring Marketing Automation Integration

SharpSpring is a marketing automation platform providing campaign management, contact management, company tracking, and opportunity management.

## Authentication

SharpSpring uses API key authentication. Provide your API key and secret in the request headers:
- `X-API-Key: {api_key}`
- `X-API-Secret: {api_secret}`

## Base URL

```
https://api.sharpspring.com/public/v1
```

## Rate Limits

- Default: 500 requests per hour
- Burst: 10 requests per second

## Available Tools

### Campaigns

| Tool | Description |
|------|-------------|
| `sharpspring_list_campaigns` | Retrieve a list of all marketing campaigns |
| `sharpspring_get_campaign` | Get detailed campaign information by ID |
| `sharpspring_create_campaign` | Create a new marketing campaign |

### Contacts

| Tool | Description |
|------|-------------|
| `sharpspring_list_contacts` | Retrieve a paginated list of all contacts |
| `sharpspring_get_contact` | Get detailed contact information by ID |
| `sharpspring_create_contact` | Create a new contact with name and email |

### Companies

| Tool | Description |
|------|-------------|
| `sharpspring_list_companies` | Retrieve a list of all companies |
| `sharpspring_get_company` | Get detailed company information by ID |

### Opportunities

| Tool | Description |
|------|-------------|
| `sharpspring_list_opportunities` | List all sales opportunities |
| `sharpspring_get_opportunity` | Get detailed opportunity information by ID |

## Tool Details

### sharpspring_list_campaigns

**Endpoint:** `GET /campaigns`

**Parameters:**
- `status` (string, optional): Filter by status (active, paused, completed)
- `page` (integer, optional): Page number for pagination

---

### sharpspring_get_campaign

**Endpoint:** `GET /campaigns/{id}`

**Parameters:**
- `id` (string, required): Campaign ID

---

### sharpspring_create_campaign

**Endpoint:** `POST /campaigns`

**Parameters:**
- `name` (string, required): Name of the campaign
- `description` (string, optional): Description of the campaign
- `status` (string, optional): Initial status of the campaign

---

### sharpspring_list_contacts

**Endpoint:** `GET /contacts`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `limit` (integer, optional): Number of records per page

---

### sharpspring_get_contact

**Endpoint:** `GET /contacts/{id}`

**Parameters:**
- `id` (string, required): Contact ID

---

### sharpspring_create_contact

**Endpoint:** `POST /contacts`

**Parameters:**
- `first_name` (string, required): First name of the contact
- `last_name` (string, required): Last name of the contact
- `email` (string, required): Email address

---

### sharpspring_list_companies

**Endpoint:** `GET /companies`

**Parameters:**
- `page` (integer, optional): Page number for pagination

---

### sharpspring_get_company

**Endpoint:** `GET /companies/{id}`

**Parameters:**
- `id` (string, required): Company ID

---

### sharpspring_list_opportunities

**Endpoint:** `GET /opportunities`

**Parameters:**
- `stage` (string, optional): Filter by pipeline stage
- `page` (integer, optional): Page number for pagination

---

### sharpspring_get_opportunity

**Endpoint:** `GET /opportunities/{id}`

**Parameters:**
- `id` (string, required): Opportunity ID

---

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid API credentials |
| 404 | Not Found - Resource does not exist |
| 422 | Unprocessable Entity - Validation error |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

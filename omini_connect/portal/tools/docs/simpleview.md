# SimpleView CRM Integration

SimpleView is a CRM platform providing contact management, company tracking, activity logging, opportunity management, and support ticket handling.

## Authentication

SimpleView uses API key authentication. Provide your API key in the `Authorization` header as `Bearer {api_key}`.

## Base URL

```
https://api.simpleview.com/v1
```

## Rate Limits

- Default: 100 requests per minute
- Burst: 20 requests per second

## Available Tools

### Contacts

| Tool | Description |
|------|-------------|
| `simpleview_list_contacts` | Retrieve a paginated list of all contacts |
| `simpleview_get_contact` | Get detailed contact information by ID |
| `simpleview_create_contact` | Create a new contact with name, email, and phone |

### Companies

| Tool | Description |
|------|-------------|
| `simpleview_list_companies` | Retrieve a paginated list of all companies |
| `simpleview_get_company` | Get detailed company information by ID |

### Activities

| Tool | Description |
|------|-------------|
| `simpleview_list_activities` | List all activities (calls, meetings, notes) |
| `simpleview_get_activity` | Get detailed activity information by ID |

### Opportunities

| Tool | Description |
|------|-------------|
| `simpleview_list_opportunities` | List all sales opportunities in the pipeline |
| `simpleview_get_opportunity` | Get detailed opportunity information by ID |

### Tickets

| Tool | Description |
|------|-------------|
| `simpleview_list_tickets` | List all support tickets |

## Tool Details

### simpleview_list_contacts

**Endpoint:** `GET /contacts`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `limit` (integer, optional): Number of records per page

---

### simpleview_get_contact

**Endpoint:** `GET /contacts/{id}`

**Parameters:**
- `id` (string, required): Contact ID

---

### simpleview_create_contact

**Endpoint:** `POST /contacts`

**Parameters:**
- `first_name` (string, required): First name of the contact
- `last_name` (string, required): Last name of the contact
- `email` (string, required): Email address
- `phone` (string, optional): Phone number

---

### simpleview_list_companies

**Endpoint:** `GET /companies`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `limit` (integer, optional): Number of records per page

---

### simpleview_get_company

**Endpoint:** `GET /companies/{id}`

**Parameters:**
- `id` (string, required): Company ID

---

### simpleview_list_activities

**Endpoint:** `GET /activities`

**Parameters:**
- `contact_id` (string, optional): Filter by contact ID
- `type` (string, optional): Filter by activity type (call, meeting, note)
- `page` (integer, optional): Page number for pagination

---

### simpleview_get_activity

**Endpoint:** `GET /activities/{id}`

**Parameters:**
- `id` (string, required): Activity ID

---

### simpleview_list_opportunities

**Endpoint:** `GET /opportunities`

**Parameters:**
- `stage` (string, optional): Filter by pipeline stage
- `page` (integer, optional): Page number for pagination

---

### simpleview_get_opportunity

**Endpoint:** `GET /opportunities/{id}`

**Parameters:**
- `id` (string, required): Opportunity ID

---

### simpleview_list_tickets

**Endpoint:** `GET /tickets`

**Parameters:**
- `status` (string, optional): Filter by status (open, closed, pending)
- `page` (integer, optional): Page number for pagination

---

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing API key |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

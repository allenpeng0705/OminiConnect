# SAS Analytics Platform

SAS is an analytics platform providing project management, reporting, dashboards, data tables, and user administration.

## Authentication

SAS uses API key authentication. Provide your API key in the `Authorization` header as `Basic {base64(api_key:)}`.

## Base URL

```
https://api.sas.com/v1
```

## Rate Limits

- Default: 100 requests per minute
- Burst: 15 requests per second

## Available Tools

### Projects

| Tool | Description |
|------|-------------|
| `sas_list_projects` | Retrieve a paginated list of all projects |
| `sas_get_project` | Get detailed project information by ID |
| `sas_create_project` | Create a new analytics project |

### Reports

| Tool | Description |
|------|-------------|
| `sas_list_reports` | List all analytics reports |
| `sas_get_report` | Get detailed report information by ID |

### Dashboards

| Tool | Description |
|------|-------------|
| `sas_list_dashboards` | List all analytics dashboards |
| `sas_get_dashboard` | Get detailed dashboard information by ID |

### Tables

| Tool | Description |
|------|-------------|
| `sas_list_tables` | List all data tables in the platform |
| `sas_get_table` | Get detailed table information by ID |

### Users

| Tool | Description |
|------|-------------|
| `sas_list_users` | List all users in the analytics platform |

## Tool Details

### sas_list_projects

**Endpoint:** `GET /projects`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `limit` (integer, optional): Number of records per page

---

### sas_get_project

**Endpoint:** `GET /projects/{id}`

**Parameters:**
- `id` (string, required): Project ID

---

### sas_create_project

**Endpoint:** `POST /projects`

**Parameters:**
- `name` (string, required): Name of the project
- `description` (string, optional): Description of the project

---

### sas_list_reports

**Endpoint:** `GET /reports`

**Parameters:**
- `project_id` (string, optional): Filter by project ID
- `page` (integer, optional): Page number for pagination

---

### sas_get_report

**Endpoint:** `GET /reports/{id}`

**Parameters:**
- `id` (string, required): Report ID

---

### sas_list_dashboards

**Endpoint:** `GET /dashboards`

**Parameters:**
- `project_id` (string, optional): Filter by project ID
- `page` (integer, optional): Page number for pagination

---

### sas_get_dashboard

**Endpoint:** `GET /dashboards/{id}`

**Parameters:**
- `id` (string, required): Dashboard ID

---

### sas_list_tables

**Endpoint:** `GET /tables`

**Parameters:**
- `project_id` (string, optional): Filter by project ID
- `page` (integer, optional): Page number for pagination

---

### sas_get_table

**Endpoint:** `GET /tables/{id}`

**Parameters:**
- `id` (string, required): Table ID

---

### sas_list_users

**Endpoint:** `GET /users`

**Parameters:**
- `role` (string, optional): Filter by user role
- `page` (integer, optional): Page number for pagination

---

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid API key |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

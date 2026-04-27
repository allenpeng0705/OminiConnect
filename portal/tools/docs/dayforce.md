# Dayforce Integration

## Overview

Dayforce is a comprehensive HCM (Human Capital Management) platform that provides APIs for managing employees, time tracking, requests (time off, leave), and organizational structure. This integration enables AI agents to interact with the Dayforce HCM API.

## Authentication

Dayforce uses header-based authentication with tenant context. Configure the following:

- **Tenant Context ID**: Your Dayforce tenant context identifier
- **Country Code**: The country code for data formatting (e.g., US, CA, AU)
- **Username/Password**: User credentials for Dayforce authentication

## Available Tools

### Employees

| Tool | Description |
|------|-------------|
| `dayforce_list_employees` | List all employees with optional filtering |
| `dayforce_get_employee` | Get details of a specific employee by employee number |
| `dayforce_create_employee` | Create a new employee record |

### Time Entries

| Tool | Description |
|------|-------------|
| `dayforce_list_time_entries` | List time entries with optional filtering by date range |
| `dayforce_get_time_entry` | Get details of a specific time entry |

### Requests

| Tool | Description |
|------|-------------|
| `dayforce_list_requests` | List requests (time off, leave, etc.) with filtering |
| `dayforce_get_request` | Get details of a specific request |
| `dayforce_approve_request` | Approve a pending request |

### Organizational Units

| Tool | Description |
|------|-------------|
| `dayforce_list_org_units` | List organizational units in the hierarchy |
| `dayforce_get_org_unit` | Get details of a specific organizational unit |

## API Base URL

```
https://{dayforce_host}/dayforce/api/{xTenantContextId}
```

## Common Use Cases

### Listing Employees

```json
{
  "xTenantContextId": "ACME_CORP",
  "country": "US",
  "isActive": true,
  "department": "Engineering",
  "limit": 50,
  "skip": 0
}
```

### Getting an Employee

```json
{
  "xTenantContextId": "ACME_CORP",
  "employeeNumber": "EMP-12345",
  "country": "US"
}
```

### Creating an Employee

```json
{
  "xTenantContextId": "ACME_CORP",
  "employeeNumber": "EMP-67890",
  "firstName": "Jane",
  "lastName": "Smith",
  "workEmail": "jane.smith@acme.com",
  "hireDate": "2024-01-15",
  "department": "Engineering",
  "jobTitle": "Software Engineer"
}
```

### Listing Time Entries

```json
{
  "xTenantContextId": "ACME_CORP",
  "startDate": "2024-01-01",
  "endDate": "2024-01-31",
  "isApproved": false,
  "country": "US"
}
```

### Approving a Request

```json
{
  "xTenantContextId": "ACME_CORP",
  "requestId": "REQ-12345",
  "comments": "Approved as requested",
  "country": "US"
}
```

## Scopes

The following permissions/scopes are used by these tools:

- `employees:read` - Read employee data
- `employees:write` - Create and modify employee records
- `time:read` - Read time entry data
- `requests:read` - Read request data
- `requests:write` - Approve and manage requests
- `organization:read` - Read organizational structure

## Rate Limits

Dayforce implements rate limiting on API requests. The integration handles pagination automatically to stay within rate limits.

## Error Handling

Common error responses:

- `400 Bad Request` - Invalid request parameters or malformed JSON
- `401 Unauthorized` - Invalid credentials or session expired
- `403 Forbidden` - Insufficient permissions for the requested operation
- `404 Not Found` - The requested resource does not exist
- `409 Conflict` - Data conflict (e.g., duplicate employee number)
- `429 Too Many Requests` - Rate limit exceeded

## Important Headers

Dayforce API requires specific headers:

- `xTenantContextId` - Tenant context identifier (required for all endpoints)
- `Country` - Country code for data formatting (required for most endpoints)

## Resources

- [Dayforce HCM API Documentation](https://www.dayforce.com/en/developer)
- [Dayforce Employee API](https://www.dayforce.com/en/developer/employee-api)
- [Dayforce Time Tracking API](https://www.dayforce.com/en/developer/time-api)
- [Ceridian Dayforce](https://www.dayforce.com/)

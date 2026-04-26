# Workday Integration

Workday provides HR and financial management including worker management, job positions, organizational structure, time off tracking, and expense management.

## Authentication

Workday uses OAuth 2.0 and SAML for authentication. Configure the following:

- **Tenant ID**: Your Workday tenant identifier
- **Client ID**: Your Workday API client ID
- **Client Secret**: Your Workday API client secret
- **Base URL**: Your Workday API base URL (e.g., `https://wd3.workday.com`)

## API Version

Current API version: v1

## Available Tools

### Workers

| Tool | Name | Description |
|------|------|-------------|
| `workday_list_workers` | List Workers | Retrieve a list of all workers (employees) |
| `workday_get_worker` | Get Worker | Retrieve detailed information for a specific worker |

### Jobs

| Tool | Name | Description |
|------|------|-------------|
| `workday_list_jobs` | List Jobs | Retrieve a list of all job positions |
| `workday_get_job` | Get Job | Retrieve detailed information for a specific job |

### Organizations

| Tool | Name | Description |
|------|------|-------------|
| `workday_list_organizations` | List Organizations | Retrieve a list of all organizations |
| `workday_get_organization` | Get Organization | Retrieve detailed information for a specific organization |

### Time Off

| Tool | Name | Description |
|------|------|-------------|
| `workday_list_time_off_requests` | List Time Off Requests | Retrieve a list of all time off requests |
| `workday_get_time_off_request` | Get Time Off Request | Retrieve detailed information for a specific time off request |

### Expenses

| Tool | Name | Description |
|------|------|-------------|
| `workday_list_expenses` | List Expenses | Retrieve a list of all expense reports |
| `workday_get_expense` | Get Expense | Retrieve detailed information for a specific expense report |

## Rate Limits

Workday API limits depend on your subscription:

- Default rate limit: 1000 requests per hour
- Concurrent requests: 5

## Error Handling

Workday API errors return standard HTTP status codes with error response body:

| Status Code | Meaning |
|-------------|---------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or expired credentials |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

## Common Status Values

### Worker Status
- `Active` - Currently employed
- `On Leave` - Currently on approved leave
- `Terminated` - No longer employed

### Job Status
- `Active` - Open for staffing
- `Inactive` - Not currently staffed

### Time Off Request Status
- `APPROVED` - Request approved
- `DENIED` - Request denied
- `PENDING` - Awaiting approval

### Expense Report Status
- `APPROVED` - Expense report approved
- `DENIED` - Expense report denied
- `PENDING` - Awaiting approval
- `PAID` - Reimbursement processed

## Example Usage

### List Active Workers

```json
{
  "tool": "workday_list_workers",
  "parameters": {
    "limit": 50,
    "status": "Active"
  }
}
```

### Get Worker Details

```json
{
  "tool": "workday_get_worker",
  "parameters": {
    "worker_id": "WDT-123456"
  }
}
```

### List Time Off Requests

```json
{
  "tool": "workday_list_time_off_requests",
  "parameters": {
    "status": "PENDING"
  }
}
```

### List Pending Expenses

```json
{
  "tool": "workday_list_expenses",
  "parameters": {
    "status": "PENDING"
  }
}
```

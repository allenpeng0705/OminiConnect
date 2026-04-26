# Salesforce Additional Tools Integration

This integration provides access to extended Salesforce capabilities including case management, solutions knowledge base, Analytics Cloud reports and dashboards, and knowledge articles.

## Authentication

Salesforce uses OAuth 2.0 for authentication. Configure the following:

- **Instance URL**: Your Salesforce instance URL (e.g., `https://myorg.salesforce.com`)
- **Client ID**: Your Salesforce connected app consumer key
- **Client Secret**: Your Salesforce connected app consumer secret

## API Version

Current API version: v59.0

## Available Tools

### Cases

| Tool | Name | Description |
|------|------|-------------|
| `salesforce_more_list_cases` | List Cases | Retrieve a list of all support cases |
| `salesforce_more_get_case` | Get Case | Retrieve detailed information for a specific case |
| `salesforce_more_create_case` | Create Case | Create a new support case |
| `salesforce_more_update_case` | Update Case | Update an existing support case |

### Solutions

| Tool | Name | Description |
|------|------|-------------|
| `salesforce_more_list_solutions` | List Solutions | Retrieve a list of all solutions |
| `salesforce_more_get_solution` | Get Solution | Retrieve detailed information for a specific solution |

### Reports

| Tool | Name | Description |
|------|------|-------------|
| `salesforce_more_list_reports` | List Reports | Retrieve a list of all Analytics Cloud reports |
| `salesforce_more_get_report` | Get Report | Retrieve detailed information for a specific report |

### Dashboards

| Tool | Name | Description |
|------|------|-------------|
| `salesforce_more_list_dashboards` | List Dashboards | Retrieve a list of all Analytics Cloud dashboards |
| `salesforce_more_get_dashboard` | Get Dashboard | Retrieve detailed information for a specific dashboard |

### Knowledge Articles

| Tool | Name | Description |
|------|------|-------------|
| `salesforce_more_list_knowledge_articles` | List Knowledge Articles | Retrieve a list of all published knowledge articles |

## Rate Limits

Salesforce API limits depend on your license type:

- Default API calls per day: 15,000 (Enterprise)
- Default API calls per day: 100,000 (Unlimited)
- Concurrent API requests limit: 25

## Error Handling

Salesforce API errors return standard HTTP status codes with error response body:

| Status Code | Meaning |
|-------------|---------|
| 400 | Bad Request - Invalid parameters or query |
| 401 | Unauthorized - Invalid or expired session |
| 403 | Forbidden - Insufficient API access |
| 404 | Not Found - sObject or record does not exist |
| 429 | Too Many Requests - API limit exceeded |
| 500 | Internal Server Error |

## Example Usage

### List Cases

```json
{
  "tool": "salesforce_more_list_cases",
  "parameters": {
    "limit": 50,
    "status": "Open",
    "priority": "High"
  }
}
```

### Create Case

```json
{
  "tool": "salesforce_more_create_case",
  "parameters": {
    "subject": "Login issue with mobile app",
    "description": "User unable to login after latest update",
    "priority": "High",
    "account_id": "0015g000006ABC123"
  }
}
```

### Get Report

```json
{
  "tool": "salesforce_more_get_report",
  "parameters": {
    "report_id": "00O5g000008DEF456"
  }
}
```

### List Knowledge Articles

```json
{
  "tool": "salesforce_more_list_knowledge_articles",
  "parameters": {
    "language": "en_US",
    "category": "Product Documentation"
  }
}
```

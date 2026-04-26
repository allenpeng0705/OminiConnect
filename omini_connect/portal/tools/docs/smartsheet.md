# Smartsheet Provider Documentation

## Overview

Smartsheet is a cloud-based platform for work management and collaboration, providing tools for sheet management, project tracking, and team coordination.

## Authentication

Smartsheet uses OAuth 2.0 for authentication. The following scopes are available:

- `READ_SHEETS` - Read access to sheets and their contents
- `WRITE_SHEETS` - Create and modify sheets and rows
- `CREATE_SHEETS` - Create new sheets
- `ADMIN_USERS` - Administrative access to user management

## Base URL

```
https://api.smartsheet.com/2.0
```

## Rate Limits

- Default: 300 requests per minute per API token
- Enterprise: Higher limits available upon request

## Available Tools

### Sheets

| Tool | Description |
|------|-------------|
| `smartsheet_list_sheets` | Retrieve a list of all sheets accessible to the user |
| `smartsheet_get_sheet` | Retrieve details of a specific sheet including columns and rows |
| `smartsheet_create_sheet` | Create a new sheet in the specified workspace or folder |

### Rows

| Tool | Description |
|------|-------------|
| `smartsheet_list_rows` | Retrieve all rows from a specific sheet |
| `smartsheet_get_row` | Retrieve details of a specific row including cell values |
| `smartsheet_add_row` | Add one or more rows to a sheet |
| `smartsheet_update_row` | Update an existing row in a sheet |

### Columns

| Tool | Description |
|------|-------------|
| `smartsheet_list_columns` | Retrieve all columns from a specific sheet |
| `smartsheet_get_column` | Retrieve details of a specific column including type and options |

### Users

| Tool | Description |
|------|-------------|
| `smartsheet_list_users` | Retrieve a list of all users in the organization |

## Tool Details

### smartsheet_list_sheets

List all sheets accessible to the authenticated user.

**Endpoint:** `GET /sheets`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page` | integer | No | Page number for pagination (default: 1) |
| `pageSize` | integer | No | Number of results per page (default: 100, max: 1000) |
| `modifiedSince` | string | No | Return sheets modified since this timestamp (ISO 8601 format) |

**Example Request:**
```bash
curl -H "Authorization: Bearer {token}" \
     "https://api.smartsheet.com/2.0/sheets?pageSize=50"
```

**Example Response:**
```json
{
  "pageNumber": 1,
  "pageSize": 50,
  "totalPages": 1,
  "totalCount": 3,
  "data": [
    {
      "id": 123456789,
      "name": "Project Tracker",
      "accessLevel": "OWNER",
      "permalink": "https://app.smartsheet.com/sheets/...",
      "createdAt": "2024-01-15T10:30:00Z",
      "modifiedAt": "2024-03-20T14:45:00Z"
    }
  ]
}
```

---

### smartsheet_get_sheet

Retrieve details of a specific sheet including columns and rows.

**Endpoint:** `GET /sheets/{sheetId}`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the sheet to retrieve |
| `include` | string | No | Comma-separated list of optional data to include |
| `page` | integer | No | Page number for pagination |
| `pageSize` | integer | No | Number of rows per page |

---

### smartsheet_create_sheet

Create a new sheet in the specified workspace or folder.

**Endpoint:** `POST /sheets`

**Scopes:** `CREATE_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Name of the new sheet |
| `columns` | array | Yes | Array of column definitions |
| `workspaceId` | integer | No | ID of the workspace to create the sheet in |
| `folderId` | integer | No | ID of the folder to create the sheet in |

**Example Request:**
```bash
curl -X POST -H "Authorization: Bearer {token}" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "New Project Sheet",
       "columns": [
         {"title": "Task Name", "type": "TEXT_NUMBER", "primary": true},
         {"title": "Status", "type": "DROP_DOWN", "options": ["Not Started", "In Progress", "Complete"]},
         {"title": "Due Date", "type": "DATE"}
       ]
     }' \
     "https://api.smartsheet.com/2.0/sheets"
```

---

### smartsheet_list_rows

Retrieve all rows from a specific sheet.

**Endpoint:** `GET /sheets/{sheetId}/rows`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the sheet |
| `page` | integer | No | Page number for pagination |
| `pageSize` | integer | No | Number of results per page |
| `include` | string | No | Include additional data |
| `modifiedSince` | string | No | Return rows modified since this timestamp |

---

### smartsheet_get_row

Retrieve details of a specific row including cell values.

**Endpoint:** `GET /sheets/{sheetId}/rows/{rowId}`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the parent sheet |
| `rowId` | integer | Yes | The ID of the row to retrieve |
| `include` | string | No | Include additional data |

---

### smartsheet_add_row

Add one or more rows to a sheet.

**Endpoint:** `POST /sheets/{sheetId}/rows`

**Scopes:** `WRITE_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the sheet |
| `rows` | array | Yes | Array of row objects to add |

---

### smartsheet_update_row

Update an existing row in a sheet.

**Endpoint:** `PUT /sheets/{sheetId}/rows/{rowId}`

**Scopes:** `WRITE_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the parent sheet |
| `rowId` | integer | Yes | The ID of the row to update |
| `cells` | array | Yes | Array of cell values to update |
| `locked` | boolean | No | Lock or unlock the row |
| `expanded` | boolean | No | Expand or collapse the row |

---

### smartsheet_list_columns

Retrieve all columns from a specific sheet.

**Endpoint:** `GET /sheets/{sheetId}/columns`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the sheet |
| `page` | integer | No | Page number for pagination |
| `pageSize` | integer | No | Number of results per page |

---

### smartsheet_get_column

Retrieve details of a specific column including type and options.

**Endpoint:** `GET /sheets/{sheetId}/columns/{columnId}`

**Scopes:** `READ_SHEETS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sheetId` | integer | Yes | The ID of the parent sheet |
| `columnId` | integer | Yes | The ID of the column to retrieve |

---

### smartsheet_list_users

Retrieve a list of all users in the organization.

**Endpoint:** `GET /users`

**Scopes:** `ADMIN_USERS`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `pageSize` | integer | No | Number of results per page |
| `email` | string | No | Filter by email address |

---

## Common Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Rate Limit Exceeded |
| 500 | Internal Server Error |

## Additional Resources

- [Smartsheet API Documentation](https://smartsheet.redoc.ly/)
- [OAuth 2.0 Setup](https://www.smartsheet.com/developers/oauth-20)
- [Rate Limiting](https://smartsheet.redoc.ly/#section/Rate-Limiting)

# Rows

Rows is a spreadsheet platform that combines the simplicity of spreadsheets with powerful collaboration features. It supports tables, rows, columns, shares, and user management.

## Connection

- **Auth Type**: OAuth 2.0
- **Base URL**: `https://api.rows.com/api/v1`
- **Documentation**: https://docs.rows.com

## Available Scopes

| Scope | Description |
|-------|-------------|
| `spreadsheets:read` | Read access to spreadsheets |
| `spreadsheets:write` | Write access to spreadsheets |
| `tables:read` | Read access to tables |
| `tables:write` | Write access to tables |
| `rows:read` | Read access to rows |
| `rows:write` | Write access to rows |
| `users:read` | Read access to users |

## Tool Registry

### Spreadsheet Management

#### list_spreadsheets
List all accessible spreadsheets.

**Endpoint**: `GET /spreadsheets`

**Scopes**: `spreadsheets:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Maximum spreadsheets to return |
| offset | integer | No | Pagination offset |

---

#### get_spreadsheet
Retrieve details and metadata of a specific spreadsheet.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}`

**Scopes**: `spreadsheets:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |

---

#### create_spreadsheet
Create a new spreadsheet.

**Endpoint**: `POST /spreadsheets`

**Scopes**: `spreadsheets:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Spreadsheet name |
| folderId | string | No | Folder to create in |

---

### Table Operations

#### list_tables
List all tables in a spreadsheet.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}/tables`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |

---

#### get_table
Get schema and metadata for a specific table.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}/tables/{tableId}`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |
| tableId | string | Yes | Table identifier |

---

### Row Operations

#### list_rows
Retrieve rows from a table with optional filtering.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}/tables/{tableId}/rows`

**Scopes**: `rows:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |
| tableId | string | Yes | Table identifier |
| limit | integer | No | Maximum rows to return |
| offset | integer | No | Pagination offset |
| filter | object | No | Filter criteria |

---

#### get_row
Get a single row by ID.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}/tables/{tableId}/rows/{rowId}`

**Scopes**: `rows:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |
| tableId | string | Yes | Table identifier |
| rowId | string | Yes | Row identifier |

---

#### insert_row
Insert a new row into a table.

**Endpoint**: `POST /spreadsheets/{spreadsheetId}/tables/{tableId}/rows`

**Scopes**: `rows:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |
| tableId | string | Yes | Table identifier |
| data | object | Yes | Row data |
| position | integer | No | Position to insert at |

---

#### update_row
Update an existing row.

**Endpoint**: `PUT /spreadsheets/{spreadsheetId}/tables/{tableId}/rows/{rowId}`

**Scopes**: `rows:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |
| tableId | string | Yes | Table identifier |
| rowId | string | Yes | Row identifier |
| data | object | Yes | Updated row data |

---

### Collaboration

#### list_collaborators
List all collaborators on a spreadsheet.

**Endpoint**: `GET /spreadsheets/{spreadsheetId}/collaborators`

**Scopes**: `users:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| spreadsheetId | string | Yes | Spreadsheet identifier |

---

## Usage Example

```python
# List spreadsheets
response = await client.request(
    method="GET",
    endpoint="/spreadsheets",
    scopes=["spreadsheets:read"]
)

# Insert a row
response = await client.request(
    method="POST",
    endpoint="/spreadsheets/sp123/tables/t456/rows",
    scopes=["rows:write"],
    data={"Name": "New Entry", "Value": 100}
)
```
# Stackby

Stackby is a database/spreadsheet hybrid platform that combines the familiarity of spreadsheets with the power of databases. It supports tables, rows, views, and user management.

## Connection

- **Auth Type**: OAuth 2.0
- **Base URL**: `https://api.stackby.com/api/v1`
- **Documentation**: https://docs.stackby.com

## Available Scopes

| Scope | Description |
|-------|-------------|
| `databases:read` | Read access to databases |
| `databases:write` | Write access to databases |
| `tables:read` | Read access to tables |
| `tables:write` | Write access to tables |
| `rows:read` | Read access to rows |
| `rows:write` | Write access to rows |
| `views:read` | Read access to views |

## Tool Registry

### Database Management

#### list_databases
List all accessible Stackby databases.

**Endpoint**: `GET /v1/databases`

**Scopes**: `databases:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Maximum databases to return |
| offset | integer | No | Pagination offset |

---

#### get_database
Get details and metadata of a specific database.

**Endpoint**: `GET /v1/databases/{databaseId}`

**Scopes**: `databases:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |

---

#### create_database
Create a new Stackby database.

**Endpoint**: `POST /v1/databases`

**Scopes**: `databases:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Database name |
| schema | object | No | Schema definition for tables |

---

### Table Operations

#### list_tables
List all tables in a database.

**Endpoint**: `GET /v1/databases/{databaseId}/tables`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |

---

#### get_table
Get schema and metadata for a specific table.

**Endpoint**: `GET /v1/databases/{databaseId}/tables/{tableId}`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |

---

### Row Operations

#### list_rows
Retrieve rows from a table with optional filtering.

**Endpoint**: `GET /v1/databases/{databaseId}/tables/{tableId}/rows`

**Scopes**: `rows:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |
| limit | integer | No | Maximum rows to return |
| offset | integer | No | Pagination offset |
| filter | string | No | Filter expression |

---

#### get_row
Get a single row by ID.

**Endpoint**: `GET /v1/databases/{databaseId}/tables/{tableId}/rows/{rowId}`

**Scopes**: `rows:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |
| rowId | string | Yes | Row identifier |

---

#### create_row
Create a new row in a table.

**Endpoint**: `POST /v1/databases/{databaseId}/tables/{tableId}/rows`

**Scopes**: `rows:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |
| data | object | Yes | Row data as key-value pairs |

---

#### update_row
Update an existing row.

**Endpoint**: `PUT /v1/databases/{databaseId}/tables/{tableId}/rows/{rowId}`

**Scopes**: `rows:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |
| rowId | string | Yes | Row identifier |
| data | object | Yes | Updated row data |

---

### View Operations

#### list_views
List all views available for a table.

**Endpoint**: `GET /v1/databases/{databaseId}/tables/{tableId}/views`

**Scopes**: `views:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| databaseId | string | Yes | Database identifier |
| tableId | string | Yes | Table identifier |

---

## Usage Example

```python
# List databases
response = await client.request(
    method="GET",
    endpoint="/v1/databases",
    scopes=["databases:read"]
)

# Create a row
response = await client.request(
    method="POST",
    endpoint="/v1/databases/db123/tables/table456/rows",
    scopes=["rows:write"],
    data={"Name": "New Entry", "Status": "Active"}
)
```
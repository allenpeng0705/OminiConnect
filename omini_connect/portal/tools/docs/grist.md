# Grist

Grist is a spreadsheet/database hybrid that combines the familiarity of spreadsheets with the power of a relational database. It supports tables, records, views, formulas, and user management.

## Connection

- **Auth Type**: OAuth 2.0
- **Base URL**: `https://api.grist.com/api/v1`
- **Documentation**: https://docs.grist.com

## Available Scopes

| Scope | Description |
|-------|-------------|
| `documents:read` | Read access to documents |
| `documents:write` | Write access to documents |
| `tables:read` | Read access to tables |
| `tables:write` | Write access to tables |
| `records:read` | Read access to records |
| `records:write` | Write access to records |
| `views:read` | Read access to views |

## Tool Registry

### Document Management

#### list_documents
List all accessible Grist documents.

**Endpoint**: `GET /documents`

**Scopes**: `documents:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Maximum documents to return |
| offset | integer | No | Pagination offset |

---

#### get_document
Retrieve details and metadata of a specific document.

**Endpoint**: `GET /documents/{documentId}`

**Scopes**: `documents:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |

---

#### create_document
Create a new Grist document.

**Endpoint**: `POST /documents`

**Scopes**: `documents:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string | Yes | Document name |
| workspaceId | string | No | Workspace to create in |

---

### Table Operations

#### list_tables
List all tables in a document.

**Endpoint**: `GET /documents/{documentId}/tables`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |

---

#### get_table
Get schema and metadata for a specific table.

**Endpoint**: `GET /documents/{documentId}/tables/{tableId}`

**Scopes**: `tables:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |
| tableId | string | Yes | Table identifier |

---

### Record Operations

#### list_records
Retrieve records from a table with optional filtering.

**Endpoint**: `GET /documents/{documentId}/tables/{tableId}/records`

**Scopes**: `records:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |
| tableId | string | Yes | Table identifier |
| limit | integer | No | Maximum records to return |
| offset | integer | No | Pagination offset |
| filter | object | No | Filter criteria |

---

#### get_record
Get a single record by ID.

**Endpoint**: `GET /documents/{documentId}/tables/{tableId}/records/{recordId}`

**Scopes**: `records:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |
| tableId | string | Yes | Table identifier |
| recordId | string | Yes | Record identifier |

---

#### create_record
Create a new record in a table.

**Endpoint**: `POST /documents/{documentId}/tables/{tableId}/records`

**Scopes**: `records:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |
| tableId | string | Yes | Table identifier |
| data | object | Yes | Record data |

---

#### update_record
Update an existing record.

**Endpoint**: `PUT /documents/{documentId}/tables/{tableId}/records/{recordId}`

**Scopes**: `records:write`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |
| tableId | string | Yes | Table identifier |
| recordId | string | Yes | Record identifier |
| data | object | Yes | Updated record data |

---

### View Operations

#### list_views
List all views available in a document.

**Endpoint**: `GET /documents/{documentId}/views`

**Scopes**: `views:read`

**Parameters**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| documentId | string | Yes | Document identifier |

---

## Usage Example

```python
# List documents
response = await client.request(
    method="GET",
    endpoint="/documents",
    scopes=["documents:read"]
)

# Create a record
response = await client.request(
    method="POST",
    endpoint="/documents/doc123/tables/table456/records",
    scopes=["records:write"],
    data={"Name": "New Entry", "Status": "Active"}
)
```
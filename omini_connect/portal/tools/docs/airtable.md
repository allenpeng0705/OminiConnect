# Airtable Tools

Provider: `airtable` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Airtable REST API. They allow AI agents to manage bases, tables, fields, and records; query data with filters and sorting; and perform CRUD operations on structured data. Airtable combines the simplicity of a spreadsheet with the power of a database.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Airtable
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `data.records:read`, `data.records:write`, `schema.bases:read`, `schema.bases:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `airtable_list_bases` | List all bases | GET | /v0/meta/bases |
| `airtable_get_base` | Get base details | GET | /v0/meta/bases/{baseId} |
| `airtable_list_tables` | List tables in a base | GET | /v0/meta/bases/{baseId}/tables |
| `airtable_get_table` | Get table schema | GET | /v0/meta/bases/{baseId}/tables/{tableId} |
| `airtable_list_records` | List records in a table | GET | /v0/{baseId}/{tableId} |
| `airtable_get_record` | Get a single record | GET | /v0/{baseId}/{tableId}/{recordId} |
| `airtable_create_record` | Create a new record | POST | /v0/{baseId}/{tableId} |
| `airtable_update_record` | Update a record | PATCH | /v0/{baseId}/{tableId}/{recordId} |
| `airtable_list_fields` | List fields in a table | GET | /v0/meta/bases/{baseId}/tables/{tableId}/fields |
| `airtable_get_field` | Get field details | GET | /v0/meta/bases/{baseId}/tables/{tableId}/fields/{fieldId} |

---

## Tool Details

### airtable_list_bases

**What it does**: Lists all Airtable bases the authenticated user has access to.

**When to use**: Find available bases to work with, get base IDs and names.

**Arguments**: None

**Example LLM prompt**: "List all my Airtable bases"

---

### airtable_get_base

**What it does**: Gets details about a specific Airtable base.

**When to use**: Understand base structure before listing tables.

**Arguments**:
- `baseId` (required): The Airtable base ID

**Example LLM prompt**: "Get details for base appABC123"

---

### airtable_list_tables

**What it does**: Lists all tables in an Airtable base.

**When to use**: Find available tables to query or update.

**Arguments**:
- `baseId` (required): The Airtable base ID

**Example LLM prompt**: "List all tables in base appABC123"

---

### airtable_get_table

**What it does**: Gets detailed schema of an Airtable table including field names, types, and descriptions.

**When to use**: Understand table structure before querying records.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID

**Example LLM prompt**: "Get the schema for the Tasks table in base appABC123"

---

### airtable_list_records

**What it does**: Lists records from an Airtable table with pagination. Supports filtering and sorting.

**When to use**: Retrieve records, paginate through large datasets, filter by conditions.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID or name
- `pageSize` (optional): Number of records per page (max 100)
- `offset` (optional): Pagination offset token
- `filterByFormula` (optional): Filter formula
- `fields` (optional): Fields to include array

**Example LLM prompt**: "List all records in the Tasks table where Status equals Active, sorted by Due Date"

---

### airtable_get_record

**What it does**: Gets a single record by ID with all its field values.

**When to use**: Read full record details before updating or deleting.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID or name
- `recordId` (required): The record ID

**Example LLM prompt**: "Get record rec123456 from the Tasks table"

---

### airtable_create_record

**What it does**: Creates a new record in an Airtable table with specified field values.

**When to use**: Add new rows of data, create new items or entries.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID or name
- `fields` (required): Record field values as key-value pairs

**Example LLM prompt**: "Create a new record in the Tasks table with Name 'Buy groceries' and Due Date '2024-01-15'"

---

### airtable_update_record

**What it does**: Updates an existing record. Only provided fields will be modified.

**When to use**: Change specific fields on a record without affecting others.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID or name
- `recordId` (required): The record ID
- `fields` (required): Fields to update as key-value pairs

**Example LLM prompt**: "Update the record rec123456 in Tasks table to set Status to Completed"

---

### airtable_list_fields

**What it does**: Lists all field definitions for a specific table.

**When to use**: Understand table schema before creating or updating records.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID

**Example LLM prompt**: "List all fields in the Tasks table"

---

### airtable_get_field

**What it does**: Gets metadata for a specific field including name, type, and description.

**When to use**: Understand a specific field before filtering or sorting by it.

**Arguments**:
- `baseId` (required): The Airtable base ID
- `tableId` (required): The table ID
- `fieldId` (required): The field ID

**Example LLM prompt**: "Get details for the Status field in the Tasks table"

---

## Airtable API Notes

- **Base IDs**: Start with `app` prefix (e.g., `appABC123`)
- **Table IDs**: Can use table name as string in the endpoint
- **Record IDs**: Start with `rec` prefix (e.g., `recABC123`)
- **Field IDs**: Start with `fld` prefix (e.g., `fldABC123`)
- **Filter formulas**: Airtable's formula language supports text, numeric, date, and logical operations
- **Field types**: Text, number, date, checkbox, single/multi select, user, attachment, and more
- **Rate limits**: 5 requests per second per base for API access
- **Pagination**: Use offset token from response for next page
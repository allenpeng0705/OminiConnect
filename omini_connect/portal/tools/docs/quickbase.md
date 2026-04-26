# Quickbase Tools

Provider: `quickbase` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Quickbase is a low-code database platform for building custom applications. These tools allow AI agents to manage tables, records, fields, and run queries on your Quickbase databases.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Quickbase
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tables:read`, `tables:read`, `records:read`, `records:write`, `reports:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `quickbase_list_tables` | List all tables | GET | /tables |
| `quickbase_get_table` | Get table details | GET | /tables/{tableId} |
| `quickbase_list_records` | List records in a table | GET | /records |
| `quickbase_get_record` | Get a specific record | GET | /records/{recordId} |
| `quickbase_create_record` | Create a new record | POST | /records |
| `quickbase_update_record` | Update an existing record | PUT | /records/{recordId} |
| `quickbase_delete_record` | Delete a record | DELETE | /records/{recordId} |
| `quickbase_run_query` | Run a query on a table | POST | /queries/run |
| `quickbase_list_fields` | List fields in a table | GET | /tables/{tableId}/fields |
| `quickbase_get_report` | Get a report | GET | /reports/{reportId} |

---

## Tool Details

### quickbase_list_tables

**What it does**: Returns a list of all tables in your Quickbase realm.

**When to use**: Discover available databases and tables.

**Arguments**: None

**Example LLM prompt**: "List all tables in my Quickbase"

---

### quickbase_get_table

**What it does**: Gets details of a specific table.

**When to use**: Understand table structure and settings.

**Arguments**:
- `tableId` (required): The table ID

**Example LLM prompt**: "Get details for table T12345"

---

### quickbase_list_records

**What it does**: Returns a paginated list of records from a table.

**When to use**: Retrieve data from your database.

**Arguments**:
- `tableId` (required): The table ID
- `limit` (optional): Number of records (default 100, max 1000)
- `skip` (optional): Records to skip for pagination

**Example LLM prompt**: "List first 50 records from table T12345"

---

### quickbase_get_record

**What it does**: Gets a specific record by ID.

**When to use**: Retrieve individual record details.

**Arguments**:
- `recordId` (required): The record ID
- `tableId` (required): The table ID

**Example LLM prompt**: "Get record R12345 from table T12345"

---

### quickbase_create_record

**What it does**: Creates a new record in a table.

**When to use**: Add new data to your database.

**Arguments**:
- `tableId` (required): The table ID
- `fields` (required): Field values as key-value pairs

**Example LLM prompt**: "Create a record in table T12345 with name 'John' and email 'john@example.com'"

---

### quickbase_update_record

**What it does**: Updates an existing record.

**When to use**: Modify existing data.

**Arguments**:
- `recordId` (required): The record ID
- `tableId` (required): The table ID
- `fields` (required): Updated field values

**Example LLM prompt**: "Update record R12345 in table T12345 with phone '555-1234'"

---

### quickbase_delete_record

**What it does**: Deletes a record from a table.

**When to use**: Remove unwanted data.

**Arguments**:
- `recordId` (required): The record ID
- `tableId` (required): The table ID

**Example LLM prompt**: "Delete record R12345 from table T12345"

---

### quickbase_run_query

**What it does**: Runs a query to filter and retrieve records.

**When to use**: Search and filter database records.

**Arguments**:
- `tableId` (required): The table ID
- `query` (required): Quickbase query string
- `limit` (optional): Max records to return

**Example LLM prompt**: "Find all records where status equals 'active' in table T12345"

---

### quickbase_list_fields

**What it does**: Lists all fields (columns) in a table.

**When to use**: Understand table schema.

**Arguments**:
- `tableId` (required): The table ID

**Example LLM prompt**: "List all fields in table T12345"

---

### quickbase_get_report

**What it does**: Gets a saved report from a table.

**When to use**: Retrieve pre-configured report data.

**Arguments**:
- `reportId` (required): The report ID
- `tableId` (required): The table ID

**Example LLM prompt**: "Get report RPT123 from table T12345"

---

## Quickbase API Notes

- Tables and records have string IDs
- Query syntax follows Quickbase's query language
- Field values are passed as key-value pairs in the fields object

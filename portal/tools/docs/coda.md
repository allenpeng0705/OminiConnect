# Coda Tools

Provider: `coda` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Coda API. They allow AI agents to manage documents, tables, and rows; query data with filters; and perform CRUD operations. Coda combines documents and spreadsheets into a unified workspace with powerful tables.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Coda
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `docs:read`, `docs:write`, `tables:read`, `tables:write`, `rows:read`, `rows:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `coda_list_docs` | List all documents | GET | /docs |
| `coda_get_doc` | Get document details | GET | /docs/{docId} |
| `coda_create_doc` | Create a new document | POST | /docs |
| `coda_list_tables` | List tables in a document | GET | /docs/{docId}/tables |
| `coda_get_table` | Get table schema | GET | /docs/{docId}/tables/{tableId} |
| `coda_list_rows` | List rows in a table | GET | /docs/{docId}/tables/{tableId}/rows |
| `coda_get_row` | Get a single row | GET | /docs/{docId}/tables/{tableId}/rows/{rowId} |
| `coda_create_row` | Create a new row | POST | /docs/{docId}/tables/{tableId}/rows |
| `coda_update_row` | Update a row | PUT | /docs/{docId}/tables/{tableId}/rows/{rowId} |
| `coda_list_collaborators` | List document collaborators | GET | /docs/{docId}/collaborators |

---

## Tool Details

### coda_list_docs

**What it does**: Lists all accessible Coda documents. Returns doc metadata including title, owner, and workspace.

**When to use**: Find available documents to work with.

**Arguments**:
- `limit` (optional): Maximum number of documents to return
- `offset` (optional): Number of documents to skip for pagination

**Example LLM prompt**: "List all my Coda documents"

---

### coda_get_doc

**What it does**: Gets details and metadata for a specific document by ID including name, workspace, and settings.

**When to use**: Understand document structure before working with tables.

**Arguments**:
- `docId` (required): The unique identifier of the document

**Example LLM prompt**: "Get details for document doc123"

---

### coda_create_doc

**What it does**: Creates a new Coda document with specified title in a workspace or folder.

**When to use**: Create new workspaces, project docs, or databases.

**Arguments**:
- `title` (required): Title of the new document
- `folderId` (optional): Folder ID to create document in

**Example LLM prompt**: "Create a new document titled 'Project Tracker'"

---

### coda_list_tables

**What it does**: Lists all tables within a specific document. Returns table metadata and column schemas.

**When to use**: Find available tables to query or update.

**Arguments**:
- `docId` (required): The unique identifier of the document

**Example LLM prompt**: "List all tables in document doc123"

---

### coda_get_table

**What it does**: Gets details, schema, and structure of a specific table including column definitions.

**When to use**: Understand column structure before reading or writing rows.

**Arguments**:
- `docId` (required): The unique identifier of the document
- `tableId` (required): The unique identifier of the table

**Example LLM prompt**: "Get the schema for table tbl456 in document doc123"

---

### coda_list_rows

**What it does**: Retrieves rows from a specific table with optional filtering, sorting, and pagination.

**When to use**: Read data from tables, filter by conditions, paginate through results.

**Arguments**:
- `docId` (required): The unique identifier of the document
- `tableId` (required): The unique identifier of the table
- `limit` (optional): Maximum number of rows to return
- `offset` (optional): Number of rows to skip for pagination
- `filter` (optional): Filter expression for rows

**Example LLM prompt**: "List all rows in the Tasks table where Status equals 'Active'"

---

### coda_get_row

**What it does**: Gets a single row by its unique identifier including all cell values.

**When to use**: Read full row details before updating or for detailed inspection.

**Arguments**:
- `docId` (required): The unique identifier of the document
- `tableId` (required): The unique identifier of the table
- `rowId` (required): The unique identifier of the row

**Example LLM prompt**: "Get the row with ID row789 from the Tasks table"

---

### coda_create_row

**What it does**: Inserts a new row into a specified table with cell values for each column.

**When to use**: Add new entries, create tasks, or insert data into tables.

**Arguments**:
- `docId` (required): The unique identifier of the document
- `tableId` (required): The unique identifier of the table
- `cells` (required): Row cell values as array of column-value objects

**Example LLM prompt**: "Create a new row in the Tasks table with Name 'Complete report' and DueDate '2024-01-20'"

---

### coda_update_row

**What it does**: Updates an existing row with new cell values in a specified table.

**When to use**: Modify row values, update task status, or edit existing entries.

**Arguments**:
- `docId` (required): The unique identifier of the document
- `tableId` (required): The unique identifier of the table
- `rowId` (required): The unique identifier of the row
- `cells` (required): Updated row cell values as array of column-value objects

**Example LLM prompt**: "Update row row789 in the Tasks table to set Status to 'Completed'"

---

### coda_list_collaborators

**What it does**: Lists all collaborators on a specific document including their email and permission level.

**When to use**: See who has access to a document, find team members.

**Arguments**:
- `docId` (required): The unique identifier of the document

**Example LLM prompt**: "List all collaborators on document doc123"

---

## Coda API Notes

- **Doc IDs**: Start with `doc-` prefix (e.g., `doc-ABC123`)
- **Table IDs**: Start with `table-` prefix (e.g., `table-ABC123`)
- **Row IDs**: Start with `row-` prefix (e.g., `row-ABC123`)
- **Cells format**: Array of objects with `column` name and `value` (e.g., `[{"column": "Name", "value": "Task 1"}]`)
- **Filter expressions**: Use Coda formula syntax for filtering rows
- **Pagination**: Use offset for paginated results, limit for page size
- **Column names**: Case-sensitive names used to identify columns in cell objects
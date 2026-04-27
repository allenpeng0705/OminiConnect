# Microsoft Excel Tools

Provider: `microsoft-excel` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Excel API. They allow AI agents to manage workbooks, worksheets, tables, and cell data in OneDrive. **Requires Microsoft Excel OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Excel)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `excel_list_workbooks` | List workbooks | GET | /v1.0/me/drive/root/children |
| `excel_get_workbook` | Get workbook details | GET | /v1.0/me/drive/items/{itemId} |
| `excel_list_worksheets` | List worksheets | GET | /v1.0/me/drive/items/{itemId}/workbook/worksheets |
| `excel_get_worksheet` | Get worksheet details | GET | /v1.0/me/drive/items/{itemId}/workbook/worksheets/{worksheetId} |
| `excel_list_tables` | List tables | GET | /v1.0/me/drive/items/{itemId}/workbook/worksheets/{worksheetId}/tables |
| `excel_get_table` | Get table details | GET | /v1.0/me/drive/items/{itemId}/workbook/tables/{tableId} |
| `excel_list_rows` | List table rows | GET | /v1.0/me/drive/items/{itemId}/workbook/tables/{tableId}/rows |
| `excel_update_row` | Update a row | PATCH | /v1.0/me/drive/items/{itemId}/workbook/tables/{tableId}/rows/{rowIndex} |
| `excel_add_row` | Add a row | POST | /v1.0/me/drive/items/{itemId}/workbook/tables/{tableId}/rows |
| `excel_get_range` | Get cell range | GET | /v1.0/me/drive/items/{itemId}/workbook/worksheets/{worksheetId}/range |

---

## Tool Details

### excel_list_workbooks

**What it does**: Lists all Excel workbooks accessible to the user.

**When to use**: Find workbooks, browse files.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all Excel files in my OneDrive"

---

### excel_get_workbook

**What it does**: Gets details of a specific workbook.

**When to use**: Check workbook metadata.

**Arguments**:
- `itemId` (required): Workbook item ID

**Example LLM prompt**: "Get details for workbook ITEM-12345"

---

### excel_list_worksheets

**What it does**: Lists all worksheets in a workbook.

**When to use**: Navigate workbook structure.

**Arguments**:
- `itemId` (required): Workbook item ID

**Example LLM prompt**: "List all sheets in workbook ITEM-12345"

---

### excel_get_worksheet

**What it does**: Gets details of a specific worksheet.

**When to use**: Get worksheet information.

**Arguments**:
- `itemId` (required): Workbook item ID
- `worksheetId` (required): Worksheet ID or name

**Example LLM prompt**: "Get details for sheet Sheet1 in workbook ITEM-12345"

---

### excel_list_tables

**What it does**: Lists all tables in a worksheet.

**When to use**: Find tables in worksheet.

**Arguments**:
- `itemId` (required): Workbook item ID
- `worksheetId` (required): Worksheet ID or name

**Example LLM prompt**: "List all tables in sheet Sheet1"

---

### excel_get_table

**What it does**: Gets details of a specific table.

**When to use**: Check table structure.

**Arguments**:
- `itemId` (required): Workbook item ID
- `tableId` (required): Table ID

**Example LLM prompt**: "Get details for table TABLE-12345"

---

### excel_list_rows

**What it does**: Lists rows from a table.

**When to use**: Read table data.

**Arguments**:
- `itemId` (required): Workbook item ID
- `tableId` (required): Table ID
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all rows in table TABLE-12345"

---

### excel_update_row

**What it does**: Updates a specific row in a table.

**When to use**: Modify table data.

**Arguments**:
- `itemId` (required): Workbook item ID
- `tableId` (required): Table ID
- `rowIndex` (required): Row index (0-based)
- `values` (required): Row values

**Example LLM prompt**: "Update row 5 in table TABLE-12345 with new values"

---

### excel_add_row

**What it does**: Adds a new row to a table.

**When to use**: Insert new data into table.

**Arguments**:
- `itemId` (required): Workbook item ID
- `tableId` (required): Table ID
- `values` (required): Row values

**Example LLM prompt**: "Add a new row to table TABLE-12345"

---

### excel_get_range

**What it does**: Gets values from a specific range in a worksheet.

**When to use**: Read cell values, bulk data access.

**Arguments**:
- `itemId` (required): Workbook item ID
- `worksheetId` (required): Worksheet ID or name
- `address` (optional): Range address (default A1:Z100)

**Example LLM prompt**: "Get values from range A1:D10 in sheet Sheet1"

---

## Excel API Notes

- **Workbooks**: Excel files (.xlsx) in OneDrive
- **Worksheets**: Individual sheets in workbook
- **Tables**: Formatted Excel tables
- **Ranges**: Cell ranges with values
- **Item IDs**: OneDrive item identifiers

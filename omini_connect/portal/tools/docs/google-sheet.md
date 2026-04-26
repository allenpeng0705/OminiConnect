# Google Sheets Tools

Provider: `google-sheet` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Sheets API. They allow AI agents to manage spreadsheets, read and write cell values, and manage sheets. **Requires Google OAuth2 with Sheets permissions.**

## Authentication

**Nango OAUTH2 (Google Sheets)**:
- User authenticates via OAuth2 with Sheets scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://sheets.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_sheet_list_spreadsheets` | List spreadsheets | GET | /v4/spreadsheets |
| `google_sheet_get_spreadsheet` | Get spreadsheet details | GET | /v4/spreadsheets/{spreadsheet_id} |
| `google_sheet_create_spreadsheet` | Create spreadsheet | POST | /v4/spreadsheets |
| `google_sheet_get_values` | Get cell values | GET | /v4/spreadsheets/{spreadsheet_id}/values/{range} |
| `google_sheet_update_values` | Update cell values | PUT | /v4/spreadsheets/{spreadsheet_id}/values/{range} |
| `google_sheet_append_values` | Append values | POST | /v4/spreadsheets/{spreadsheet_id}/values/{range}:append |
| `google_sheet_list_sheets` | List sheets | GET | /v4/spreadsheets/{spreadsheet_id} |
| `google_sheet_batch_update` | Batch update | POST | /v4/spreadsheets/{spreadsheet_id}:batchUpdate |
| `google_sheet_get_dimension` | Get dimension info | GET | /v4/spreadsheets/{spreadsheet_id}/values/{range} |
| `google_sheet_create_sheet` | Create new sheet | POST | /v4/spreadsheets/{spreadsheet_id}:batchUpdate |

---

## Tool Details

### google_sheet_list_spreadsheets

**What it does**: Lists spreadsheets in Google Drive.

**When to use**: Browse available spreadsheets.

**Arguments**: None

**Example LLM prompt**: "List my spreadsheets"

---

### google_sheet_get_spreadsheet

**What it does**: Gets spreadsheet details.

**When to use**: Get spreadsheet metadata.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID

**Example LLM prompt**: "Get details for spreadsheet abc123"

---

### google_sheet_create_spreadsheet

**What it does**: Creates a new spreadsheet.

**When to use**: Create new spreadsheets.

**Arguments**:
- `title` (required): Spreadsheet title
- `sheets` (optional): Sheet titles

**Example LLM prompt**: "Create a spreadsheet titled 'Sales Data'"

---

### google_sheet_get_values

**What it does**: Gets values from a range.

**When to use**: Read cell data.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `range` (required): Cell range (e.g., Sheet1!A1:B10)

**Example LLM prompt**: "Get values from Sheet1!A1:C10"

---

### google_sheet_update_values

**What it does**: Updates cell values.

**When to use**: Write data to cells.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `range` (required): Cell range
- `values` (required): 2D array of values

**Example LLM prompt**: "Update Sheet1!A1:B2 with [[1,2],[3,4]]"

---

### google_sheet_append_values

**What it does**: Appends values to a sheet.

**When to use**: Add new rows of data.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `range` (required): Cell range
- `values` (required): 2D array of values

**Example LLM prompt**: "Append row [date, amount] to Sheet1"

---

### google_sheet_list_sheets

**What it does**: Lists sheets in a spreadsheet.

**When to use**: See available sheets.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID

**Example LLM prompt**: "List sheets in spreadsheet abc123"

---

### google_sheet_batch_update

**What it does**: Performs batch updates on a spreadsheet.

**When to use**: Make multiple changes at once.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `requests` (required): Array of requests

**Example LLM prompt**: "Batch update spreadsheet abc123"

---

### google_sheet_get_dimension

**What it does**: Gets dimension information.

**When to use**: Get row/column info.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `sheet_id` (required): Sheet ID

**Example LLM prompt**: "Get dimension info for sheet 0"

---

### google_sheet_create_sheet

**What it does**: Creates a new sheet in a spreadsheet.

**When to use**: Add new worksheets.

**Arguments**:
- `spreadsheet_id` (required): Spreadsheet ID
- `title` (required): Sheet title

**Example LLM prompt**: "Add new sheet named 'Report' to spreadsheet abc123"

---

## Google Sheets API Notes

- **Spreadsheet ID**: Found in the spreadsheet URL
- **Range format**: SheetName!A1:B10
- **Values**: 2D arrays for bulk operations
- **A1 notation**: Standard spreadsheet range syntax
- **Batch updates**: For multiple formatting or structural changes

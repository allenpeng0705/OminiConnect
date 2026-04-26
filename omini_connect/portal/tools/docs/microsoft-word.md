# Microsoft Word Tools

Provider: `microsoft-word` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Word API. They allow AI agents to manage documents, paragraphs, tables, and styles in OneDrive. **Requires Word OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Word)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `word_list_documents` | List documents | GET | /v1.0/me/drive/root/children |
| `word_get_document` | Get document details | GET | /v1.0/me/drive/items/{itemId} |
| `word_list_content` | List document content | GET | /v1.0/me/drive/items/{itemId}/word/document/paragraphs |
| `word_get_paragraph` | Get paragraph content | GET | /v1.0/me/drive/items/{itemId}/word/document/paragraphs/{paragraphId} |
| `word_update_paragraph` | Update paragraph | PATCH | /v1.0/me/drive/items/{itemId}/word/document/paragraphs/{paragraphId} |
| `word_list_tables` | List tables in document | GET | /v1.0/me/drive/items/{itemId}/word/document/tables |
| `word_get_table` | Get table content | GET | /v1.0/me/drive/items/{itemId}/word/document/tables/{tableId} |
| `word_insert_paragraph` | Insert paragraph | POST | /v1.0/me/drive/items/{itemId}/word/document/paragraphs |
| `word_list_styles` | List document styles | GET | /v1.0/me/drive/items/{itemId}/word/document/styles |
| `word_apply_style` | Apply style to text | PATCH | /v1.0/me/drive/items/{itemId}/word/document/paragraphs/{paragraphId}/formatting |

---

## Tool Details

### word_list_documents

**What it does**: Lists all Word documents accessible to the user.

**When to use**: Find documents, browse files.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all Word files in my OneDrive"

---

### word_get_document

**What it does**: Gets details of a specific Word document.

**When to use**: Check document metadata.

**Arguments**:
- `itemId` (required): Document item ID

**Example LLM prompt**: "Get details for document ITEM-12345"

---

### word_list_content

**What it does**: Lists all paragraphs in a Word document.

**When to use**: Read document content, find text.

**Arguments**:
- `itemId` (required): Document item ID

**Example LLM prompt**: "List all paragraphs in document ITEM-12345"

---

### word_get_paragraph

**What it does**: Gets content of a specific paragraph.

**When to use**: Read paragraph text, check formatting.

**Arguments**:
- `itemId` (required): Document item ID
- `paragraphId` (required): Paragraph ID

**Example LLM prompt**: "Get content of paragraph 1 in document ITEM-12345"

---

### word_update_paragraph

**What it does**: Updates content of a specific paragraph.

**When to use**: Edit document text.

**Arguments**:
- `itemId` (required): Document item ID
- `paragraphId` (required): Paragraph ID
- `text` (required): New paragraph text

**Example LLM prompt**: "Update paragraph 1 in document ITEM-12345 with new text"

---

### word_list_tables

**What it does**: Lists all tables in a Word document.

**When to use**: Find tables in document.

**Arguments**:
- `itemId` (required): Document item ID

**Example LLM prompt**: "List all tables in document ITEM-12345"

---

### word_get_table

**What it does**: Gets content of a specific table.

**When to use**: Read table data, check table structure.

**Arguments**:
- `itemId` (required): Document item ID
- `tableId` (required): Table ID

**Example LLM prompt**: "Get content of table 1 in document ITEM-12345"

---

### word_insert_paragraph

**What it does**: Inserts a new paragraph into the document.

**When to use**: Add new content to document.

**Arguments**:
- `itemId` (required): Document item ID
- `text` (required): Paragraph text
- `position` (optional): Position to insert (after paragraph ID)

**Example LLM prompt**: "Insert a new paragraph in document ITEM-12345"

---

### word_list_styles

**What it does**: Lists all available styles in a document.

**When to use**: Find available formatting.

**Arguments**:
- `itemId` (required): Document item ID

**Example LLM prompt**: "List all styles in document ITEM-12345"

---

### word_apply_style

**What it does**: Applies a style to a paragraph or range.

**When to use**: Format text consistently.

**Arguments**:
- `itemId` (required): Document item ID
- `paragraphId` (required): Paragraph ID
- `styleName` (required): Name of the style to apply

**Example LLM prompt**: "Apply Heading 1 style to paragraph 1 in document ITEM-12345"

---

## Word API Notes

- **Documents**: Word files (.docx) in OneDrive
- **Paragraphs**: Text blocks in document
- **Tables**: Tabular data structures
- **Styles**: Named formatting (Heading 1, Normal, etc.)
- **Item IDs**: OneDrive item identifiers

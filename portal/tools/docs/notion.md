# Notion Tools

Provider: `notion` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Notion API. They allow AI agents to search, read, and write pages, databases, and blocks in Notion workspaces. Notion is a collaborative workspace for notes, wikis, and databases.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Notion
- Token stored in Nango, accessed via `connection_ref`
- Scope: `Notion`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `notion_list_pages` | List pages accessible to the user | POST | /v1/search |
| `notion_get_page` | Get a specific page by ID | GET | /v1/pages/{page_id} |
| `notion_create_page` | Create a new page | POST | /v1/pages |
| `notion_list_databases` | List databases accessible to the user | POST | /v1/search |
| `notion_get_database` | Get a specific database by ID | GET | /v1/databases/{database_id} |
| `notion_create_database` | Create a new database | POST | /v1/databases |
| `notion_list_blocks` | List blocks in a page or database | GET | /v1/blocks/{block_id}/children |
| `notion_get_block` | Get a specific block by ID | GET | /v1/blocks/{block_id} |
| `notion_get_user` | Get a user by ID | GET | /v1/users/{user_id} |
| `notion_search` | Search pages and databases by title | POST | /v1/search |

---

## Tool Details

### notion_list_pages

**What it does**: Lists all pages the authenticated user has access to.

**When to use**: Browse available pages in the workspace, find pages for reading or editing.

**Arguments**:
- `filter` (optional): Filter by type: `{property: 'object', value: 'page'}`
- `page_size` (optional): Number of results (default 10, max 100)
- `start_cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all pages I have access to in Notion"

---

### notion_get_page

**What it does**: Retrieves a page by ID including its properties and content blocks.

**When to use**: Read full page content, get page metadata before editing.

**Arguments**:
- `page_id` (required): The page ID (32-character hex string)

**Example LLM prompt**: "Get the content of page abc123def456..."

---

### notion_create_page

**What it does**: Creates a new page as a child of a parent page or inside a database.

**When to use**: Add new notes, create entries in databases, build documentation.

**Arguments**:
- `parent_id` (required): Parent page ID or database ID
- `parent_type` (required): Parent type: `page_id` or `database_id`
- `properties` (optional): Page properties (title required for database parent)
- `children` (optional): Initial block content

**Example LLM prompt**: "Create a new page called 'Meeting Notes' as a child of page abc123"

---

### notion_list_databases

**What it does**: Lists all databases the authenticated user has access to.

**When to use**: Find available databases to query or add entries to.

**Arguments**:
- `filter` (optional): Filter by type: `{property: 'object', value: 'database'}`
- `page_size` (optional): Number of results (default 10, max 100)
- `start_cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all databases I have access to"

---

### notion_get_database

**What it does**: Retrieves a database by ID including its title and property schema.

**When to use**: Understand database structure before querying or creating entries.

**Arguments**:
- `database_id` (required): The database ID (32-character hex string)

**Example LLM prompt**: "Show me the schema of database abc123def456..."

---

### notion_create_database

**What it does**: Creates a new database as a child of a parent page with specified schema.

**When to use**: Set up new tracking systems, create project databases.

**Arguments**:
- `parent_id` (required): Parent page ID
- `title` (required): Database title as rich text array
- `properties` (required): Database schema with property names and types

**Example LLM prompt**: "Create a new database called 'Project Tracker' in page abc123"

---

### notion_list_blocks

**What it does**: Lists all blocks (content elements) in a page. Returns block hierarchy with children nested.

**When to use**: Read page content, see all paragraphs, headings, lists, etc.

**Arguments**:
- `block_id` (required): The block or page ID
- `page_size` (optional): Number of results (default 10, max 100)
- `start_cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all blocks in page abc123"

---

### notion_get_block

**What it does**: Retrieves a specific block by its ID including content and children.

**When to use**: Read individual block content, get specific elements.

**Arguments**:
- `block_id` (required): The block ID (32-character hex string)

**Example LLM prompt**: "Get the block with ID xyz789abc..."

---

### notion_get_user

**What it does**: Retrieves a user by their ID. Returns user profile and bot info if applicable.

**When to use**: Get user information for attributions, assignments, or contact details.

**Arguments**:
- `user_id` (required): The user ID

**Example LLM prompt**: "Get details for user usr123"

---

### notion_search

**What it does**: Searches pages and databases by title. Filter by object type and sort results.

**When to use**: Find specific pages or databases when you know the name but not the ID.

**Arguments**:
- `query` (optional): Search query text
- `filter` (optional): Filter by object type (page or database)
- `sort` (optional): Sort direction and timestamp field
- `page_size` (optional): Number of results (default 10, max 100)
- `start_cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "Search for pages with 'meeting notes' in the title"

---

## Notion API Notes

- **Page/Database IDs**: 32-character hex strings (e.g., `abc123def456ghi789jkl012mno345p`)
- **Block types**: paragraph, heading_1, heading_2, heading_3, bulleted_list_item, numbered_list_item, to_do, toggle, code, quote, callout, image, video, file, embed, bookmark, table, etc.
- **Database property types**: title, rich_text, number, select, multi_select, date, checkbox, url, email, phone_number, formula, relation, rollup, created_time, created_by, last_edited_time, last_edited_by
- **Rich text**: Arrays of text objects with `plain_text`, `href`, and inline formatting
- **Rate limits**: Implement backoff for repeated calls
- **Archived pages**: Can be restored by updating with `archived: false`
- **Search**: Uses POST to /v1/search endpoint, filter by object type to narrow results
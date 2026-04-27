# Notion (MCP) Tools

Provider: `notion-mcp` | Engine: `nango` | Auth: MCP OAuth2 via Nango

## Overview

These tools wrap the Notion MCP (Model Context Protocol) API. They allow AI agents to search pages, manage databases, create and update content, and collaborate with comments. **Requires Notion OAuth authentication via Nango MCP.**

## Authentication

**MCP OAuth2**:
- Uses Notion's MCP OAuth flow
- User authenticates via Nango Connect with Notion
- Dynamic client registration supported
- Authorization URL: `https://mcp.notion.com/authorize`
- Token URL: `https://mcp.notion.com/token`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `notion_mcp_search` | Search pages | GET | /search |
| `notion_mcp_get_page` | Get page details | GET | /pages/{page_id} |
| `notion_mcp_create_page` | Create new page | POST | /pages |
| `notion_mcp_update_page` | Update page content | PATCH | /pages/{page_id} |
| `notion_mcp_list_databases` | List databases | GET | /databases |
| `notion_mcp_query_database` | Query database | POST | /databases/{database_id}/query |
| `notion_mcp_create_database_entry` | Create database entry | POST | /pages |
| `notion_mcp_list_comments` | List comments | GET | /comments |
| `notion_mcp_add_comment` | Add comment | POST | /comments |
| `notion_mcp_get_block` | Get block content | GET | /blocks/{block_id} |

---

## Tool Details

### notion_mcp_search

**What it does**: Searches all pages and databases in Notion.

**When to use**: Find specific content, locate pages by title.

**Arguments**:
- `query` (required): Search query
- `filter` (optional): Filter by property value
- `sort` (optional): Sort options

**Example LLM prompt**: "Search for pages about quarterly reports"

---

### notion_mcp_get_page

**What it does**: Gets detailed information for a specific page.

**When to use**: View page content, metadata.

**Arguments**:
- `page_id` (required): Page ID

**Example LLM prompt**: "Get details for page ABC123"

---

### notion_mcp_create_page

**What it does**: Creates a new page in Notion.

**When to use**: Add new content, create notes.

**Arguments**:
- `parent_id` (required): Parent page or database ID
- `properties` (optional): Page properties
- `children` (optional): Page content blocks

**Example LLM prompt**: "Create a new page under Projects"

---

### notion_mcp_update_page

**What it does**: Updates an existing page's properties and content.

**When to use**: Modify page content, update properties.

**Arguments**:
- `page_id` (required): Page ID
- `properties` (optional): Properties to update

**Example LLM prompt**: "Update the status of page ABC123 to Done"

---

### notion_mcp_list_databases

**What it does**: Lists all databases accessible to the user.

**When to use**: Browse available databases, find data sources.

**Arguments**: None

**Example LLM prompt**: "List all my Notion databases"

---

### notion_mcp_query_database

**What it does**: Queries a Notion database with filters and sorts.

**When to use**: Filter data, find specific entries.

**Arguments**:
- `database_id` (required): Database ID
- `filter` (optional): Filter conditions
- `sorts` (optional): Sort conditions
- `page_size` (optional): Results per page (default 100)

**Example LLM prompt**: "Query the Tasks database for items with status In Progress"

---

### notion_mcp_create_database_entry

**What it does**: Creates a new entry in a Notion database.

**When to use**: Add new records, create tasks.

**Arguments**:
- `database_id` (required): Database ID
- `properties` (required): Entry properties

**Example LLM prompt**: "Create a new task in the Tasks database"

---

### notion_mcp_list_comments

**What it does**: Lists comments on a page or block.

**When to use**: View discussions, track feedback.

**Arguments**:
- `block_id` (optional): Block or page ID

**Example LLM prompt**: "List all comments on page ABC123"

---

### notion_mcp_add_comment

**What it does**: Adds a comment to a page or discussion.

**When to use**: Provide feedback, start discussions.

**Arguments**:
- `parent_id` (required): Page or discussion ID
- `content` (required): Comment text

**Example LLM prompt**: "Add a comment to page ABC123 saying 'Looks good!'"

---

### notion_mcp_get_block

**What it does**: Gets content of a specific block.

**When to use**: Read block content, get page structure.

**Arguments**:
- `block_id` (required): Block ID

**Example LLM prompt**: "Get the content of block XYZ789"

---

## Notion MCP Notes

- **Page IDs**: 32 character hexadecimal strings (with hyphens)
- **Database queries**: Support rich filter and sort syntax
- **Block structure**: Pages are made of nested blocks
- **Rich text**: Support for formatted text in properties and content
- **Parent references**: Pages can be children of other pages or databases

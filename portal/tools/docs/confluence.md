# Confluence Tools

Provider: `confluence` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Confluence Cloud REST API (v2). They allow AI agents to manage spaces, pages, comments, and attachments; search for content; and perform document operations. Confluence is Atlassian's team workspace for documentation and collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Confluence
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read:content`, `create:content`, `update:content`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `confluence_list_pages` | List pages in a space | GET | /wiki/api/v2/spaces/{spaceKey}/pages |
| `confluence_get_page` | Get page content | GET | /wiki/api/v2/pages/{pageId} |
| `confluence_create_page` | Create a new page | POST | /wiki/api/v2/pages |
| `confluence_update_page` | Update a page | PUT | /wiki/api/v2/pages/{pageId} |
| `confluence_list_spaces` | List all spaces | GET | /wiki/api/v2/spaces |
| `confluence_get_space` | Get space details | GET | /wiki/api/v2/spaces/{spaceKey} |
| `confluence_list_comments` | List page comments | GET | /wiki/api/v2/pages/{pageId}/comments |
| `confluence_get_comment` | Get comment details | GET | /wiki/api/v2/comments/{commentId} |
| `confluence_list_attachments` | List page attachments | GET | /wiki/api/v2/pages/{pageId}/attachments |
| `confluence_get_attachment` | Get attachment details | GET | /wiki/api/v2/attachments/{attachmentId} |

---

## Tool Details

### confluence_list_pages

**What it does**: Lists all pages in a Confluence space. Returns page metadata including title, ID, and status.

**When to use**: Browse available pages, find documentation in a space, discover content.

**Arguments**:
- `spaceKey` (required): The space key (e.g., ENG, TEAM)
- `limit` (optional): Maximum number of results
- `cursor` (optional): Pagination cursor for next page
- `status` (optional): Page status filter (`current`, `draft`, `archived`)

**Example LLM prompt**: "List all pages in the ENG space"

---

### confluence_get_page

**What it does**: Gets detailed content of a Confluence page including body in specified format.

**When to use**: Read page content, get full details for editing or analysis.

**Arguments**:
- `pageId` (required): The page ID (numeric)
- `body_format` (optional): Body format (`storage`, `editor`, `view`)

**Example LLM prompt**: "Get page 123456789 content in storage format"

---

### confluence_create_page

**What it does**: Creates a new page in a Confluence space with title and HTML body content.

**When to use**: Add new documentation, create meeting notes, capture information.

**Arguments**:
- `spaceId` (required): The space ID (numeric, not space key)
- `title` (required): Page title
- `body` (required): Page body content with `value` (HTML) and `representation`
- `parentId` (optional): Parent page ID for hierarchy

**Example LLM prompt**: "Create a new page titled 'Sprint Planning Notes' in space 123 with body content"

---

### confluence_update_page

**What it does**: Updates an existing Confluence page with new title, body, or version for optimistic locking.

**When to use**: Edit documentation, update meeting notes, revise content.

**Arguments**:
- `pageId` (required): The page ID (numeric)
- `title` (optional): New page title
- `body` (optional): New body content
- `version` (required): Current version for optimistic locking (object with `number`)

**Example LLM prompt**: "Update page 123456789 to version 3 with new title 'Updated Sprint Planning'"

---

### confluence_list_spaces

**What it does**: Lists all Confluence spaces the user has access to. Returns space metadata and details.

**When to use**: Find available spaces, see team workspaces, discover documentation.

**Arguments**:
- `limit` (optional): Maximum number of results
- `cursor` (optional): Pagination cursor for next page
- `status` (optional): Status filter (`current`, `archived`)

**Example LLM prompt**: "List all Confluence spaces I have access to"

---

### confluence_get_space

**What it does**: Gets detailed information about a Confluence space including name, key, description, and settings.

**When to use**: Understand space structure, get space metadata.

**Arguments**:
- `spaceKey` (required): The space key (e.g., ENG, TEAM)

**Example LLM prompt**: "Get details for the ENG space"

---

### confluence_list_comments

**What it does**: Lists all comments on a Confluence page. Returns comment metadata and content.

**When to use**: Read discussion on a page, see feedback and questions, review collaboration.

**Arguments**:
- `pageId` (required): The page ID (numeric)
- `limit` (optional): Maximum number of results
- `cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all comments on page 123456789"

---

### confluence_get_comment

**What it does**: Gets detailed information about a specific comment including body and metadata.

**When to use**: Read full comment content, get comment metadata.

**Arguments**:
- `commentId` (required): The comment ID (numeric)

**Example LLM prompt**: "Get comment cmnt123 details"

---

### confluence_list_attachments

**What it does**: Lists all attachments on a Confluence page. Returns attachment metadata and download URLs.

**When to use**: Find attached files, see shared documents, locate downloadable resources.

**Arguments**:
- `pageId` (required): The page ID (numeric)
- `limit` (optional): Maximum number of results
- `cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all attachments on page 123456789"

---

### confluence_get_attachment

**What it does**: Gets detailed information about a specific attachment including filename, size, and media type.

**When to use**: Get attachment metadata, find download URLs, understand file details.

**Arguments**:
- `attachmentId` (required): The attachment ID (numeric)

**Example LLM prompt**: "Get attachment details for atch123"

---

## Confluence API Notes

- **Space ID vs Space Key**: Space ID is numeric (e.g., `123`), Space Key is text (e.g., `ENG`). Use `spaceKey` for space operations, `spaceId` for page creation.
- **Page IDs**: Numeric IDs for pages (e.g., `123456789`)
- **Comment IDs**: Numeric IDs for comments
- **Attachment IDs**: Numeric IDs for attachments
- **CQL (Confluence Query Language)**: Supports `siteSearch~`, `type`, `title`, `creator`, `created`, `space`, and more
- **Body formats**: `storage` returns Atlassian Document Format, `view` returns rendered HTML, `editor` returns editor format
- **Version locking**: Update requires current version number to prevent conflicts
- **Comments**: Can be threaded with parent comment ID
- **Attachments**: This tool only lists attachments, upload requires separate endpoint
- **Pagination**: Uses cursor-based pagination for efficient traversal of large result sets
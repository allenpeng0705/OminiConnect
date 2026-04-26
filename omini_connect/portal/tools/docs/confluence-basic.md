# Confluence (Basic Auth) Tools

Provider: `confluence-basic` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Confluence Cloud API. Confluence is a team workspace for creating, sharing, and organizing knowledge. **Requires Confluence credentials (email and API token).**

## Authentication

**Nango BASIC**:
- User provides their Confluence email and API token
- Credentials passed via HTTP Basic Authentication
- Base URL: `https://${connectionConfig.subdomain}.atlassian.net`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `confluence_list_spaces` | List spaces | GET | /wiki/api/v2/spaces |
| `confluence_get_space` | Get space details | GET | /wiki/api/v2/spaces/{spaceKey} |
| `confluence_list_pages` | List pages | GET | /wiki/api/v2/spaces/{spaceKey}/pages |
| `confluence_get_page` | Get page details | GET | /wiki/api/v2/pages/{pageId} |
| `confluence_create_page` | Create a page | POST | /wiki/api/v2/pages |
| `confluence_update_page` | Update a page | PUT | /wiki/api/v2/pages/{pageId} |
| `confluence_delete_page` | Delete a page | DELETE | /wiki/api/v2/pages/{pageId} |
| `confluence_list_comments` | List comments | GET | /wiki/api/v2/pages/{pageId}/comments |
| `confluence_create_comment` | Create a comment | POST | /wiki/api/v2/pages/{pageId}/comments |
| `confluence_search_content` | Search content | GET | /wiki/api/v2/search |

---

## Tool Details

### confluence_list_spaces

**What it does**: Lists all spaces in Confluence.

**When to use**: Find spaces to browse or contribute to.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `expand` (optional): Fields to expand

**Example LLM prompt**: "List all Confluence spaces"

---

### confluence_get_space

**What it does**: Gets details of a specific space.

**When to use**: View space settings and homepage.

**Arguments**:
- `spaceKey` (required): Space key

**Example LLM prompt**: "Get space DEV details"

---

### confluence_list_pages

**What it does**: Lists all pages in a space.

**When to use**: Browse documentation in a space.

**Arguments**:
- `spaceKey` (required): Space key
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List pages in space DEV"

---

### confluence_get_page

**What it does**: Gets details of a specific page.

**When to use**: View page content and metadata.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "Get page 123456 details"

---

### confluence_create_page

**What it does**: Creates a new page in a space.

**When to use**: Add new documentation to a space.

**Arguments**:
- `spaceId` (required): Space ID
- `title` (required): Page title
- `body` (required): Page content (storage format)
- `parentId` (optional): Parent page ID for hierarchy

**Example LLM prompt**: "Create a page called 'API Documentation' in space DEV"

---

### confluence_update_page

**What it does**: Updates an existing page.

**When to use**: Modify documentation content.

**Arguments**:
- `pageId` (required): Page ID
- `title` (optional): New title
- `body` (optional): New content
- `version` (optional): Version info for optimistic locking

**Example LLM prompt**: "Update page 123456 with new content"

---

### confluence_delete_page

**What it does**: Deletes a page.

**When to use**: Remove outdated documentation.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "Delete page 123456"

---

### confluence_list_comments

**What it does**: Lists all comments on a page.

**When to use**: View discussion on a page.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "List comments on page 123456"

---

### confluence_create_comment

**What it does**: Creates a new comment on a page.

**When to use**: Add discussion to documentation.

**Arguments**:
- `pageId` (required): Page ID
- `content` (required): Comment content (HTML)

**Example LLM prompt**: "Add a comment to page 123456"

---

### confluence_search_content

**What it does**: Searches for content in Confluence.

**When to use**: Find documentation by keyword.

**Arguments**:
- `cql` (required): Confluence Query Language query
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "Search for pages containing 'API documentation'"

---

## Confluence API Notes

- **Subdomain**: Your Atlassian cloud subdomain
- **Spaces**: Top-level containers for content
- **Pages**: Individual documentation pages
- **CQL**: Confluence Query Language for search
- **Storage Format**: Confluence content uses storage format (XML)

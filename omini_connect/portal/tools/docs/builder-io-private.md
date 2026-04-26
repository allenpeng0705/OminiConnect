# Builder.io (Private) Tools

Provider: `builder-io-private` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Builder.io Content API (Private). They allow AI agents to manage pages, content models, and content entries. Builder.io is a visual CMS and content platform for building websites and applications.

## Authentication

**Nango API_KEY**:
- User provides Builder.io private API key
- Token stored in Nango, accessed via `connection_ref`
- Requires domain configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `builder_io_private_list_pages` | List pages | GET | /api/v1/pages |
| `builder_io_private_get_page` | Get page content | GET | /api/v1/pages/{pageId} |
| `builder_io_private_create_page` | Create a page | POST | /api/v1/pages |
| `builder_io_private_update_page` | Update a page | PUT | /api/v1/pages/{pageId} |
| `builder_io_private_delete_page` | Delete a page | DELETE | /api/v1/pages/{pageId} |
| `builder_io_private_list_models` | List content models | GET | /api/v1/models |
| `builder_io_private_get_model` | Get model details | GET | /api/v1/models/{modelId} |
| `builder_io_private_list_entries` | List content entries | GET | /api/v1/entries/{modelId} |
| `builder_io_private_get_entry` | Get entry details | GET | /api/v1/entries/{modelId}/{entryId} |
| `builder_io_private_create_entry` | Create content entry | POST | /api/v1/entries/{modelId} |

---

## Tool Details

### builder_io_private_list_pages

**What it does**: Lists all pages in Builder.io.

**When to use**: Browse pages, find specific content.

**Arguments**:
- `limit` (optional): Max pages to return (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all pages in Builder.io"

---

### builder_io_private_get_page

**What it does**: Gets content for a specific page.

**When to use**: View page content, check published state.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "Get page with ID abc-123"

---

### builder_io_private_create_page

**What it does**: Creates a new page in Builder.io.

**When to use**: Add new pages to the website.

**Arguments**:
- `name` (required): Page name
- `slug` (required): URL slug
- `content` (optional): Page content (JSON)

**Example LLM prompt**: "Create a page named 'About' with slug /about"

---

### builder_io_private_update_page

**What it does**: Updates an existing page.

**When to use**: Modify page content, publish changes.

**Arguments**:
- `pageId` (required): Page ID
- `name` (optional): Page name
- `content` (optional): Page content (JSON)

**Example LLM prompt**: "Update page abc-123 with new content"

---

### builder_io_private_delete_page

**What it does**: Deletes a page from Builder.io.

**When to use**: Remove unwanted pages.

**Arguments**:
- `pageId` (required): Page ID

**Example LLM prompt**: "Delete page abc-123"

---

### builder_io_private_list_models

**What it does**: Lists all content models in Builder.io.

**When to use**: View available content types.

**Arguments**: None required

**Example LLM prompt**: "List all content models"

---

### builder_io_private_get_model

**What it does**: Gets details for a specific content model.

**When to use**: Check model schema, field definitions.

**Arguments**:
- `modelId` (required): Model ID

**Example LLM prompt**: "Get model with ID m-456"

---

### builder_io_private_list_entries

**What it does**: Lists content entries for a model.

**When to use**: Browse content items, find specific entries.

**Arguments**:
- `modelId` (required): Model ID
- `limit` (optional): Max entries (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all blog posts"

---

### builder_io_private_get_entry

**What it does**: Gets details for a specific content entry.

**When to use**: View entry content, check fields.

**Arguments**:
- `modelId` (required): Model ID
- `entryId` (required): Entry ID

**Example LLM prompt**: "Get blog post with ID post-789"

---

### builder_io_private_create_entry

**What it does**: Creates a new content entry.

**When to use**: Add new content to the CMS.

**Arguments**:
- `modelId` (required): Model ID
- `data` (required): Entry data (JSON)

**Example LLM prompt**: "Create a new blog post entry"

---

## Builder.io Private API Notes

- **Private API**: Uses private key for write access
- **Pages**: Full page content with visual editor data
- **Models**: Content type schemas defining entry structure
- **Entries**: Individual content items of a specific model
- **Content**: Stored as JSON, includes visual components

# Builder.io (Public) Tools

Provider: `builder-io-public` | Engine: `nango` | Auth: API_KEY via Nango (public key - read only)

## Overview

These tools wrap the Builder.io Content API (Public). They allow AI agents to read pages, content models, and content entries. The public API is read-only and uses the public API key.

## Authentication

**Nango API_KEY**:
- User provides Builder.io public API key
- Token stored in Nango, accessed via `connection_ref`
- Read-only access to content

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `builder_io_public_list_pages` | List pages | GET | /api/v1/pages |
| `builder_io_public_get_page` | Get page content | GET | /api/v1/pages/{pageIdOrPath} |
| `builder_io_public_list_models` | List content models | GET | /api/v1/models |
| `builder_io_public_get_model` | Get model details | GET | /api/v1/models/{modelId} |
| `builder_io_public_list_entries` | List content entries | GET | /api/v1/entries/{modelId} |
| `builder_io_public_get_entry` | Get entry details | GET | /api/v1/entries/{modelId}/{entryId} |
| `builder_io_public_get_site_info` | Get site information | GET | /api/v1/site-info |
| `builder_io_public_search_content` | Search content | GET | /api/v1/search |
| `builder_io_public_get_model_json` | Get model JSON schema | GET | /api/v1/models/{modelId}/json |
| `builder_io_public_list_namespaces` | List content namespaces | GET | /api/v1/namespaces |

---

## Tool Details

### builder_io_public_list_pages

**What it does**: Lists all pages in Builder.io.

**When to use**: Browse pages, find content.

**Arguments**:
- `limit` (optional): Max pages (default 20)
- `offset` (optional): Offset (default 0)

**Example LLM prompt**: "List all pages"

---

### builder_io_public_get_page

**What it does**: Gets content for a specific page.

**When to use**: View page content.

**Arguments**:
- `pageIdOrPath` (required): Page ID or URL path

**Example LLM prompt**: "Get page /about"

---

### builder_io_public_list_models

**What it does**: Lists all content models.

**When to use**: View available content types.

**Arguments**: None required

**Example LLM prompt**: "List all content models"

---

### builder_io_public_get_model

**What it does**: Gets details for a specific model.

**When to use**: Check model schema.

**Arguments**:
- `modelId` (required): Model ID

**Example LLM prompt**: "Get model m-456"

---

### builder_io_public_list_entries

**What it does**: Lists content entries for a model.

**When to use**: Browse content items.

**Arguments**:
- `modelId` (required): Model ID
- `limit` (optional): Max entries (default 20)
- `offset` (optional): Offset (default 0)

**Example LLM prompt**: "List all blog posts"

---

### builder_io_public_get_entry

**What it does**: Gets details for a specific entry.

**When to use**: View entry content.

**Arguments**:
- `modelId` (required): Model ID
- `entryId` (required): Entry ID

**Example LLM prompt**: "Get blog post post-789"

---

### builder_io_public_get_site_info

**What it does**: Gets site information.

**When to use**: Check site configuration.

**Arguments**: None required

**Example LLM prompt**: "Get site info"

---

### builder_io_public_search_content

**What it does**: Searches across Builder.io content.

**When to use**: Find specific content.

**Arguments**:
- `query` (required): Search query
- `model` (optional): Filter by model
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "Search for 'privacy policy'"

---

### builder_io_public_get_model_json

**What it does**: Gets JSON schema for a model.

**When to use**: Understand model structure.

**Arguments**:
- `modelId` (required): Model ID

**Example LLM prompt**: "Get JSON schema for model m-456"

---

### builder_io_public_list_namespaces

**What it does**: Lists content namespaces.

**When to use**: View content organization.

**Arguments**: None required

**Example LLM prompt**: "List all namespaces"

---

## Builder.io Public API Notes

- **Public Key**: Read-only access to content
- **No Write Operations**: Cannot create/update/delete content
- **Pages**: Can be queried by ID or URL path
- **Models**: Define content structure
- **Entries**: Individual content items

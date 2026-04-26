# Webflow Tools

Provider: `webflow` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Webflow API. They allow AI agents to manage sites, CMS collections, items, and assets on behalf of the authenticated user.

## Authentication

**Nango (required)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `sites:read`, `collections:read`, `collections:write`, `assets:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `webflow_list_sites` | List all sites | GET | /sites |
| `webflow_get_site` | Get site details | GET | /sites/{site_id} |
| `webflow_list_collections` | List CMS collections in a site | GET | /sites/{site_id}/collections |
| `webflow_get_collection` | Get collection schema and fields | GET | /collections/{collection_id} |
| `webflow_list_items` | List items in a collection | GET | /collections/{collection_id}/items |
| `webflow_get_item` | Get a specific item | GET | /collections/{collection_id}/items/{item_id} |
| `webflow_create_item` | Create a new item in a collection | POST | /collections/{collection_id}/items |
| `webflow_update_item` | Update an existing item | PATCH | /collections/{collection_id}/items/{item_id} |
| `webflow_list_assets` | List assets in a site | GET | /sites/{site_id}/assets |
| `webflow_get_asset` | Get asset details | GET | /assets/{asset_id} |

---

## Tool Details

### webflow_list_sites

**What it does**: List all Webflow sites accessible to the authenticated account.

**When to use**: Before managing collections or content, know which sites are available.

**Arguments**: None

**Example LLM prompt**: "List all my Webflow sites"

---

### webflow_get_site

**What it does**: Get detailed information about a specific site including domains and publish status.

**When to use**: Check site configuration before managing content.

**Arguments**:
- `site_id` (required): Site ID

**Example LLM prompt**: "Get details for site abc123"

---

### webflow_list_collections

**What it does**: List all CMS collections in a site with their schemas.

**When to use**: Discover what content types exist before creating or querying items.

**Arguments**:
- `site_id` (required): Site ID

**Example LLM prompt**: "What CMS collections are in site abc123?"

---

### webflow_get_collection

**What it does**: Get the schema and field definitions for a specific collection.

**When to use**: Understand field names and types before creating items.

**Arguments**:
- `collection_id` (required): Collection ID

**Example LLM prompt**: "What's the schema for the blog-posts collection?"

---

### webflow_list_items

**What it does**: List items in a CMS collection with optional filtering and pagination.

**When to use**: Browse content items. Filter by field values to find specific content.

**Arguments**:
- `collection_id` (required): Collection ID
- `limit` (optional): Number of results per page (default 100)
- `offset` (optional): Offset for pagination
- `order` (optional): Field to order by

**Example LLM prompt**: "Show me all published blog posts"

---

### webflow_get_item

**What it does**: Get a specific item from a CMS collection by ID.

**When to use**: Read full item details before editing or publishing.

**Arguments**:
- `collection_id` (required): Collection ID
- `item_id` (required): Item ID

**Example LLM prompt**: "Get the item with ID xyz789 from the blog-posts collection"

---

### webflow_create_item

**What it does**: Create a new item in a CMS collection.

**When to use**: Add new content items programmatically.

**Arguments**:
- `collection_id` (required): Collection ID
- `fields` (required): Item field values as key-value pairs
- `live` (optional): Publish immediately (default false)

**Example LLM prompt**: "Create a new blog post titled 'Hello World' in the blog-posts collection"

---

### webflow_update_item

**What it does**: Update an existing item in a CMS collection with new field values.

**When to use**: Modify content items. Use PATCH for partial updates.

**Arguments**:
- `collection_id` (required): Collection ID
- `item_id` (required): Item ID
- `fields` (required): Updated field values
- `live` (optional): Publish immediately

**Example LLM prompt**: "Update the blog post xyz789 to change the status to published"

---

### webflow_list_assets

**What it does**: List all assets (images, documents) in a site with URLs and metadata.

**When to use**: Find existing media to use in content items.

**Arguments**:
- `site_id` (required): Site ID
- `limit` (optional): Number of results per page (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all images in the site assets"

---

### webflow_get_asset

**What it does**: Get details of a specific asset including URL, file size, and type.

**When to use**: Get asset details before using in content.

**Arguments**:
- `asset_id` (required): Asset ID

**Example LLM prompt**: "Get details for asset abc456"

---

## Webflow API Reference

These tools use the Webflow API. See official docs for full details:
- https://developers.webflow.com/
- Rate limits: 60 requests/minute for most endpoints
- Pagination: Use `limit` and `offset` parameters
- All dates: ISO 8601 format

# Contentful Tools

Provider: `contentful` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Contentful Management API. They allow AI agents to manage spaces, content entries, assets, and locales on behalf of the authenticated user.

## Authentication

**Nango (required)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `content_management.read`, `content_management`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `contentful_list_spaces` | List all spaces accessible to the user | GET | /spaces |
| `contentful_get_space` | Get details of a specific space | GET | /spaces/{space_id} |
| `contentful_list_entries` | List entries in a space | GET | /spaces/{space_id}/entries |
| `contentful_get_entry` | Get a specific entry by ID | GET | /spaces/{space_id}/entries/{entry_id} |
| `contentful_create_entry` | Create a new entry in a content type | POST | /spaces/{space_id}/entries |
| `contentful_list_assets` | List assets in a space | GET | /spaces/{space_id}/assets |
| `contentful_get_asset` | Get a specific asset by ID | GET | /spaces/{space_id}/assets/{asset_id} |
| `contentful_upload_asset` | Upload a media asset to a space | POST | /spaces/{space_id}/assets |
| `contentful_list_locales` | List locales in a space | GET | /spaces/{space_id}/locales |
| `contentful_get_locale` | Get a specific locale | GET | /spaces/{space_id}/locales/{locale_id} |

---

## Tool Details

### contentful_list_spaces

**What it does**: Returns a paginated list of spaces the authenticated user has access to.

**When to use**: Before creating entries or managing content, an agent should list spaces to know what's available.

**Arguments**:
- `limit` (optional): Number of results per page (default 100, max 1000)
- `skip` (optional): Number of results to skip for pagination

**Example LLM prompt**: "List all my Contentful spaces"

---

### contentful_get_space

**What it does**: Get details of a specific space including name, locales, and settings.

**When to use**: Understand space configuration before managing content within it.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "Get details for the my-project space"

---

### contentful_list_entries

**What it does**: Lists entries in a space. Optionally filter by content type, tags, or status.

**When to use**: Browse and search content entries. Filter by content type to find specific types of content.

**Arguments**:
- `space_id` (required): Space ID
- `content_type` (optional): Filter by content type ID
- `limit` (optional): Number of results per page (default 100)
- `skip` (optional): Number of results to skip
- `order` (optional): Field to order by (e.g., -sys.createdAt)
- `select` (optional): Fields to include in response

**Example LLM prompt**: "Show me all blog posts in the production space"

---

### contentful_get_entry

**What it does**: Get a specific entry by ID including fields, metadata, and version info.

**When to use**: Read the full content of an entry before editing or publishing it.

**Arguments**:
- `space_id` (required): Space ID
- `entry_id` (required): Entry ID
- `version` (optional): Expected version for optimistic locking

**Example LLM prompt**: "Get entry abc123 from the blog space"

---

### contentful_create_entry

**What it does**: Creates a new entry in a specified content type.

**When to use**: Create new content programmatically. Include all required fields for the content type.

**Arguments**:
- `space_id` (required): Space ID
- `content_type_id` (required): Content type ID
- `fields` (required): Entry fields as key-value pairs
- `locale` (optional): Locale code (default en-US)

**Example LLM prompt**: "Create a new blog post titled 'Getting Started with AI' in the production space"

---

### contentful_list_assets

**What it does**: Lists assets (images, videos, files) in a space with metadata.

**When to use**: Find and browse media assets before inserting them into content.

**Arguments**:
- `space_id` (required): Space ID
- `limit` (optional): Number of results per page
- `skip` (optional): Number of results to skip
- `order` (optional): Field to order by

**Example LLM prompt**: "List all images in the assets folder"

---

### contentful_get_asset

**What it does**: Get a specific asset including file metadata, URL, and processing status.

**When to use**: Check asset details, get the CDN URL, or verify processing status.

**Arguments**:
- `space_id` (required): Space ID
- `asset_id` (required): Asset ID

**Example LLM prompt**: "Get the hero-image asset details"

---

### contentful_upload_asset

**What it does**: Upload a media asset to a space. After upload, asset must be processed and published.

**When to use**: Upload new images, videos, or documents to use in content entries.

**Arguments**:
- `space_id` (required): Space ID
- `file` (required): Base64 encoded file content
- `filename` (required): Original filename
- `content_type` (required): MIME type (e.g., image/png, video/mp4)
- `title` (optional): Asset title
- `description` (optional): Asset description

**Example LLM prompt**: "Upload a new logo image to the assets"

---

### contentful_list_locales

**What it does**: Lists all locales configured in a space with their fallback configuration.

**When to use**: Before creating or updating entries in multiple locales.

**Arguments**:
- `space_id` (required): Space ID
- `limit` (optional): Number of results per page

**Example LLM prompt**: "What locales are configured in the production space?"

---

### contentful_get_locale

**What it does**: Get details of a specific locale including code, name, and fallback settings.

**When to use**: Check locale configuration before localizing content.

**Arguments**:
- `space_id` (required): Space ID
- `locale_id` (required): Locale ID

**Example LLM prompt**: "Get details for the fr-FR locale"

---

## Contentful API Reference

These tools use the Contentful Management API. See official docs for full details:
- https://www.contentful.com/developers/docs/references/content-management-api/
- Rate limits: Depends on plan (Enterprise has higher limits)
- Pagination: Use `limit` and `skip` parameters
- All dates: ISO 8601 format

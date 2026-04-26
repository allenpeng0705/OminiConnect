# Strapi Tools

Provider: `strapi` | Engine: `nango` | Auth: API Token (via Nango)

## Overview

These tools wrap the Strapi REST API. They allow AI agents to manage collection types, items, media files, and users in a Strapi instance.

## Authentication

**Nango (required)**:
- User authenticates via Nango Connect with API token
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `api` (full access)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `strapi_list_collections` | List all collection types | GET | /content-types |
| `strapi_get_collection` | Get collection type definition | GET | /content-types/{collection_type} |
| `strapi_create_item` | Create a new item in a collection | POST | /{collection_type} |
| `strapi_update_item` | Update an existing item | PUT | /{collection_type}/{id} |
| `strapi_delete_item` | Delete an item | DELETE | /{collection_type}/{id} |
| `strapi_list_media` | List uploaded media files | GET | /upload/files |
| `strapi_upload_media` | Upload a media file | POST | /upload |
| `strapi_list_users` | List users | GET | /users |
| `strapi_get_user` | Get user details | GET | /users/{id} |
| `strapi_list_roles` | List user roles | GET | /users-permissions/roles |

---

## Tool Details

### strapi_list_collections

**What it does**: Returns all collection types in the Strapi instance with their schemas.

**When to use**: Discover what content types exist before creating or querying items.

**Arguments**: None

**Example LLM prompt**: "What collections are available in this Strapi instance?"

---

### strapi_get_collection

**What it does**: Get the schema definition for a specific collection type.

**When to use**: Understand field names, types, and validation rules before creating items.

**Arguments**:
- `collection_type` (required): Collection type name (e.g., articles, products)

**Example LLM prompt**: "What's the schema for the articles collection?"

---

### strapi_create_item

**What it does**: Create a new item in a specified collection with the provided field values.

**When to use**: Add new content items programmatically. Ensure all required fields are provided.

**Arguments**:
- `collection_type` (required): Collection type name
- `data` (required): Item field values as key-value pairs

**Example LLM prompt**: "Create a new article titled 'Hello World' with content 'This is my first post'"

---

### strapi_update_item

**What it does**: Update an existing item with new values. Partial updates supported.

**When to use**: Modify content items. Provide only the fields to update.

**Arguments**:
- `collection_type` (required): Collection type name
- `id` (required): Item ID
- `data` (required): Updated field values

**Example LLM prompt**: "Update article 42 to change the title to 'Updated Title'"

---

### strapi_delete_item

**What it does**: Delete an item from a collection by ID. This action is irreversible.

**When to use**: Remove unwanted content items.

**Arguments**:
- `collection_type` (required): Collection type name
- `id` (required): Item ID

**Example LLM prompt**: "Delete the article with ID 42"

---

### strapi_list_media

**What it does**: List all uploaded media files with metadata including name, size, and URL.

**When to use**: Find existing images, videos, or documents to use in content.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Number of results per page (default 25)

**Example LLM prompt**: "List all uploaded images"

---

### strapi_upload_media

**What it does**: Upload a media file to the Strapi media library.

**When to use**: Add new images, videos, or documents to use in content.

**Arguments**:
- `file` (required): Base64 encoded file content
- `filename` (required): Original filename
- `mime` (required): MIME type (e.g., image/jpeg)
- `folder` (optional): Folder path in media library

**Example LLM prompt**: "Upload a new logo image called logo.png"

---

### strapi_list_users

**What it does**: List all users with details including email, username, and roles.

**When to use**: Find users, check user roles, or manage permissions.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Results per page (default 25)
- `sort` (optional): Sort field (e.g., username:asc)

**Example LLM prompt**: "List all users in the system"

---

### strapi_get_user

**What it does**: Get details of a specific user including email, username, roles, and blocked status.

**When to use**: Check user details before updating or assigning roles.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user ID 5"

---

### strapi_list_roles

**What it does**: List all user roles with their permissions.

**When to use**: Understand role-based access control before assigning users to roles.

**Arguments**: None

**Example LLM prompt**: "What roles are available and what are their permissions?"

---

## Strapi API Reference

These tools use the Strapi REST API. See official docs for full details:
- https://docs.strapi.io/developer-docs/latest/api/rest-api.html
- Rate limits: None (depends on server configuration)
- Pagination: Use `page` and `pageSize` parameters
- All dates: ISO 8601 format

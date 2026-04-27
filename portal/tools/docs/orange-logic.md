# Orange Logic Tools

Provider: `orange-logic` | Engine: `nango` | Auth: Two-Step (Client Credentials) via Nango

## Overview

These tools wrap the Orange Logic Digital Asset Management API. They allow AI agents to manage digital assets, collections, and metadata. **Requires Orange Logic Client Credentials authentication.**

## Authentication

**Two-Step (Client Credentials)**:
- Uses OAuth2 Client Credentials flow
- Client ID and Secret passed via Nango
- Token URL: `https://{serverUrl}/webapi/security/clientcredentialsauthentication/authenticate_46H_v1`
- Base URL: `https://{serverUrl}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `orange_list_assets` | List digital assets | GET | /webapi/assets |
| `orange_get_asset` | Get asset details | GET | /webapi/assets/{id} |
| `orange_upload_asset` | Upload asset | POST | /webapi/assets |
| `orange_update_asset` | Update asset metadata | PUT | /webapi/assets/{id} |
| `orange_delete_asset` | Delete asset | DELETE | /webapi/assets/{id} |
| `orange_list_collections` | List collections | GET | /webapi/collections |
| `orange_create_collection` | Create collection | POST | /webapi/collections |
| `orange_add_to_collection` | Add asset to collection | POST | /webapi/collections/{collectionId}/assets |
| `orange_search_assets` | Search assets | GET | /webapi/assets/search |
| `orange_get_download_url` | Get download URL | GET | /webapi/assets/{id}/download |

---

## Tool Details

### orange_list_assets

**What it does**: Lists all digital assets in Orange Logic DAM.

**When to use**: Browse asset library, find media files.

**Arguments**:
- `limit` (optional): Number of assets (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all images in the DAM"

---

### orange_get_asset

**What it does**: Gets detailed information for a specific asset.

**When to use**: View asset metadata, check file details.

**Arguments**:
- `id` (required): Asset ID

**Example LLM prompt**: "Get details for asset 12345"

---

### orange_upload_asset

**What it does**: Uploads a new digital asset.

**When to use**: Add new media, import files.

**Arguments**:
- `name` (required): Asset name
- `file` (required): File content (base64-encoded)
- `collection_id` (optional): Collection ID

**Example LLM prompt**: "Upload a new image called 'hero-banner.jpg'"

---

### orange_update_asset

**What it does**: Updates asset metadata.

**When to use**: Modify asset information, update tags.

**Arguments**:
- `id` (required): Asset ID
- `name` (optional): Asset name
- `metadata` (optional): Asset metadata

**Example LLM prompt**: "Update asset 12345 metadata with new tags"

---

### orange_delete_asset

**What it does**: Deletes a digital asset.

**When to use**: Remove old files, clean up library.

**Arguments**:
- `id` (required): Asset ID

**Example LLM prompt**: "Delete asset 12345"

---

### orange_list_collections

**What it does**: Lists all collections in Orange Logic DAM.

**When to use**: Browse collections, organize assets.

**Arguments**: None

**Example LLM prompt**: "List all collections"

---

### orange_create_collection

**What it does**: Creates a new collection.

**When to use**: Organize assets, create folders.

**Arguments**:
- `name` (required): Collection name
- `description` (optional): Collection description

**Example LLM prompt**: "Create a collection called 'Q4 Marketing'"

---

### orange_add_to_collection

**What it does**: Adds an asset to a collection.

**When to use**: Organize assets, categorize media.

**Arguments**:
- `collectionId` (required): Collection ID
- `assetId` (required): Asset ID

**Example LLM prompt**: "Add asset 12345 to collection 100"

---

### orange_search_assets

**What it does**: Searches for digital assets.

**When to use**: Find assets by name, tag, or keyword.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for assets with 'product' in the name"

---

### orange_get_download_url

**What it does**: Gets a download URL for an asset.

**When to use**: Download files, export media.

**Arguments**:
- `id` (required): Asset ID

**Example LLM prompt**: "Get download URL for asset 12345"

---

## Orange Logic Notes

- **Server URL**: Your Orange Logic DAM instance
- **Client credentials**: OAuth2 client ID and secret
- **Digital assets**: Images, videos, documents, etc.
- **Collections**: Organize assets into folders/categories
- **Metadata**: Custom fields for asset description

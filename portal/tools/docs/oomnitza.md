# Oomnitza Tools

Provider: `oomnitza` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Oomnitza asset management API. They allow AI agents to manage assets, users, assignments, and generate reports. **Requires Oomnitza API key authentication.**

## Authentication

**API Key**:
- User provides Oomnitza API key
- Key passed via `Authorization2` header
- Base URL: `https://{subdomain}.oomnitza.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `oomnitza_list_assets` | List assets | GET | /api/v3/assets |
| `oomnitza_get_asset` | Get asset details | GET | /api/v3/assets/{id} |
| `oomnitza_create_asset` | Create asset | POST | /api/v3/assets |
| `oomnitza_update_asset` | Update asset | PUT | /api/v3/assets/{id} |
| `oomnitza_list_users` | List users | GET | /api/v3/users |
| `oomnitza_get_user` | Get user details | GET | /api/v3/users/{id} |
| `oomnitza_assign_asset` | Assign asset to user | POST | /api/v3/assets/{id}/assign |
| `oomnitza_list_locations` | List locations | GET | /api/v3/locations |
| `oomnitza_get_reports` | Get reports | GET | /api/v3/reports |
| `oomnitza_search` | Search records | GET | /api/v3/search |

---

## Tool Details

### oomnitza_list_assets

**What it does**: Lists all assets in Oomnitza.

**When to use**: Browse asset inventory, find equipment.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Items per page (default 20)

**Example LLM prompt**: "List all laptops in inventory"

---

### oomnitza_get_asset

**What it does**: Gets detailed information for a specific asset.

**When to use**: View asset details, check assignment status.

**Arguments**:
- `id` (required): Asset ID

**Example LLM prompt**: "Get details for asset ABC123"

---

### oomnitza_create_asset

**What it does**: Creates a new asset in Oomnitza.

**When to use**: Add new equipment, register assets.

**Arguments**:
- `name` (required): Asset name
- `type` (required): Asset type
- `serial_number` (optional): Serial number

**Example LLM prompt**: "Create a new laptop asset called MacBook Pro 16"

---

### oomnitza_update_asset

**What it does**: Updates an existing asset.

**When to use**: Modify asset details, change status.

**Arguments**:
- `id` (required): Asset ID
- `status` (optional): Asset status

**Example LLM prompt**: "Update asset ABC123 status to retired"

---

### oomnitza_list_users

**What it does**: Lists all users in Oomnitza.

**When to use**: Browse user directory, find asset holders.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all Oomnitza users"

---

### oomnitza_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user profile, assigned assets.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### oomnitza_assign_asset

**What it does**: Assigns an asset to a user.

**When to use**: Issue equipment, track assignments.

**Arguments**:
- `id` (required): Asset ID
- `user_id` (required): User ID

**Example LLM prompt**: "Assign asset ABC123 to user 456"

---

### oomnitza_list_locations

**What it does**: Lists all locations in Oomnitza.

**When to use**: Browse locations, find asset locations.

**Arguments**: None

**Example LLM prompt**: "List all locations"

---

### oomnitza_get_reports

**What it does**: Gets reports from Oomnitza.

**When to use**: Generate analytics, export data.

**Arguments**: None

**Example LLM prompt**: "Get available reports"

---

### oomnitza_search

**What it does**: Searches for assets and records.

**When to use**: Find equipment, locate records.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for 'MacBook' in assets"

---

## Oomnitza Notes

- **Subdomain**: Your Oomnitza instance subdomain
- **Asset types**: Laptops, phones, monitors, etc.
- **Assignments**: Track who has which equipment
- **Locations**: Physical or logical locations
- **Reports**: Asset analytics and summaries

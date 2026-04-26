# Canva Tools

Provider: `canva` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Canva Connect API. They allow AI agents to interact with designs, folders, assets, and users in Canva's design platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `design:content:read`, `design:content:write`, `folder:read`, `folder:write`, `asset:read`, `asset:write`, `team:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `canva_list_designs` | List designs | GET | /v1/designs |
| `canva_get_design` | Get a specific design | GET | /v1/designs/{designId} |
| `canva_create_design` | Create a new design | POST | /v1/designs |
| `canva_list_folders` | List folders | GET | /v1/folders |
| `canva_get_folder` | Get a specific folder | GET | /v1/folders/{folderId} |
| `canva_create_folder` | Create a new folder | POST | /v1/folders |
| `canva_list_assets` | List assets | GET | /v1/assets |
| `canva_upload_asset` | Upload an asset | POST | /v1/assets |
| `canva_list_users` | List users in team | GET | /v1/users |
| `canva_get_user` | Get a specific user | GET | /v1/users/{userId} |

---

## Tool Details

### canva_list_designs

**What it does**: Lists all designs in the user's account or within a specific folder. Designs are visual content created in Canva.

**When to use**: Browse designs or find a specific design to work with.

**Arguments**:
- `folder_id` (optional): Filter by folder ID
- `continuation` (optional): Pagination continuation token
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all my Canva designs"

---

### canva_get_design

**What it does**: Returns details of a specific design including its title, thumbnail, creation date, and edit URL.

**When to use**: Get design metadata before exporting or modifying it.

**Arguments**:
- `designId` (required): Design ID

**Example LLM prompt**: "Get details for design DAFake123456789"

---

### canva_create_design

**What it does**: Creates a new design in Canva. Specify the design type (e.g., social media post, presentation, document) and optional title.

**When to use**: Create new visual content for a campaign, presentation, or project.

**Arguments**:
- `title` (optional): Design title
- `design_type` (required): Design type specification (object with type property)
- `folder_id` (optional): Folder ID to create the design in

**Example LLM prompt**: "Create a new Instagram post design"

---

### canva_list_folders

**What it does**: Lists all folders in the user's account or within a specific parent folder. Use folders to organize designs.

**When to use**: Browse folder structure to find or organize designs.

**Arguments**:
- `folder_id` (optional): Parent folder ID to list subfolders from
- `continuation` (optional): Pagination continuation token
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all my Canva folders"

---

### canva_get_folder

**What it does**: Returns details of a specific folder including its name, parent folder, and creation date.

**When to use**: Get folder information before listing its contents.

**Arguments**:
- `folderId` (required): Folder ID

**Example LLM prompt**: "Get details for folder FAKEFolder123"

---

### canva_create_folder

**What it does**: Creates a new folder to organize designs. Specify the folder name and optional parent folder.

**When to use**: Create a new organizational structure for designs.

**Arguments**:
- `name` (required): Folder name
- `parent_folder_id` (optional): Parent folder ID to create folder within

**Example LLM prompt**: "Create a new folder called 'Q2 Campaign'"

---

### canva_list_assets

**What it does**: Lists all assets in the user's account. Assets include images, videos, and other files uploaded to Canva.

**When to use**: Browse uploaded assets to find images or files for designs.

**Arguments**:
- `continuation` (optional): Pagination continuation token
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all my uploaded assets"

---

### canva_upload_asset

**What it does**: Uploads a new asset to Canva. The asset can be an image, video, or other supported file type.

**When to use**: Add new images or files to use in designs.

**Arguments**:
- `name` (required): Asset name
- `content` (required): Base64 encoded file content or URL to asset
- `mime_type` (required): MIME type of the asset (e.g., image/png, video/mp4)
- `folder_id` (optional): Folder ID to upload the asset to

**Example LLM prompt**: "Upload a new logo image to my assets"

---

### canva_list_users

**What it does**: Lists all users in a Canva team. Returns user information including name, email, and role.

**When to use**: See team members or find specific users.

**Arguments**:
- `continuation` (optional): Pagination continuation token
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all users in our Canva team"

---

### canva_get_user

**What it does**: Returns details of a specific user including their name, email, avatar, and team role.

**When to use**: Retrieve user information for personalization or permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user alice@example.com"

---

## Canva API Reference

These tools use the Canva Connect API. See official docs for full details:
- https://www.canva.com/developers/docs/connect/
- Rate limits: Vary by plan
- Pagination: Use continuation token for efficient traversal
- All dates: ISO 8601 format

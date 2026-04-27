# Autodesk Tools

Provider: `autodesk` | Engine: `nango` | Auth: OAuth2 (via Nango)

## Overview

These tools wrap the Autodesk Construction Cloud API. They allow AI agents to interact with projects, users, files, folders, and items on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `account:read`, `account:write`, `bucket:create`, `bucket:delete`, `bucket:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `autodesk_list_projects` | List projects in the account | GET | /construction/projects |
| `autodesk_get_project` | Get details of a specific project | GET | /construction/projects/{project_id} |
| `autodesk_list_users` | List users in a project | GET | /construction/projects/{project_id}/users |
| `autodesk_get_user` | Get details of a specific user | GET | /construction/projects/{project_id}/users/{user_id} |
| `autodesk_list_items` | List items in a project | GET | /construction/projects/{project_id}/items |
| `autodesk_get_item` | Get details of a specific item | GET | /construction/projects/{project_id}/items/{item_id} |
| `autodesk_list_folders` | List folders in a project | GET | /construction/projects/{project_id}/folders |
| `autodesk_get_folder` | Get details of a specific folder | GET | /construction/projects/{project_id}/folders/{folder_id} |
| `autodesk_upload_file` | Upload a file to a project | POST | /construction/projects/{project_id}/storage |
| `autodesk_get_project_details` | Get detailed info about a project | GET | /construction/projects/{project_id}/details |

---

## Tool Details

### autodesk_list_projects

**What it does**: Returns a paginated list of projects in the Autodesk account.

**When to use**: List all accessible projects before creating items, folders, or managing users.

**Arguments**:
- `limit` (optional, max 100): default 50
- `offset` (optional): default 0

**Example LLM prompt**: "List all projects in my Autodesk account"

---

### autodesk_get_project

**What it does**: Get details of a specific project including name, description, status, and creation date.

**When to use**: Understand project configuration or get context before making changes.

**Arguments**:
- `project_id` (required): Unique identifier of the project

**Example LLM prompt**: "Get details for project abc123"

---

### autodesk_list_users

**What it does**: List all users in a specific project with their email, name, and role.

**When to use**: Find project members or check permissions before sharing files.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `limit` (optional, max 100): default 50
- `offset` (optional): default 0

**Example LLM prompt**: "List all users in project abc123"

---

### autodesk_get_user

**What it does**: Get details of a specific user including email, name, and project permissions.

**When to use**: Check user details or verify user access before operations.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `user_id` (required): Unique identifier of the user

**Example LLM prompt**: "Get details for user john@example.com in project abc123"

---

### autodesk_list_items

**What it does**: List all items (files, models, documents) in a project.

**When to use**: Browse project contents or find specific files.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `folder_id` (optional): Filter by folder ID
- `limit` (optional, max 100): default 50
- `offset` (optional): default 0

**Example LLM prompt**: "List all items in project abc123"

---

### autodesk_get_item

**What it does**: Get details of a specific item including name, type, version history, and storage location.

**When to use**: Get file metadata or check version information.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `item_id` (required): Unique identifier of the item

**Example LLM prompt**: "Get details for item xyz456 in project abc123"

---

### autodesk_list_folders

**What it does**: List all folders in a project including name and contents count.

**When to use**: Navigate project structure or find folder locations.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `parent_folder_id` (optional): Filter by parent folder
- `limit` (optional, max 100): default 50
- `offset` (optional): default 0

**Example LLM prompt**: "List all folders in project abc123"

---

### autodesk_get_folder

**What it does**: Get details of a specific folder including name, parent, and contents.

**When to use**: Get folder metadata or check contents before uploading.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `folder_id` (required): Unique identifier of the folder

**Example LLM prompt**: "Get details for folder def789 in project abc123"

---

### autodesk_upload_file

**What it does**: Upload a file to a project folder.

**When to use**: Add design files, documents, or other assets to a project.

**Arguments**:
- `project_id` (required): Unique identifier of the project
- `folder_id` (optional): Destination folder ID
- `file_name` (required): Name of the file to upload
- `content_type` (optional): MIME type, default `application/octet-stream`

**Example LLM prompt**: "Upload the floorplan.dwg file to project abc123"

---

### autodesk_get_project_details

**What it does**: Get detailed information about a project including statistics, member count, and storage usage.

**When to use**: Get project analytics or check storage consumption.

**Arguments**:
- `project_id` (required): Unique identifier of the project

**Example LLM prompt**: "Get detailed stats for project abc123"

---

## Autodesk API Reference

These tools use the Autodesk Construction Cloud API. See official docs for full details:
- https://developer.autodesk.com/en/docs/construction/overview
- Rate limits: Vary by plan
- Pagination: Use `limit` and `offset` parameters
- All dates: ISO 8601 format

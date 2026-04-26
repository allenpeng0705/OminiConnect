# OneDrive Tools

Provider: `onedrive` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Microsoft Graph API for OneDrive. They allow AI agents to browse files and folders, upload and download content, manage permissions, share files, and work with multiple drives. **Requires Microsoft/OneDrive OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Microsoft/OneDrive
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `Files.Read`, `Files.ReadWrite`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onedrive_list_files` | List files in folder | GET | /v1.0/me/drive/root/children |
| `onedrive_get_file` | Get file metadata | GET | /v1.0/me/drive/items/{item_id} |
| `onedrive_upload_file` | Upload a file | PUT | /v1.0/me/drive/items/{parent_id}:/{name}:/content |
| `onedrive_download_file` | Download file content | GET | /v1.0/me/drive/items/{item_id}/content |
| `onedrive_create_folder` | Create a folder | POST | /v1.0/me/drive/items/{parent_id}/children |
| `onedrive_delete_file` | Delete a file | DELETE | /v1.0/me/drive/items/{item_id} |
| `onedrive_list_permissions` | List file permissions | GET | /v1.0/me/drive/items/{item_id}/permissions |
| `onedrive_get_permission` | Get permission details | GET | /v1.0/me/drive/items/{item_id}/permissions/{permission_id} |
| `onedrive_share_file` | Share a file | POST | /v1.0/me/drive/items/{item_id}/invite |
| `onedrive_list_drives` | List available drives | GET | /v1.0/me/drives |

---

## Tool Details

### onedrive_list_files

**What it does**: Lists files and folders in a OneDrive folder.

**When to use**: Browse folders, find files, navigate OneDrive structure.

**Arguments**:
- `folder_id` (optional): Folder ID (use `root` for root, default: `root`)
- `limit` (optional): Max results (default 20, max 200)

**Example LLM prompt**: "List all files in my OneDrive root folder"

---

### onedrive_get_file

**What it does**: Gets metadata for a OneDrive file or folder.

**When to use**: Check file details, verify file exists.

**Arguments**:
- `item_id` (required): Item ID (file or folder ID)

**Example LLM prompt**: "Get info about the file with ID abc123xyz"

---

### onedrive_upload_file

**What it does**: Creates or updates a file with content in OneDrive.

**When to use**: Save documents, upload content to OneDrive.

**Arguments**:
- `name` (required): File name
- `parent_id` (optional): Parent folder ID or `root` (default: `root`)
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload a file called 'report.pdf' to my OneDrive root"

---

### onedrive_download_file

**What it does**: Downloads file content from OneDrive.

**When to use**: Read file content, retrieve documents.

**Arguments**:
- `item_id` (required): Item ID

**Example LLM prompt**: "Download the file with ID abc123xyz"

---

### onedrive_create_folder

**What it does**: Creates a new folder in OneDrive.

**When to use**: Organize files, create project directories.

**Arguments**:
- `name` (required): Folder name
- `parent_id` (required): Parent folder ID or `root`

**Example LLM prompt**: "Create a folder called 'Project Files' in my OneDrive root"

---

### onedrive_delete_file

**What it does**: Deletes a file or folder from OneDrive.

**When to use**: Remove unwanted files, clean up storage.

**Arguments**:
- `item_id` (required): Item ID to delete

**Example LLM prompt**: "Delete the file with ID abc123xyz"

---

### onedrive_list_permissions

**What it does**: Lists all permissions on a file or folder.

**When to use**: See who has access, audit sharing settings.

**Arguments**:
- `item_id` (required): Item ID

**Example LLM prompt**: "List all permissions on the file with ID abc123xyz"

---

### onedrive_get_permission

**What it does**: Gets details of a specific permission on a file or folder.

**When to use**: Check specific permission details, verify access level.

**Arguments**:
- `item_id` (required): Item ID
- `permission_id` (required): Permission ID

**Example LLM prompt**: "Get details about permission ID 456 on file abc123xyz"

---

### onedrive_share_file

**What it does**: Creates a sharing link or grants access to a file or folder.

**When to use**: Share files with others, create collaboration links.

**Arguments**:
- `item_id` (required): Item ID to share
- `recipients` (required): Array of recipient emails
- `roles` (required): Roles to grant (`read`, `write`)
- `message` (optional): Message to recipients

**Example LLM prompt**: "Share the file with ID abc123xyz with john@example.com with read access"

---

### onedrive_list_drives

**What it does**: Lists all drives (OneDrive instances) available to the user.

**When to use**: See available drives, switch between personal and business OneDrive.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all my OneDrive drives"

---

## OneDrive API Notes

- **Item ID format**: Long alphanumeric strings (e.g., `A66D7C43B2D8...`)
- **Root folder ID**: Use `root` as the folder ID for the root
- **Content upload**: Files must be base64-encoded
- **Conflict behavior**: By default, uploads fail if a file with the same name exists
- **Multiple drives**: Users may have multiple OneDrive instances (personal, business)
- **Rate limits**: Apply to API calls — implement backoff for heavy use

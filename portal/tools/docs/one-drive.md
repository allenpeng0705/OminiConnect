# OneDrive for Business Tools

Provider: `one-drive` | Engine: `nango` | Auth: OAuth via Nango (Microsoft/OneDrive for Business)

## Overview

These tools wrap the Microsoft Graph API for OneDrive for Business. They allow AI agents to browse files, upload and download content, manage folders, and share files. **Requires Microsoft OAuth access for OneDrive for Business.**

## Authentication

**OAuth via Nango**:
- Alias for Microsoft integration
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Uses Microsoft Graph API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `one_drive_list_drives` | List drives | GET | /v1.0/drives |
| `one_drive_list_files` | List files in folder | GET | /v1.0/drives/{driveId}/root/children |
| `one_drive_get_file` | Get file metadata | GET | /v1.0/drives/{driveId}/items/{itemId} |
| `one_drive_download_file` | Download file content | GET | /v1.0/drives/{driveId}/items/{itemId}/content |
| `one_drive_upload_file` | Upload file | PUT | /v1.0/drives/{driveId}/items/{parentId}:/{name}:/content |
| `one_drive_create_folder` | Create folder | POST | /v1.0/drives/{driveId}/items/{parentId}/children |
| `one_drive_delete` | Delete item | DELETE | /v1.0/drives/{driveId}/items/{itemId} |
| `one_drive_move` | Move item | PATCH | /v1.0/drives/{driveId}/items/{itemId} |
| `one_drive_share` | Share item | POST | /v1.0/drives/{driveId}/items/{itemId}/createLink |
| `one_drive_search` | Search files | GET | /v1.0/drives/{driveId}/root/search(q='{query}') |

---

## Tool Details

### one_drive_list_drives

**What it does**: Lists all drives (OneDrive for Business) available to the user.

**When to use**: Find available drives, list SharePoint sites.

**Arguments**: None

**Example LLM prompt**: "List all my OneDrive drives"

---

### one_drive_list_files

**What it does**: Lists files and folders in a drive or folder.

**When to use**: Browse folders, navigate storage.

**Arguments**:
- `driveId` (optional): Drive ID (uses default if not specified)
- `path` (optional): Folder path (default: root `/`)

**Example LLM prompt**: "List files in /Documents folder"

---

### one_drive_get_file

**What it does**: Gets metadata for a specific file or folder.

**When to use**: Check file details, size, modified date.

**Arguments**:
- `driveId` (required): Drive ID
- `itemId` (required): Item ID

**Example LLM prompt**: "Get metadata for file ABC123"

---

### one_drive_download_file

**What it does**: Downloads file content.

**When to use**: Read file content, retrieve documents.

**Arguments**:
- `driveId` (required): Drive ID
- `itemId` (required): Item ID

**Example LLM prompt**: "Download the file ABC123"

---

### one_drive_upload_file

**What it does**: Uploads or creates a file with content.

**When to use**: Save documents, backup files.

**Arguments**:
- `driveId` (optional): Drive ID
- `parentId` (required): Parent folder ID
- `name` (required): File name
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload hello.txt with content 'Hello World' to /Documents"

---

### one_drive_create_folder

**What it does**: Creates a new folder in a drive.

**When to use**: Organize files, create directories.

**Arguments**:
- `driveId` (optional): Drive ID
- `parentId` (required): Parent folder ID
- `name` (required): Folder name

**Example LLM prompt**: "Create folder called /Projects/NewProject"

---

### one_drive_delete

**What it does**: Deletes a file or folder.

**When to use**: Remove files, clean up storage.

**Arguments**:
- `driveId` (optional): Drive ID
- `itemId` (required): Item ID

**Example LLM prompt**: "Delete file ABC123"

---

### one_drive_move

**What it does**: Moves a file or folder to a new location.

**When to use**: Organize files, reorganize structure.

**Arguments**:
- `driveId` (optional): Drive ID
- `itemId` (required): Item ID to move
- `parentId` (required): Destination parent folder ID

**Example LLM prompt**: "Move file ABC123 to folder XYZ456"

---

### one_drive_share

**What it does**: Shares a file or folder with others.

**When to use**: Collaborate, share documents.

**Arguments**:
- `driveId` (optional): Drive ID
- `itemId` (required): Item ID
- `type` (optional): Link type (view, edit, default: view)

**Example LLM prompt**: "Create a view link for file ABC123"

---

### one_drive_search

**What it does**: Searches for files in OneDrive.

**When to use**: Find files by name, locate content.

**Arguments**:
- `driveId` (optional): Drive ID
- `query` (required): Search query

**Example LLM prompt**: "Search for files named 'report'"

---

## OneDrive API Notes

- **Microsoft Graph**: Uses Microsoft Graph API endpoints
- **Drive ID**: Unique identifier for each drive/site
- **Item ID**: Unique identifier for files/folders
- **Content upload**: Files must be base64-encoded
- **Sharing links**: Can be view or edit links

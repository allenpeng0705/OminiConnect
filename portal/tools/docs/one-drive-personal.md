# OneDrive Personal Tools

Provider: `one-drive-personal` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Microsoft Graph API for personal OneDrive. They allow AI agents to browse files, upload and download content, manage folders, and share files. **Requires Microsoft OAuth access for personal OneDrive.**

## Authentication

**OAuth2**:
- User authenticates via Nango Connect with Microsoft personal account
- Authorization URL: `https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize`
- Token URL: `https://login.microsoftonline.com/consumers/oauth2/v2.0/token`
- Base URL: `https://api.onedrive.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onedrive_list_files` | List files in folder | GET | /v1.0/drive/root/children |
| `onedrive_get_file` | Get file metadata | GET | /v1.0/drive/items/{itemId} |
| `onedrive_download_file` | Download file content | GET | /v1.0/drive/items/{itemId}/content |
| `onedrive_upload_file` | Upload file | PUT | /v1.0/drive/root:/{name}:/content |
| `onedrive_create_folder` | Create folder | POST | /v1.0/drive/root/children |
| `onedrive_delete` | Delete item | DELETE | /v1.0/drive/items/{itemId} |
| `onedrive_move` | Move item | PATCH | /v1.0/drive/items/{itemId} |
| `onedrive_share` | Share item | POST | /v1.0/drive/items/{itemId}/createLink |
| `onedrive_search` | Search files | GET | /v1.0/drive/root/search(q='{query}') |
| `onedrive_get_user` | Get user info | GET | /v1.0/me |

---

## Tool Details

### onedrive_list_files

**What it does**: Lists files and folders in OneDrive.

**When to use**: Browse folders, navigate storage.

**Arguments**:
- `path` (optional): Folder path (default: root `/`)

**Example LLM prompt**: "List files in /Documents folder"

---

### onedrive_get_file

**What it does**: Gets metadata for a specific file or folder.

**When to use**: Check file details, size, modified date.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Get metadata for file ABC123"

---

### onedrive_download_file

**What it does**: Downloads file content.

**When to use**: Read file content, retrieve documents.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Download the file ABC123"

---

### onedrive_upload_file

**What it does**: Uploads or creates a file with content.

**When to use**: Save documents, backup files.

**Arguments**:
- `path` (optional): File path
- `name` (required): File name
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload hello.txt with content 'Hello World' to /Documents"

---

### onedrive_create_folder

**What it does**: Creates a new folder in OneDrive.

**When to use**: Organize files, create directories.

**Arguments**:
- `path` (required): Parent folder path
- `name` (required): Folder name

**Example LLM prompt**: "Create folder called /Projects/NewProject"

---

### onedrive_delete

**What it does**: Deletes a file or folder.

**When to use**: Remove files, clean up storage.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Delete file ABC123"

---

### onedrive_move

**What it does**: Moves a file or folder to a new location.

**When to use**: Organize files, reorganize structure.

**Arguments**:
- `itemId` (required): Item ID to move
- `parentId` (required): Destination parent folder ID

**Example LLM prompt**: "Move file ABC123 to folder XYZ456"

---

### onedrive_share

**What it does**: Shares a file or folder with others.

**When to use**: Collaborate, share documents.

**Arguments**:
- `itemId` (required): Item ID
- `type` (optional): Link type (view, edit, default: view)

**Example LLM prompt**: "Create a view link for file ABC123"

---

### onedrive_search

**What it does**: Searches for files in OneDrive.

**When to use**: Find files by name, locate content.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for files named 'report'"

---

### onedrive_get_user

**What it does**: Gets current user information.

**When to use**: Get user profile, verify account.

**Arguments**: None

**Example LLM prompt**: "Get my user info"

---

## OneDrive Personal Notes

- **Personal accounts**: Uses consumer Microsoft accounts
- **API endpoint**: `api.onedrive.com` for personal OneDrive
- **Item ID**: Unique identifier for files/folders
- **Content upload**: Files must be base64-encoded
- **Sharing links**: Can be view or edit links

# Google Drive Tools

Provider: `googledrive` | Engine: `nango` | Auth: OAuth via Nango (alias: `google`)

## Overview

These tools wrap the Google Drive API. They allow AI agents to browse files, create and upload documents, manage folders, share files with users, and work with comments and permissions. **Requires Google Drive OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Google (Drive)
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `drive.file`, `drive.readonly`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `googledrive_list_files` | List files in folder | GET | /drive/v3/files |
| `googledrive_get_file` | Get file metadata | GET | /drive/v3/files/{file_id} |
| `googledrive_create_file` | Create a file | POST | /drive/v3/files |
| `googledrive_upload_file` | Upload file content | PATCH | /upload/drive/v3/files/{file_id} |
| `googledrive_create_folder` | Create a folder | POST | /drive/v3/files |
| `googledrive_share_file` | Share a file | POST | /drive/v3/files/{file_id}/permissions |
| `googledrive_list_permissions` | List file permissions | GET | /drive/v3/files/{file_id}/permissions |
| `googledrive_get_permission` | Get permission details | GET | /drive/v3/files/{file_id}/permissions/{permission_id} |
| `googledrive_list_comments` | List comments on file | GET | /drive/v3/files/{file_id}/comments |
| `googledrive_get_comment` | Get comment details | GET | /drive/v3/files/{file_id}/comments/{comment_id} |

---

## Tool Details

### googledrive_list_files

**What it does**: Lists files and folders in Google Drive.

**When to use**: Browse folders, find files, navigate Drive structure.

**Arguments**:
- `folder_id` (optional): Folder ID (use `root` for My Drive, default: `root`)
- `page_size` (optional): Max results (default 100, max 1000)
- `page_token` (optional): Token for next page

**Example LLM prompt**: "List all files in my Google Drive root folder"

---

### googledrive_get_file

**What it does**: Gets metadata for a Google Drive file.

**When to use**: Check file details, verify file exists, get file properties.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Get info about the file with ID abc123xyz"

---

### googledrive_create_file

**What it does**: Creates a new file in Google Drive.

**When to use**: Create new documents, spreadsheets, or other files.

**Arguments**:
- `name` (required): File name
- `mime_type` (optional): MIME type (e.g., `text/plain`, `application/json`)
- `parent_id` (optional): Parent folder ID

**Example LLM prompt**: "Create a new file called 'Meeting Notes' in my Drive"

---

### googledrive_upload_file

**What it does**: Uploads content to an existing Google Drive file.

**When to use**: Update file content, overwrite existing files.

**Arguments**:
- `file_id` (required): File ID
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload new content to the file with ID abc123xyz"

---

### googledrive_create_folder

**What it does**: Creates a new folder in Google Drive.

**When to use**: Organize files, create project directories.

**Arguments**:
- `name` (required): Folder name
- `parent_id` (optional): Parent folder ID (default: `root`)

**Example LLM prompt**: "Create a folder called 'Project Files' in my Drive"

---

### googledrive_share_file

**What it does**: Shares a Google Drive file with a user or group.

**When to use**: Grant access to files, collaborate with others.

**Arguments**:
- `file_id` (required): File ID
- `type` (required): Type: `user`, `group`, `domain`, `anyone`
- `role` (required): Role: `reader`, `writer`, `commenter`, `owner`
- `email` (optional): Email address (for user or group type)

**Example LLM prompt**: "Share the file with ID abc123xyz with john@example.com as writer"

---

### googledrive_list_permissions

**What it does**: Lists all permissions on a file.

**When to use**: See who has access to a file, audit sharing.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "List all permissions on the file with ID abc123xyz"

---

### googledrive_get_permission

**What it does**: Gets details of a specific permission on a file.

**When to use**: Check specific permission details, verify access level.

**Arguments**:
- `file_id` (required): File ID
- `permission_id` (required): Permission ID

**Example LLM prompt**: "Get details about permission ID 456 on file abc123xyz"

---

### googledrive_list_comments

**What it does**: Lists comments on a Google Drive file.

**When to use**: View discussions, see feedback on documents.

**Arguments**:
- `file_id` (required): File ID
- `page_size` (optional): Max results (default 20)

**Example LLM prompt**: "List all comments on the file with ID abc123xyz"

---

### googledrive_get_comment

**What it does**: Gets details of a specific comment.

**When to use**: Read full comment content, view replies.

**Arguments**:
- `file_id` (required): File ID
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details about comment ID 789 on file abc123xyz"

---

## Google Drive API Notes

- **Folder ID `root`**: Represents the user's My Drive root
- **File ID format**: Long alphanumeric strings (e.g., `1abcDEF123...`)
- **MIME types**: Common types include `text/plain`, `application/json`, `application/pdf`, `image/png`
- **Content upload**: Files must be base64-encoded
- **Sharing**: Files can be shared with specific users, groups, domains, or made public
- **Rate limits**: Apply to API calls — implement backoff for heavy use

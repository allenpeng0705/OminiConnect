# Google Drive Tools

Provider: `google-drive` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Drive API. They allow AI agents to manage files, permissions, revisions, and search. **Requires Google OAuth2 with Drive permissions.**

## Authentication

**Nango OAUTH2 (Google Drive)**:
- User authenticates via OAuth2 with Drive scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://www.googleapis.com/drive/v3`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_drive_list_files` | List files | GET | /drive/v3/files |
| `google_drive_get_file` | Get file details | GET | /drive/v3/files/{fileId} |
| `google_drive_create_file` | Create file | POST | /drive/v3/files |
| `google_drive_update_file` | Update file | PATCH | /drive/v3/files/{fileId} |
| `google_drive_delete_file` | Delete file | DELETE | /drive/v3/files/{fileId} |
| `google_drive_list_permissions` | List permissions | GET | /drive/v3/files/{fileId}/permissions |
| `google_drive_create_permission` | Create permission | POST | /drive/v3/files/{fileId}/permissions |
| `google_drive_list_revisions` | List revisions | GET | /drive/v3/files/{fileId}/revisions |
| `google_drive_get_revision` | Get revision | GET | /drive/v3/files/{fileId}/revisions/{revisionId} |
| `google_drive_search_files` | Search files | GET | /drive/v3/files |

---

## Tool Details

### google_drive_list_files

**What it does**: Lists files in Google Drive.

**When to use**: Browse user's Drive files.

**Arguments**:
- `q` (optional): Search query
- `pageSize` (optional): Number of results (default 20)

**Example LLM prompt**: "List files in my Drive"

---

### google_drive_get_file

**What it does**: Gets detailed information about a file.

**When to use**: Get file metadata.

**Arguments**:
- `fileId` (required): File ID

**Example LLM prompt**: "Get details for file abc123"

---

### google_drive_create_file

**What it does**: Creates a new file in Google Drive.

**When to use**: Create new files.

**Arguments**:
- `name` (required): File name
- `mimeType` (optional): MIME type
- `parents` (optional): Parent folder IDs

**Example LLM prompt**: "Create a new file named 'Report.txt'"

---

### google_drive_update_file

**What it does**: Updates a file's metadata.

**When to use**: Rename files.

**Arguments**:
- `fileId` (required): File ID
- `name` (optional): New file name

**Example LLM prompt**: "Rename file abc123 to 'New Report.txt'"

---

### google_drive_delete_file

**What it does**: Deletes a file from Google Drive.

**When to use**: Remove files.

**Arguments**:
- `fileId` (required): File ID

**Example LLM prompt**: "Delete file abc123"

---

### google_drive_list_permissions

**What it does**: Lists permissions for a file.

**When to use**: See who has access.

**Arguments**:
- `fileId` (required): File ID

**Example LLM prompt**: "List permissions for file abc123"

---

### google_drive_create_permission

**What it does**: Creates a permission for a file.

**When to use**: Share files.

**Arguments**:
- `fileId` (required): File ID
- `type` (required): Permission type (user, group, domain, anyone)
- `role` (required): Role (reader, writer, commenter, owner)
- `emailAddress` (optional): Email for user/group

**Example LLM prompt**: "Share file abc123 with user john@email.com as writer"

---

### google_drive_list_revisions

**What it does**: Lists revisions of a file.

**When to use**: See version history.

**Arguments**:
- `fileId` (required): File ID

**Example LLM prompt**: "List revisions for file abc123"

---

### google_drive_get_revision

**What it does**: Gets a specific revision of a file.

**When to use**: View file at a point in time.

**Arguments**:
- `fileId` (required): File ID
- `revisionId` (required): Revision ID

**Example LLM prompt**: "Get revision xyz789 of file abc123"

---

### google_drive_search_files

**What it does**: Searches for files in Google Drive.

**When to use**: Find files by name or type.

**Arguments**:
- `q` (required): Search query
- `pageSize` (optional): Number of results (default 20)

**Example LLM prompt**: "Search for files named 'report'"

---

## Google Drive API Notes

- **File ID**: Unique identifier for each file
- **Queries**: Use Drive query syntax (name, mimeType, modifiedTime)
- **Permissions**: Share with users, groups, domains, or make public
- **Roles**: reader, commenter, writer, owner
- **Revision limit**: Some file types have limited revision history

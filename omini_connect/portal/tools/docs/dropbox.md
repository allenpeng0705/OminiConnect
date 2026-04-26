# Dropbox Tools

Provider: `dropbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Dropbox API v2. They allow AI agents to browse files, upload and download content, manage folders, and share files with team members. **Requires Dropbox OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dropbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `files.content.read`, `files.content.write`, `files.metadata.read`, `files.metadata.write`, `sharing.read`, `sharing.write`, `team.read`, `account_info.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dropbox_list_files` | List files in folder | POST | /2/files/list_folder |
| `dropbox_get_file` | Get file metadata | POST | /2/files/get_metadata |
| `dropbox_download_file` | Download file content | POST | /2/files/download |
| `dropbox_upload_file` | Upload file content | POST | /2/files/upload |
| `dropbox_create_folder` | Create a folder | POST | /2/files/create_folder_v2 |
| `dropbox_delete_file` | Delete file or folder | POST | /2/files/delete_v2 |
| `dropbox_search_files` | Search for files | POST | /2/files/search_v2 |
| `dropbox_list_shared_links` | List shared links | POST | /2/sharing/list_shared_links |
| `dropbox_create_shared_link` | Create a shared link | POST | /2/sharing/create_shared_link |
| `dropbox_list_folder_members` | List folder members | POST | /2/sharing/list_folder_members |

---

## Tool Details

### dropbox_list_files

**What it does**: Lists files and folders in a Dropbox directory.

**When to use**: Browse folders, find files, navigate Dropbox structure.

**Arguments**:
- `path` (optional): Folder path (default: `/`)
- `recursive` (optional): Include subfolders (default: false)
- `limit` (optional): Max files (default 20, max 200)

**Example LLM prompt**: "List all files in my Dropbox /Projects folder"

---

### dropbox_get_file

**What it does**: Gets metadata for a file or folder (size, modified date, etc.).

**When to use**: Check file details without downloading, verify file exists.

**Arguments**:
- `path` (required): File or folder path

**Example LLM prompt**: "Get info about /Documents/report.pdf"

---

### dropbox_download_file

**What it does**: Downloads file content as base64-encoded data.

**When to use**: Read file content, retrieve documents.

**Arguments**:
- `path` (required): File path

**Example LLM prompt**: "Download the file /Documents/notes.txt"

---

### dropbox_upload_file

**What it does**: Uploads or creates a file with content.

**When to use**: Save documents, backup files, write content to Dropbox.

**Arguments**:
- `path` (required): Destination path
- `content` (required): File content
- `mode` (optional): Write mode (`add`, `overwrite`, `update`)

**Example LLM prompt**: "Upload 'Hello World' to /Documents/hello.txt"

---

### dropbox_create_folder

**What it does**: Creates a new folder in Dropbox.

**When to use**: Organize files, create project directories.

**Arguments**:
- `path` (required): Folder path to create

**Example LLM prompt**: "Create a folder called /Projects/NewProject"

---

### dropbox_delete_file

**What it does**: Deletes a file or folder (folders deleted recursively).

**When to use**: Remove unwanted files, clean up storage.

**Arguments**:
- `path` (required): Path to delete

**Example LLM prompt**: "Delete /Documents/old_file.txt"

---

### dropbox_search_files

**What it does**: Searches for files and folders matching a query.

**When to use**: Find files by name, locate content across Dropbox.

**Arguments**:
- `query` (required): Search query string
- `path` (optional): Folder to search in (default: `/`)
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "Find all files named 'report' in my Dropbox"

---

### dropbox_list_shared_links

**What it does**: Lists all shared links for files and folders.

**When to use**: See what is already shared, find shared content.

**Arguments**:
- `path` (optional): File or folder path

**Example LLM prompt**: "List all shared links for files in /Shared"

---

### dropbox_create_shared_link

**What it does**: Creates or modifies a shared link for a file or folder.

**When to use**: Share files with external users, create public access links.

**Arguments**:
- `path` (required): File or folder path
- `settings` (optional): Link settings (visibility, expires)

**Example LLM prompt**: "Create a shared link for /Documents/report.pdf"

---

### dropbox_list_folder_members

**What it does**: Lists members of a shared folder with their access level.

**When to use**: View who has access to a shared folder, manage folder sharing.

**Arguments**:
- `folder_id` (required): Shared folder ID

**Example LLM prompt**: "List all members who have access to the /Shared folder"

---

## Dropbox API Notes

- **Path format**: Full paths starting with `/` (e.g., `/Documents/file.txt`)
- **Content upload**: Files must be base64-encoded for upload endpoint
- **Recursive delete**: Deleting a folder removes all contents
- **Search**: Matches file names, not content
- **Shared links**: Can be public or team-only depending on settings
- **Rate limits**: Apply to API calls — implement backoff for heavy use

# Egnyte Tools

Provider: `egnyte` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Egnyte API. They allow AI agents to manage files, folders, permissions, and users. Egnyte is an enterprise file sync and sharing platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Egnyte
- Token stored in Nango, accessed via `connection_ref`
- Subdomain configured per connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `egnyte_list_files` | List files and folders | GET | /pubapi/v1/fs |
| `egnyte_get_file` | Get file details | GET | /pubapi/v1/fs/{path} |
| `egnyte_download_file` | Download file content | GET | /pubapi/v1/fs-content/{path} |
| `egnyte_upload_file` | Upload file content | POST | /pubapi/v1/fs-content/{path} |
| `egnyte_create_folder` | Create a folder | POST | /pubapi/v1/fs/{path} |
| `egnyte_delete_file` | Delete file or folder | DELETE | /pubapi/v1/fs/{path} |
| `egnyte_list_permissions` | List permissions | GET | /pubapi/v1/permissions |
| `egnyte_get_permissions` | Get permissions for path | GET | /pubapi/v1/permissions |
| `egnyte_list_users` | List users | GET | /pubapi/v1/users |
| `egnyte_get_user` | Get user details | GET | /pubapi/v1/users/{user_id} |

---

## Tool Details

### egnyte_list_files

**What it does**: Lists files and folders at a given path.

**When to use**: Browse file system, navigate folders, find files.

**Arguments**:
- `path` (optional): Folder path (default: /)
- `limit` (optional): Max results (default 100, max 1000)

**Example LLM prompt**: "List all files in /Projects"

---

### egnyte_get_file

**What it does**: Gets metadata for a file or folder.

**When to use**: Check file details, verify existence, get file information.

**Arguments**:
- `path` (required): File or folder path

**Example LLM prompt**: "Get details for file /Documents/report.pdf"

---

### egnyte_download_file

**What it does**: Downloads file content.

**When to use**: Retrieve file content, get documents, access stored files.

**Arguments**:
- `path` (required): File path

**Example LLM prompt**: "Download file /Documents/report.pdf"

---

### egnyte_upload_file

**What it does**: Uploads or creates a file.

**When to use**: Save documents, backup files, create new content.

**Arguments**:
- `path` (required): Destination path
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload a file to /Documents/newfile.txt"

---

### egnyte_create_folder

**What it does**: Creates a new folder.

**When to use**: Organize files, create directories, set up structure.

**Arguments**:
- `path` (required): Folder path to create

**Example LLM prompt**: "Create folder /Projects/NewProject"

---

### egnyte_delete_file

**What it does**: Deletes a file or folder.

**When to use**: Remove unwanted files, clean up storage, delete old content.

**Arguments**:
- `path` (required): File or folder path to delete

**Example LLM prompt**: "Delete file /Documents/oldfile.txt"

---

### egnyte_list_permissions

**What it does**: Lists all permission policies.

**When to use**: View permission structure, manage access control, audit permissions.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all permission policies"

---

### egnyte_get_permissions

**What it does**: Gets permissions for a specific path.

**When to use**: Check who has access, verify permissions, review access rights.

**Arguments**:
- `path` (required): Path to get permissions for

**Example LLM prompt**: "Get permissions for /Shared folder"

---

### egnyte_list_users

**What it does**: Lists all users in the Egnyte deployment.

**When to use**: View user list, find users, manage user access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all Egnyte users"

---

### egnyte_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user details, verify user permissions, review user status.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-123"

---

## Egnyte API Notes

- **File Sync Platform**: Enterprise content management and file sync
- **Subdomain**: Configured per connection (e.g., company.egnyte.com)
- **Files**: Stored with full path references
- **Permissions**: Access control for files and folders
- **Users**: Enterprise user directory
- **OAuth Flow**: Standard OAuth2 with subdomain routing
- **PubAPI**: Public API for file operations

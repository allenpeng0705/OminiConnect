# Next Cloud OCS Tools

Provider: `next-cloud-ocs` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Next Cloud OCS API. They allow AI agents to browse files, manage folders, share content, and administer users and groups. **Requires Next Cloud Basic Auth credentials.**

## Authentication

**Basic Auth**:
- User provides Next Cloud username and password
- Credentials passed via Nango
- Base URL: `https://{domain}/ocs/v1.php`
- Special header: `OCS-APIRequest: true`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `next_cloud_list_files` | List files in folder | GET | /files/{user}/home |
| `next_cloud_get_file` | Get file metadata | GET | /files/{user}/home/{path} |
| `next_cloud_create_folder` | Create folder | MKCOL | /files/{user}/home/{path} |
| `next_cloud_delete` | Delete file or folder | DELETE | /files/{user}/home/{path} |
| `next_cloud_move` | Move file or folder | MOVE | /files/{user}/home/{path} |
| `next_cloud_share` | Share file or folder | POST | /shares |
| `next_cloud_list_shares` | List shares | GET | /shares |
| `next_cloud_list_users` | List users | GET | /cloud/users |
| `next_cloud_get_user` | Get user details | GET | /cloud/users/{userId} |
| `next_cloud_list_groups` | List groups | GET | /cloud/groups |

---

## Tool Details

### next_cloud_list_files

**What it does**: Lists files and folders in a Next Cloud directory.

**When to use**: Browse folders, find files, navigate storage.

**Arguments**:
- `path` (optional): Folder path (default: `/`)

**Example LLM prompt**: "List all files in the /Documents folder"

---

### next_cloud_get_file

**What it does**: Gets metadata for a specific file or folder.

**When to use**: Check file details, verify existence.

**Arguments**:
- `path` (required): File or folder path

**Example LLM prompt**: "Get metadata for /Documents/report.pdf"

---

### next_cloud_create_folder

**What it does**: Creates a new folder in Next Cloud.

**When to use**: Organize files, create directories.

**Arguments**:
- `path` (required): Folder path to create

**Example LLM prompt**: "Create a folder called /Projects/NewProject"

---

### next_cloud_delete

**What it does**: Deletes a file or folder in Next Cloud.

**When to use**: Remove files, clean up storage.

**Arguments**:
- `path` (required): File or folder path to delete

**Example LLM prompt**: "Delete /Documents/old_file.txt"

---

### next_cloud_move

**What it does**: Moves a file or folder to a new location.

**When to use**: Organize files, rename locations.

**Arguments**:
- `path` (required): Source path
- `destination` (required): Destination path

**Example LLM prompt**: "Move /Documents/file.txt to /Archive/file.txt"

---

### next_cloud_share

**What it does**: Shares a file or folder with other users.

**When to use**: Collaborate, share documents.

**Arguments**:
- `path` (required): File or folder path to share
- `share_type` (required): Share type (0=user, 1=group, 3=public)
- `permissions` (optional): Permissions (1=read, 2=write, 31=all)

**Example LLM prompt**: "Share /Documents/report.pdf with user john"

---

### next_cloud_list_shares

**What it does**: Lists all shared files and folders.

**When to use**: View shared content, manage shares.

**Arguments**:
- `shared_with_me` (optional): List shares with me (default true)

**Example LLM prompt**: "List all files shared with me"

---

### next_cloud_list_users

**What it does**: Lists all users in Next Cloud.

**When to use**: Browse users, admin tasks.

**Arguments**: None

**Example LLM prompt**: "List all Next Cloud users"

---

### next_cloud_get_user

**What it does**: Gets details for a specific user.

**When to use**: Check user info, quotas, permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user john"

---

### next_cloud_list_groups

**What it does**: Lists all groups in Next Cloud.

**When to use**: View group structure, manage permissions.

**Arguments**: None

**Example LLM prompt**: "List all groups"

---

## Next Cloud OCS Notes

- **Path format**: Full paths starting with `/`
- **User placeholder**: Use `{user}` in paths (replaced with authenticated user)
- **Share types**: 0 = user, 1 = group, 3 = public link
- **Permissions**: 1 = read, 2 = write, 4 = delete, 8 = share, 31 = all
- **OCS header**: All requests require `OCS-APIREQUEST: true` header

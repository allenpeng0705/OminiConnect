# Box Tools

Provider: `box` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Box API. They allow AI agents to browse files and folders, upload and download content, manage enterprise users and groups, and work with tasks. **Requires Box OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Box
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `base_item.read`, `base_item.write`, `manage_enterprise`, `group_read`, `task_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `box_list_files` | List files in folder | GET | /2.0/folders/{folder_id}/items |
| `box_get_file` | Get file metadata | GET | /2.0/files/{file_id} |
| `box_upload_file` | Upload a file | POST | /2.0/files/content |
| `box_create_folder` | Create a folder | POST | /2.0/folders |
| `box_delete_file` | Delete a file | DELETE | /2.0/files/{file_id} |
| `box_list_users` | List users in enterprise | GET | /2.0/users |
| `box_get_user` | Get user info | GET | /2.0/users/{user_id} |
| `box_list_groups` | List groups | GET | /2.0/groups |
| `box_get_group` | Get group info | GET | /2.0/groups/{group_id} |
| `box_list_tasks` | List tasks on file | GET | /2.0/files/{file_id}/tasks |

---

## Tool Details

### box_list_files

**What it does**: Lists files and folders in a Box folder.

**When to use**: Browse folders, navigate Box structure.

**Arguments**:
- `folder_id` (optional): Folder ID (use `0` for root, default: `0`)
- `limit` (optional): Max results (default 100, max 1000)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all files in the Box root folder"

---

### box_get_file

**What it does**: Gets metadata for a Box file.

**When to use**: Check file details, verify file exists.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Get info about the file with ID 12345"

---

### box_upload_file

**What it does**: Uploads a file to a Box folder.

**When to use**: Save documents, upload content to Box.

**Arguments**:
- `name` (required): File name
- `parent_id` (optional): Parent folder ID (default: `0`)
- `content` (required): File content (base64-encoded)

**Example LLM prompt**: "Upload a file called report.pdf to the root folder"

---

### box_create_folder

**What it does**: Creates a new folder in Box.

**When to use**: Organize files, create project directories.

**Arguments**:
- `name` (required): Folder name
- `parent_id` (optional): Parent folder ID (default: `0` for root)

**Example LLM prompt**: "Create a folder called 'NewProject' in the root"

---

### box_delete_file

**What it does**: Permanently deletes a Box file (moves to trash).

**When to use**: Remove unwanted files, clean up storage.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Delete the file with ID 12345"

---

### box_list_users

**What it does**: Lists users in the enterprise.

**When to use**: View enterprise directory, find users.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all users in our Box enterprise"

---

### box_get_user

**What it does**: Gets information about a Box user by ID or email.

**When to use**: Look up user details, find user information.

**Arguments**:
- `user_id` (optional): User ID (use `me` for current user, default: `me`)

**Example LLM prompt**: "Get information about the user with email john@example.com"

---

### box_list_groups

**What it does**: Lists groups in the enterprise.

**When to use**: View available groups, find group information.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all groups in our Box enterprise"

---

### box_get_group

**What it does**: Gets information about a group.

**When to use**: View group details, see group members.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details about the Engineering group"

---

### box_list_tasks

**What it does**: Lists tasks on a file.

**When to use**: See assigned tasks, track file-related work.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "List all tasks on the file with ID 12345"

---

## Box API Notes

- **Folder ID 0**: Represents the root folder
- **Content upload**: Files must be base64-encoded
- **Trash behavior**: Deleted files go to trash and can be restored within 30 days
- **Pagination**: Use `limit` and `offset` for large result sets
- **Rate limits**: Apply to API calls — implement backoff for heavy use

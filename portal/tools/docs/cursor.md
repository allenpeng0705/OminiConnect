# Cursor Tools

Provider: `cursor` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Cursor AI API. They allow AI agents to manage projects, files, and changelists. Cursor is an AI-powered code editor built on top of VS Code.

## Authentication

**Nango API_KEY**:
- User provides their Cursor API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Format: `key_` followed by 64 character hex string

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cursor_list_projects` | List projects | GET | /v0/projects |
| `cursor_get_project` | Get project details | GET | /v0/projects/{project_id} |
| `cursor_list_files` | List files in project | GET | /v0/files |
| `cursor_get_file` | Get file content | GET | /v0/files/{file_id} |
| `cursor_create_file` | Create a new file | POST | /v0/files |
| `cursor_update_file` | Update file content | PUT | /v0/files/{file_id} |
| `cursor_delete_file` | Delete a file | DELETE | /v0/files/{file_id} |
| `cursor_list_changelists` | List changelists | GET | /v0/changelists |
| `cursor_create_changelist` | Create a changelist | POST | /v0/changelists |
| `cursor_get_me` | Get current user info | GET | /v0/me |

---

## Tool Details

### cursor_list_projects

**What it does**: Lists all projects in the Cursor workspace.

**When to use**: Browse available projects, select project for file operations.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all my Cursor projects"

---

### cursor_get_project

**What it does**: Gets detailed project information including settings and configuration.

**When to use**: Review project settings, check project metadata, understand project structure.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project abc123"

---

### cursor_list_files

**What it does**: Lists all files in a specific project.

**When to use**: Browse project files, navigate folder structure, find specific files.

**Arguments**:
- `project_id` (required): Project ID
- `path` (optional): Folder path (default: /)

**Example LLM prompt**: "List all files in project abc123"

---

### cursor_get_file

**What it does**: Gets the content of a specific file.

**When to use**: Read file content, review code, get file for editing.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Get content of file f-456"

---

### cursor_create_file

**What it does**: Creates a new file in a project with specified content.

**When to use**: Add new source files, create configuration files, write new code.

**Arguments**:
- `project_id` (required): Project ID
- `path` (required): File path within project
- `content` (required): File content

**Example LLM prompt**: "Create a new file called hello.py in project abc123 with content print('hello')"

---

### cursor_update_file

**What it does**: Updates the content of an existing file.

**When to use**: Modify code, update configurations, edit existing files.

**Arguments**:
- `file_id` (required): File ID
- `content` (required): New file content

**Example LLM prompt**: "Update file f-456 with new content"

---

### cursor_delete_file

**What it does**: Deletes a file from a project.

**When to use**: Remove unwanted files, clean up project, delete obsolete code.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Delete file f-456"

---

### cursor_list_changelists

**What it does**: Lists all changelists (code change sets) in a project.

**When to use**: View pending changes, review code modifications, track development progress.

**Arguments**:
- `project_id` (required): Project ID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all changelists in project abc123"

---

### cursor_create_changelist

**What it does**: Creates a new named changelist for grouping code changes.

**When to use**: Organize related changes, prepare code for review, track feature development.

**Arguments**:
- `project_id` (required): Project ID
- `name` (required): Changelist name
- `description` (optional): Changelist description

**Example LLM prompt**: "Create a changelist named 'feature-login' in project abc123"

---

### cursor_get_me

**What it does**: Gets information about the currently authenticated user.

**When to use**: Verify authentication, get user profile, check account details.

**Arguments**: None

**Example LLM prompt**: "Get my Cursor user info"

---

## Cursor API Notes

- **API Key Format**: `key_` followed by 64 character hexadecimal string
- **Projects**: Code projects/workspaces in Cursor
- **Files**: Source files within projects, organized in folders
- **Changelists**: Named sets of code changes, similar to code reviews
- **Verification Endpoint**: GET /v0/me confirms API key validity

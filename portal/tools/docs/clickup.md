# ClickUp

## Overview

ClickUp is a project management platform that offers tasks, lists, spaces, folders, and goals for organizing work across teams.

## Authentication

ClickUp uses API token authentication. Your API token can be found in your ClickUp settings under **Settings > Apps**.

## Base URL

```
https://api.clickup.com/api/v2/
```

## Rate Limits

ClickUp API rate limits vary by plan. Free tier has 100 requests per minute. Check ClickUp documentation for your plan limits.

## Available Tools

### Tasks

| Tool | Description |
|------|-------------|
| `clickup_list_tasks` | Retrieve a list of tasks, optionally filtered by list or other criteria |
| `clickup_get_task` | Retrieve details of a specific task by ID |
| `clickup_create_task` | Create a new task in a list |
| `clickup_update_task` | Update an existing task with new values |

### Lists

| Tool | Description |
|------|-------------|
| `clickup_list_lists` | Retrieve a list of lists, optionally filtered by folder or space |
| `clickup_get_list` | Retrieve details of a specific list by ID |

### Spaces

| Tool | Description |
|------|-------------|
| `clickup_list_spaces` | Retrieve a list of all spaces in the workspace |
| `clickup_get_space` | Retrieve details of a specific space by ID |

### Folders

| Tool | Description |
|------|-------------|
| `clickup_list_folders` | Retrieve a list of folders, optionally within a space |
| `clickup_get_folder` | Retrieve details of a specific folder by ID |

## Tool Details

### clickup_list_tasks

Retrieve a paginated list of tasks with optional filters.

**Endpoint:** `GET /list/{listId}/task`

**Scopes:** `tasks:read`

**Parameters:**
- `listId` (string, required): Filter tasks by list ID
- `spaceId` (string, optional): Filter tasks by space ID
- `projectId` (string, optional): Filter tasks by project ID
- `status` (string, optional): Filter by task status
- `include_closed` (boolean, optional): Include closed tasks
- `page` (integer, optional): Page number for pagination

### clickup_get_task

Retrieve details of a specific task.

**Endpoint:** `GET /task/{taskId}`

**Scopes:** `tasks:read`

**Parameters:**
- `taskId` (string, required): The unique ID of the task

### clickup_create_task

Create a new task in a list.

**Endpoint:** `POST /list/{listId}/task`

**Scopes:** `tasks:write`

**Parameters:**
- `listId` (string, required): List ID to create the task in
- `name` (string, required): Task name
- `description` (string, optional): Task description
- `assignees` (array of strings, optional): List of user IDs to assign
- `status` (string, optional): Task status
- `priority` (integer, optional): Task priority (1-4)
- `dueDate` (string, optional): Task due date (Unix timestamp in milliseconds)

### clickup_update_task

Update an existing task with new values.

**Endpoint:** `PUT /task/{taskId}`

**Scopes:** `tasks:write`

**Parameters:**
- `taskId` (string, required): The unique ID of the task
- `name` (string, optional): New task name
- `description` (string, optional): New task description
- `status` (string, optional): New task status
- `priority` (integer, optional): New task priority (1-4)
- `dueDate` (string, optional): New due date (Unix timestamp in milliseconds)

### clickup_list_lists

Retrieve a paginated list of lists with optional filters.

**Endpoint:** `GET /space/{spaceId}/list`

**Scopes:** `lists:read`

**Parameters:**
- `spaceId` (string, required): Filter by space ID
- `folderId` (string, optional): Filter by folder ID
- `page` (integer, optional): Page number for pagination

### clickup_get_list

Retrieve details of a specific list.

**Endpoint:** `GET /list/{listId}`

**Scopes:** `lists:read`

**Parameters:**
- `listId` (string, required): The unique ID of the list

### clickup_list_spaces

Retrieve a paginated list of all spaces in the workspace.

**Endpoint:** `GET /workspace/{workspaceId}/space`

**Scopes:** `spaces:read`

**Parameters:**
- `workspaceId` (string, required): Workspace ID
- `page` (integer, optional): Page number for pagination

### clickup_get_space

Retrieve details of a specific space.

**Endpoint:** `GET /space/{spaceId}`

**Scopes:** `spaces:read`

**Parameters:**
- `spaceId` (string, required): The unique ID of the space

### clickup_list_folders

Retrieve a paginated list of folders with optional filters.

**Endpoint:** `GET /space/{spaceId}/folder`

**Scopes:** `folders:read`

**Parameters:**
- `spaceId` (string, required): Filter by space ID
- `page` (integer, optional): Page number for pagination

### clickup_get_folder

Retrieve details of a specific folder.

**Endpoint:** `GET /folder/{folderId}`

**Scopes:** `folders:read`

**Parameters:**
- `folderId` (string, required): The unique ID of the folder

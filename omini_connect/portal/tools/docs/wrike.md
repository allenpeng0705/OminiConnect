# Wrike

## Overview

Wrike is a project management platform that provides tools for managing tasks, projects, folders, and workspaces with powerful collaboration features.

## Authentication

Wrike uses OAuth 2.0 authentication. You will need to authorize your account through the standard OAuth flow to obtain access tokens.

## Base URL

```
https://www.wrike.com/api/v4/
```

## Rate Limits

Wrike API has rate limiting. The exact limits depend on your subscription level. Check the Wrike documentation for details.

## Available Tools

### Tasks

| Tool | Description |
|------|-------------|
| `wrike_list_tasks` | Retrieve a list of tasks, optionally filtered by project or folder |
| `wrike_get_task` | Retrieve details of a specific task by ID |
| `wrike_create_task` | Create a new task in a project or folder |
| `wrike_update_task` | Update an existing task with new values |

### Projects

| Tool | Description |
|------|-------------|
| `wrike_list_projects` | Retrieve a list of all projects |
| `wrike_get_project` | Retrieve details of a specific project by ID |

### Folders

| Tool | Description |
|------|-------------|
| `wrike_list_folders` | Retrieve a list of folders, optionally within a workspace |
| `wrike_get_folder` | Retrieve details of a specific folder by ID |

### Workspaces

| Tool | Description |
|------|-------------|
| `wrike_list_workspaces` | Retrieve a list of all workspaces |
| `wrike_get_workspace` | Retrieve details of a specific workspace by ID |

## Tool Details

### wrike_list_tasks

Retrieve a paginated list of tasks with optional filters.

**Endpoint:** `GET /tasks`

**Scopes:** `tasks:read`

**Parameters:**
- `projectId` (string, optional): Filter tasks by project ID
- `folderId` (string, optional): Filter tasks by folder ID
- `status` (string, optional): Filter by task status
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of tasks per page

### wrike_get_task

Retrieve details of a specific task.

**Endpoint:** `GET /tasks/{taskId}`

**Scopes:** `tasks:read`

**Parameters:**
- `taskId` (string, required): The unique ID of the task

### wrike_create_task

Create a new task in a project or folder.

**Endpoint:** `POST /tasks`

**Scopes:** `tasks:write`

**Parameters:**
- `title` (string, required): Task title
- `description` (string, optional): Task description
- `parentId` (string, optional): Parent folder or project ID
- `assignees` (array of strings, optional): List of user IDs to assign
- `dueDate` (string, optional): Task due date (ISO 8601)

### wrike_update_task

Update an existing task with new values.

**Endpoint:** `PUT /tasks/{taskId}`

**Scopes:** `tasks:write`

**Parameters:**
- `taskId` (string, required): The unique ID of the task
- `title` (string, optional): New task title
- `description` (string, optional): New task description
- `status` (string, optional): New task status
- `dueDate` (string, optional): New due date (ISO 8601)

### wrike_list_projects

Retrieve a paginated list of all projects.

**Endpoint:** `GET /projects`

**Scopes:** `projects:read`

**Parameters:**
- `spaceId` (string, optional): Filter by space ID
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of projects per page

### wrike_get_project

Retrieve details of a specific project.

**Endpoint:** `GET /projects/{projectId}`

**Scopes:** `projects:read`

**Parameters:**
- `projectId` (string, required): The unique ID of the project

### wrike_list_folders

Retrieve a paginated list of folders with optional filters.

**Endpoint:** `GET /folders`

**Scopes:** `folders:read`

**Parameters:**
- `workspaceId` (string, optional): Filter by workspace ID
- `parentId` (string, optional): Filter by parent folder ID
- `page` (integer, optional): Page number for pagination

### wrike_get_folder

Retrieve details of a specific folder.

**Endpoint:** `GET /folders/{folderId}`

**Scopes:** `folders:read`

**Parameters:**
- `folderId` (string, required): The unique ID of the folder

### wrike_list_workspaces

Retrieve a paginated list of all workspaces.

**Endpoint:** `GET /workspaces`

**Scopes:** `workspaces:read`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of workspaces per page

### wrike_get_workspace

Retrieve details of a specific workspace.

**Endpoint:** `GET /workspaces/{workspaceId}`

**Scopes:** `workspaces:read`

**Parameters:**
- `workspaceId` (string, required): The unique ID of the workspace

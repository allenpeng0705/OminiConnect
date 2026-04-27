# Teamwork

## Overview

Teamwork is a project management platform that provides tools for managing projects, tasks, teams, and time tracking.

## Authentication

Teamwork uses API key authentication. Your API key can be found in your Teamwork account settings under **My Account > API Keys**.

## Base URL

```
https://yourdomain.teamwork.com/
```

## Rate Limits

Teamwork API rate limits vary by plan. Check your plan details in Teamwork settings.

## Available Tools

### Projects

| Tool | Description |
|------|-------------|
| `teamwork_list_projects` | Retrieve a list of all projects in Teamwork |
| `teamwork_get_project` | Retrieve details of a specific project by ID |
| `teamwork_create_project` | Create a new project in Teamwork |

### Tasks

| Tool | Description |
|------|-------------|
| `teamwork_list_tasks` | Retrieve a list of tasks, optionally filtered by project |
| `teamwork_get_task` | Retrieve details of a specific task by ID |
| `teamwork_create_task` | Create a new task in a project |

### Users

| Tool | Description |
|------|-------------|
| `teamwork_list_users` | Retrieve a list of all users in the account |
| `teamwork_get_user` | Retrieve details of a specific user by ID |

### Time Tracking

| Tool | Description |
|------|-------------|
| `teamwork_list_time_entries` | Retrieve a list of time entries |
| `teamwork_get_time_entry` | Retrieve details of a specific time entry by ID |

## Tool Details

### teamwork_list_projects

Retrieve a paginated list of all projects.

**Endpoint:** `GET /projects.json`

**Scopes:** `projects:read`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of projects per page
- `status` (string, optional): Filter by project status (active, on hold, completed)

### teamwork_get_project

Retrieve details of a specific project.

**Endpoint:** `GET /projects/{projectId}.json`

**Scopes:** `projects:read`

**Parameters:**
- `projectId` (string, required): The unique ID of the project

### teamwork_create_project

Create a new project in Teamwork.

**Endpoint:** `POST /projects.json`

**Scopes:** `projects:write`

**Parameters:**
- `name` (string, required): Project name
- `description` (string, optional): Project description
- `status` (string, optional): Project status (active, on hold, completed)
- `startDate` (string, optional): Project start date (YYYY-MM-DD)
- `endDate` (string, optional): Project end date (YYYY-MM-DD)

### teamwork_list_tasks

Retrieve a list of tasks with optional filters.

**Endpoint:** `GET /tasks.json`

**Scopes:** `tasks:read`

**Parameters:**
- `projectId` (string, optional): Filter tasks by project ID
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of tasks per page
- `status` (string, optional): Filter by task status (open, completed)

### teamwork_get_task

Retrieve details of a specific task.

**Endpoint:** `GET /tasks/{taskId}.json`

**Scopes:** `tasks:read`

**Parameters:**
- `taskId` (string, required): The unique ID of the task

### teamwork_create_task

Create a new task in a project.

**Endpoint:** `POST /tasks.json`

**Scopes:** `tasks:write`

**Parameters:**
- `projectId` (string, required): Project ID to create the task in
- `name` (string, required): Task name
- `description` (string, optional): Task description
- `dueDate` (string, optional): Task due date (YYYY-MM-DD)
- `assignedTo` (string, optional): User ID to assign the task to

### teamwork_list_users

Retrieve a list of all users in the account.

**Endpoint:** `GET /people.json`

**Scopes:** `people:read`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of users per page

### teamwork_get_user

Retrieve details of a specific user.

**Endpoint:** `GET /people/{userId}.json`

**Scopes:** `people:read`

**Parameters:**
- `userId` (string, required): The unique ID of the user

### teamwork_list_time_entries

Retrieve a list of time entries with optional filters.

**Endpoint:** `GET /time_entries.json`

**Scopes:** `time:read`

**Parameters:**
- `projectId` (string, optional): Filter time entries by project ID
- `taskId` (string, optional): Filter time entries by task ID
- `userId` (string, optional): Filter time entries by user ID
- `page` (integer, optional): Page number for pagination

### teamwork_get_time_entry

Retrieve details of a specific time entry.

**Endpoint:** `GET /time_entries/{entryId}.json`

**Scopes:** `time:read`

**Parameters:**
- `entryId` (string, required): The unique ID of the time entry

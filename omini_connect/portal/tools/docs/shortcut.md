# Shortcut

## Overview

Shortcut is an issue tracking and project management platform that provides tools for managing stories, epics, members, and groups.

## Authentication

Shortcut uses API token authentication. Your API token can be found in your Shortcut settings under **Settings > API**.

## Base URL

```
https://api.shortcut.io/api/v1/
```

## Rate Limits

Shortcut API rate limits vary by plan. The default is 100 requests per minute. Check Shortcut documentation for your plan limits.

## Available Tools

### Stories

| Tool | Description |
|------|-------------|
| `shortcut_list_stories` | Retrieve a list of stories, optionally filtered by project or epic |
| `shortcut_get_story` | Retrieve details of a specific story by ID |
| `shortcut_create_story` | Create a new story in a project |
| `shortcut_update_story` | Update an existing story with new values |

### Epics

| Tool | Description |
|------|-------------|
| `shortcut_list_epics` | Retrieve a list of all epics |
| `shortcut_get_epic` | Retrieve details of a specific epic by ID |

### Members

| Tool | Description |
|------|-------------|
| `shortcut_list_members` | Retrieve a list of all members in the workspace |
| `shortcut_get_member` | Retrieve details of a specific member by ID |

### Groups

| Tool | Description |
|------|-------------|
| `shortcut_list_groups` | Retrieve a list of all groups in the workspace |
| `shortcut_get_group` | Retrieve details of a specific group by ID |

## Tool Details

### shortcut_list_stories

Retrieve a paginated list of stories with optional filters.

**Endpoint:** `GET /stories`

**Scopes:** `stories:read`

**Parameters:**
- `projectId` (string, optional): Filter stories by project ID
- `epicId` (string, optional): Filter stories by epic ID
- `state` (string, optional): Filter by story state
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of stories per page

### shortcut_get_story

Retrieve details of a specific story.

**Endpoint:** `GET /stories/{storyId}`

**Scopes:** `stories:read`

**Parameters:**
- `storyId` (string, required): The unique ID of the story

### shortcut_create_story

Create a new story in a project.

**Endpoint:** `POST /stories`

**Scopes:** `stories:write`

**Parameters:**
- `projectId` (string, required): Project ID to create the story in
- `name` (string, required): Story name
- `description` (string, optional): Story description
- `storyType` (string, optional): Story type (feature, chore, bug)
- `epicId` (string, optional): Epic ID to associate the story with
- `state` (string, optional): Initial story state

### shortcut_update_story

Update an existing story with new values.

**Endpoint:** `PUT /stories/{storyId}`

**Scopes:** `stories:write`

**Parameters:**
- `storyId` (string, required): The unique ID of the story
- `name` (string, optional): New story name
- `description` (string, optional): New story description
- `state` (string, optional): New story state
- `epicId` (string, optional): New epic ID

### shortcut_list_epics

Retrieve a paginated list of all epics.

**Endpoint:** `GET /epics`

**Scopes:** `epics:read`

**Parameters:**
- `projectId` (string, optional): Filter epics by project ID
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of epics per page

### shortcut_get_epic

Retrieve details of a specific epic.

**Endpoint:** `GET /epics/{epicId}`

**Scopes:** `epics:read`

**Parameters:**
- `epicId` (string, required): The unique ID of the epic

### shortcut_list_members

Retrieve a paginated list of all members in the workspace.

**Endpoint:** `GET /members`

**Scopes:** `members:read`

**Parameters:**
- `projectId` (string, optional): Filter members by project ID
- `page` (integer, optional): Page number for pagination

### shortcut_get_member

Retrieve details of a specific member.

**Endpoint:** `GET /members/{memberId}`

**Scopes:** `members:read`

**Parameters:**
- `memberId` (string, required): The unique ID of the member

### shortcut_list_groups

Retrieve a paginated list of all groups in the workspace.

**Endpoint:** `GET /groups`

**Scopes:** `groups:read`

**Parameters:**
- `page` (integer, optional): Page number for pagination
- `pageSize` (integer, optional): Number of groups per page

### shortcut_get_group

Retrieve details of a specific group.

**Endpoint:** `GET /groups/{groupId}`

**Scopes:** `groups:read`

**Parameters:**
- `groupId` (string, required): The unique ID of the group

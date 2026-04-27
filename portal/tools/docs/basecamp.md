# Basecamp Provider Documentation

## Overview

Basecamp is a project management and team communication platform that provides tools for organizing projects, managing tasks, storing documents, and coordinating teams.

## Authentication

Basecamp uses OAuth 2.0 for authentication. The following scopes are available:

- `read` - Read access to projects, messages, documents, and more
- `write` - Create and modify projects, todos, documents, and more

## Base URL

```
https://3.basecampapi.com/{account_id}/cc/api/v1
```

## Rate Limits

- Default: 300 requests per minute per user
- Burst allowance of up to 1000 requests

## Available Tools

### Projects

| Tool | Description |
|------|-------------|
| `basecamp_list_projects` | Retrieve a list of all projects accessible to the authenticated user |
| `basecamp_get_project` | Retrieve details of a specific project including its structure and participants |
| `basecamp_create_project` | Create a new project in Basecamp |

### Todos

| Tool | Description |
|------|-------------|
| `basecamp_list_todos` | Retrieve all todo items from a project, organized by todo sets |
| `basecamp_get_todo` | Retrieve details of a specific todo item including comments and assignments |
| `basecamp_create_todo` | Create a new todo item in a project's todo set |

### Documents

| Tool | Description |
|------|-------------|
| `basecamp_list_documents` | Retrieve all documents from a project's documents vault |
| `basecamp_get_document` | Retrieve details and content of a specific document |

### Events

| Tool | Description |
|------|-------------|
| `basecamp_list_events` | Retrieve recent activity events from a project |

### People

| Tool | Description |
|------|-------------|
| `basecamp_get_person` | Retrieve profile information for a specific user |

## Tool Details

### basecamp_list_projects

List all projects accessible to the authenticated user.

**Endpoint:** `GET /projects.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `status` | string | No | Filter by status: active, archived, or trashed |

**Example Request:**
```bash
curl -H "Authorization: Bearer {token}" \
     "https://3.basecampapi.com/12345/cc/api/v1/projects.json?status=active"
```

**Example Response:**
```json
[
  {
    "id": 1001,
    "name": "Website Redesign",
    "description": "Complete overhaul of company website",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-03-20T14:45:00Z",
    "archived": false,
    "participants": [
      {"name": "John Doe", "person_id": 5001}
    ]
  }
]
```

---

### basecamp_get_project

Retrieve details of a specific project including its structure and participants.

**Endpoint:** `GET /projects/{projectId}.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the project to retrieve |

---

### basecamp_create_project

Create a new project in Basecamp.

**Endpoint:** `POST /projects.json`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Name of the new project |
| `description` | string | No | Description of the project |
| `topics` | array | No | Array of topic IDs to associate with the project |
| `bookmarks` | array | No | Array of bookmark URLs to add to the project |

**Example Request:**
```bash
curl -X POST -H "Authorization: Bearer {token}" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "New Product Launch",
       "description": "Q2 product launch campaign"
     }' \
     "https://3.basecampapi.com/12345/cc/api/v1/projects.json"
```

---

### basecamp_list_todos

Retrieve all todo items from a project, organized by todo sets.

**Endpoint:** `GET /projects/{projectId}/todos.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the project |
| `filter` | string | No | Filter: active, pending, or archived |
| `page` | integer | No | Page number for pagination |

---

### basecamp_get_todo

Retrieve details of a specific todo item including comments and assignments.

**Endpoint:** `GET /projects/{projectId}/todos/{todoId}.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the parent project |
| `todoId` | integer | Yes | The ID of the todo to retrieve |

---

### basecamp_create_todo

Create a new todo item in a project's todo set.

**Endpoint:** `POST /projects/{projectId}/todos.json`

**Scopes:** `write`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the project |
| `content` | string | Yes | The todo item content/description |
| `due_on` | string | No | Due date (YYYY-MM-DD format) |
| `assignee_ids` | array | No | Array of user IDs to assign |
| `parent_todo_id` | integer | No | Parent todo ID for subtodos |

---

### basecamp_list_documents

Retrieve all documents from a project's documents vault.

**Endpoint:** `GET /projects/{projectId}/documents.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the project |
| `page` | integer | No | Page number for pagination |

---

### basecamp_get_document

Retrieve details and content of a specific document.

**Endpoint:** `GET /projects/{projectId}/documents/{documentId}.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the parent project |
| `documentId` | integer | Yes | The ID of the document to retrieve |

---

### basecamp_list_events

Retrieve recent activity events from a project.

**Endpoint:** `GET /projects/{projectId}/events.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `projectId` | integer | Yes | The ID of the project |
| `page` | integer | No | Page number for pagination |
| `since` | string | No | Return events after this timestamp (ISO 8601) |

---

### basecamp_get_person

Retrieve profile information for a specific user.

**Endpoint:** `GET /people/{userId}.json`

**Scopes:** `read`

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `userId` | integer | Yes | The ID of the user to retrieve |

---

## Common Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Rate Limit Exceeded |
| 500 | Internal Server Error |

## Additional Resources

- [Basecamp API Documentation](https://github.com/basecamp/bc3-api)
- [OAuth 2.0 Setup](https://github.com/basecamp/bc3-api#authentication)
- [Rate Limiting](https://github.com/basecamp/bc3-api#rate-limiting)

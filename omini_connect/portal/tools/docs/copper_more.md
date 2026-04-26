# Copper More

## Overview

Copper More provides extended CRM capabilities including project management, milestones, time tracking, and tagging. This integration allows AI agents to interact with these extended Copper features through OminiConnect.

## Connection

### Authentication
- **Auth Type**: OAuth 2.0
- **Provider**: Copper
- **Scopes**: `projects:read`, `projects:write`, `tasks:read`, `tasks:write`, `milestones:read`, `milestones:write`, `time_entries:read`, `time_entries:write`, `tags:read`

## Available Tools

### Projects

#### list_projects
Retrieve a list of projects from Copper CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |
| `search` | string | No | Search term for project name |

#### get_project
Retrieve detailed information for a specific project by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Project ID |

### Tasks

#### list_tasks
Retrieve a list of tasks from Copper CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |
| `project_id` | integer | No | Filter by project ID |
| `assignee_id` | integer | No | Filter by assignee user ID |
| `status` | string | No | Filter by status: open, completed |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |

#### get_task
Retrieve detailed information for a specific task by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Task ID |

### Milestones

#### list_milestones
Retrieve a list of milestones from Copper CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |
| `project_id` | integer | No | Filter by project ID |
| `status` | string | No | Filter by status |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |

#### get_milestone
Retrieve detailed information for a specific milestone by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Milestone ID |

### Time Entries

#### list_time_entries
Retrieve a list of time tracking entries from Copper CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |
| `project_id` | integer | No | Filter by project ID |
| `task_id` | integer | No | Filter by task ID |
| `user_id` | integer | No | Filter by user ID |
| `sort_by` | string | No | Field to sort by |
| `sort_order` | string | No | asc or desc |

#### get_time_entry
Retrieve detailed information for a specific time entry by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Time entry ID |

### Tags

#### list_tags
Retrieve a list of all tags in Copper CRM.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | integer | No | Page number for pagination |
| `per_page` | integer | No | Number of results per page |
| `entity_type` | string | No | Filter by entity type (project, task, etc.) |

#### get_tag
Retrieve detailed information for a specific tag by ID.

**Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | integer | Yes | Tag ID |

## Usage Example

```json
{
  "tool": "copper_more_list_projects",
  "parameters": {
    "per_page": 20,
    "search": "Q1 Marketing"
  }
}
```

## Rate Limits

Please refer to Copper API documentation for current rate limits.
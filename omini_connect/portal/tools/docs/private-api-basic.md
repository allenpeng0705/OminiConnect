# Private API Basic Auth Tools

Provider: `private-api-basic` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap a Private API using Basic Authentication. They allow AI agents to manage resources and items through a generic CRUD interface. **Requires Private API Basic Authentication (username + password).**

## Authentication

**Nango Basic Auth**:
- Uses username and password for Basic authentication
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://my-private-api

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `private_basic_list_resources` | List resources | GET | /resources |
| `private_basic_get_resource` | Get resource details | GET | /resources/{resourceId} |
| `private_basic_create_resource` | Create a resource | POST | /resources |
| `private_basic_update_resource` | Update a resource | PUT | /resources/{resourceId} |
| `private_basic_delete_resource` | Delete a resource | DELETE | /resources/{resourceId} |
| `private_basic_list_items` | List items | GET | /items |
| `private_basic_get_item` | Get item details | GET | /items/{itemId} |
| `private_basic_search` | Search resources | GET | /search |
| `private_basic_get_status` | Get API status | GET | /status |
| `private_basic_get_health` | Get health check | GET | /health |

---

## Tool Details

### private_basic_list_resources

**What it does**: Lists all resources from the private API.

**When to use**: Browse available resources.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all resources"

---

### private_basic_get_resource

**What it does**: Gets detailed information about a specific resource.

**When to use**: Get resource details.

**Arguments**:
- `resourceId` (required): Resource ID

**Example LLM prompt**: "Get details for resource 12345"

---

### private_basic_create_resource

**What it does**: Creates a new resource.

**When to use**: Add new data to the API.

**Arguments**:
- `name` (required): Resource name
- `data` (optional): Resource data

**Example LLM prompt**: "Create a new resource called 'Test'"

---

### private_basic_update_resource

**What it does**: Updates an existing resource.

**When to use**: Modify existing data.

**Arguments**:
- `resourceId` (required): Resource ID
- `name` (optional): Resource name
- `data` (optional): Resource data

**Example LLM prompt**: "Update resource 12345 with new data"

---

### private_basic_delete_resource

**What it does**: Deletes a resource.

**When to use**: Remove data from the API.

**Arguments**:
- `resourceId` (required): Resource ID

**Example LLM prompt**: "Delete resource 12345"

---

### private_basic_list_items

**What it does**: Lists all items from the private API.

**When to use**: Browse items.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all items"

---

### private_basic_get_item

**What it does**: Gets detailed information about a specific item.

**When to use**: Get item details.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Get details for item 67890"

---

### private_basic_search

**What it does**: Searches resources by query.

**When to use**: Find specific data.

**Arguments**:
- `q` (required): Search query

**Example LLM prompt**: "Search for 'test'"

---

### private_basic_get_status

**What it does**: Gets API status information.

**When to use**: Check API availability.

**Arguments**: None

**Example LLM prompt**: "What's the API status?"

---

### private_basic_get_health

**What it does**: Gets health check information.

**When to use**: Verify API health.

**Arguments**: None

**Example LLM prompt**: "Check API health"

---

## Private API Basic Auth Notes

- **Basic Auth**: Uses username/password for authentication
- **Generic Template**: Adapt endpoints to your specific API
- **Rate limits**: Apply to API calls

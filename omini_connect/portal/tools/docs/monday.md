# Monday Tools

Provider: `monday` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Monday.com API. They allow AI agents to interact with boards, items, updates, users, and groups on behalf of the authenticated workspace.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `monday_list_boards` | List all accessible boards | GET | /boards |
| `monday_get_board` | Get board details | GET | /boards/{board_id} |
| `monday_create_board` | Create a new board | POST | /boards |
| `monday_list_items` | List items in a board | GET | /boards/{board_id}/items |
| `monday_get_item` | Get item details | GET | /boards/{board_id}/items/{item_id} |
| `monday_create_item` | Create a new item | POST | /boards/{board_id}/items |
| `monday_update_item` | Update an item | PUT | /boards/{board_id}/items/{item_id} |
| `monday_list_updates` | List updates on an item | GET | /boards/{board_id}/items/{item_id}/updates |
| `monday_get_update` | Get update details | GET | /updates/{update_id} |
| `monday_list_users` | List workspace users | GET | /users |

---

## Tool Details

### monday_list_boards

**What it does**: Returns a list of all boards the authenticated user has access to. Includes board name, state, views, and activity.

**When to use**: Browse available boards before creating items or searching for specific work.

**Arguments**:
- `limit` (optional): Max boards to return (default 100)
- `state` (optional): Filter by state (`all`, `active`, `archived`, `deleted`)

**Example LLM prompt**: "Show me all active boards in our workspace"

---

### monday_get_board

**What it does**: Returns detailed information about a specific board including columns, views, groups, and activity log.

**When to use**: Get board configuration, understand column structure, or view board-wide settings.

**Arguments**:
- `board_id` (required): Board ID

**Example LLM prompt**: "Show me details of board 12345678"

---

### monday_create_board

**What it does**: Creates a new board in a workspace with the given name and type.

**When to use**: Create new projects, workspaces, or tracking boards from conversation.

**Arguments**:
- `board_name` (required): Board name
- `workspace_id` (required): Workspace ID
- `board_kind` (optional): Board kind (`public`, `private`, `shareable`)

**Example LLM prompt**: "Create a new board called 'Product Roadmap' in workspace 123"

---

### monday_list_items

**What it does**: Returns a list of all items (tasks) in a board. Can be filtered by group or limited to a specific page.

**When to use**: Browse tasks in a board, find items by group, or get an overview of board contents.

**Arguments**:
- `board_id` (required): Board ID
- `group_id` (optional): Filter by group ID
- `limit` (optional): Max items to return (default 100)

**Example LLM prompt**: "What items are in the 'Sprint 1' group of board 123?"

---

### monday_get_item

**What it does**: Returns detailed information about a specific item including all column values, assignees, and subitems.

**When to use**: Get full item context before editing, updating, or adding comments.

**Arguments**:
- `board_id` (required): Board ID
- `item_id` (required): Item ID

**Example LLM prompt**: "Show me details of item 987654321"

---

### monday_create_item

**What it does**: Creates a new item (task) in a board with the given name, column values, and group.

**When to use**: Create bugs, tasks, features, or any work item from conversation.

**Arguments**:
- `board_id` (required): Board ID
- `group_id` (optional): Group ID to add item to
- `item_name` (required): Item name/title
- `column_values` (optional): JSON string of column values

**Example LLM prompt**: "Create an item called 'Fix login bug' in board 123 with priority High"

---

### monday_update_item

**What it does**: Updates an existing item. Can change name, column values, or move to a different group.

**When to use**: Change item details, update column values, or reorganize items between groups.

**Arguments**:
- `board_id` (required): Board ID
- `item_id` (required): Item ID
- `item_name` (optional): New item name
- `column_values` (optional): JSON string of column values
- `group_id` (optional): Move to group ID

**Example LLM prompt**: "Update item 987654321 to mark it as Done"

---

### monday_list_updates

**What it does**: Returns a list of all updates (comments) on an item. Includes text, author, and timestamp.

**When to use**: View conversation history on an item, find context, or see who has commented.

**Arguments**:
- `board_id` (required): Board ID
- `item_id` (required): Item ID
- `limit` (optional): Max updates to return (default 25)

**Example LLM prompt**: "Show me all updates on item 987654321"

---

### monday_get_update

**What it does**: Returns detailed information about a specific update including text, author, and metadata.

**When to use**: Get full context of a specific comment or update.

**Arguments**:
- `update_id` (required): Update ID

**Example LLM prompt**: "Show me update 555555"

---

### monday_list_users

**What it does**: Returns a list of all users in a workspace.

**When to use**: Find team members, assign items, or see who has access to a workspace.

**Arguments**:
- `workspace_id` (required): Workspace ID
- `limit` (optional): Max users to return (default 100)

**Example LLM prompt**: "Who are all the users in workspace 123?"

---

## Monday API Reference

These tools use the Monday.com API. See official docs for full details:
- https://developer.monday.com/docs/api/
- Rate limits: Varies by plan
- Pagination: Use `limit` parameter
- All dates: ISO 8601 format (UTC)

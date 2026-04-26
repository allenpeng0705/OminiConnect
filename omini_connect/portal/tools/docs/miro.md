# Miro Tools

Provider: `miro` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Miro API. They allow AI agents to manage boards, items, members, and teams. Miro is a visual collaboration platform for teams to brainstorm, plan, and design together.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Miro
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `boards:read`, `boards:write`, `teams:read`, `teams:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `miro_list_boards` | List all boards | GET | /boards |
| `miro_get_board` | Get board details | GET | /boards/{board_id} |
| `miro_create_board` | Create a new board | POST | /boards |
| `miro_list_items` | List items on a board | GET | /boards/{board_id}/items |
| `miro_get_item` | Get item details | GET | /boards/{board_id}/items/{item_id} |
| `miro_create_item` | Create an item on a board | POST | /boards/{board_id}/items |
| `miro_list_members` | List board members | GET | /boards/{board_id}/members |
| `miro_get_member` | Get member details | GET | /boards/{board_id}/members/{user_id} |
| `miro_list_teams` | List all teams | GET | /teams |
| `miro_get_team` | Get team details | GET | /teams/{team_id} |

---

## Tool Details

### miro_list_boards

**What it does**: Lists all Miro boards the user has access to.

**When to use**: Find boards to work with, browse available workspaces, or discover team boards.

**Arguments**:
- `limit` (optional): Number of results (default 50)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all my Miro boards"

---

### miro_get_board

**What it does**: Gets details about a specific Miro board including its structure and settings.

**When to use**: Understand board organization before working with items or members.

**Arguments**:
- `board_id` (required): Board ID

**Example LLM prompt**: "Get details for board abc123"

---

### miro_create_board

**What it does**: Creates a new Miro board in a team or workspace.

**When to use**: Start a new visual collaboration space for brainstorming or planning.

**Arguments**:
- `name` (required): Board name
- `description` (optional): Board description
- `team_id` (optional): Team ID to create board in
- `is_private` (optional): Whether the board is private

**Example LLM prompt**: "Create a new board called 'Sprint Planning'"

---

### miro_list_items

**What it does**: Lists all items on a Miro board. Items include frames, sticky notes, shapes, text, images, and more.

**When to use**: Explore board content, find specific items, or analyze visual layouts.

**Arguments**:
- `board_id` (required): Board ID
- `limit` (optional): Number of results (default 50)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all items on board abc123"

---

### miro_get_item

**What it does**: Gets details about a specific item on a Miro board.

**When to use**: Get detailed information about a specific frame, sticky note, shape, or other item.

**Arguments**:
- `board_id` (required): Board ID
- `item_id` (required): Item ID

**Example LLM prompt**: "Get details for item xyz on board abc123"

---

### miro_create_item

**What it does**: Creates a new item on a Miro board. Supports various item types including sticky notes, shapes, text, and more.

**When to use**: Add content to boards, create visual structures, or annotate designs.

**Arguments**:
- `board_id` (required): Board ID
- `type` (required): Item type (e.g., sticky_note, shape, text)
- `data` (optional): Item data based on type
- `position` (optional): Position on the board

**Example LLM prompt**: "Add a sticky note saying 'Review needed' to board abc123"

---

### miro_list_members

**What it does**: Lists all members of a Miro board.

**When to use**: See who has access to a board, find collaborators, or check permissions.

**Arguments**:
- `board_id` (required): Board ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all members of board abc123"

---

### miro_get_member

**What it does**: Gets details about a specific member of a Miro board.

**When to use**: Get member profile information and role on a board.

**Arguments**:
- `board_id` (required): Board ID
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user xyz on board abc123"

---

### miro_list_teams

**What it does**: Lists all Miro teams the user belongs to.

**When to use**: Find teams to work with, list available workspaces.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all my Miro teams"

---

### miro_get_team

**What it does**: Gets details about a specific Miro team.

**When to use**: Understand team structure before listing members or creating boards.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team abc123"

---

## Miro API Notes

- **Teams**: Miro teams contain members and boards. Users can belong to multiple teams.
- **Boards**: Boards are the primary workspace in Miro, containing items, frames, and comments.
- **Items**: Various content types including frames, sticky notes, shapes, text, images, connectors, and widgets.
- **Members**: Users who have access to a board with specific roles (owner, editor, commenter, viewer).
- **Pagination**: Use cursor-based pagination for large result sets.
- **Rate Limits**: 200 requests per minute per token.

# FigJam Tools

Provider: `figjam` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the FigJam API. They allow AI agents to interact with boards, items, members, teams, and comments in the FigJam collaborative whiteboard platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `boards:read`, `boards:write`, `members:read`, `teams:read`, `comments:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `figjam_list_boards` | List all boards | GET | /boards |
| `figjam_get_board` | Get details of a specific board | GET | /boards/{id} |
| `figjam_create_board` | Create a new board | POST | /boards |
| `figjam_list_items` | List items on a board | GET | /boards/{board_id}/items |
| `figjam_get_item` | Get details of a specific item | GET | /boards/{board_id}/items/{id} |
| `figjam_list_members` | List all members | GET | /members |
| `figjam_get_member` | Get details of a specific member | GET | /members/{id} |
| `figjam_list_teams` | List all teams | GET | /teams |
| `figjam_get_team` | Get details of a specific team | GET | /teams/{id} |
| `figjam_list_comments` | List comments on a board | GET | /boards/{board_id}/comments |

---

## Tool Details

### figjam_list_boards

**What it does**: Returns a paginated list of FigJam boards the authenticated user has access to.

**When to use**: Browse available whiteboards, find recent boards, or filter by team.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `sort` (optional): Sort order: modified, created, name
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all boards modified in the last week"

---

### figjam_get_board

**What it does**: Get details of a specific board including name, description, and metadata.

**When to use**: Get board information before adding items or comments.

**Arguments**:
- `id` (required): Board ID

**Example LLM prompt**: "Get details for board board123"

---

### figjam_create_board

**What it does**: Creates a new FigJam board, optionally under a team and with a template.

**When to use**: Start a new brainstorming session or create a workspace for a project.

**Arguments**:
- `name` (required): Board name
- `description` (optional): Board description
- `team_id` (optional): Team ID to create the board under
- `template_id` (optional): Use a template as starting point

**Example LLM prompt**: "Create a new board called 'Sprint Planning' under team team456"

---

### figjam_list_items

**What it does**: Returns a paginated list of all items on a board including shapes, sticky notes, connectors, and text.

**When to use**: Retrieve board content for analysis or find specific elements.

**Arguments**:
- `board_id` (required): Board ID
- `type` (optional): Filter by item type: shape, sticky, text, connector, embed
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all sticky notes on board board123"

---

### figjam_get_item

**What it does**: Get details of a specific item including position, size, style, and content.

**When to use**: Read the content of a specific shape, sticky note, or text element.

**Arguments**:
- `board_id` (required): Board ID
- `id` (required): Item ID

**Example LLM prompt**: "Get the content of item item789 on board board123"

---

### figjam_list_members

**What it does**: Returns a paginated list of all members in the workspace with their roles.

**When to use**: View workspace members or check access permissions.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `role` (optional): Filter by role: owner, editor, viewer
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all editors in the workspace"

---

### figjam_get_member

**What it does**: Get details of a specific member including email, name, role, and last active time.

**When to use**: Look up member details or check when someone was last active.

**Arguments**:
- `id` (required): Member ID

**Example LLM prompt**: "Get details for member mem123"

---

### figjam_list_teams

**What it does**: Returns a paginated list of all teams with their member counts and board counts.

**When to use**: Browse teams in the workspace or understand organizational structure.

**Arguments**:
- `per_page` (optional): Results per page (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all teams in the workspace"

---

### figjam_get_team

**What it does**: Get details of a specific team including name, description, members, and boards.

**When to use**: View team composition or find boards within a team.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Get details for team team456 including all its members"

---

### figjam_list_comments

**What it does**: Returns a paginated list of all comments on a board with author and resolved status.

**When to use**: Review feedback on a board or find unresolved discussion points.

**Arguments**:
- `board_id` (required): Board ID
- `resolved` (optional): Filter by resolved status (boolean)
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Show all unresolved comments on board board123"

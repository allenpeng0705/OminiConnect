# Miro Additional Tools

Provider: `miro_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Miro REST API (v2). They allow AI agents to interact with teams, members, boards, projects, and frames in Miro's visual collaboration platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `boards:read`, `boards:write` etc.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `miro_more_list_teams` | List teams in the organization | GET | /v2/teams |
| `miro_more_get_team` | Get a specific team | GET | /v2/teams/{teamId} |
| `miro_more_list_members` | List members of a team | GET | /v2/teams/{teamId}/members |
| `miro_more_get_member` | Get a specific team member | GET | /v2/teams/{teamId}/members/{userId} |
| `miro_more_list_boards` | List all accessible boards | GET | /v2/boards |
| `miro_more_get_board` | Get a specific board | GET | /v2/boards/{boardId} |
| `miro_more_create_board` | Create a new board | POST | /v2/boards |
| `miro_more_list_projects` | List projects in a team | GET | /v2/projects |
| `miro_more_get_project` | Get a specific project | GET | /v2/projects/{projectId} |
| `miro_more_list_frames` | List frames on a board | GET | /v2/boards/{boardId}/frames |

---

## Tool Details

### miro_more_list_teams

**What it does**: Returns a paginated list of all teams in the organization that the authenticated user belongs to.

**When to use**: Browse available teams to find members, boards, or projects.

**Arguments**:
- `cursor` (optional): Pagination cursor for next page
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all teams in our Miro organization"

---

### miro_more_get_team

**What it does**: Returns details of a specific team including name, description, and member count.

**When to use**: Get team context before listing members or managing access.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get details for the Design Systems team"

---

### miro_more_list_members

**What it does**: Lists all members of a specific team with their user information.

**When to use**: Find team members or verify who has access to a team.

**Arguments**:
- `teamId` (required): Team ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "Show me all members of the Product team"

---

### miro_more_get_member

**What it does**: Returns details of a specific team member including their role and permissions.

**When to use**: Check a user's role or access level within a team.

**Arguments**:
- `teamId` (required): Team ID
- `userId` (required): User ID

**Example LLM prompt**: "What is alice@example.com's role in the Design Systems team?"

---

### miro_more_list_boards

**What it does**: Lists all boards accessible to the authenticated user, optionally filtered by team.

**When to use**: Browse boards to find specific projects or collaboration spaces.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all boards in the Product team workspace"

---

### miro_more_get_board

**What it does**: Returns details of a specific board including name, description, owner, and sharing settings.

**When to use**: Get board metadata before accessing its contents or frames.

**Arguments**:
- `boardId` (required): Board ID

**Example LLM prompt**: "Get details for board bXVYz123456789"

---

### miro_more_create_board

**What it does**: Creates a new board in a specified team. The board starts empty and can be populated with frames and widgets.

**When to use**: Initialize a new project space or collaboration canvas.

**Arguments**:
- `name` (required): Board name
- `description` (optional): Board description
- `team_id` (optional): Team ID to create the board in
- `is_private` (optional): Make board private (default false)

**Example LLM prompt**: "Create a new board called 'Q2 Planning' in the Product team"

---

### miro_more_list_projects

**What it does**: Lists all projects in a team. Projects are top-level containers that group related boards together.

**When to use**: Organize boards by initiative or find boards within a project.

**Arguments**:
- `team_id` (optional): Filter by team ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "What projects exist in the Design Systems team?"

---

### miro_more_get_project

**What it does**: Returns details of a specific project including its name, description, and associated boards.

**When to use**: Understand the scope of a project before accessing its boards.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project proj123456789"

---

### miro_more_list_frames

**What it does**: Lists all frames on a specific board. Frames are containers that organize content.

**When to use**: Browse a board's structure to find specific content or sections.

**Arguments**:
- `boardId` (required): Board ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "Show me all frames on the Q2 Planning board"

---

## Miro API Reference

These tools use the Miro REST API v2. See official docs for full details:
- https://developers.miro.com/reference
- Rate limits: Vary by plan
- Pagination: Use cursor for efficient traversal
- All dates: ISO 8601 format

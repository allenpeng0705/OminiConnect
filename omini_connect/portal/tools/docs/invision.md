# InVision Tools

Provider: `invision` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the InVision API. They allow AI agents to browse prototypes, screens, comments, users, and teams. InVision is a design collaboration and prototyping platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with InVision
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `prototypes:read`, `screens:read`, `comments:read`, `users:read`, `teams:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `invision_list_prototypes` | List all prototypes | GET | /api/v1/prototypes |
| `invision_get_prototype` | Get prototype details | GET | /api/v1/prototypes/{prototype_id} |
| `invision_list_screens` | List screens in a prototype | GET | /api/v1/prototypes/{prototype_id}/screens |
| `invision_get_screen` | Get screen details | GET | /api/v1/screens/{screen_id} |
| `invision_list_comments` | List comments on a screen | GET | /api/v1/screens/{screen_id}/comments |
| `invision_get_comment` | Get comment details | GET | /api/v1/comments/{comment_id} |
| `invision_list_users` | List users | GET | /api/v1/users |
| `invision_get_user` | Get user details | GET | /api/v1/users/{user_id} |
| `invision_list_teams` | List teams | GET | /api/v1/teams |
| `invision_get_team` | Get team details | GET | /api/v1/teams/{team_id} |

---

## Tool Details

### invision_list_prototypes

**What it does**: Lists all InVision prototypes the user has access to.

**When to use**: Browse available prototypes, find specific projects, or explore design collaborations.

**Arguments**:
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all my InVision prototypes"

---

### invision_get_prototype

**What it does**: Gets details about a specific InVision prototype including its screens and metadata.

**When to use**: Understand prototype structure before exploring screens or comments.

**Arguments**:
- `prototype_id` (required): Prototype ID

**Example LLM prompt**: "Get details for prototype abc123"

---

### invision_list_screens

**What it does**: Lists all screens in an InVision prototype with thumbnails and metadata.

**When to use**: Browse screens in a prototype, find specific designs, or extract screen flow.

**Arguments**:
- `prototype_id` (required): Prototype ID
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all screens in prototype abc123"

---

### invision_get_screen

**What it does**: Gets details about a specific screen including its metadata and annotations.

**When to use**: Get detailed screen information, view annotation data, or check screen metadata.

**Arguments**:
- `screen_id` (required): Screen ID

**Example LLM prompt**: "Get details for screen xyz"

---

### invision_list_comments

**What it does**: Lists all comments on an InVision screen or prototype.

**When to use**: Read design feedback, review discussions, or understand stakeholder comments.

**Arguments**:
- `screen_id` (required): Screen ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "What comments are on screen abc123?"

---

### invision_get_comment

**What it does**: Gets details about a specific comment including author and timestamp.

**When to use**: Read full comment details, see annotation location, or check reply threads.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment abc123"

---

### invision_list_users

**What it does**: Lists all users in the InVision workspace.

**When to use**: Find team members, discover collaborators, or check user profiles.

**Arguments**:
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all users in the workspace"

---

### invision_get_user

**What it does**: Gets details about a specific InVision user including their profile and activity.

**When to use**: Get user information, see their projects, or find contact details.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user xyz"

---

### invision_list_teams

**What it does**: Lists all teams in the InVision workspace.

**When to use**: Browse team structure, find team members, or explore team projects.

**Arguments**:
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all teams in the workspace"

---

### invision_get_team

**What it does**: Gets details about a specific InVision team including members and projects.

**When to use**: Understand team structure, see team members, or explore team collaborations.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team abc123"

---

## InVision API Notes

- **Prototypes**: Interactive mockups that simulate app or website flows.
- **Screens**: Individual design frames within a prototype.
- **Comments**: Feedback attached to specific locations on screens.
- **Users**: Workspace members with various roles and permissions.
- **Teams**: Groups of users collaborating on shared projects.
- **Pagination**: Use offset-based pagination for large result sets.

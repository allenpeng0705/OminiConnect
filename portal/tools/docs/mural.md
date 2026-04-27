# Mural Tools

Provider: `mural` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Mural REST API. They allow AI agents to interact with rooms, murals, templates, and users in Mural's visual collaboration platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `room:read`, `room:write`, `mural:read`, `mural:write`, `template:read`, `workspace:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mural_list_rooms` | List rooms in the workspace | GET | /v1/rooms |
| `mural_get_room` | Get a specific room | GET | /v1/rooms/{roomId} |
| `mural_create_room` | Create a new room | POST | /v1/rooms |
| `mural_list_murals` | List murals in a room | GET | /v1/rooms/{roomId}/murals |
| `mural_get_mural` | Get a specific mural | GET | /v1/murals/{muralId} |
| `mural_create_mural` | Create a new mural | POST | /v1/rooms/{roomId}/murals |
| `mural_list_templates` | List available templates | GET | /v1/templates |
| `mural_get_template` | Get a specific template | GET | /v1/templates/{templateId} |
| `mural_list_users` | List users in workspace | GET | /v1/users |
| `mural_get_user` | Get a specific user | GET | /v1/users/{userId} |

---

## Tool Details

### mural_list_rooms

**What it does**: Returns a paginated list of all rooms in the workspace. Rooms are virtual workspaces containing murals and team members.

**When to use**: Browse available rooms to find specific collaboration spaces.

**Arguments**:
- `workspace_id` (optional): Filter by workspace ID
- `cursor` (optional): Pagination cursor for next page
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all rooms in our Mural workspace"

---

### mural_get_room

**What it does**: Returns details of a specific room including its name, description, member count, and mural count.

**When to use**: Get room context before accessing its murals.

**Arguments**:
- `roomId` (required): Room ID

**Example LLM prompt**: "Get details for room room123456"

---

### mural_create_room

**What it does**: Creates a new room in a workspace. Rooms are containers for murals and provide collaborative spaces for teams.

**When to use**: Set up a new project space or team area.

**Arguments**:
- `name` (required): Room name
- `description` (optional): Room description
- `workspace_id` (optional): Workspace ID to create the room in
- `is_private` (optional): Make room private (default false)

**Example LLM prompt**: "Create a new room called 'Sprint Planning' in workspace ws-123"

---

### mural_list_murals

**What it does**: Lists all murals within a specific room. Murals are visual collaboration canvases with sticky notes, images, shapes, and other elements.

**When to use**: Browse a room's murals to find specific content or collaboration spaces.

**Arguments**:
- `roomId` (required): Room ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "Show me all murals in the Sprint Planning room"

---

### mural_get_mural

**What it does**: Returns details of a specific mural including its name, description, size, and current state.

**When to use**: Get mural metadata before accessing its contents.

**Arguments**:
- `muralId` (required): Mural ID

**Example LLM prompt**: "Get details for mural mural-789xyz"

---

### mural_create_mural

**What it does**: Creates a new mural in a specified room. Murals start as blank canvases that can be populated with visual elements. Optionally use a template to get started.

**When to use**: Create a new visual collaboration space for a meeting, brainstorm, or workshop.

**Arguments**:
- `roomId` (required): Room ID
- `name` (required): Mural name
- `description` (optional): Mural description
- `template_id` (optional): Template ID to use for the mural
- `width` (optional): Mural width in pixels (default 3000)
- `height` (optional): Mural height in pixels (default 2000)

**Example LLM prompt**: "Create a new mural called 'Q2 Roadmap' in room room-123 using the Kanban template"

---

### mural_list_templates

**What it does**: Lists all available templates in the workspace. Templates provide predefined structures for murals including brainstorming frameworks and workshops.

**When to use**: Find appropriate templates before creating a new mural.

**Arguments**:
- `workspace_id` (optional): Filter by workspace ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "What templates are available for retrospectives?"

---

### mural_get_template

**What it does**: Returns details of a specific template including its name, description, and structure.

**When to use**: Understand what a template offers before creating a mural from it.

**Arguments**:
- `templateId` (required): Template ID

**Example LLM prompt**: "Get details for template tmpl-456"

---

### mural_list_users

**What it does**: Lists all users in the workspace with their name, email, and role information.

**When to use**: Find team members or check workspace membership.

**Arguments**:
- `workspace_id` (optional): Filter by workspace ID
- `cursor` (optional): Pagination cursor
- `limit` (optional): Number of results per page (max 50, default 20)

**Example LLM prompt**: "List all users in our workspace"

---

### mural_get_user

**What it does**: Returns details of a specific user including their name, email, avatar, and permissions.

**When to use**: Verify user information or check their role in the workspace.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user alice@example.com"

---

## Mural API Reference

These tools use the Mural API. See official docs for full details:
- https://developers.mural.co/
- Rate limits: Vary by plan
- Pagination: Use cursor for efficient traversal
- All dates: ISO 8601 format

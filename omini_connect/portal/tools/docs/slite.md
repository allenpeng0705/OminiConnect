# Slite Tools

Provider: `slite` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Slite API. They allow AI agents to manage notes, channels, members, and permissions. Slite is a team knowledge platform focused on notes, documentation, and collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Slite
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `notes:read`, `notes:write`, `channels:read`, `channels:write`, `members:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `slite_list_notes` | List notes | GET | /api/v1/notes |
| `slite_get_note` | Get note details | GET | /api/v1/notes/{id} |
| `slite_create_note` | Create a new note | POST | /api/v1/notes |
| `slite_list_channels` | List channels | GET | /api/v1/channels |
| `slite_get_channel` | Get channel details | GET | /api/v1/channels/{id} |
| `slite_list_members` | List members | GET | /api/v1/members |
| `slite_get_member` | Get member details | GET | /api/v1/members/{id} |
| `slite_search_notes` | Search notes | GET | /api/v1/notes/search |
| `slite_share_note` | Share a note | POST | /api/v1/notes/{id}/share |
| `slite_update_permissions` | Update note permissions | PUT | /api/v1/notes/{id}/permissions |

---

## Tool Details

### slite_list_notes

**What it does**: Lists all notes with optional filtering by channel or status.

**When to use**: Browse notes, find notes in a channel, see active or archived notes.

**Arguments**:
- `channel_id` (optional): Filter by channel ID
- `status` (optional): Filter by status (active, archived)
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all notes in channel abc-123"

---

### slite_get_note

**What it does**: Gets detailed information about a specific note including title, content, author, and timestamps.

**When to use**: Read note content, understand authorship and history.

**Arguments**:
- `id` (required): Note ID

**Example LLM prompt**: "Get details for note xyz-456"

---

### slite_create_note

**What it does**: Creates a new note with title, content, and optional channel association.

**When to use**: Create new documentation, capture team knowledge.

**Arguments**:
- `title` (required): Note title
- `content` (optional): Note content (Markdown)
- `channel_id` (optional): Channel ID

**Example LLM prompt**: "Create a new note titled 'Q2 Roadmap' in channel abc-123"

---

### slite_list_channels

**What it does**: Lists all channels with optional filtering by type.

**When to use**: Browse channels, find topic areas, see team workspaces.

**Arguments**:
- `type` (optional): Filter by channel type
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all channels"

---

### slite_get_channel

**What it does**: Gets detailed information about a specific channel including name, description, and member count.

**When to use**: Understand channel purpose, see member count.

**Arguments**:
- `id` (required): Channel ID

**Example LLM prompt**: "Get details for channel xyz-456"

---

### slite_list_members

**What it does**: Lists all members with optional filtering by role or status.

**When to use**: Find team members, see who has access.

**Arguments**:
- `role` (optional): Filter by role
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all members with admin role"

---

### slite_get_member

**What it does**: Gets detailed information about a specific member including name, email, and role.

**When to use**: Check member details, verify permissions.

**Arguments**:
- `id` (required): Member ID

**Example LLM prompt**: "Get details for member uuu-999"

---

### slite_search_notes

**What it does**: Searches notes by keyword with optional channel filtering.

**When to use**: Find specific content, search documentation by keyword.

**Arguments**:
- `q` (required): Search query
- `channel_id` (optional): Limit search to channel ID
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "Search for notes containing 'roadmap'"

---

### slite_share_note

**What it does**: Shares a note with specific members or makes it publicly accessible.

**When to use**: Collaborate with team members, publish note externally.

**Arguments**:
- `id` (required): Note ID
- `member_ids` (optional): Member IDs to share with
- `public` (optional): Make note publicly accessible

**Example LLM prompt**: "Share note xyz-456 with member uuu-999"

---

### slite_update_permissions

**What it does**: Updates permissions for a note. Changes access level for specific members.

**When to use**: Change member access from view to edit, revoke access.

**Arguments**:
- `id` (required): Note ID
- `member_id` (required): Member ID
- `permission` (required): Permission level (view, edit, admin)

**Example LLM prompt**: "Update note xyz-456 to give member uuu-999 edit permission"

---

## Slite API Notes

- **Notes**: The main content unit in Slite. Support Markdown content, channels, and sharing.
- **Channels**: Organizational units that contain notes. Can represent teams, topics, or projects.
- **Members**: Team members with roles and permissions. Can be invited to channels and notes.
- **Permissions**: Access levels (view, edit, admin) control what members can do with notes.
- **Sharing**: Notes can be shared with specific members or made publicly accessible.
- **Search**: Full-text search across note titles and content with relevance ranking.
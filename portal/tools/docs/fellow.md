# Fellow Tools

Provider: `fellow` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fellow API. They allow AI agents to manage meetings, notes, agenda items, and action items. Fellow is a meeting management and productivity platform.

## Authentication

**Nango API_KEY**:
- User provides workspace subdomain and API key via Nango Connect
- Key is passed in the x-api-key header
- Subdomain determines API base URL: `https://{subdomain}.fellow.app/api`
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fellow_get_user` | Get current user | GET | /v1/me |
| `fellow_list_meetings` | List meetings | GET | /v1/meetings |
| `fellow_get_meeting` | Get meeting details | GET | /v1/meetings/{id} |
| `fellow_list_notes` | List notes | GET | /v1/notes |
| `fellow_get_note` | Get note details | GET | /v1/notes/{id} |
| `fellow_list_agenda_items` | List agenda items | GET | /v1/meetings/{meetingId}/agenda_items |
| `fellow_list_actions` | List action items | GET | /v1/action_items |
| `fellow_get_action` | Get action item details | GET | /v1/action_items/{id} |
| `fellow_list_reflections` | List reflections | GET | /v1/reflections |
| `fellow_get_workspace` | Get workspace info | GET | /v1/workspace |

---

## Tool Details

### fellow_get_user

**What it does**: Gets current authenticated user profile.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Get my Fellow profile"

---

### fellow_list_meetings

**What it does**: Lists all meetings in the workspace.

**When to use**: Browse meeting history.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List my meetings"

---

### fellow_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: View meeting information, agenda, notes.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting abc123"

---

### fellow_list_notes

**What it does**: Lists all notes in the workspace.

**When to use**: Browse notes, find meeting notes.

**Arguments**:
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "List notes for meeting abc123"

---

### fellow_get_note

**What it does**: Gets details of a specific note.

**When to use**: View note content.

**Arguments**:
- `id` (required): Note ID

**Example LLM prompt**: "Get note xyz789"

---

### fellow_list_agenda_items

**What it does**: Lists agenda items for a meeting.

**When to use**: View meeting agenda.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "List agenda items for meeting abc123"

---

### fellow_list_actions

**What it does**: Lists all action items.

**When to use**: Track tasks, find assigned items.

**Arguments**:
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "List all action items"

---

### fellow_get_action

**What it does**: Gets details of a specific action item.

**When to use**: View action details, assignee, due date.

**Arguments**:
- `id` (required): Action item ID

**Example LLM prompt**: "Get action item def456"

---

### fellow_list_reflections

**What it does**: Lists all reflections.

**When to use**: View meeting reflections, learning.

**Arguments**: None

**Example LLM prompt**: "List all reflections"

---

### fellow_get_workspace

**What it does**: Gets workspace information.

**When to use**: Get workspace details, settings.

**Arguments**: None

**Example LLM prompt**: "Get workspace information"

---

## Fellow API Notes

- **Workspace Subdomain**: Each Fellow workspace has a unique subdomain
- **Meetings**: Core entity with agenda, notes, and actions
- **Notes**: Meeting notes and documentation
- **Agenda Items**: Structured meeting agenda
- **Action Items**: Tasks and follow-ups from meetings
- **Reflections**: Meeting learnings and insights

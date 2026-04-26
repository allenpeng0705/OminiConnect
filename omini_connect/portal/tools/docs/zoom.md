# Zoom Tools

Provider: `zoom` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Zoom API. They allow AI agents to manage users, meetings, participants, and cloud recordings. Zoom is the leading video conferencing platform for teams.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoom
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `zoom_user:read:list_users`, `zoom_meeting:read:list_meetings`, `zoom_meeting:write:create`, `zoom_meeting:write:update`, `zoom_meeting:write:delete`, `zoom_recording:read:list_recordings`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoom_list_meetings` | List meetings | GET | /users/{user_id}/meetings |
| `zoom_get_meeting` | Get meeting details | GET | /meetings/{meeting_id} |
| `zoom_create_meeting` | Create a meeting | POST | /users/{user_id}/meetings |
| `zoom_update_meeting` | Update a meeting | PUT | /meetings/{meeting_id} |
| `zoom_delete_meeting` | Delete a meeting | DELETE | /meetings/{meeting_id} |
| `zoom_list_participants` | List meeting participants | GET | /meetings/{meeting_id}/participants |
| `zoom_get_participant` | Get participant details | GET | /meetings/{meeting_id}/participants/{participant_id} |
| `zoom_list_recordings` | List cloud recordings | GET | /users/{user_id}/recordings |
| `zoom_get_recording` | Get recording details | GET | /meetings/{meeting_id}/recordings |
| `zoom_list_users` | List users | GET | /users |

---

## Tool Details

### zoom_list_meetings

**What it does**: Lists meetings for a user, including upcoming and past meetings.

**When to use**: Find scheduled meetings, see meeting history.

**Arguments**:
- `user_id` (required): User ID (or `me` for current user)
- `type` (optional): `upcoming`, `live`, or `past`
- `page_size` (optional): Max meetings (default 30)

**Example LLM prompt**: "List all upcoming meetings for user me"

---

### zoom_get_meeting

**What it does**: Gets detailed information about a specific meeting including join URL and settings.

**When to use**: Get meeting link, check settings before joining, see participant count.

**Arguments**:
- `meeting_id` (required): Meeting ID (numeric)

**Example LLM prompt**: "Get details for meeting 123456789"

---

### zoom_create_meeting

**What it does**: Creates a new meeting for a user.

**When to use**: Schedule new meetings, set up video calls, create recurring meetings.

**Arguments**:
- `user_id` (required): User ID (or `me` for current user)
- `topic` (required): Meeting topic
- `type` (optional): 1=instant, 2=scheduled, 3=recurring no fixed time, 8=recurring fixed time (default 2)
- `start_time` (optional): Start time in ISO 8601 format
- `duration` (optional): Duration in minutes
- `timezone` (optional): Timezone (e.g., `America/New_York`)
- `agenda` (optional): Meeting description
- `settings` (optional): Object with host_video, participant_video, etc.

**Example LLM prompt**: "Create a meeting called 'Team Standup' for tomorrow at 9am with duration 30 minutes"

---

### zoom_update_meeting

**What it does**: Updates an existing meeting's details.

**When to use**: Reschedule meetings, change duration, update topic.

**Arguments**:
- `meeting_id` (required): Meeting ID
- `topic` (optional): New topic
- `start_time` (optional): New start time
- `duration` (optional): New duration
- `agenda` (optional): New agenda
- `settings` (optional): New meeting settings

**Example LLM prompt**: "Update meeting 123456789 to start at 10am instead"

---

### zoom_delete_meeting

**What it does**: Deletes a meeting and invalidates its join link.

**When to use**: Cancel meetings, remove unwanted scheduled meetings.

**Arguments**:
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "Delete meeting 123456789"

---

### zoom_list_participants

**What it does**: Lists all participants who have joined a specific meeting.

**When to use**: See who attended a meeting, track participation.

**Arguments**:
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "List all participants in meeting 123456789"

---

### zoom_get_participant

**What it does**: Gets details about a specific participant in a meeting.

**When to use**: Check individual participant's connection quality, device info, and participation duration.

**Arguments**:
- `meeting_id` (required): Meeting ID
- `participant_id` (required): Participant ID

**Example LLM prompt**: "Get details about participant abc123 in meeting 123456789"

---

### zoom_list_recordings

**What it does**: Lists cloud recordings for a user including meeting recordings.

**When to use**: Find recorded meetings, get download links, see storage usage.

**Arguments**:
- `user_id` (required): User ID (or `me` for current user)
- `page_size` (optional): Max results (default 30)

**Example LLM prompt**: "List all recordings for me"

---

### zoom_get_recording

**What it does**: Gets details about a specific cloud recording.

**When to use**: Get download URLs, file sizes, and recording date/time.

**Arguments**:
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "Get recording details for meeting 123456789"

---

### zoom_list_users

**What it does**: Lists all users in the Zoom account.

**When to use**: Find team members, check user status, look up user IDs.

**Arguments**:
- `status` (optional): Filter by `active`, `inactive`, or `pending`
- `page_size` (optional): Max users (default 30, max 300)

**Example LLM prompt**: "List all active users in our Zoom account"

---

## Zoom API Notes

- **Meeting IDs**: Numeric IDs (e.g., 123456789) - not the same as meeting UUIDs
- **User ID**: Can use `me` for the authenticated user, or find user IDs via list_users
- **Timezones**: Use IANA timezone format (e.g., `America/Los_Angeles`)
- **Recordings**: Cloud recordings are stored per user - check list_recordings for each user
- **Rate Limits**: 100 requests/second for most endpoints

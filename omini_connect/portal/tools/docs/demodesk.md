# DemoDesk Tools

Provider: `demodesk` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the DemoDesk API. They allow AI agents to manage meetings, rooms, participants, and recordings. DemoDesk is a video conferencing and meeting platform.

## Authentication

**Nango API_KEY**:
- User provides their DemoDesk API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Passed in headers as `api-key`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `demodesk_list_meetings` | List meetings | GET | /meetings |
| `demodesk_get_meeting` | Get meeting details | GET | /meetings/{meeting_id} |
| `demodesk_create_meeting` | Create a meeting | POST | /meetings |
| `demodesk_cancel_meeting` | Cancel a meeting | DELETE | /meetings/{meeting_id} |
| `demodesk_list_rooms` | List meeting rooms | GET | /rooms |
| `demodesk_get_room` | Get room details | GET | /rooms/{room_id} |
| `demodesk_list_participants` | List participants | GET | /meetings/{meeting_id}/participants |
| `demodesk_get_participant` | Get participant details | GET | /meetings/{meeting_id}/participants/{participant_id} |
| `demodesk_list_recordings` | List recordings | GET | /recordings |
| `demodesk_get_recording` | Get recording details | GET | /recordings/{recording_id} |

---

## Tool Details

### demodesk_list_meetings

**What it does**: Lists all meetings with optional status filtering.

**When to use**: View scheduled meetings, find upcoming sessions, track meeting history.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `status` (optional): Filter by scheduled, started, ended, or cancelled

**Example LLM prompt**: "List all scheduled meetings"

---

### demodesk_get_meeting

**What it does**: Gets detailed meeting information.

**When to use**: Review meeting details, check participants, verify meeting settings.

**Arguments**:
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting m-123"

---

### demodesk_create_meeting

**What it does**: Creates a new meeting with specified settings.

**When to use**: Schedule new meetings, set up video sessions, create webinar events.

**Arguments**:
- `title` (required): Meeting title
- `start_time` (optional): Start time (ISO 8601 format)
- `duration` (optional): Duration in minutes
- `room_id` (optional): Room ID to use

**Example LLM prompt**: "Create a meeting titled 'Team Standup' for tomorrow at 9am"

---

### demodesk_cancel_meeting

**What it does**: Cancels a scheduled meeting.

**When to use**: Cancel unwanted meetings, reschedule sessions, remove obsolete meetings.

**Arguments**:
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "Cancel meeting m-123"

---

### demodesk_list_rooms

**What it does**: Lists all available meeting rooms.

**When to use**: View room inventory, find available rooms, check room capacity.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all meeting rooms"

---

### demodesk_get_room

**What it does**: Gets detailed room information.

**When to use**: Check room capacity, verify room settings, view room availability.

**Arguments**:
- `room_id` (required): Room ID

**Example LLM prompt**: "Get details for room r-456"

---

### demodesk_list_participants

**What it does**: Lists all participants in a meeting.

**When to use**: View attendee list, check attendance, manage meeting access.

**Arguments**:
- `meeting_id` (required): Meeting ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all participants in meeting m-123"

---

### demodesk_get_participant

**What it does**: Gets detailed participant information.

**When to use**: Check participant details, verify attendance, review participant role.

**Arguments**:
- `meeting_id` (required): Meeting ID
- `participant_id` (required): Participant ID

**Example LLM prompt**: "Get details for participant p-789 in meeting m-123"

---

### demodesk_list_recordings

**What it does**: Lists all meeting recordings.

**When to use**: Browse recordings, find past sessions, manage recorded content.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `meeting_id` (optional): Filter by meeting ID

**Example LLM prompt**: "List all recordings from last week"

---

### demodesk_get_recording

**What it does**: Gets detailed recording information.

**When to use**: Get recording URL, check recording duration, verify recording status.

**Arguments**:
- `recording_id` (required): Recording ID

**Example LLM prompt**: "Get details for recording rec-101"

---

## DemoDesk API Notes

- **Meetings**: Video conferencing sessions with participants
- **Rooms**: Persistent meeting spaces with configurable settings
- **Participants**: Individuals attending a specific meeting
- **Recordings**: Captured video/audio of past meetings
- **API Key**: Passed in headers as `api-key`
- **Status Flow**: scheduled -> started -> ended

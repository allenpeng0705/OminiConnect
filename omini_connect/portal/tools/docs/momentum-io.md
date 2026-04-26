# Momentum.io Tools

Provider: `momentum-io` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Momentum.io API. They allow AI agents to manage meetings, participants, and recordings. **Requires Momentum.io API key.**

## Authentication

**Nango API_KEY**:
- User provides Momentum.io API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `x-api-key: ${apiKey}`
- Base URL: `https://api.momentum.io`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `momentum_list_meetings` | List all meetings | GET | /v1/meetings |
| `momentum_get_meeting` | Get meeting details | GET | /v1/meetings/{meetingId} |
| `momentum_create_meeting` | Create a meeting | POST | /v1/meetings |
| `momentum_update_meeting` | Update a meeting | PUT | /v1/meetings/{meetingId} |
| `momentum_delete_meeting` | Delete a meeting | DELETE | /v1/meetings/{meetingId} |
| `momentum_list_participants` | List participants | GET | /v1/meetings/{meetingId}/participants |
| `momentum_add_participant` | Add a participant | POST | /v1/meetings/{meetingId}/participants |
| `momentum_remove_participant` | Remove a participant | DELETE | /v1/meetings/{meetingId}/participants/{participantId} |
| `momentum_list_recordings` | List recordings | GET | /v1/recordings |
| `momentum_get_recording` | Get recording details | GET | /v1/recordings/{recordingId} |

---

## Tool Details

### momentum_list_meetings

**What it does**: Lists all meetings in Momentum.io.

**When to use**: Browse meetings, find upcoming sessions.

**Arguments**:
- `from_date` (optional): From date (ISO 8601)
- `to_date` (optional): To date (ISO 8601)
- `status` (optional): Filter by status (scheduled, started, ended)
- `host_id` (optional): Filter by host ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all meetings for today"

---

### momentum_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: Get meeting info, join details.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting MTG-12345"

---

### momentum_create_meeting

**What it does**: Creates a new meeting in Momentum.io.

**When to use**: Schedule new meetings.

**Arguments**:
- `title` (required): Meeting title
- `start_time` (required): Start time (ISO 8601)
- `duration` (optional): Duration in minutes
- `host_id` (optional): Host user ID

**Example LLM prompt**: "Create a meeting called 'Team Standup' for tomorrow at 9am"

---

### momentum_update_meeting

**What it does**: Updates an existing meeting.

**When to use**: Reschedule meetings, change details.

**Arguments**:
- `meetingId` (required): Meeting ID
- `title` (optional): Meeting title
- `start_time` (optional): Start time (ISO 8601)
- `duration` (optional): Duration in minutes

**Example LLM prompt**: "Update meeting MTG-12345 to start at 10am"

---

### momentum_delete_meeting

**What it does**: Deletes a meeting from Momentum.io.

**When to use**: Cancel meetings.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "Delete meeting MTG-12345"

---

### momentum_list_participants

**What it does**: Lists all participants in a meeting.

**When to use**: View attendee list, check attendance.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "List participants in meeting MTG-12345"

---

### momentum_add_participant

**What it does**: Adds a participant to a meeting.

**When to use**: Invite attendees, add presenters.

**Arguments**:
- `meetingId` (required): Meeting ID
- `user_id` (required): User ID to add
- `role` (optional): Participant role (host, attendee) (default attendee)

**Example LLM prompt**: "Add user USR-12345 to meeting MTG-12345 as host"

---

### momentum_remove_participant

**What it does**: Removes a participant from a meeting.

**When to use**: Remove attendees, cancel invitations.

**Arguments**:
- `meetingId` (required): Meeting ID
- `participantId` (required): Participant ID

**Example LLM prompt**: "Remove participant PART-12345 from meeting MTG-12345"

---

### momentum_list_recordings

**What it does**: Lists all meeting recordings.

**When to use**: Find recorded meetings.

**Arguments**:
- `meeting_id` (optional): Filter by meeting ID
- `from_date` (optional): From date
- `to_date` (optional): To date
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all recordings from this month"

---

### momentum_get_recording

**What it does**: Gets details of a specific recording.

**When to use**: Get recording URL, duration.

**Arguments**:
- `recordingId` (required): Recording ID

**Example LLM prompt**: "Get details for recording REC-12345"

---

## Momentum.io Notes

- **Video meetings**: Online meeting platform
- **Meetings**: Scheduled video sessions
- **Participants**: Attendees and hosts
- **Recordings**: Meeting recordings storage
- **Duration**: Meeting length in minutes

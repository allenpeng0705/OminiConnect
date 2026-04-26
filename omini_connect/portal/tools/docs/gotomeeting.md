# GoToMeeting Tools

Provider: `gotomeeting` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the GoToMeeting API. They allow AI agents to create, manage, and track online meetings and video conferences. GoToMeeting is a popular video conferencing platform for professional meetings and team collaboration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with GoToMeeting
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `meetings:r`, `meetings:w`, `meetings:d`, `recordings:r`, `users:r`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gotomeeting_list_meetings` | List all meetings | GET | /meetings |
| `gotomeeting_get_meeting` | Get meeting details | GET | /meetings/{meetingId} |
| `gotomeeting_create_meeting` | Create a new meeting | POST | /meetings |
| `gotomeeting_update_meeting` | Update a meeting | PUT | /meetings/{meetingId} |
| `gotomeeting_delete_meeting` | Delete a meeting | DELETE | /meetings/{meetingId} |
| `gotomeeting_list_participants` | List meeting participants | GET | /meetings/{meetingId}/participants |
| `gotomeeting_get_participant` | Get participant details | GET | /meetings/{meetingId}/participants/{participantId} |
| `gotomeeting_list_recordings` | List meeting recordings | GET | /recordings |
| `gotomeeting_get_recording` | Get recording details | GET | /recordings/{recordingId} |
| `gotomeeting_list_users` | List users | GET | /users |

---

## Tool Details

### gotomeeting_list_meetings

**What it does**: Lists all scheduled meetings for the authenticated user. Filter by date range, topic, or status.

**When to use**: See scheduled meetings, find past meetings, search by subject.

**Arguments**:
- `from` (optional): Start date (YYYY-MM-DD)
- `to` (optional): End date (YYYY-MM-DD)
- `subject` (optional): Filter by subject
- `status` (optional): Filter by status (`active`, `started`, `ended`)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all upcoming meetings for this week"

---

### gotomeeting_get_meeting

**What it does**: Gets detailed information about a specific meeting including participants, timing, and meeting settings.

**When to use**: Get full meeting details before joining or before making changes.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "What are the details of meeting 123456789?"

---

### gotomeeting_create_meeting

**What it does**: Creates a new GoToMeeting session with subject, start time, duration, and optional participant list.

**When to use**: Schedule a new meeting or video conference.

**Arguments**:
- `subject` (required): Meeting subject
- `start_time` (required): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)
- `duration` (optional): Duration in minutes
- `password` (optional): Meeting password
- `timezone` (optional): Timezone
- `conference_call_info` (optional): Conference call settings
- `auto_record` (optional): Auto-record meeting (default false)

**Example LLM prompt**: "Schedule a team meeting for tomorrow at 2pm Pacific time for 45 minutes"

---

### gotomeeting_update_meeting

**What it does**: Updates an existing meeting subject, time, or settings. Cannot update a meeting that has already started.

**When to use**: Reschedule a meeting or change its topic.

**Arguments**:
- `meetingId` (required): Meeting ID
- `subject` (optional): New meeting subject
- `start_time` (optional): New start time (ISO 8601)
- `end_time` (optional): New end time (ISO 8601)
- `duration` (optional): New duration in minutes
- `password` (optional): New meeting password
- `timezone` (optional): Timezone

**Example LLM prompt**: "Change meeting 12345 to start at 3pm instead"

---

### gotomeeting_delete_meeting

**What it does**: Deletes a scheduled meeting. Cannot delete meetings that are currently in progress.

**When to use**: Cancel an unwanted meeting.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "Delete the meeting with ID 12345"

---

### gotomeeting_list_participants

**What it does**: Lists all participants for a specific meeting including join time, leave time, and email addresses.

**When to use**: See who attended a meeting, get attendee list for follow-up.

**Arguments**:
- `meetingId` (required): Meeting ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Who attended the quarterly review meeting?"

---

### gotomeeting_get_participant

**What it does**: Gets detailed information about a specific participant including their join/leave times and device info.

**When to use**: Track individual attendance and participation.

**Arguments**:
- `meetingId` (required): Meeting ID
- `participantId` (required): Participant ID

**Example LLM prompt**: "When did John join the meeting?"

---

### gotomeeting_list_recordings

**What it does**: Lists all recorded meetings. Filter by date range, subject, or meeting ID.

**When to use**: Find past meeting recordings for review or sharing.

**Arguments**:
- `from` (optional): Start date (YYYY-MM-DD)
- `to` (optional): End date (YYYY-MM-DD)
- `meeting_id` (optional): Filter by meeting ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all recordings from the last month"

---

### gotomeeting_get_recording

**What it does**: Gets detailed information about a specific recording including download URL, duration, and file size.

**When to use**: Get recording details before downloading or sharing.

**Arguments**:
- `recordingId` (required): Recording ID

**Example LLM prompt**: "What is the download URL for recording 789?"

---

### gotomeeting_list_users

**What it does**: Lists all users in the account. Requires admin permissions for listing all users.

**When to use**: See all users in the organization, find user information for scheduling.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all users in our GoToMeeting account"

---

## GoToMeeting API Notes

- **Meeting IDs**: Numeric identifiers for meetings (e.g., `123456789`)
- **Participant IDs**: Numeric identifiers for meeting participants
- **Recording IDs**: Numeric identifiers for meeting recordings
- **ISO 8601**: All timestamps use ISO 8601 format (e.g., `2024-01-15T09:00:00Z`)
- **Timezones**: Use IANA timezone database names (e.g., `America/New_York`, `Europe/London`)
- **Recording retention**: Recordings may be automatically deleted after a configurable period
- **Join URLs**: Each meeting has a unique join URL that can be shared with participants
- **Meeting status**: Meetings can be `active` (upcoming), `started` (in progress), or `ended`
- **Admin scope**: Listing all users requires administrator permissions
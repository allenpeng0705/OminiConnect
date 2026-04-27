# Google Meet Tools

Provider: `google-meet` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Meet API. They allow AI agents to manage meeting spaces, participants, recordings, and access rules. **Requires Google OAuth2 with Meet permissions.**

## Authentication

**Nango OAUTH2 (Google Meet)**:
- User authenticates via OAuth2 with Meet scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://meet.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_meet_list_spaces` | List spaces | GET | /v2/spaces |
| `google_meet_create_space` | Create space | POST | /v2/spaces |
| `google_meet_get_space` | Get space details | GET | /v2/spaces/{space_id} |
| `google_meet_list_recordings` | List recordings | GET | /v2/recordings |
| `google_meet_list_participants` | List participants | GET | /v2/spaces/{space_id}/participants |
| `google_meet_create_access_rule` | Create access rule | POST | /v2/spaces/{space_id}/accessRules |
| `google_meet_list_access_rules` | List access rules | GET | /v2/spaces/{space_id}/accessRules |
| `google_meet_get_participant` | Get participant | GET | /v2/spaces/{space_id}/participants/{participant_id} |
| `google_meet_list_codecs` | List codecs | GET | /v2/codecs |
| `google_meet_update_space` | Update space | PATCH | /v2/spaces/{space_id} |

---

## Tool Details

### google_meet_list_spaces

**What it does**: Lists meeting spaces in Google Meet.

**When to use**: Browse available meeting spaces.

**Arguments**: None

**Example LLM prompt**: "List all meeting spaces"

---

### google_meet_create_space

**What it does**: Creates a new meeting space.

**When to use**: Create a new Meet session.

**Arguments**:
- `meetingTitle` (required): Meeting title

**Example LLM prompt**: "Create a meeting titled 'Team Standup'"

---

### google_meet_get_space

**What it does**: Gets details about a meeting space.

**When to use**: Get space ID and join URL.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "Get details for space abc123"

---

### google_meet_list_recordings

**What it does**: Lists recordings from Meet.

**When to use**: Find recorded meetings.

**Arguments**: None

**Example LLM prompt**: "List all recordings"

---

### google_meet_list_participants

**What it does**: Lists participants in a space.

**When to use**: See who joined a meeting.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "List participants in space abc123"

---

### google_meet_create_access_rule

**What it does**: Creates an access rule for a space.

**When to use**: Manage meeting access.

**Arguments**:
- `space_id` (required): Space ID
- `principal` (required): Principal (email or group)
- `role` (required): Role (creator, moderator, attendee)

**Example LLM prompt**: "Add user john@email.com as attendee to space abc123"

---

### google_meet_list_access_rules

**What it does**: Lists access rules for a space.

**When to use**: See who can access a space.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "List access rules for space abc123"

---

### google_meet_get_participant

**What it does**: Gets details about a participant.

**When to use**: Get participant info.

**Arguments**:
- `space_id` (required): Space ID
- `participant_id` (required): Participant ID

**Example LLM prompt**: "Get details for participant xyz789"

---

### google_meet_list_codecs

**What it does**: Lists available codecs for Meet.

**When to use**: Check codec support.

**Arguments**: None

**Example LLM prompt**: "List available codecs"

---

### google_meet_update_space

**What it does**: Updates a meeting space.

**When to use**: Rename a meeting.

**Arguments**:
- `space_id` (required): Space ID
- `meetingTitle` (optional): New meeting title

**Example LLM prompt**: "Update space abc123 with new title"

---

## Google Meet API Notes

- **Spaces**: Virtual rooms for meetings
- **Access rules**: Control who can join
- **Roles**: creator, moderator, attendee
- **Recordings**: Stored meeting recordings
- **Participants**: Users who joined

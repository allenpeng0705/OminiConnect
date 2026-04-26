# Chorus Tools

Provider: `chorus` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Chorus API. Chorus is a conversation intelligence platform that records and analyzes sales calls and meetings. **Requires Chorus API key.**

## Authentication

**Nango API_KEY**:
- User provides their Chorus API key
- Token passed via `Authorization: Bearer` header
- Base URL: `https://chorus.ai`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `chorus_list_users` | List users | GET | /api/v1/users |
| `chorus_get_user` | Get user details | GET | /api/v1/users/{id} |
| `chorus_list_calls` | List calls | GET | /api/v1/calls |
| `chorus_get_call` | Get call details | GET | /api/v1/calls/{id} |
| `chorus_list_meetings` | List meetings | GET | /api/v1/meetings |
| `chorus_get_meeting` | Get meeting details | GET | /api/v1/meetings/{id} |
| `chorus_list_teams` | List teams | GET | /api/v1/teams |
| `chorus_get_team` | Get team details | GET | /api/v1/teams/{id} |
| `chorus_list_analytics` | List analytics | GET | /api/v1/analytics |
| `chorus_get_call_recording` | Get call recording | GET | /api/v1/calls/{id}/recording |

---

## Tool Details

### chorus_list_users

**What it does**: Lists all users in the organization.

**When to use**: Find team members, view organization.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Chorus users"

---

### chorus_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile and activity.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### chorus_list_calls

**What it does**: Lists all calls with optional filters.

**When to use**: Find sales calls, view call history.

**Arguments**:
- `user_id` (optional): Filter by user
- `from_date` (optional): Start date
- `to_date` (optional): End date
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List calls from the last week"

---

### chorus_get_call

**What it does**: Gets details of a specific call.

**When to use**: View call metadata, participants.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get call 456 details"

---

### chorus_list_meetings

**What it does**: Lists all meetings.

**When to use**: View meeting history.

**Arguments**:
- `user_id` (optional): Filter by user
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List meetings for user 123"

---

### chorus_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: View meeting details and participants.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get meeting 789 details"

---

### chorus_list_teams

**What it does**: Lists all teams in the organization.

**When to use**: View team structure.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all teams"

---

### chorus_get_team

**What it does**: Gets details of a specific team.

**When to use**: View team members and settings.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Get team 101 details"

---

### chorus_list_analytics

**What it does**: Lists analytics data for calls and meetings.

**When to use**: Generate reports on conversation data.

**Arguments**:
- `from_date` (optional): Start date
- `to_date` (optional): End date

**Example LLM prompt**: "Get analytics for the last month"

---

### chorus_get_call_recording

**What it does**: Gets the recording for a call.

**When to use**: Access call recording audio or video.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get recording for call 456"

---

## Chorus API Notes

- **Calls**: Recorded sales calls with conversation intelligence
- **Meetings**: Video meetings captured by Chorus
- **Analytics**: Aggregated insights from conversations
- **Recordings**: Audio/video of calls and meetings

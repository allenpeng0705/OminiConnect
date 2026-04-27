# Fathom Tools

Provider: `fathom` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fathom API. They allow AI agents to manage meetings, clips, highlights, and transcripts. Fathom is a video meeting intelligence platform.

## Authentication

**Nango API_KEY**:
- User provides their Fathom API key via Nango Connect
- Key is passed in the x-api-key header
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fathom_list_teams` | List teams | GET | /external/v1/teams |
| `fathom_get_team` | Get team details | GET | /external/v1/teams/{id} |
| `fathom_list_meetings` | List meetings | GET | /external/v1/meetings |
| `fathom_get_meeting` | Get meeting details | GET | /external/v1/meetings/{id} |
| `fathom_list_clips` | List video clips | GET | /external/v1/clips |
| `fathom_get_clip` | Get clip details | GET | /external/v1/clips/{id} |
| `fathom_list_highlights` | List highlights | GET | /external/v1/highlights |
| `fathom_get_summary` | Get meeting summary | GET | /external/v1/meetings/{id}/summary |
| `fathom_list_messages` | List messages | GET | /external/v1/messages |
| `fathom_search_transcript` | Search transcript | GET | /external/v1/transcripts/search |

---

## Tool Details

### fathom_list_teams

**What it does**: Lists all teams in the account.

**When to use**: Browse team structure.

**Arguments**: None

**Example LLM prompt**: "List all teams"

---

### fathom_get_team

**What it does**: Gets details of a specific team.

**When to use**: View team information.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Get details for team abc123"

---

### fathom_list_meetings

**What it does**: Lists all meetings.

**When to use**: Browse meeting history.

**Arguments**:
- `team_id` (optional): Filter by team
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List recent meetings"

---

### fathom_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: View meeting information.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting xyz789"

---

### fathom_list_clips

**What it does**: Lists all video clips from meetings.

**When to use**: Browse created clips.

**Arguments**:
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "List clips for meeting xyz789"

---

### fathom_get_clip

**What it does**: Gets details of a specific clip.

**When to use**: Get clip information, playback URL.

**Arguments**:
- `id` (required): Clip ID

**Example LLM prompt**: "Get details for clip def456"

---

### fathom_list_highlights

**What it does**: Lists all highlights from meetings.

**When to use**: Browse key moments.

**Arguments**:
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "List highlights for meeting xyz789"

---

### fathom_get_summary

**What it does**: Gets AI-generated summary of a meeting.

**When to use**: Quick meeting recap.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get summary for meeting xyz789"

---

### fathom_list_messages

**What it does**: Lists all messages.

**When to use**: View chat messages from meetings.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List recent messages"

---

### fathom_search_transcript

**What it does**: Searches meeting transcripts.

**When to use**: Find specific content in meetings.

**Arguments**:
- `q` (required): Search query
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "Search transcripts for 'budget discussion'"

---

## Fathom API Notes

- **Teams**: Groupings of users in the organization
- **Meetings**: Video meetings with transcripts
- **Clips**: Short video segments from meetings
- **Highlights**: Key moments identified in meetings
- **Transcripts**: Full text of meeting conversations
- **Webhooks**: Support for webhook notifications

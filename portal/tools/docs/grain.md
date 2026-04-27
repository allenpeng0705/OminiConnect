# Grain Tools

Provider: `grain` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Grain API. They allow AI agents to manage video recordings, transcripts, highlights, and participants. Grain is a video meeting intelligence platform that records, transcribes, and highlights key moments from meetings.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Grain
- Token stored in Nango, accessed via `connection_ref`
- Permissions configured at app level in Grain dashboard

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `grain_list_recordings` | List recordings | GET | /api/v1/recordings |
| `grain_get_recording` | Get recording details | GET | /api/v1/recordings/{id} |
| `grain_list_transcripts` | List transcripts | GET | /api/v1/transcripts |
| `grain_get_transcript` | Get transcript details | GET | /api/v1/transcripts/{id} |
| `grain_list_highlights` | List highlights | GET | /api/v1/highlights |
| `grain_create_highlight` | Create a highlight | POST | /api/v1/highlights |
| `grain_list_participants` | List participants | GET | /api/v1/participants |
| `grain_list_workspaces` | List workspaces | GET | /api/v1/workspaces |
| `grain_list_speakers` | List speakers | GET | /api/v1/speakers |
| `grain_list_meetings` | List meetings | GET | /api/v1/meetings |

---

## Tool Details

### grain_list_recordings

**What it does**: Lists all recordings from Grain.

**When to use**: Browse meeting recordings, find specific recordings.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all recent recordings"

---

### grain_get_recording

**What it does**: Gets detailed information about a specific recording.

**When to use**: Get video URL, check recording metadata.

**Arguments**:
- `id` (required): Recording ID

**Example LLM prompt**: "Get recording with id abc123"

---

### grain_list_transcripts

**What it does**: Lists all transcripts from Grain recordings.

**When to use**: Find available transcripts for recordings.

**Arguments**:
- `recording_id` (optional): Filter by recording ID

**Example LLM prompt**: "List transcripts for recording abc123"

---

### grain_get_transcript

**What it does**: Gets detailed transcript for a specific recording.

**When to use**: Read full transcript, get speaker timestamps.

**Arguments**:
- `id` (required): Transcript ID

**Example LLM prompt**: "Get transcript for recording abc123"

---

### grain_list_highlights

**What it does**: Lists all highlights from Grain recordings.

**When to use**: View key moments marked in recordings.

**Arguments**:
- `recording_id` (optional): Filter by recording ID

**Example LLM prompt**: "List all highlights"

---

### grain_create_highlight

**What it does**: Creates a new highlight in a recording.

**When to use**: Mark a key moment for easy reference.

**Arguments**:
- `recording_id` (required): Recording ID
- `start_time` (required): Start time in seconds
- `end_time` (required): End time in seconds
- `label` (optional): Highlight label

**Example LLM prompt**: "Create a highlight from 5:00 to 5:30 for recording abc123"

---

### grain_list_participants

**What it does**: Lists all participants from recordings.

**When to use**: View meeting attendees, check participant list.

**Arguments**:
- `recording_id` (optional): Filter by recording ID

**Example LLM prompt**: "List participants for recording abc123"

---

### grain_list_workspaces

**What it does**: Lists all workspaces in your Grain account.

**When to use**: Switch between workspaces, check workspace settings.

**Arguments**: None

**Example LLM prompt**: "List all workspaces"

---

### grain_list_speakers

**What it does**: Lists all speakers identified in recordings.

**When to use**: View speaker profiles, check speaking time.

**Arguments**: None

**Example LLM prompt**: "List all speakers"

---

### grain_list_meetings

**What it does**: Lists all meetings from Grain.

**When to use**: Browse meeting history, find meetings.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all meetings"

---

## Grain API Notes

- **API Base URL**: https://api.grain.com
- **Recordings**: Stored with unique IDs, include video URL and metadata
- **Transcripts**: Include speaker segments with timestamps
- **Highlights**: User-defined key moments in recordings
- **Participants**: People who attended meetings
- **Workspaces**: Organize recordings and settings

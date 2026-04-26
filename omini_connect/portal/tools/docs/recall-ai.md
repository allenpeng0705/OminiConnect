# Recallai Tools

Provider: `recall-ai` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Recall.ai is a meeting intelligence platform that provides transcription, AI summaries, and action item extraction from meetings. These tools allow AI agents to access meeting data, transcripts, summaries, and action items.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Recall.ai
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `meetings:read`, `meetings:write`, `transcripts:read`, `summaries:read`, `action-items:read`, `participants:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recall-ai_list_meetings` | List all meetings | GET | /v1/meetings |
| `recall-ai_get_meeting` | Get meeting details | GET | /v1/meetings/{meetingId} |
| `recall-ai_list_transcripts` | List meeting transcripts | GET | /v1/meetings/{meetingId}/transcripts |
| `recall-ai_get_transcript` | Get transcript details | GET | /v1/transcripts/{transcriptId} |
| `recall-ai_list_summaries` | List AI summaries | GET | /v1/meetings/{meetingId}/summaries |
| `recall-ai_get_summary` | Get summary details | GET | /v1/summaries/{summaryId} |
| `recall-ai_list_action_items` | List action items | GET | /v1/meetings/{meetingId}/action-items |
| `recall-ai_create_meeting` | Create a meeting | POST | /v1/meetings |
| `recall-ai_delete_meeting` | Delete a meeting | DELETE | /v1/meetings/{meetingId} |
| `recall-ai_list_participants` | List participants | GET | /v1/meetings/{meetingId}/participants |

---

## Tool Details

### recall-ai_list_meetings

**What it does**: Returns a paginated list of all meetings.

**When to use**: Browse meeting history, find meetings by date.

**Arguments**:
- `limit` (optional): Number of meetings (default 50)
- `cursor` (optional): Pagination cursor
- `startDate` (optional): Filter from date
- `endDate` (optional): Filter to date

**Example LLM prompt**: "List all meetings from last week"

---

### recall-ai_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: Get meeting title, time, duration, and status.

**Arguments**:
- `meetingId` (required): The meeting ID

**Example LLM prompt**: "Get details for meeting mt_abc123"

---

### recall-ai_list_transcripts

**What it does**: Lists all transcripts for a meeting.

**When to use**: Get the full conversation from a meeting.

**Arguments**:
- `meetingId` (required): The meeting ID

**Example LLM prompt**: "List transcripts for meeting mt_abc123"

---

### recall-ai_get_transcript

**What it does**: Gets a specific transcript with speaker segments.

**When to use**: Read the full meeting conversation.

**Arguments**:
- `transcriptId` (required): The transcript ID

**Example LLM prompt**: "Get transcript tr_xyz789"

---

### recall-ai_list_summaries

**What it does**: Lists AI-generated summaries for a meeting.

**When to use**: Get quick overview of meeting content.

**Arguments**:
- `meetingId` (required): The meeting ID

**Example LLM prompt**: "List summaries for meeting mt_abc123"

---

### recall-ai_get_summary

**What it does**: Gets AI-generated summary details.

**When to use**: Read the summary of a meeting.

**Arguments**:
- `summaryId` (required): The summary ID

**Example LLM prompt**: "Get summary sm_456"

---

### recall-ai_list_action_items

**What it does**: Lists all action items extracted from meetings.

**When to use**: Find tasks assigned in meetings.

**Arguments**:
- `meetingId` (required): The meeting ID

**Example LLM prompt**: "List action items from the sales call"

---

### recall-ai_create_meeting

**What it does**: Creates a new meeting for processing.

**When to use**: Schedule a meeting to be recorded.

**Arguments**:
- `title` (required): Meeting title
- `scheduledTime` (optional): Start time
- `duration` (optional): Duration in minutes

**Example LLM prompt**: "Create a meeting called 'Team Sync'"

---

### recall-ai_delete_meeting

**What it does**: Deletes a meeting and all its data.

**When to use**: Remove unwanted meetings.

**Arguments**:
- `meetingId` (required): The meeting ID to delete

**Example LLM prompt**: "Delete meeting mt_abc123"

---

### recall-ai_list_participants

**What it does**: Lists all participants in a meeting.

**When to use**: See who attended a meeting.

**Arguments**:
- `meetingId` (required): The meeting ID

**Example LLM prompt**: "List participants in meeting mt_abc123"

---

## Recall.ai Notes

- Recall.ai automatically transcribes and summarizes meetings
- Action items are extracted using AI
- Speaker segments in transcripts identify who said what
- Cursor-based pagination for large meeting lists

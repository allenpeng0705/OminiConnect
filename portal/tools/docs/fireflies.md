# Fireflies Tools

Provider: `fireflies` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fireflies API. They allow AI agents to manage meeting transcripts, users, and analytics. Fireflies is a meeting intelligence platform that records, transcribes, and summarizes meetings.

## Authentication

**Nango API_KEY**:
- User provides their Fireflies API key via Nango Connect
- Key is passed in the Authorization header as Bearer token
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fireflies_list_transcripts` | List transcripts | GET | /api/v1/transcripts |
| `fireflies_get_transcript` | Get transcript details | GET | /api/v1/transcripts/{id} |
| `fireflies_list_users` | List team members | GET | /api/v1/users |
| `fireflies_get_user` | Get user details | GET | /api/v1/users/{id} |
| `fireflies_list_meetings` | List meetings | GET | /api/v1/meetings |
| `fireflies_get_meeting` | Get meeting details | GET | /api/v1/meetings/{id} |
| `fireflies_list_summaries` | List AI summaries | GET | /api/v1/summaries |
| `fireflies_search_transcript` | Search transcripts | GET | /api/v1/transcripts/search |
| `fireflies_list_analytics` | List analytics | GET | /api/v1/analytics |
| `fireflies_get_overview` | Get analytics overview | GET | /api/v1/analytics/overview |

---

## Tool Details

### fireflies_list_transcripts

**What it does**: Lists all meeting transcripts.

**When to use**: Browse transcript history.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all transcripts"

---

### fireflies_get_transcript

**What it does**: Gets details of a specific transcript.

**When to use**: View full transcript content.

**Arguments**:
- `id` (required): Transcript ID

**Example LLM prompt**: "Get transcript abc123"

---

### fireflies_list_users

**What it does**: Lists all team members.

**When to use**: View team roster.

**Arguments**: None

**Example LLM prompt**: "List all team members"

---

### fireflies_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user information.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user xyz789"

---

### fireflies_list_meetings

**What it does**: Lists all meetings.

**When to use**: Browse meeting history.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all meetings"

---

### fireflies_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: View meeting information.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get meeting def456"

---

### fireflies_list_summaries

**What it does**: Lists AI-generated summaries.

**When to use**: Browse meeting summaries.

**Arguments**:
- `meeting_id` (optional): Filter by meeting

**Example LLM prompt**: "List summaries for meeting def456"

---

### fireflies_search_transcript

**What it does**: Searches across transcripts.

**When to use**: Find specific content in meetings.

**Arguments**:
- `q` (required): Search query

**Example LLM prompt**: "Search for 'budget discussion' in transcripts"

---

### fireflies_list_analytics

**What it does**: Lists meeting analytics.

**When to use**: View meeting metrics, trends.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get analytics for the last month"

---

### fireflies_get_overview

**What it does**: Gets analytics overview.

**When to use**: Quick summary of meeting stats.

**Arguments**: None

**Example LLM prompt**: "Get analytics overview"

---

## Fireflies API Notes

- **Transcripts**: Full text of meeting conversations
- **Meetings**: Video meetings with associated data
- **Summaries**: AI-generated recaps of meetings
- **Users**: Team members in the workspace
- **Analytics**: Meeting metrics and trends
- **Search**: Full-text search across transcripts

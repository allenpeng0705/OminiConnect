# Avoma Tools

Provider: `avoma` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Avoma API. They allow AI agents to manage meetings, recordings, users, and search across content. Avoma is an AI-powered meeting intelligence platform that provides transcription, note-taking, and analytics for meetings.

## Authentication

**Nango API_KEY**:
- User provides API key from Avoma
- Token stored in Nango, accessed via `connection_ref`
- Bearer token in Authorization header

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `avoma_list_users` | List users | GET | /v1/users |
| `avoma_get_user` | Get user details | GET | /v1/users/{userId} |
| `avoma_list_meetings` | List meetings | GET | /v1/meetings |
| `avoma_get_meeting` | Get meeting details | GET | /v1/meetings/{meetingId} |
| `avoma_list_recordings` | List recordings | GET | /v1/recordings |
| `avoma_get_recording` | Get recording details | GET | /v1/recordings/{recordingId} |
| `avoma_list_accounts` | List accounts | GET | /v1/accounts |
| `avoma_get_account` | Get account details | GET | /v1/accounts/{accountId} |
| `avoma_list_collaborators` | List collaborators | GET | /v1/collaborators |
| `avoma_search_content` | Search across content | GET | /v1/search |

---

## Tool Details

### avoma_list_users

**What it does**: Lists all users in the Avoma organization.

**When to use**: View user directory, check team members.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Users per page (default 20)

**Example LLM prompt**: "List all users in Avoma"

---

### avoma_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Check user role, settings, and activity.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user ID 123"

---

### avoma_list_meetings

**What it does**: Lists all meetings with optional date and user filters.

**When to use**: Find meetings, review meeting history.

**Arguments**:
- `userId` (optional): Filter by user ID
- `startDate` (optional): Filter start date (ISO 8601)
- `endDate` (optional): Filter end date (ISO 8601)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Meetings per page (default 20)

**Example LLM prompt**: "List all meetings from the last week"

---

### avoma_get_meeting

**What it does**: Gets detailed meeting information including participants and notes.

**When to use**: Review specific meeting details and content.

**Arguments**:
- `meetingId` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting M-12345"

---

### avoma_list_recordings

**What it does**: Lists all meeting recordings.

**When to use**: Find recorded meetings, access past call recordings.

**Arguments**:
- `meetingId` (optional): Filter by meeting ID
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all recordings"

---

### avoma_get_recording

**What it does**: Gets recording details including transcript and highlights.

**When to use**: Review meeting recording, read transcript.

**Arguments**:
- `recordingId` (required): Recording ID

**Example LLM prompt**: "Get recording R-12345"

---

### avoma_list_accounts

**What it does**: Lists all accounts in the organization.

**When to use**: View account structure, manage accounts.

**Arguments**: None required

**Example LLM prompt**: "List all accounts"

---

### avoma_get_account

**What it does**: Gets details of a specific account.

**When to use**: Check account settings and configuration.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account A-123"

---

### avoma_list_collaborators

**What it does**: Lists all collaborators in the organization.

**When to use**: View team members with access.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all collaborators"

---

### avoma_search_content

**What it does**: Searches across meetings, recordings, transcripts, and notes.

**When to use**: Find specific content or topics discussed in meetings.

**Arguments**:
- `query` (required): Search query string
- `type` (optional): Content type filter (meeting, recording, transcript, all)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Results per page (default 20)

**Example LLM prompt**: "Search for discussions about product roadmap"

---

## Avoma API Notes

- **Meeting Intelligence**: Avoma automatically transcribes and summarizes meetings
- **Search**: Search supports transcripts, notes, titles, and topics
- **Recordings**: Associated with meetings and include full transcripts
- **User Roles**: Users can be admins, members, or guests

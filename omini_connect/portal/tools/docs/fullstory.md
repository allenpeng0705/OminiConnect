# FullStory Tools

Provider: `fullstory` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the FullStory API. They allow AI agents to analyze digital behavior through session recordings, custom events, user segments, and advanced session search. FullStory is a digital intelligence platform that captures user sessions and provides powerful analytics capabilities.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with FullStory
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `fullstory.recordings.read`, `fullstory.events.read`, `fullstory.users.read`, `fullstory.segments.read`, `fullstory.sessions.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fullstory_list_recordings` | List session recordings | GET | /v1/recordings |
| `fullstory_get_recording` | Get recording details | GET | /v1/recordings/{recordingId} |
| `fullstory_list_events` | List custom events | GET | /v1/events |
| `fullstory_get_event` | Get event details | GET | /v1/events/{eventId} |
| `fullstory_list_users` | List identified users | GET | /v1/users |
| `fullstory_get_user` | Get user details | GET | /v1/users/{userId} |
| `fullstory_list_segments` | List user segments | GET | /v1/segments |
| `fullstory_get_segment` | Get segment details | GET | /v1/segments/{segmentId} |
| `fullstory_search_sessions` | Search sessions | POST | /v1/sessions/search |
| `fullstory_get_session` | Get session details | GET | /v1/sessions/{sessionId} |

---

## Tool Details

### fullstory_list_recordings

**What it does**: Retrieves a list of session recordings with optional filtering by user and pagination.

**When to use**: Find session recordings to analyze user behavior patterns on your platform.

**Arguments**:
- `limit` (optional): Maximum number of recordings to return (default 50)
- `offset` (optional): Offset for pagination
- `userId` (optional): Filter recordings by user ID

**Example LLM prompt**: "List recent session recordings"

---

### fullstory_get_recording

**What it does**: Retrieves details about a specific session recording including events, actions, and metadata.

**When to use**: Analyze an individual user's journey to understand their experience.

**Arguments**:
- `recordingId` (required): Unique identifier of the recording

**Example LLM prompt**: "Get recording rec123"

---

### fullstory_list_events

**What it does**: Retrieves a list of custom events with optional time range filtering.

**When to use**: Find specific user actions that have been tracked in your application.

**Arguments**:
- `limit` (optional): Maximum number of events to return
- `startTime` (optional): Start time in ISO 8601 format
- `endTime` (optional): End time in ISO 8601 format

**Example LLM prompt**: "List custom events from the last week"

---

### fullstory_get_event

**What it does**: Retrieves details about a specific custom event including properties and occurrence count.

**When to use**: Analyze a specific user action and its context.

**Arguments**:
- `eventId` (required): Unique identifier of the event

**Example LLM prompt**: "Get details for event evt456"

---

### fullstory_list_users

**What it does**: Retrieves a list of identified users with their attributes and session history.

**When to use**: Find specific users to analyze their behavior across sessions.

**Arguments**:
- `limit` (optional): Maximum number of users to return
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List identified users"

---

### fullstory_get_user

**What it does**: Retrieves details about a specific user including all their sessions and attributes.

**When to use**: Investigate a specific user's complete journey on your platform.

**Arguments**:
- `userId` (required): Unique identifier of the user

**Example LLM prompt**: "Get user usr789"

---

### fullstory_list_segments

**What it does**: Retrieves a list of user segments for targeting and analytics.

**When to use**: Find segments for targeting users or analyzing group behavior.

**Arguments**:
- `limit` (optional): Maximum number of segments to return

**Example LLM prompt**: "List all user segments"

---

### fullstory_get_segment

**What it does**: Retrieves details about a specific segment including membership rules and user count.

**When to use**: Review segment criteria and understand who belongs to a segment.

**Arguments**:
- `segmentId` (required): Unique identifier of the segment

**Example LLM prompt**: "Get segment seg123"

---

### fullstory_search_sessions

**What it does**: Searches for sessions matching specific criteria using FullStory's query language.

**When to use**: Find sessions with specific behaviors, errors, or user actions.

**Arguments**:
- `query` (required): Search query for session filtering
- `filters` (optional): Additional filters for session search
- `limit` (optional): Maximum number of sessions to return

**Example LLM prompt**: "Search for sessions with checkout errors"

---

### fullstory_get_session

**What it does**: Retrieves details about a specific session including all events and user actions.

**When to use**: Analyze a complete user session to understand their experience.

**Arguments**:
- `sessionId` (required): Unique identifier of the session

**Example LLM prompt**: "Get session sess456"

---

## FullStory API Notes

- **Recordings**: Session recordings capturing all user activity (clicks, scrolls, errors)
- **Events**: Custom events you define to track specific user actions
- **Users**: Identified users with consolidated data across sessions
- **Segments**: Groups of users matching specific criteria for targeting
- **Sessions**: Individual browsing sessions with full event history
- **Search**: FullStory's query language allows powerful session filtering
- **Pagination**: Use limit and offset parameters for paginated results
- **Time filtering**: Events support time-based filtering with startTime and endTime
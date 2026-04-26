# Webex Tools

Provider: `webex` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Webex API. They allow AI agents to manage meetings, messages, teams, and recordings on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `meeting:read`, `meeting:write`, `spark:all`, `team:read`, `meeting:recordings:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `webex_list_meetings` | List meetings for the authenticated user | GET | /meetings |
| `webex_get_meeting` | Get details of a specific meeting | GET | /meetings/{meetingId} |
| `webex_create_meeting` | Create a new meeting | POST | /meetings |
| `webex_update_meeting` | Update an existing meeting | PUT | /meetings/{meetingId} |
| `webex_list_messages` | List messages in a room | GET | /messages |
| `webex_get_message` | Get details of a specific message | GET | /messages/{messageId} |
| `webex_send_message` | Send a message to a room | POST | /messages |
| `webex_list_teams` | List teams the user belongs to | GET | /teams |
| `webex_get_team` | Get details of a specific team | GET | /teams/{teamId} |
| `webex_list_recordings` | List recordings | GET | /recordings |

---

## Tool Details

### webex_list_meetings

**What it does**: Lists meetings for the authenticated user. Returns meeting details including title, start time, duration, and participant info.

**When to use**: View upcoming meetings or check scheduled meetings within a date range.

**Arguments**:
- `meetingType` (optional): scheduled or scheduledType
- `from` (optional): Start date/time filter (ISO 8601)
- `to` (optional): End date/time filter (ISO 8601)
- `max` (optional, max 200): default 50

**Example LLM prompt**: "List my meetings for next week"

---

### webex_get_meeting

**What it does**: Gets details of a specific meeting including title, description, timing, and host info.

**When to use**: Retrieve full meeting details before updating or joining.

**Arguments**:
- `meetingId` (required): Webex meeting ID

**Example LLM prompt**: "Get details for meeting abc123"

---

### webex_create_meeting

**What it does**: Creates a new Webex meeting. Specify title, start time, duration, and settings.

**When to use**: Schedule meetings programmatically.

**Arguments**:
- `title` (required): Meeting title
- `agenda` (optional): Meeting agenda/description
- `start` (required): Start time (ISO 8601)
- `end` (optional): End time (ISO 8601)
- `duration` (optional): Duration in minutes
- `timezone` (optional): Timezone (e.g., America/Los_Angeles)
- `recurrence` (optional): Recurrence settings for recurring meetings
- `password` (optional): Meeting password
- `settings` (optional): Meeting settings (hostKey, joinBeforeHost, etc.)

**Example LLM prompt**: "Create a meeting titled 'Team Sync' for tomorrow at 2pm"

---

### webex_update_meeting

**What it does**: Updates an existing meeting's title, time, duration, or other settings.

**When to use**: Reschedule or modify meeting details.

**Arguments**:
- `meetingId` (required): Webex meeting ID
- `title` (optional): New meeting title
- `agenda` (optional): New meeting agenda
- `start` (optional): New start time (ISO 8601)
- `end` (optional): New end time (ISO 8601)
- `duration` (optional): New duration in minutes
- `timezone` (optional): New timezone
- `password` (optional): New meeting password

**Example LLM prompt**: "Update meeting abc123 to start at 3pm instead"

---

### webex_list_messages

**What it does**: Lists messages in a Webex room (space). Returns message text, sender, and timestamp.

**When to use**: Read conversation history or monitor room activity.

**Arguments**:
- `roomId` (optional): Room ID to list messages from
- `roomType` (optional): Filter by room type: direct or group
- `mentionedPeople` (optional): Filter messages mentioning a person
- `before` (optional): Messages before this date (ISO 8601)
- `beforeMessage` (optional): Messages before this message ID
- `max` (optional, max 200): default 50

**Example LLM prompt**: "Show me recent messages in room abc123"

---

### webex_get_message

**What it does**: Gets details of a specific message including text content, attachments, and metadata.

**When to use**: Retrieve full message details including quoted replies.

**Arguments**:
- `messageId` (required): Webex message ID

**Example LLM prompt**: "Get the full content of message xyz789"

---

### webex_send_message

**What it does**: Sends a message to a Webex room or person. Include text, markdown, or file attachments.

**When to use**: Send notifications, responses, or alerts.

**Arguments**:
- `roomId` (optional): Room ID to send to
- `toPersonId` (optional): Person ID to send direct message
- `toPersonEmail` (optional): Email address to send direct message
- `text` (optional): Message text content
- `markdown` (optional): Markdown formatted message
- `attachments` (optional): Message attachments

**Example LLM prompt**: "Send a message to room abc123 saying 'Meeting starts in 5 minutes'"

---

### webex_list_teams

**What it does**: Lists teams that the authenticated user belongs to. Teams organize rooms and members for collaboration.

**When to use**: Find available teams or see what teams you belong to.

**Arguments**:
- `max` (optional, max 1000): default 100

**Example LLM prompt**: "List all Webex teams I'm a member of"

---

### webex_get_team

**What it does**: Gets details of a specific team including name, description, and member count.

**When to use**: Learn about a team before joining or creating rooms within it.

**Arguments**:
- `teamId` (required): Webex team ID

**Example LLM prompt**: "Get details for team abc123"

---

### webex_list_recordings

**What it does**: Lists recorded meetings for the authenticated user. Returns recording details including meeting topic, duration, and download URLs.

**When to use**: Find and access past meeting recordings.

**Arguments**:
- `siteUrl` (optional): Filter by Webex site URL
- `max` (optional, max 1000): default 50

**Example LLM prompt**: "List all my meeting recordings from this month"

---

## Webex API Reference

These tools use the Webex API. See official docs for full details:
- https://developer.webex.com/docs
- Rate limits: Vary by endpoint and plan
- Pagination: Use `max` parameter for list endpoints
- All dates: ISO 8601 format (UTC)

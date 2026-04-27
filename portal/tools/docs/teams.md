# Microsoft Teams Tools

Provider: `teams` | Engine: `nango` | Auth: OAuth via Nango (Microsoft Graph)

## Overview

These tools wrap the Microsoft Graph API for Teams functionality. They allow AI agents to read channels, messages, members, and meetings, and send messages to channels on behalf of the authenticated user.

## Authentication

**Nango OAuth (Microsoft Graph)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `Channel.ReadBasic.All`, `Channel.Read.All`, `ChannelMessage.Read.All`, `ChannelMessage.Send`, `Team.ReadBasic.All`, `TeamMember.Read.All`, `OnlineMeeting.Read.All`, `Files.Read`, `Sites.Read.All`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teams_list_channels` | List channels in a team | GET | /teams/{team_id}/channels |
| `teams_get_channel` | Get channel info | GET | /teams/{team_id}/channels/{channel_id} |
| `teams_list_messages` | List messages in a channel | GET | /teams/{team_id}/channels/{channel_id}/messages |
| `teams_get_message` | Get a message | GET | /teams/{team_id}/channels/{channel_id}/messages/{message_id} |
| `teams_send_message` | Send a message to a channel | POST | /teams/{team_id}/channels/{channel_id}/messages |
| `teams_list_members` | List team members | GET | /teams/{team_id}/members |
| `teams_get_member` | Get member info | GET | /teams/{team_id}/members/{member_id} |
| `teams_list_meetings` | List meetings | GET | /teams/{team_id}/schedule/meetings |
| `teams_get_meeting` | Get meeting details | GET | /teams/{team_id}/schedule/meetings/{meeting_id} |
| `teams_list_files` | List files in a team | GET | /teams/{team_id}/channels/{channel_id}/files |

---

## Tool Details

### teams_list_channels

**What it does**: Lists all channels in a team.

**When to use**: Find available channels, understand team structure.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "List all channels in the Engineering team"

---

### teams_get_channel

**What it does**: Gets detailed information about a specific channel.

**When to use**: Check channel description, membership type, or settings.

**Arguments**:
- `team_id` (required): Team ID
- `channel_id` (required): Channel ID

**Example LLM prompt**: "Get info about the General channel in team 12345"

---

### teams_list_messages

**What it does**: Lists messages in a channel.

**When to use**: Read channel history, find specific messages.

**Arguments**:
- `team_id` (required): Team ID
- `channel_id` (required): Channel ID
- `top` (optional): Number of messages (default 50, max 50)

**Example LLM prompt**: "Show me the last 20 messages in the General channel"

---

### teams_get_message

**What it does**: Gets a specific message by its ID.

**When to use**: Read full message content and metadata.

**Arguments**:
- `team_id` (required): Team ID
- `channel_id` (required): Channel ID
- `message_id` (required): Message ID

**Example LLM prompt**: "Get message abc123 from the General channel"

---

### teams_send_message

**What it does**: Sends a message to a Teams channel.

**When to use**: Send notifications, updates, or respond in channels on behalf of the user.

**Arguments**:
- `team_id` (required): Team ID
- `channel_id` (required): Channel ID
- `body` (required): Message body with content property

**Example LLM prompt**: "Post 'Deployment complete!' to the Engineering team's Deployments channel"

---

### teams_list_members

**What it does**: Lists all members of a team.

**When to use**: See who's in the team, check member roles.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "List all members of the Engineering team"

---

### teams_get_member

**What it does**: Gets information about a specific team member.

**When to use**: Check member profile, roles, or display name.

**Arguments**:
- `team_id` (required): Team ID
- `member_id` (required): Member ID (user ID)

**Example LLM prompt**: "Get info about member 45678 in team 12345"

---

### teams_list_meetings

**What it does**: Lists all meetings for a team.

**When to use**: Find scheduled meetings, see meeting history with join links.

**Arguments**:
- `team_id` (required): Team ID
- `start_time` (optional): Start time for range query (ISO 8601)
- `end_time` (optional): End time for range query (ISO 8601)

**Example LLM prompt**: "List all upcoming meetings for the Engineering team"

---

### teams_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: Get join URL, attendees, and recording links.

**Arguments**:
- `team_id` (required): Team ID
- `meeting_id` (required): Meeting ID

**Example LLM prompt**: "Get details for meeting 78901 in team 12345"

---

### teams_list_files

**What it does**: Lists files shared in a team channel.

**When to use**: Find documents shared in a channel, access team files.

**Arguments**:
- `team_id` (required): Team ID
- `channel_id` (required): Channel ID

**Example LLM prompt**: "List all files in the General channel of the Engineering team"

---

## Microsoft Teams API Notes

- Uses Microsoft Graph API (not the legacy Teams API)
- **Team ID**: Can be found in the Teams admin center or via team listing
- **Channel IDs**: Unique within a team, different across teams
- **Rate Limits**: Vary by endpoint; follow Microsoft Graph best practices
- **Files**: Returned via SharePoint/OneDrive integration

# Microsoft Teams Tools

Provider: `microsoft-teams` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Teams API. They allow AI agents to manage teams, channels, messages, and meetings. **Requires Microsoft Teams OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Teams)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teams_list_teams` | List teams | GET | /v1.0/me/joinedTeams |
| `teams_get_team` | Get team details | GET | /v1.0/teams/{teamId} |
| `teams_list_channels` | List channels in a team | GET | /v1.0/teams/{teamId}/channels |
| `teams_get_channel` | Get channel details | GET | /v1.0/teams/{teamId}/channels/{channelId} |
| `teams_list_messages` | List messages in a channel | GET | /v1.0/teams/{teamId}/channels/{channelId}/messages |
| `teams_get_message` | Get message details | GET | /v1.0/teams/{teamId}/channels/{channelId}/messages/{messageId} |
| `teams_send_message` | Send a message | POST | /v1.0/teams/{teamId}/channels/{channelId}/messages |
| `teams_list_members` | List team members | GET | /v1.0/teams/{teamId}/members |
| `teams_add_member` | Add a member to team | POST | /v1.0/teams/{teamId}/members |
| `teams_list_meetings` | List meetings | GET | /v1.0/me/calendar/events |

---

## Tool Details

### teams_list_teams

**What it does**: Lists all Microsoft Teams accessible to the user.

**When to use**: Browse teams, find team IDs.

**Arguments**: None

**Example LLM prompt**: "List all teams I'm a member of"

---

### teams_get_team

**What it does**: Gets details of a specific team.

**When to use**: Check team settings, member count.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get details for team 12345678-1234-1234-1234-123456789012"

---

### teams_list_channels

**What it does**: Lists all channels in a team.

**When to use**: Navigate team structure, find channels.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "List all channels in team 12345678-1234-1234-1234-123456789012"

---

### teams_get_channel

**What it does**: Gets details of a specific channel.

**When to use**: Check channel info, settings.

**Arguments**:
- `teamId` (required): Team ID
- `channelId` (required): Channel ID

**Example LLM prompt**: "Get details for channel 12345678-1234-1234-1234-123456789012"

---

### teams_list_messages

**What it does**: Lists messages in a channel.

**When to use**: Read channel conversations.

**Arguments**:
- `teamId` (required): Team ID
- `channelId` (required): Channel ID
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List recent messages in channel 12345678-1234-1234-1234-123456789012"

---

### teams_get_message

**What it does**: Gets details of a specific message.

**When to use**: Read message content, check reactions.

**Arguments**:
- `teamId` (required): Team ID
- `channelId` (required): Channel ID
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message 12345678-1234-1234-1234-123456789012"

---

### teams_send_message

**What it does**: Sends a message to a channel.

**When to use**: Post announcements, reply in channel.

**Arguments**:
- `teamId` (required): Team ID
- `channelId` (required): Channel ID
- `body` (required): Message content

**Example LLM prompt**: "Send a message 'Hello team' to channel 12345678-1234-1234-1234-123456789012"

---

### teams_list_members

**What it does**: Lists all members of a team.

**When to use**: View team roster, check membership.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "List all members of team 12345678-1234-1234-1234-123456789012"

---

### teams_add_member

**What it does**: Adds a member to a team.

**When to use**: Onboard new team members.

**Arguments**:
- `teamId` (required): Team ID
- `userId` (required): User ID to add
- `roles` (optional): Member roles (default ["member"])

**Example LLM prompt**: "Add user john@example.com to team 12345678-1234-1234-1234-123456789012"

---

### teams_list_meetings

**What it does**: Lists upcoming and past meetings for the user.

**When to use**: View calendar, find meetings.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List my meetings for today"

---

## Teams API Notes

- **Teams**: Collections of channels
- **Channels**: Conversation spaces within teams
- **Messages**: Posts in channels
- **Members**: Team participants
- **Meetings**: Calendar events with Teams links

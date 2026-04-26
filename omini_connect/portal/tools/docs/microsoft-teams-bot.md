# Microsoft Teams Bot Tools

Provider: `microsoft-teams-bot` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Microsoft Teams Bot API. They allow AI agents to manage bot conversations, send messages, and manage activities. **Requires Teams Bot OAuth2.**

## Authentication

**Nango OAUTH2 (Teams Bot)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://smba.trafficmanager.net/teams`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teams_bot_list_conversations` | List conversations | GET | /v3/conversations |
| `teams_bot_get_conversation` | Get conversation details | GET | /v3/conversations/{conversationId} |
| `teams_bot_send_message` | Send bot message | POST | /v3/conversations/{conversationId}/activities |
| `teams_bot_list_members` | List conversation members | GET | /v3/conversations/{conversationId}/members |
| `teams_bot_update_activity` | Update message activity | PUT | /v3/conversations/{conversationId}/activities/{activityId} |
| `teams_bot_delete_activity` | Delete message activity | DELETE | /v3/conversations/{conversationId}/activities/{activityId} |
| `teams_bot_create_conversation` | Create new conversation | POST | /v3/conversations |
| `teams_bot_list_channels` | List team channels | GET | /v3/teams/{teamId}/channels |
| `teams_bot_get_channel` | Get channel details | GET | /v3/teams/{teamId}/channels/{channelId} |
| `teams_bot_list_teams` | List teams | GET | /v3/teams |

---

## Tool Details

### teams_bot_list_conversations

**What it does**: Lists all conversations the bot is part of.

**When to use**: Find active bot conversations.

**Arguments**: None

**Example LLM prompt**: "List all bot conversations"

---

### teams_bot_get_conversation

**What it does**: Gets details of a specific conversation.

**When to use**: Check conversation state, info.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation CON-12345"

---

### teams_bot_send_message

**What it does**: Sends a message to a conversation.

**When to use**: Reply in conversations, send notifications.

**Arguments**:
- `conversationId` (required): Conversation ID
- `text` (required): Message text
- `contentType` (optional): Content type (text, html) (default text)

**Example LLM prompt**: "Send message 'Hello' to conversation CON-12345"

---

### teams_bot_list_members

**What it does**: Lists all members in a conversation.

**When to use**: Check participant list.

**Arguments**:
- `conversationId` (required): Conversation ID

**Example LLM prompt**: "List members in conversation CON-12345"

---

### teams_bot_update_activity

**What it does**: Updates an existing message activity.

**When to use**: Edit bot messages, update content.

**Arguments**:
- `conversationId` (required): Conversation ID
- `activityId` (required): Activity ID
- `text` (required): Updated message text

**Example LLM prompt**: "Update message ACT-12345 in conversation CON-12345"

---

### teams_bot_delete_activity

**What it does**: Deletes a message activity.

**When to use**: Remove bot messages.

**Arguments**:
- `conversationId` (required): Conversation ID
- `activityId` (required): Activity ID

**Example LLM prompt**: "Delete message ACT-12345 from conversation CON-12345"

---

### teams_bot_create_conversation

**What it does**: Creates a new conversation with a user or channel.

**When to use**: Start new bot interactions.

**Arguments**:
- `bot` (optional): Bot identity
- `channel` (optional): Channel data for the new conversation

**Example LLM prompt**: "Create a new conversation"

---

### teams_bot_list_channels

**What it does**: Lists all channels in a team.

**When to use**: Find channels for bot deployment.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "List all channels in team TEAM-12345"

---

### teams_bot_get_channel

**What it does**: Gets details of a specific channel.

**When to use**: Check channel information.

**Arguments**:
- `teamId` (required): Team ID
- `channelId` (required): Channel ID

**Example LLM prompt**: "Get details for channel CHAN-12345"

---

### teams_bot_list_teams

**What it does**: Lists all teams the bot is installed in.

**When to use**: Find bot installation scope.

**Arguments**: None

**Example LLM prompt**: "List all teams with the bot"

---

## Teams Bot Notes

- **Bot Framework**: Microsoft Bot Framework integration
- **Conversations**: 1:1 or group bot interactions
- **Activities**: Messages and events in conversations
- **Teams**: Microsoft Teams platform
- **Channel data**: Platform-specific conversation metadata

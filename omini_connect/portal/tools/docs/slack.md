# Slack Tools

Provider: `slack` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Slack API. They allow AI agents to send messages, read conversations, manage channels, and work with files on behalf of the authenticated user.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `channels:read`, `channels:history`, `chat:write`, `files:read`, `files:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `slack_list_channels` | List conversations (channels, DMs) | GET | /conversations.list |
| `slack_get_channel` | Get channel info | GET | /conversations.info |
| `slack_post_message` | Post a message to a channel | POST | /chat.postMessage |
| `slack_list_messages` | Get messages in a channel | GET | /conversations.history |
| `slack_get_message` | Get a specific message | GET | /conversations.history |
| `slack_upload_file` | Upload a file to Slack | POST | /files.upload |
| `slack_list_users` | List workspace users | GET | /users.list |
| `slack_get_user` | Get user info | GET | /users.info |
| `slack_list_files` | List files | GET | /files.list |
| `slack_get_file` | Get file info | GET | /files.info |

---

## Tool Details

### slack_list_channels

**What it does**: Lists all conversations in the workspace — public channels, private channels, DMs, and group DMs.

**When to use**: Find channels to post in, browse available conversations, or list all accessible DMs.

**Arguments**:
- `types` (optional): Comma-separated types (default: `public_channel,private_channel`)
- `limit` (optional): Number of results (default 100)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all public channels in our Slack workspace"

---

### slack_get_channel

**What it does**: Gets information about a specific channel including name, purpose, member count, and settings.

**When to use**: Check channel details before posting, understand channel purpose.

**Arguments**:
- `channel` (required): Channel ID

**Example LLM prompt**: "Get info about the #general channel"

---

### slack_post_message

**What it does**: Posts a message to a Slack channel or DM. Include text content and optionally thread replies.

**When to use**: Send notifications, updates, or respond in channels on behalf of the user.

**Arguments**:
- `channel` (required): Channel ID or name (e.g., `#general`)
- `text` (required): Message text
- `thread_ts` (optional): Parent message timestamp to reply in a thread

**Example LLM prompt**: "Post 'Deployment complete!' to the #deployments channel"

---

### slack_list_messages

**What it does**: Gets message history from a channel or DM.

**When to use**: Read past messages, find information shared in a channel, or get context from a thread.

**Arguments**:
- `channel` (required): Channel ID
- `limit` (optional): Number of messages (default 100)
- `cursor` (optional): Pagination cursor
- `oldest` (optional): Start of time range (Unix timestamp)

**Example LLM prompt**: "Show me the last 20 messages in #general"

---

### slack_get_message

**What it does**: Gets a specific message by its timestamp.

**When to use**: Read full message content and metadata for a specific message.

**Arguments**:
- `channel` (required): Channel ID
- `ts` (required): Message timestamp

**Example LLM prompt**: "Get the message with timestamp 1234567890.123456 from #general"

---

### slack_upload_file

**What it does**: Uploads a file to Slack and optionally shares it to channels.

**When to use**: Share documents, code, or other files with the team.

**Arguments**:
- `channels` (required): Channel IDs to share to (comma-separated)
- `filename` (required): File name
- `content` (optional): File content (for text files)
- `title` (optional): File title

**Example LLM prompt**: "Upload report.txt to #general"

---

### slack_list_users

**What it does**: Lists all users in the workspace.

**When to use**: Find team members, get user IDs, or check who is in the workspace.

**Arguments**:
- `limit` (optional): Number of users (default 100)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all users in our Slack workspace"

---

### slack_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get a user's profile, status, or contact information.

**Arguments**:
- `user` (required): User ID

**Example LLM prompt**: "Get info about user U0123ABC456"

---

### slack_list_files

**What it does**: Lists files shared in the workspace or a specific channel.

**When to use**: Find documents, images, or other files shared in Slack.

**Arguments**:
- `channel` (optional): Filter by channel
- `count` (optional): Number of files (default 100)
- `ts_from` (optional): Start of time range (Unix timestamp)
- `ts_to` (optional): End of time range (Unix timestamp)

**Example LLM prompt**: "List all files shared in #general this week"

---

### slack_get_file

**What it does**: Gets information about a specific file including metadata, shares, and comments.

**When to use**: Get file details, check sharing permissions, or review file metadata.

**Arguments**:
- `file` (required): File ID

**Example LLM prompt**: "Get info about file F0123BCD456"

---

## Slack API Notes

- Rate limits apply to all Slack API calls
- User tokens are required for most operations
- Channel IDs start with `C`, user IDs with `U`, message timestamps with `1`
- Timestamps are Unix timestamps with microseconds (e.g., `1234567890.123456`)

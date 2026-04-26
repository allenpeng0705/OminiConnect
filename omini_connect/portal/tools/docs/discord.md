# Discord Tools

Provider: `discord` | Engine: `nango` | Auth: Bot token via Nango

## Overview

These tools wrap the Discord API via bot tokens. They allow AI agents to read channels, messages, guilds, members, and roles, and post messages to channels. **Requires a Discord bot token** with appropriate intents enabled.

## Authentication

**Nango Bot Token**:
- Bot token registered in Nango
- Token stored in Nango, accessed via `connection_ref`
- Requires `bot` scope with Server Members Intent and Message Content Intent enabled

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `discord_list_guilds` | List guilds/servers | GET | /users/@me/guilds |
| `discord_get_guild` | Get guild info | GET | /guilds/{guild_id} |
| `discord_list_channels` | List guild channels | GET | /guilds/{guild_id}/channels |
| `discord_get_channel` | Get channel info | GET | /channels/{channel_id} |
| `discord_list_messages` | List channel messages | GET | /channels/{channel_id}/messages |
| `discord_get_message` | Get a message | GET | /channels/{channel_id}/messages/{message_id} |
| `discord_post_message` | Post message to channel | POST | /channels/{channel_id}/messages |
| `discord_list_members` | List guild members | GET | /guilds/{guild_id}/members |
| `discord_get_member` | Get member info | GET | /guilds/{guild_id}/members/{user_id} |
| `discord_list_roles` | List guild roles | GET | /guilds/{guild_id}/roles |

---

## Tool Details

### discord_list_guilds

**What it does**: Lists all guilds (servers) the bot is connected to.

**When to use**: Find available servers, understand bot's connected communities.

**Arguments**:
- `limit` (optional): Number of guilds (default 100, max 200)
- `after` (optional): Guild ID to get guilds after

**Example LLM prompt**: "List all servers the bot is in"

---

### discord_get_guild

**What it does**: Gets information about a specific guild (server).

**When to use**: Get guild name, description, member count, and features.

**Arguments**:
- `guild_id` (required): Guild/server ID

**Example LLM prompt**: "Get info about guild 123456789012345678"

---

### discord_list_channels

**What it does**: Lists all channels in a Discord guild/server.

**When to use**: Find available channels, understand server structure.

**Arguments**:
- `guild_id` (required): Guild/server ID

**Example LLM prompt**: "List all channels in the Coding Devs server"

---

### discord_get_channel

**What it does**: Gets information about a specific channel.

**When to use**: Check channel topic, settings, or type.

**Arguments**:
- `channel_id` (required): Channel ID

**Example LLM prompt**: "Get info about the #general channel"

---

### discord_list_messages

**What it does**: Lists recent messages in a channel.

**When to use**: Read channel history, find specific messages.

**Arguments**:
- `channel_id` (required): Channel ID
- `limit` (optional): Number of messages (default 50, max 100)
- `before` (optional): Message ID to get older messages

**Example LLM prompt**: "Show me the last 20 messages in #general"

---

### discord_get_message

**What it does**: Gets a specific message by ID.

**When to use**: Read a specific message's full content and metadata.

**Arguments**:
- `channel_id` (required): Channel ID
- `message_id` (required): Message ID

**Example LLM prompt**: "Get message 123456789012345678 from #general"

---

### discord_post_message

**What it does**: Sends a message to a Discord channel.

**When to use**: Send announcements, respond in channels, post updates.

**Arguments**:
- `channel_id` (required): Channel ID
- `content` (required): Message content (max 2000 chars)

**Example LLM prompt**: "Post 'Meeting starting in 5 minutes!' to #announcements"

---

### discord_list_members

**What it does**: Lists members in a guild.

**When to use**: See who's in the server, check member count.

**Arguments**:
- `guild_id` (required): Guild/server ID
- `limit` (optional): Number of members (default 20, max 100)

**Example LLM prompt**: "List the first 50 members in the server"

---

### discord_get_member

**What it does**: Gets information about a specific guild member.

**When to use**: Check member profile, roles, or join date.

**Arguments**:
- `guild_id` (required): Guild/server ID
- `user_id` (required): User ID

**Example LLM prompt**: "Get info about user 123456789012345678 in the server"

---

### discord_list_roles

**What it does**: Lists all roles in a guild.

**When to use**: See role structure, permissions, and hierarchy.

**Arguments**:
- `guild_id` (required): Guild/server ID

**Example LLM prompt**: "List all roles in the server"

---

## Discord API Notes

- **Bot intents required**: Server Members Intent for member listing, Message Content Intent for message reading
- Rate limits: 10 requests per second globally
- Message content max 2000 characters
- Channel IDs and message IDs are snowflakes (large integers as strings)
- Guild ID needed for channel listing — use the server's guild ID

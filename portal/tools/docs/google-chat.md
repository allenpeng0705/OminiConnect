# Google Chat Tools

Provider: `google-chat` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Chat API. They allow AI agents to manage spaces, messages, members, and reactions in Google Chat. **Requires Google OAuth2 with Chat permissions.**

## Authentication

**Nango OAUTH2 (Google Chat)**:
- User authenticates via OAuth2 with Chat scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://chat.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_chat_list_spaces` | List spaces | GET | /chat/v1/spaces |
| `google_chat_get_space` | Get space details | GET | /chat/v1/spaces/{space_id} |
| `google_chat_list_messages` | List messages | GET | /chat/v1/spaces/{space_id}/messages |
| `google_chat_get_message` | Get message details | GET | /chat/v1/spaces/{space_id}/messages/{message_id} |
| `google_chat_create_message` | Create message | POST | /chat/v1/spaces/{space_id}/messages |
| `google_chat_list_members` | List members | GET | /chat/v1/spaces/{space_id}/members |
| `google_chat_get_member` | Get member details | GET | /chat/v1/spaces/{space_id}/members/{member_id} |
| `google_chat_list_reactions` | List reactions | GET | /chat/v1/spaces/{space_id}/messages/{message_id}/reactions |
| `google_chat_create_reaction` | Create reaction | POST | /chat/v1/spaces/{space_id}/messages/{message_id}/reactions |
| `google_chat_get_space_event` | Get space event | GET | /chat/v1/spaces/{space_id}/events/{event_id} |

---

## Tool Details

### google_chat_list_spaces

**What it does**: Lists spaces in Google Chat.

**When to use**: Browse available Chat spaces.

**Arguments**: None

**Example LLM prompt**: "List all spaces in Google Chat"

---

### google_chat_get_space

**What it does**: Gets detailed information about a space.

**When to use**: Get space name and type.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "Get details for space abc123"

---

### google_chat_list_messages

**What it does**: Lists messages in a space.

**When to use**: Read messages in a space.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "List messages in space abc123"

---

### google_chat_get_message

**What it does**: Gets detailed information about a message.

**When to use**: Get message content and attachments.

**Arguments**:
- `space_id` (required): Space ID
- `message_id` (required): Message ID

**Example LLM prompt**: "Get message xyz789 in space abc123"

---

### google_chat_create_message

**What it does**: Creates a message in a space.

**When to use**: Send messages to Chat.

**Arguments**:
- `space_id` (required): Space ID
- `text` (required): Message text

**Example LLM prompt**: "Send 'Hello!' to space abc123"

---

### google_chat_list_members

**What it does**: Lists members in a space.

**When to use**: See who is in a space.

**Arguments**:
- `space_id` (required): Space ID

**Example LLM prompt**: "List members in space abc123"

---

### google_chat_get_member

**What it does**: Gets details about a member in a space.

**When to use**: Get member information.

**Arguments**:
- `space_id` (required): Space ID
- `member_id` (required): Member ID

**Example LLM prompt**: "Get member info for member xyz789"

---

### google_chat_list_reactions

**What it does**: Lists reactions to a message.

**When to use**: See emoji reactions.

**Arguments**:
- `space_id` (required): Space ID
- `message_id` (required): Message ID

**Example LLM prompt**: "List reactions to message xyz789"

---

### google_chat_create_reaction

**What it does**: Adds a reaction to a message.

**When to use**: React to a message with emoji.

**Arguments**:
- `space_id` (required): Space ID
- `message_id` (required): Message ID
- `emoji` (required): Emoji reaction

**Example LLM prompt**: "Add thumbs up reaction to message xyz789"

---

### google_chat_get_space_event

**What it does**: Gets a specific event in a space.

**When to use**: Get event details.

**Arguments**:
- `space_id` (required): Space ID
- `event_id` (required): Event ID

**Example LLM prompt**: "Get event abc456 in space abc123"

---

## Google Chat API Notes

- **Spaces**: Chat rooms where messages are sent
- **Messages**: Text messages within spaces
- **Members**: Users in a space
- **Reactions**: Emoji reactions to messages
- **Bot scope**: Requires chat.bot scope for bot access

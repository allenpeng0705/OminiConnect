# Fanvue Tools

Provider: `fanvue` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Fanvue API. They allow AI agents to manage posts, messages, subscribers, and content. Fanvue is a platform for content creators.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Fanvue
- Token stored in Nango, accessed via `connection_ref`
- Default scopes: offline_access, offline
- Token sent as Basic auth for token requests

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fanvue_get_user` | Get current user | GET | /api/v1/me |
| `fanvue_list_posts` | List posts | GET | /api/v1/posts |
| `fanvue_get_post` | Get post details | GET | /api/v1/posts/{id} |
| `fanvue_list_messages` | List messages | GET | /api/v1/messages |
| `fanvue_send_message` | Send a message | POST | /api/v1/messages |
| `fanvue_list_subscribers` | List subscribers | GET | /api/v1/subscribers |
| `fanvue_get_subscriber` | Get subscriber details | GET | /api/v1/subscribers/{id} |
| `fanvue_list_chats` | List chat conversations | GET | /api/v1/chats |
| `fanvue_get_chat` | Get chat details | GET | /api/v1/chats/{id} |
| `fanvue_get_stats` | Get account stats | GET | /api/v1/stats |

---

## Tool Details

### fanvue_get_user

**What it does**: Gets current authenticated user profile.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Get my Fanvue profile"

---

### fanvue_list_posts

**What it does**: Lists posts from the creator.

**When to use**: Browse content, find post IDs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List my posts"

---

### fanvue_get_post

**What it does**: Gets details of a specific post.

**When to use**: View post content, check engagement.

**Arguments**:
- `id` (required): Post ID

**Example LLM prompt**: "Get details for post abc123"

---

### fanvue_list_messages

**What it does**: Lists messages from inbox.

**When to use**: View received messages.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List my messages"

---

### fanvue_send_message

**What it does**: Sends a message to a user.

**When to use**: Reply to fans, send notifications.

**Arguments**:
- `recipient_id` (required): Recipient user ID
- `content` (required): Message content

**Example LLM prompt**: "Send a message to user xyz789"

---

### fanvue_list_subscribers

**What it does**: Lists all subscribers.

**When to use**: View fan base, track subscriptions.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List my subscribers"

---

### fanvue_get_subscriber

**What it does**: Gets details of a specific subscriber.

**When to use**: View subscriber info, subscription status.

**Arguments**:
- `id` (required): Subscriber ID

**Example LLM prompt**: "Get details for subscriber 123"

---

### fanvue_list_chats

**What it does**: Lists all chat conversations.

**When to use**: View direct message threads.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List my chats"

---

### fanvue_get_chat

**What it does**: Gets details of a specific chat.

**When to use**: View chat history, messages in thread.

**Arguments**:
- `id` (required): Chat ID

**Example LLM prompt**: "Get chat abc123"

---

### fanvue_get_stats

**What it does**: Gets account statistics.

**When to use**: View performance metrics.

**Arguments**: None

**Example LLM prompt**: "Get my account stats"

---

## Fanvue API Notes

- **Posts**: Content created by the user
- **Messages**: Direct messages between users
- **Subscribers**: Fans who subscribe to the account
- **Chats**: Direct message conversations
- **Stats**: Account performance metrics
- **API Version**: Header x-fanvue-api-version (default 2025-06-26)

# Chatwork Tools

Provider: `chatwork` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Chatwork API for team chat and collaboration. They allow AI agents to interact with rooms, messages, members, and tasks.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `rooms:read`, `rooms:write`, `tasks:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `chatwork_list_rooms` | List all chat rooms | GET | /rooms |
| `chatwork_get_room` | Get details of a specific room | GET | /rooms/{room_id} |
| `chatwork_list_messages` | List messages in a room | GET | /rooms/{room_id}/messages |
| `chatwork_get_message` | Get a specific message | GET | /rooms/{room_id}/messages/{message_id} |
| `chatwork_send_message` | Send a message to a room | POST | /rooms/{room_id}/messages |
| `chatwork_list_members` | List members in a room | GET | /rooms/{room_id}/members |
| `chatwork_get_member` | Get details of a room member | GET | /rooms/{room_id}/members/{account_id} |
| `chatwork_list_tasks` | List tasks in a room | GET | /rooms/{room_id}/tasks |
| `chatwork_get_task` | Get details of a specific task | GET | /rooms/{room_id}/tasks/{task_id} |
| `chatwork_get_myprofile` | Get the authenticated user's profile | GET | /my/profile |

---

## Tool Details

### chatwork_list_rooms

**What it does**: Returns a list of all chat rooms the authenticated user belongs to. Includes direct messages and group chats.

**When to use**: Find available rooms before reading messages or sending updates.

**Arguments**: None

**Example LLM prompt**: "List all my Chatwork rooms"

---

### chatwork_get_room

**What it does**: Get details of a specific chat room including name, description, icon, and member count.

**When to use**: Understand room context before reading messages or adding members.

**Arguments**:
- `room_id` (required): Room ID

**Example LLM prompt**: "Get details for room 123456"

---

### chatwork_list_messages

**What it does**: Returns a paginated list of messages in a specific chat room. Order from oldest to newest by default.

**When to use**: Catch up on team discussions, find specific information, or review conversation history.

**Arguments**:
- `room_id` (required): Room ID
- `force` (optional): Force fetch from server (bypass cache)
- `last_message_id` (optional): Return messages sent after this ID

**Example LLM prompt**: "Show me recent messages in the engineering room"

---

### chatwork_get_message

**What it does**: Get a specific message by ID including content, sender, and timestamp.

**When to use**: Retrieve a specific message referenced by ID.

**Arguments**:
- `room_id` (required): Room ID
- `message_id` (required): Message ID

**Example LLM prompt**: "Get message 789 in room 123456"

---

### chatwork_send_message

**What it does**: Send a new message to a specific chat room. Supports text and optional to-user mentions.

**When to use**: Send team updates, reply to questions, or broadcast information.

**Arguments**:
- `room_id` (required): Room ID to send message to
- `body` (required): Message content (supports Chatwork formatting)
- `self_unread` (optional): Mark message as unread for sender (0 or 1)

**Example LLM prompt**: "Send a message to room 123456 saying 'Deployment completed successfully'"

---

### chatwork_list_members

**What it does**: Returns a list of all members in a specific chat room including their role.

**When to use**: See who is in a room before @mentioning or assigning tasks.

**Arguments**:
- `room_id` (required): Room ID

**Example LLM prompt**: "List all members in the project room"

---

### chatwork_get_member

**What it does**: Get details of a specific member in a room including their role and Chatwork ID.

**When to use**: Look up a specific member's role or contact info.

**Arguments**:
- `room_id` (required): Room ID
- `account_id` (required): Chatwork account ID of the member

**Example LLM prompt**: "Get member details for account 111 in room 123456"

---

### chatwork_list_tasks

**What it does**: Returns a list of tasks in a specific chat room. Filter by assignee or completion status.

**When to use**: Track outstanding tasks, find tasks assigned to specific people, or review open work.

**Arguments**:
- `room_id` (required): Room ID
- `account_id` (optional): Filter by assignee account ID
- `status` (optional): Filter by status: open, done

**Example LLM prompt**: "Show me all open tasks in the design room"

---

### chatwork_get_task

**What it does**: Get details of a specific task including description, assignee, due date, and status.

**When to use**: Review a specific task before completing or updating it.

**Arguments**:
- `room_id` (required): Room ID
- `task_id` (required): Task ID

**Example LLM prompt**: "Get details for task 456 in room 123456"

---

### chatwork_get_myprofile

**What it does**: Get the authenticated user's Chatwork profile including name, email, avatar, and account ID.

**When to use**: Get the current user's info for posting or task assignment.

**Arguments**: None

**Example LLM prompt**: "Get my Chatwork profile"

---

## Chatwork API Reference

These tools use the Chatwork API. See official docs for full details:
- https://developer.chatwork.com
- Rate limits: 100 requests per minute for most endpoints
- Pagination: Use `last_message_id` for cursor-based pagination
- All dates: ISO 8601 format

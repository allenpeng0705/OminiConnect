# Coscreen Tools

Provider: `coscreen` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Coscreen screen sharing platform API. They allow AI agents to manage rooms, participants, recordings, chat, and screen sharing sessions.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `rooms:read`, `rooms:write`, `participants:read`, `recordings:read`, `chat:write`, `shares:read`, `shares:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `coscreen_list_rooms` | Retrieve all screen sharing rooms | GET | /rooms |
| `coscreen_get_room` | Get details of a specific room | GET | /rooms/{room_id} |
| `coscreen_create_room` | Create a new screen sharing room | POST | /rooms |
| `coscreen_list_participants` | Retrieve participants in a room | GET | /rooms/{room_id}/participants |
| `coscreen_get_participant` | Get details of a specific participant | GET | /participants/{participant_id} |
| `coscreen_list_recordings` | Retrieve all recordings | GET | /recordings |
| `coscreen_get_recording` | Get metadata of a specific recording | GET | /recordings/{recording_id} |
| `coscreen_send_chat_message` | Send a chat message | POST | /chat/messages |
| `coscreen_list_shares` | Retrieve screen shares in a room | GET | /rooms/{room_id}/shares |
| `coscreen_create_share` | Initiate a screen sharing session | POST | /shares |

---

## Tool Details

### coscreen_list_rooms

**What it does**: Returns all screen sharing rooms in your account with optional filtering.

**When to use**: Browse available rooms before starting a session or sharing.

**Arguments**:
- `status` (optional): Filter by status (`active`, `inactive`, `archived`)
- `limit` (optional): Maximum number of rooms to return

---

### coscreen_get_room

**What it does**: Gets details of a specific room including participants and settings.

**When to use**: Check room configuration before joining or sharing.

**Arguments**:
- `room_id` (required): Unique identifier of the room

---

### coscreen_create_room

**What it does**: Creates a new screen sharing room with specified configuration.

**When to use**: Set up a new room for a meeting or collaboration session.

**Arguments**:
- `name` (required): Name of the room
- `description` (optional): Room description
- `max_participants` (optional): Maximum number of participants
- `access_type` (optional): Access type (`public`, `private`, `password`)

---

### coscreen_list_participants

**What it does**: Retrieves all participants in a specific room.

**When to use**: See who is in a room before sending messages or starting shares.

**Arguments**:
- `room_id` (required): Room identifier
- `limit` (optional): Maximum number of participants to return

---

### coscreen_get_participant

**What it does**: Gets details about a specific participant.

**When to use**: Review participant info before direct messaging.

**Arguments**:
- `participant_id` (required): Unique identifier of the participant

---

### coscreen_list_recordings

**What it does**: Retrieves all screen recordings in your account.

**When to use**: Browse past recordings for review or sharing.

**Arguments**:
- `room_id` (optional): Filter by room ID
- `limit` (optional): Maximum number of recordings to return

---

### coscreen_get_recording

**What it does**: Gets metadata and details of a specific recording.

**When to use**: Review recording info before downloading or sharing.

**Arguments**:
- `recording_id` (required): Unique identifier of the recording

---

### coscreen_send_chat_message

**What it does**: Sends a chat message to a room or specific participant.

**When to use**: Communicate with participants during or outside screen sharing sessions.

**Arguments**:
- `room_id` (optional): Target room ID
- `recipient_id` (optional): Direct message recipient ID
- `content` (required): Message content

---

### coscreen_list_shares

**What it does**: Retrieves all screen shares in a specific room.

**When to use**: See current or recent screen shares in a room.

**Arguments**:
- `room_id` (required): Room identifier
- `limit` (optional): Maximum number of shares to return

---

### coscreen_create_share

**What it does**: Initiates a screen sharing session in a room.

**When to use**: Start presenting or sharing your screen with room participants.

**Arguments**:
- `room_id` (required): Room to share in
- `share_type` (optional): Type of share (`screen`, `window`, `application`)
- `title` (optional): Share title or description

---

## Coscreen API Reference

See official docs for full details on rate limits and pagination.

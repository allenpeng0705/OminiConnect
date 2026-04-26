# Ringover Tools

Provider: `ringover` | Engine: `nango` | Auth: OAuth via Nango

## Overview

RingOver is a cloud telephony and call center platform. These tools allow AI agents to manage calls, contacts, users, notes, and recordings.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with RingOver
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `calls:read`, `calls:write`, `contacts:read`, `contacts:write`, `users:read`, `recordings:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ringover_list_calls` | List all calls | GET | /v1/calls |
| `ringover_get_call` | Get call details | GET | /v1/calls/{callId} |
| `ringover_make_call` | Initiate a call | POST | /v1/calls |
| `ringover_end_call` | End an active call | DELETE | /v1/calls/{callId} |
| `ringover_list_contacts` | List all contacts | GET | /v1/contacts |
| `ringover_create_contact` | Create a contact | POST | /v1/contacts |
| `ringover_list_users` | List team users | GET | /v1/users |
| `ringover_get_user` | Get user details | GET | /v1/users/{userId} |
| `ringover_update_call_note` | Update call note | PUT | /v1/calls/{callId}/note |
| `ringover_get_call_recordings` | Get call recordings | GET | /v1/calls/{callId}/recordings |

---

## Tool Details

### ringover_list_calls

**What it does**: Returns a list of all calls.

**When to use**: View call history, monitor call activity.

**Arguments**:
- `limit` (optional): Number of calls (default 50)
- `status` (optional): Filter by status (in-progress, completed, missed)

**Example LLM prompt**: "List all calls from today"

---

### ringover_get_call

**What it does**: Gets details of a specific call.

**When to use**: Get call duration, participants, and outcome.

**Arguments**:
- `callId` (required): The call ID

**Example LLM prompt**: "Get details for call cal_abc123"

---

### ringover_make_call

**What it does**: Initiates an outbound call.

**When to use**: Start a call to a phone number.

**Arguments**:
- `to` (required): Phone number to call
- `from` (optional): Caller ID number
- `userId` (optional): User to initiate from

**Example LLM prompt**: "Make a call to 555-1234"

---

### ringover_end_call

**What it does**: Ends an active call.

**When to use**: Terminate a call early.

**Arguments**:
- `callId` (required): The call ID to end

**Example LLM prompt**: "End call cal_abc123"

---

### ringover_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: View your contact list.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)

**Example LLM prompt**: "List all contacts"

---

### ringover_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new contacts to your database.

**Arguments**:
- `name` (required): Contact name
- `phone` (optional): Phone number
- `email` (optional): Email address

**Example LLM prompt**: "Create a contact for John Smith"

---

### ringover_list_users

**What it does**: Returns a list of all team users.

**When to use**: View team members and their status.

**Arguments**:
- `limit` (optional): Number of users (default 50)

**Example LLM prompt**: "List all team users"

---

### ringover_get_user

**What it does**: Gets details of a specific user.

**When to use**: Get user information and settings.

**Arguments**:
- `userId` (required): The user ID

**Example LLM prompt**: "Get details for user usr_xyz789"

---

### ringover_update_call_note

**What it does**: Adds or updates a note on a call.

**When to use**: Log important information from a call.

**Arguments**:
- `callId` (required): The call ID
- `note` (required): Note content

**Example LLM prompt**: "Add a note 'Customer interested in upgrade' to call cal_abc123"

---

### ringover_get_call_recordings

**What it does**: Gets recordings for a specific call.

**When to use**: Listen to call recordings.

**Arguments**:
- `callId` (required): The call ID

**Example LLM prompt**: "Get recording for call cal_abc123"

---

## RingOver Notes

- Calls can be initiated to any phone number
- Contact notes help track call information
- Recordings capture full conversation audio
- Users represent team members in your account

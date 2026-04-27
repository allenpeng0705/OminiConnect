# Ring-central Tools

Provider: `ring-central` | Engine: `nango` | Auth: OAuth via Nango

## Overview

RingCentral is a business communications platform providing messaging, video, and phone services. These tools allow AI agents to manage messages, extensions, presence status, call logs, and phone numbers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with RingCentral
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `messages:read`, `sms:write`, `extensions:read`, `presence:read`, `presence:write`, `call-log:read`, `phone-number:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ring-central_list_messages` | List all messages | GET | /restapi/v1.0/account/~/extension/~/message-store |
| `ring-central_get_message` | Get message details | GET | /restapi/v1.0/account/~/extension/~/message-store/{messageId} |
| `ring-central_send_sms` | Send SMS message | POST | /restapi/v1.0/account/~/extension/~/sms |
| `ring-central_list_extensions` | List extensions | GET | /restapi/v1.0/account/~/extension |
| `ring-central_get_extension` | Get extension details | GET | /restapi/v1.0/account/~/extension/{extensionId} |
| `ring-central_list_presence` | List presence status | GET | /restapi/v1.0/account/~/extension/~/presence |
| `ring-central_update_presence` | Update presence | PUT | /restapi/v1.0/account/~/extension/~/presence |
| `ring-central_list_call_logs` | List call logs | GET | /restapi/v1.0/account/~/extension/~/call-log |
| `ring-central_get_call_log` | Get call log entry | GET | /restapi/v1.0/account/~/extension/~/call-log/{callLogId} |
| `ring-central_list_phone_numbers` | List phone numbers | GET | /restapi/v1.0/account/~/phone-number |

---

## Tool Details

### ring-central_list_messages

**What it does**: Returns a list of all messages.

**When to use**: View SMS, MMS, and fax messages.

**Arguments**:
- `limit` (optional): Number of messages (default 50)
- `type` (optional): Message type (sms, fax, pager)

**Example LLM prompt**: "List all SMS messages"

---

### ring-central_get_message

**What it does**: Gets details of a specific message.

**When to use**: Read message content and metadata.

**Arguments**:
- `messageId` (required): The message ID

**Example LLM prompt**: "Get details for message msg_abc123"

---

### ring-central_send_sms

**What it does**: Sends an SMS message.

**When to use**: Send text messages to phone numbers.

**Arguments**:
- `to` (required): Array of recipient phone numbers
- `text` (required): Message text
- `from` (optional): Sender phone number (uses default if not specified)

**Example LLM prompt**: "Send SMS 'Hello' to 555-1234"

---

### ring-central_list_extensions

**What it does**: Returns a list of all extensions.

**When to use**: View users and departments in organization.

**Arguments**:
- `limit` (optional): Number of extensions (default 100)

**Example LLM prompt**: "List all extensions"

---

### ring-central_get_extension

**What it does**: Gets details of a specific extension.

**When to use**: Get user information and contact details.

**Arguments**:
- `extensionId` (required): The extension ID

**Example LLM prompt**: "Get details for extension 101"

---

### ring-central_list_presence

**What it does**: Returns presence status of extensions.

**When to use**: See who is available, busy, or away.

**Arguments**:
- `extensionId` (optional): Filter by extension

**Example LLM prompt**: "List presence status for all extensions"

---

### ring-central_update_presence

**What it does**: Updates your presence status.

**When to use**: Set your availability status.

**Arguments**:
- `availability` (optional): Status (Available, Busy, Away, Offline)
- `userStatus` (optional): User status (Available, Busy)

**Example LLM prompt**: "Set my status to Busy"

---

### ring-central_list_call_logs

**What it does**: Returns a list of call logs.

**When to use**: View call history.

**Arguments**:
- `limit` (optional): Number of entries (default 50)
- `type` (optional): Call type (all, inbound, outbound)

**Example LLM prompt**: "List all outbound calls from this week"

---

### ring-central_get_call_log

**What it does**: Gets details of a specific call log entry.

**When to use**: Get call details including duration and recording.

**Arguments**:
- `callLogId` (required): The call log ID

**Example LLM prompt**: "Get details for call log cl_xyz789"

---

### ring-central_list_phone_numbers

**What it does**: Returns all phone numbers.

**When to use**: View available phone numbers.

**Arguments**:
- `limit` (optional): Number of phone numbers (default 100)

**Example LLM prompt**: "List all phone numbers"

---

## RingCentral API Notes

- Extensions are internal user accounts (typically 3-digit numbers)
- Messages include SMS, MMS, and fax
- Presence shows real-time availability status
- Call logs track all telephony activity

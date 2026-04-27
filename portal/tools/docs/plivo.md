# Plivo Tools

Provider: `plivo` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Plivo API. They allow AI agents to send SMS messages, manage phone calls, handle call recordings, and manage applications. Plivo is a cloud communications platform with global phone number coverage.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Plivo
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `plivo_list_messages` | List SMS messages | GET | /Account/{auth_id}/Message/ |
| `plivo_get_message` | Get message details | GET | /Account/{auth_id}/Message/{message_uuid}/ |
| `plivo_send_sms` | Send SMS message | POST | /Account/{auth_id}/Message/ |
| `plivo_list_calls` | List phone calls | GET | /Account/{auth_id}/Call/ |
| `plivo_get_call` | Get call details | GET | /Account/{auth_id}/Call/{call_uuid}/ |
| `plivo_create_call` | Initiate a call | POST | /Account/{auth_id}/Call/ |
| `plivo_list_recordings` | List call recordings | GET | /Account/{auth_id}/Recording/ |
| `plivo_get_recording` | Get recording details | GET | /Account/{auth_id}/Recording/{recording_id}/ |
| `plivo_list_applications` | List applications | GET | /Account/{auth_id}/Application/ |
| `plivo_get_application` | Get application details | GET | /Account/{auth_id}/Application/{application_id}/ |

---

## Tool Details

### plivo_list_messages

**What it does**: Lists all SMS messages sent or received through Plivo.

**When to use**: Review message history, check delivery status, find messages by sender/recipient.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List the last 20 messages sent to +1234567890"

---

### plivo_get_message

**What it does**: Gets detailed information about a specific Plivo message.

**When to use**: Check message status, view delivery details, get message metadata.

**Arguments**:
- `message_uuid` (required): Message UUID

**Example LLM prompt**: "Get details for message abc123-def456"

---

### plivo_send_sms

**What it does**: Sends an SMS to a single recipient via Plivo.

**When to use**: Send notifications, alerts, or promotional messages to customers.

**Arguments**:
- `src` (required): Plivo phone number to send from
- `dst` (required): Recipient phone number
- `text` (required): Message text

**Example LLM prompt**: "Send 'Your code is 1234' from +1987654321 to +1234567890"

---

### plivo_list_calls

**What it does**: Lists all phone calls made through Plivo.

**When to use**: Review call history, check call status, analyze call patterns.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all calls from last week"

---

### plivo_get_call

**What it does**: Gets detailed information about a specific Plivo call.

**When to use**: Check call details, verify call was completed, analyze call outcome.

**Arguments**:
- `call_uuid` (required): Call UUID

**Example LLM prompt**: "Get details for call abc123-def456-789"

---

### plivo_create_call

**What it does**: Initiates a new outbound phone call via Plivo with XML instructions.

**When to use**: Start automated call flows, initiate conference calls, deliver voice notifications.

**Arguments**:
- `from` (required): Plivo phone number to call from
- `to` (required): Recipient phone number
- `answer_url` (required): XML URL for call handling
- `ring_url` (optional): URL to play during ringing
- `hangup_url` (optional): URL to call when call ends

**Example LLM prompt**: "Call +1234567890 from +1987654321 with answer URL https://myapp.com/plivo/answer"

---

### plivo_list_recordings

**What it does**: Lists all call recordings stored in Plivo.

**When to use**: Find recordings for quality assurance, compliance, or customer service review.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all recordings from today"

---

### plivo_get_recording

**What it does**: Gets details about a specific call recording.

**When to use**: Review recording content, get audio for transcription, verify recording exists.

**Arguments**:
- `recording_id` (required): Recording ID

**Example LLM prompt**: "Get details for recording abc123def456"

---

### plivo_list_applications

**What it does**: Lists all Plivo applications.

**When to use**: View your applications, check application configurations, find application IDs.

**Arguments**:
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all my Plivo applications"

---

### plivo_get_application

**What it does**: Gets details about a specific Plivo application.

**When to use**: Review application settings, check endpoint configuration, verify app setup.

**Arguments**:
- `application_id` (required): Application ID

**Example LLM prompt**: "Get details for application abc123"

---

## Plivo API Notes

- **Phone Numbers**: Must be in E.164 format (e.g., +1234567890)
- **Plivo XML**: Plivo uses its own XML format for call handling instructions
- **Answer URL**: Must return valid Plivo XML that instructs how to handle the call
- **Message UUID**: Plivo assigns a UUID to each message for tracking
- **Call UUID**: Each call gets a unique UUID for retrieval and recordings
- **Applications**: Group phone numbers and endpoints for easier management

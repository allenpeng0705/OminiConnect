# RingCentral Tools

Provider: `ringcentral` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the RingCentral API. They allow AI agents to manage messages, calls, meetings, fax, and presence status on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `ReadMessages`, `SMS`, `ReadCallLog`, `Meeting:Read`, `Faxes`, `ReadPresence`, `EditPresence`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ringcentral_list_messages` | List messages (SMS, MMS, voicemail) | GET | /restapi/v1.0/account/~/extension/~/message-store |
| `ringcentral_get_message` | Get details of a specific message | GET | /restapi/v1.0/account/~/extension/~/message-store/{messageId} |
| `ringcentral_send_sms` | Send an SMS message | POST | /restapi/v1.0/account/~/extension/~/sms |
| `ringcentral_list_call_logs` | List call history | GET | /restapi/v1.0/account/~/extension/~/call-log |
| `ringcentral_get_call_log` | Get details of a specific call | GET | /restapi/v1.0/account/~/extension/~/call-log/{callId} |
| `ringcentral_list_meetings` | List scheduled meetings | GET | /restapi/v1.0/account/~/extension/~/meeting |
| `ringcentral_get_meeting` | Get meeting details | GET | /restapi/v1.0/account/~/extension/~/meeting/{meetingId} |
| `ringcentral_send_fax` | Send a fax | POST | /restapi/v1.0/account/~/extension/~/fax |
| `ringcentral_get_presence` | Get user presence status | GET | /restapi/v1.0/account/~/extension/~/presence |
| `ringcentral_update_presence` | Update user presence status | PUT | /restapi/v1.0/account/~/extension/~/presence |

---

## Tool Details

### ringcentral_list_messages

**What it does**: Lists messages (SMS, MMS, voicemail) for the authenticated user or a specific extension. Returns message details including sender, recipient, timestamp, and read status.

**When to use**: View message history, check for voicemails, or find specific conversations.

**Arguments**:
- `extensionId` (optional, default '~'): Extension ID
- `type` (optional): Message type: SMS, MMS, Fax, Voicemail
- `direction` (optional): Inbound or Outbound
- `readStatus` (optional): Read or Unread
- `dateFrom` (optional): Start date filter (ISO 8601)
- `dateTo` (optional): End date filter (ISO 8601)
- `per_page` (optional, max 1000): default 100
- `page` (optional): default 1

**Example LLM prompt**: "List all unread SMS messages"

---

### ringcentral_get_message

**What it does**: Gets details of a specific message including content, attachments, and delivery status.

**When to use**: Read full message content or check attachment availability.

**Arguments**:
- `messageId` (required): Message ID
- `extensionId` (optional, default '~'): Extension ID

**Example LLM prompt**: "Get the full content of message xyz789"

---

### ringcentral_send_sms

**What it does**: Sends an SMS message to a phone number. Include text content and optional attachments.

**When to use**: Send notifications, alerts, or two-factor authentication codes.

**Arguments**:
- `from` (required): Sender phone number (object with phoneNumber)
- `to` (required): Array of recipient phone numbers (array of objects with phoneNumber)
- `text` (required): SMS text content

**Example LLM prompt**: "Send an SMS to +1234567890 saying 'Meeting rescheduled to 3pm'"

---

### ringcentral_list_call_logs

**What it does**: Lists call history for an extension. Returns call details including duration, direction, result, and timestamps.

**When to use**: Review recent call history or track missed calls.

**Arguments**:
- `extensionId` (optional, default '~'): Extension ID
- `type` (optional): Call type: Voice, Fax
- `direction` (optional): Inbound, Outbound
- `dateFrom` (optional): Start date (ISO 8601)
- `dateTo` (optional): End date (ISO 8601)
- `view` (optional): Simple or Detailed (default: Simple)
- `per_page` (optional, max 1000): default 100
- `page` (optional): default 1

**Example LLM prompt**: "Show me my call logs from last week"

---

### ringcentral_get_call_log

**What it does**: Gets details of a specific call record including duration, recording URL, and participants.

**When to use**: Retrieve detailed call information or access call recordings.

**Arguments**:
- `callId` (required): Call log ID
- `extensionId` (optional, default '~'): Extension ID

**Example LLM prompt**: "Get details for call abc123"

---

### ringcentral_list_meetings

**What it does**: Lists scheduled RingCentral Video or cloud meetings for an extension. Returns meeting details including topic, start time, and join URLs.

**When to use**: View upcoming meetings or find scheduled meetings.

**Arguments**:
- `extensionId` (optional, default '~'): Extension ID
- `type` (optional): Scheduled or Instant
- `status` (optional): Scheduled, Started, Finished
- `per_page` (optional, max 1000): default 100
- `page` (optional): default 1

**Example LLM prompt**: "List all my scheduled meetings"

---

### ringcentral_get_meeting

**What it does**: Gets details of a specific meeting including topic, timing, host info, and join URLs.

**When to use**: Retrieve meeting details before starting or joining.

**Arguments**:
- `meetingId` (required): Meeting ID
- `extensionId` (optional, default '~'): Extension ID

**Example LLM prompt**: "Get details for meeting abc123"

---

### ringcentral_send_fax

**What it does**: Sends a fax message with optional attachments (PDF, TIFF).

**When to use**: Send documents via fax programmatically.

**Arguments**:
- `to` (required): Array of recipient fax numbers (array of objects with phoneNumber)
- `faxResolution` (optional): High or Low (default: High)
- `coverPageText` (optional): Cover page text
- `attachments` (optional): Fax attachments (PDF, TIFF) with filename, contentType, and base64 content

**Example LLM prompt**: "Send a fax to +1234567890 with the document attached"

---

### ringcentral_get_presence

**What it does**: Gets presence status for a user or specific extension. Returns availability, call status, and Do Not Disturb settings.

**When to use**: Check if colleagues are available before calling or sending messages.

**Arguments**:
- `extensionId` (optional, default '~'): Extension ID
- `detailed` (optional): Return detailed presence info (default: false)

**Example LLM prompt**: "Check if John is available (extension 123)"

---

### ringcentral_update_presence

**What it does**: Updates presence status for the authenticated user. Set availability, DND status, and user-visible status message.

**When to use**: Set your availability status programmatically, such as when in a meeting or away.

**Arguments**:
- `extensionId` (optional, default '~'): Extension ID
- `availability` (optional): Available, Busy, Away, Offline
- `userStatus` (optional): Busy or Available
- `dndStatus` (optional): Disabled or Enabled
- `statusMessage` (optional): Custom status message (max 100 chars)

**Example LLM prompt**: "Set my status to 'Busy' with status message 'In a meeting'"

---

## RingCentral API Reference

These tools use the RingCentral API. See official docs for full details:
- https://developers.ringcentral.com
- Rate limits: Vary by endpoint and plan
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format

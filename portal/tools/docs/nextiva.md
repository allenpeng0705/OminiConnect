# Nextiva Tools

Provider: `nextiva` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Nextiva API. They allow AI agents to manage users, call logs, voicemails, extensions, and make calls. Nextiva is a cloud-based business communications platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Nextiva
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nextiva_list_users` | List users | GET | /users |
| `nextiva_get_user` | Get user details | GET | /users/{user_id} |
| `nextiva_list_call_logs` | List call logs | GET | /call_logs |
| `nextiva_get_call_log` | Get call log details | GET | /call_logs/{call_log_id} |
| `nextiva_list_voicemails` | List voicemails | GET | /voicemails |
| `nextiva_get_voicemail` | Get voicemail details | GET | /voicemails/{voicemail_id} |
| `nextiva_list_extensions` | List extensions | GET | /extensions |
| `nextiva_get_extension` | Get extension details | GET | /extensions/{extension_id} |
| `nextiva_create_call` | Create a call | POST | /calls |
| `nextiva_send_sms` | Send SMS | POST | /sms |

---

## Tool Details

### nextiva_list_users

**What it does**: Lists all users in Nextiva with pagination.

**When to use**: Browse users, find team members, see department structure.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Users per page (default 25)

**Example LLM prompt**: "List all users in Nextiva"

---

### nextiva_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, check extension and presence status.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### nextiva_list_call_logs

**What it does**: Lists all call logs with optional date filtering.

**When to use**: See call history, analyze call patterns, track metrics.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Call logs per page (default 25)
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)

**Example LLM prompt**: "List all call logs from the last week"

---

### nextiva_get_call_log

**What it does**: Gets detailed information about a specific call log.

**When to use**: Get call details, access recordings, see disposition.

**Arguments**:
- `call_log_id` (required): Call Log ID

**Example LLM prompt**: "Get details for call log abc123"

---

### nextiva_list_voicemails

**What it does**: Lists all voicemails with optional read status filtering.

**When to use**: Check for new messages, manage voicemail queue.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Voicemails per page (default 25)
- `is_read` (optional): Filter by read status

**Example LLM prompt**: "List all unread voicemails"

---

### nextiva_get_voicemail

**What it does**: Gets detailed information about a specific voicemail.

**When to use**: Listen to voicemail, access transcription and audio URL.

**Arguments**:
- `voicemail_id` (required): Voicemail ID

**Example LLM prompt**: "Get details for voicemail abc123"

---

### nextiva_list_extensions

**What it does**: Lists all extensions in Nextiva with pagination.

**When to use**: Browse extensions, find users by extension number.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Extensions per page (default 25)

**Example LLM prompt**: "List all extensions in Nextiva"

---

### nextiva_get_extension

**What it does**: Gets detailed information about a specific extension.

**When to use**: Get extension details, check features and user assignment.

**Arguments**:
- `extension_id` (required): Extension ID

**Example LLM prompt**: "Get details for extension abc123"

---

### nextiva_create_call

**What it does**: Initiates a new outbound call from an extension.

**When to use**: Make outbound calls, connect customers to agents.

**Arguments**:
- `from_extension` (required): Source extension
- `to_phone_number` (required): Destination phone number

**Example LLM prompt**: "Make a call from extension 100 to phone number 555-1234"

---

### nextiva_send_sms

**What it does**: Sends an SMS message from a Nextiva extension.

**When to use**: Send text notifications, communicate with customers via SMS.

**Arguments**:
- `from_extension` (required): Source extension
- `to_phone_number` (required): Destination phone number
- `message` (required): SMS message content

**Example LLM prompt**: "Send an SMS from extension 100 to 555-1234 with message 'Your appointment is confirmed'"

---

## Nextiva API Notes

- **Users**: Team members with assigned extensions and roles
- **Call Logs**: Complete record of all calls with duration and recordings
- **Voicemails**: Audio messages left by callers with optional transcription
- **Extensions**: Internal phone numbers that route to users or teams
- **Calls**: Outbound calls can be initiated from any extension
- **SMS**: Text messaging from business extensions

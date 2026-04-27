# Sinch Tools

Provider: `sinch` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Sinch API. They allow AI agents to send SMS and WhatsApp messages, verify phone numbers, manage voice applications, and handle conversations. Sinch is a cloud communications platform specializing in voice and SMS services.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Sinch
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sinch_list_messages` | List SMS messages | GET | /v1/projects/{project_id}/messages |
| `sinch_get_message` | Get message details | GET | /v1/projects/{project_id}/messages/{message_id} |
| `sinch_send_sms` | Send SMS message | POST | /v1/projects/{project_id}/messages:send |
| `sinch_send_whatsapp` | Send WhatsApp message | POST | /v1/projects/{project_id}/messages:send |
| `sinch_verify_number` | Verify phone number | POST | /v1/projects/{project_id}/verifications |
| `sinch_initiate_call` | Initiate a call | POST | /v1/projects/{project_id}/callouts |
| `sinch_list_voice_applications` | List voice applications | GET | /v1/projects/{project_id}/applications |
| `sinch_get_voice_application` | Get voice application details | GET | /v1/projects/{project_id}/applications/{application_id} |
| `sinch_get_customer_support_ticket` | Get support ticket | GET | /v1/projects/{project_id}/tickets/{ticket_id} |
| `sinch_list_conversations` | List conversations | GET | /v1/projects/{project_id}/conversations |

---

## Tool Details

### sinch_list_messages

**What it does**: Lists all SMS messages sent or received through Sinch.

**When to use**: Review message history, check delivery status, find messages by time range.

**Arguments**:
- `page_size` (optional): Number of results per page (default 100)
- `page_token` (optional): Token for next page

**Example LLM prompt**: "List the last 50 messages"

---

### sinch_get_message

**What it does**: Gets detailed information about a specific Sinch message.

**When to use**: Check message delivery, view message details, get error information.

**Arguments**:
- `message_id` (required): Message ID

**Example LLM prompt**: "Get details for message abc123"

---

### sinch_send_sms

**What it does**: Sends an SMS message via Sinch.

**When to use**: Send notifications, alerts, or promotional messages to customers.

**Arguments**:
- `from` (required): Sender phone number
- `to` (required): Array of recipient phone numbers
- `body` (required): Message text

**Example LLM prompt**: "Send an SMS to +1234567890 with text 'Hello, your code is 1234'"

---

### sinch_send_whatsapp

**What it does**: Sends a WhatsApp message via Sinch.

**When to use**: Send rich messaging on WhatsApp to customers.

**Arguments**:
- `from` (required): Sender WhatsApp number
- `to` (required): Array of recipient WhatsApp numbers
- `body` (required): Message text
- `channel` (optional): Channel type (whatsapp)

**Example LLM prompt**: "Send a WhatsApp message to +1234567890 with 'Welcome!'"

---

### sinch_verify_number

**What it does**: Starts a phone number verification via SMS or voice.

**When to use**: Verify user phone numbers for registration, two-factor authentication, or identity confirmation.

**Arguments**:
- `endpoint` (required): Phone number to verify
- `method` (required): Verification method (sms, voice)
- `reference_id` (optional): Custom reference ID

**Example LLM prompt**: "Start SMS verification for +1234567890"

---

### sinch_initiate_call

**What it does**: Initiates a new outbound phone call via Sinch.

**When to use**: Start automated call flows, initiate customer callbacks, deliver voice notifications.

**Arguments**:
- `method` (required): Callout method (tts, flash)
- `destination` (required): Recipient phone number
- `domain` (optional): Call domain (ppe for production)

**Example LLM prompt**: "Call +1234567890 using TTS with message 'Your order is ready'"

---

### sinch_list_voice_applications

**What it does**: Lists all voice applications in your Sinch project.

**When to use**: View your applications, check configurations, find application IDs.

**Arguments**:
- `page_size` (optional): Number of results per page (default 100)
- `page_token` (optional): Token for next page

**Example LLM prompt**: "List all voice applications in my project"

---

### sinch_get_voice_application

**What it does**: Gets details about a specific voice application.

**When to use**: Review application settings, check endpoint configuration, verify app setup.

**Arguments**:
- `application_id` (required): Application ID

**Example LLM prompt**: "Get details for voice application abc123"

---

### sinch_get_customer_support_ticket

**What it does**: Gets details about a specific customer support ticket.

**When to use**: Check ticket status, review support case details, track resolution.

**Arguments**:
- `ticket_id` (required): Ticket ID

**Example LLM prompt**: "Get details for support ticket TKT123456"

---

### sinch_list_conversations

**What it does**: Lists all conversations in your Sinch project.

**When to use**: View conversation history, find active chats, track customer interactions.

**Arguments**:
- `page_size` (optional): Number of results per page (default 100)
- `page_token` (optional): Token for next page

**Example LLM prompt**: "List all conversations from today"

---

## Sinch API Notes

- **Phone Numbers**: Must be in E.164 format (e.g., +1234567890)
- **Project ID**: Sinch uses project-based organization; your project ID identifies your account
- **Verification Types**: SMS (code via text), Voice (code via voice), Flash Call (missed call for ID)
- **Callout Methods**: TTS (text-to-speech), Flash (flash call for ID)
- **Message Batches**: Sinch supports sending to multiple recipients in a single request
- **WhatsApp**: Requires WhatsApp channel access through Sinch
- **Conversations**: Sinch's conversation API provides omnichannel messaging across SMS, WhatsApp, and other channels

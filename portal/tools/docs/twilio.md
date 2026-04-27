# Twilio Tools

Provider: `twilio` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Twilio API. They allow AI agents to send SMS/MMS messages, manage phone calls, handle call recordings, and manage conferences. Twilio is a leading cloud communications platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Twilio
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twilio_list_messages` | List SMS/MMS messages | GET | /2010-04-01/Accounts/{AccountSid}/Messages.json |
| `twilio_get_message` | Get message details | GET | /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}.json |
| `twilio_send_sms` | Send SMS message | POST | /2010-04-01/Accounts/{AccountSid}/Messages.json |
| `twilio_list_calls` | List phone calls | GET | /2010-04-01/Accounts/{AccountSid}/Calls.json |
| `twilio_get_call` | Get call details | GET | /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}.json |
| `twilio_create_call` | Initiate a call | POST | /2010-04-01/Accounts/{AccountSid}/Calls.json |
| `twilio_list_recordings` | List call recordings | GET | /2010-04-01/Accounts/{AccountSid}/Recordings.json |
| `twilio_get_recording` | Get recording details | GET | /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}.json |
| `twilio_list_conferences` | List conferences | GET | /2010-04-01/Accounts/{AccountSid}/Conferences.json |
| `twilio_get_conference` | Get conference details | GET | /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}.json |

---

## Tool Details

### twilio_list_messages

**What it does**: Lists all SMS and MMS messages sent or received through Twilio.

**When to use**: Review message history, check delivery status, find messages by sender/recipient.

**Arguments**:
- `to` (optional): Filter by recipient phone number
- `from` (optional): Filter by sender phone number
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List the last 20 messages sent to +1234567890"

---

### twilio_get_message

**What it does**: Gets detailed information about a specific Twilio message including status, price, and timing.

**When to use**: Check if a message was delivered, view message details, track costs.

**Arguments**:
- `MessageSid` (required): Message SID

**Example LLM prompt**: "Get details for message SM1234567890abcdef"

---

### twilio_send_sms

**What it does**: Sends an SMS message via Twilio.

**When to use**: Send notifications, alerts, two-factor authentication codes, or marketing messages.

**Arguments**:
- `to` (required): Recipient phone number
- `from` (required): Twilio phone number to send from
- `body` (required): Message text

**Example LLM prompt**: "Send an SMS from +1987654321 to +1234567890 with text 'Your code is 1234'"

---

### twilio_list_calls

**What it does**: Lists all phone calls made through Twilio with filtering options.

**When to use**: Review call history, check call status, analyze call patterns.

**Arguments**:
- `to` (optional): Filter by recipient phone number
- `from` (optional): Filter by caller phone number
- `status` (optional): Filter by status (queued, ringing, in-progress, completed, busy, failed)
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all completed calls from last week"

---

### twilio_get_call

**What it does**: Gets detailed information about a specific Twilio call including duration, cost, and status.

**When to use**: Check call details, review recording availability, analyze call outcome.

**Arguments**:
- `CallSid` (required): Call SID

**Example LLM prompt**: "Get details for call CA1234567890abcdef"

---

### twilio_create_call

**What it does**: Initiates a new outbound phone call via Twilio with TwiML handling.

**When to use**: Start automated call flows, initiate conference calls, deliver notifications.

**Arguments**:
- `to` (required): Recipient phone number
- `from` (required): Twilio phone number to call from
- `url` (required): TwiML URL for call handling
- `method` (optional): HTTP method for TwiML URL (GET or POST)
- `statusCallback` (optional): URL for call status callbacks

**Example LLM prompt**: "Start a call from +1987654321 to +1234567890 with TwiML at https://myapp.com/twiml/start"

---

### twilio_list_recordings

**What it does**: Lists all call recordings stored in Twilio.

**When to use**: Find recordings for quality assurance, compliance, or customer service review.

**Arguments**:
- `callSid` (optional): Filter by call SID
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all recordings from today"

---

### twilio_get_recording

**What it does**: Gets details about a specific call recording including duration and audio URL.

**When to use**: Review recording content, get audio for transcription, verify recording exists.

**Arguments**:
- `RecordingSid` (required): Recording SID

**Example LLM prompt**: "Get details for recording RE1234567890abcdef"

---

### twilio_list_conferences

**What it does**: Lists all conferences in your Twilio account.

**When to use**: Monitor active conferences, review conference history, check participant counts.

**Arguments**:
- `status` (optional): Filter by status (initiated, in-progress, completed)
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all active conferences"

---

### twilio_get_conference

**What it does**: Gets detailed information about a specific conference including participants and status.

**When to use**: Check conference details, see who's participating, monitor conference health.

**Arguments**:
- `ConferenceSid` (required): Conference SID

**Example LLM prompt**: "Get details for conference CF1234567890abcdef"

---

## Twilio API Notes

- **Phone Numbers**: Must be in E.164 format (e.g., +1234567890)
- **TwiML**: TwiML (Twilio Markup Language) controls call behavior
- **Message Status**: Messages can be queued, sent, delivered, received, failed, or undelivered
- **Recording Storage**: Recordings are stored by Twilio and accessible via the API
- **Conference Participants**: Conferences can have multiple participants
- **Rate Limits**: API rate limits apply based on your Twilio plan

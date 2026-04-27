# BlandAI Tools

Provider: `blandai` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Bland AI API. They allow AI agents to make and manage AI-powered phone calls, create campaigns, and access call analytics. Bland AI provides voice AI agents that can conduct natural conversations over the phone.

## Authentication

**Nango API_KEY**:
- User provides API key from Bland AI
- Token stored in Nango, accessed via `connection_ref`
- API key passed in Authorization header

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `blandai_list_calls` | List calls | GET | /v1/calls |
| `blandai_get_call` | Get call details | GET | /v1/calls/{callId} |
| `blandai_create_call` | Create outbound call | POST | /v1/calls |
| `blandai_list_campaigns` | List campaigns | GET | /v1/campaigns |
| `blandai_get_campaign` | Get campaign details | GET | /v1/campaigns/{campaignId} |
| `blandai_list_recordings` | List recordings | GET | /v1/recordings |
| `blandai_get_recording` | Get recording details | GET | /v1/recordings/{recordingId} |
| `blandai_list_contacts` | List contacts | GET | /v1/contacts |
| `blandai_get_contact` | Get contact details | GET | /v1/contacts/{contactId} |
| `blandai_get_stats` | Get call statistics | GET | /v1/stats |

---

## Tool Details

### blandai_list_calls

**What it does**: Lists all calls made through Bland AI.

**When to use**: View call history, track conversations.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Calls per page (default 20)

**Example LLM prompt**: "List all calls from this week"

---

### blandai_get_call

**What it does**: Gets details for a specific call.

**When to use**: Review call details, check status.

**Arguments**:
- `callId` (required): Call ID

**Example LLM prompt**: "Get details for call C-12345"

---

### blandai_create_call

**What it does**: Creates an outbound call with AI voice.

**When to use**: Initiate AI-powered phone calls, automate outreach.

**Arguments**:
- `phone_number` (required): Recipient phone number
- `from` (optional): Caller ID number
- `task` (required): Task description for AI
- `voice` (optional): Voice ID to use
- `background_track` (optional): Background audio URL

**Example LLM prompt**: "Call +1234567890 and remind them about their appointment"

---

### blandai_list_campaigns

**What it does**: Lists all call campaigns.

**When to use**: View bulk calling campaigns.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Campaigns per page (default 20)

**Example LLM prompt**: "List all campaigns"

---

### blandai_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Check campaign status, view results.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign CAM-100"

---

### blandai_list_recordings

**What it does**: Lists all call recordings.

**When to use**: Access call recordings, review conversations.

**Arguments**:
- `callId` (optional): Filter by call ID
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Recordings per page (default 20)

**Example LLM prompt**: "List all recordings"

---

### blandai_get_recording

**What it does**: Gets details for a specific recording.

**When to use**: Access specific recording, get transcript.

**Arguments**:
- `recordingId` (required): Recording ID

**Example LLM prompt**: "Get recording R-456"

---

### blandai_list_contacts

**What it does**: Lists all contacts in Bland AI.

**When to use**: Manage contact database, find recipients.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Contacts per page (default 20)

**Example LLM prompt**: "List all contacts"

---

### blandai_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: View contact info, check history.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact C-789"

---

### blandai_get_stats

**What it does**: Gets call statistics and analytics.

**When to use**: Analyze call performance, track metrics.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get call stats for this month"

---

## BlandAI API Notes

- **AI Voice**: Calls are powered by AI that can have natural conversations
- **Campaigns**: Batch calls to multiple recipients
- **Recordings**: All calls can be recorded for review
- **Task-based**: AI is given tasks/instructions to complete during calls
- **Phone Numbers**: E.164 format recommended (+1XXXXXXXXXX)

# Clari Copilot Tools

Provider: `clari-copilot` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Clari Copilot API. Clari Copilot is a revenue intelligence platform that records and analyzes sales calls and meetings. **Requires Clari Copilot API key.**

## Authentication

**Nango API_KEY**:
- User provides their Clari Copilot API key and password
- Key passed via `x-api-key` header
- Password passed via `x-api-password` header
- Base URL: `https://rest-api.copilot.clari.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clari_copilot_list_calls` | List calls | GET | /calls |
| `clari_copilot_get_call` | Get call details | GET | /calls/{id} |
| `clari_copilot_list_meetings` | List meetings | GET | /meetings |
| `clari_copilot_get_meeting` | Get meeting details | GET | /meetings/{id} |
| `clari_copilot_list_contacts` | List contacts | GET | /contacts |
| `clari_copilot_get_contact` | Get contact details | GET | /contacts/{id} |
| `clari_copilot_list_accounts` | List accounts | GET | /accounts |
| `clari_copilot_get_account` | Get account details | GET | /accounts/{id} |
| `clari_copilot_list_activities` | List activities | GET | /activities |
| `clari_copilot_get_call_recording` | Get call recording | GET | /calls/{id}/recording |

---

## Tool Details

### clari_copilot_list_calls

**What it does**: Lists all recorded calls.

**When to use**: View sales call history.

**Arguments**:
- `account_id` (optional): Filter by account
- `from_date` (optional): Start date
- `to_date` (optional): End date

**Example LLM prompt**: "List calls from the last week"

---

### clari_copilot_get_call

**What it does**: Gets details of a specific call.

**When to use**: View call details, participants, notes.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get call 123 details"

---

### clari_copilot_list_meetings

**What it does**: Lists all recorded meetings.

**When to use**: View meeting history.

**Arguments**:
- `account_id` (optional): Filter by account

**Example LLM prompt**: "List meetings for account 123"

---

### clari_copilot_get_meeting

**What it does**: Gets details of a specific meeting.

**When to use**: View meeting details and notes.

**Arguments**:
- `id` (required): Meeting ID

**Example LLM prompt**: "Get meeting 456 details"

---

### clari_copilot_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find customer contacts.

**Arguments**:
- `account_id` (optional): Filter by account

**Example LLM prompt**: "List contacts"

---

### clari_copilot_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 789 details"

---

### clari_copilot_list_accounts

**What it does**: Lists all accounts.

**When to use**: View customer accounts.

**Arguments**: None

**Example LLM prompt**: "List all accounts"

---

### clari_copilot_get_account

**What it does**: Gets details of a specific account.

**When to use**: View account details and activity.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get account 101 details"

---

### clari_copilot_list_activities

**What it does**: Lists all sales activities.

**When to use**: View engagement history.

**Arguments**:
- `account_id` (optional): Filter by account

**Example LLM prompt**: "List activities for account 123"

---

### clari_copilot_get_call_recording

**What it does**: Gets the recording for a call.

**When to use**: Listen to or review call content.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get recording for call 123"

---

## Clari Copilot API Notes

- **Dual Header Auth**: Uses both `x-api-key` and `x-api-password`
- **Calls**: Recorded sales calls with analysis
- **Meetings**: Video meetings captured by Clari
- **Accounts**: Customer accounts in Clari
- **Activities**: Engagement activities with contacts

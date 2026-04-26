# CloudTalk Tools

Provider: `cloudtalk` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the CloudTalk API. CloudTalk is a cloud-based call center solution for managing customer communications. **Requires CloudTalk API credentials.**

## Authentication

**Nango BASIC**:
- User provides their CloudTalk API Access ID and Access Key
- Credentials passed via HTTP Basic Authentication
- Base URL: `https://my.cloudtalk.io/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cloudtalk_list_contacts` | List contacts | GET | /contacts |
| `cloudtalk_get_contact` | Get contact details | GET | /contacts/{id} |
| `cloudtalk_list_calls` | List calls | GET | /calls |
| `cloudtalk_get_call` | Get call details | GET | /calls/{id} |
| `cloudtalk_list_agents` | List agents | GET | /agents |
| `cloudtalk_get_agent` | Get agent details | GET | /agents/{id} |
| `cloudtalk_list_campaigns` | List campaigns | GET | /campaigns |
| `cloudtalk_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `cloudtalk_list_call_history` | List call history | GET | /callhistory |
| `cloudtalk_get_call_recording` | Get call recording | GET | /calls/{id}/recording |

---

## Tool Details

### cloudtalk_list_contacts

**What it does**: Lists all contacts.

**When to use**: View customer contact list.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all CloudTalk contacts"

---

### cloudtalk_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information and history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 123 details"

---

### cloudtalk_list_calls

**What it does**: Lists all calls with optional date filtering.

**When to use**: View call logs and recordings.

**Arguments**:
- `from_date` (optional): Start date
- `to_date` (optional): End date
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List calls from the last week"

---

### cloudtalk_get_call

**What it does**: Gets details of a specific call.

**When to use**: View call details and participants.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get call 456 details"

---

### cloudtalk_list_agents

**What it does**: Lists all agents.

**When to use**: View call center team.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all agents"

---

### cloudtalk_get_agent

**What it does**: Gets details of a specific agent.

**When to use**: View agent profile and status.

**Arguments**:
- `id` (required): Agent ID

**Example LLM prompt**: "Get agent 789 details"

---

### cloudtalk_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View outbound campaigns.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all campaigns"

---

### cloudtalk_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: View campaign statistics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign 101 details"

---

### cloudtalk_list_call_history

**What it does**: Lists all call history.

**When to use**: View complete call logs.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List call history"

---

### cloudtalk_get_call_recording

**What it does**: Gets the recording for a specific call.

**When to use**: Listen to call recordings.

**Arguments**:
- `id` (required): Call ID

**Example LLM prompt**: "Get recording for call 456"

---

## CloudTalk API Notes

- **BASIC Auth**: Access ID and Access Key via HTTP Basic Auth
- **Calls**: Inbound and outbound call records
- **Contacts**: Customer contact information
- **Agents**: Call center agents handling calls
- **Campaigns**: Outbound marketing campaigns

# Retell-ai Tools

Provider: `retell-ai` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Retell AI provides voice AI agents for phone calls. These tools allow AI agents to manage calls, agents, transcripts, and analytics for voice-enabled applications.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Retell AI
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `calls:read`, `calls:write`, `agents:read`, `agents:write`, `analytics:read`, `phone-numbers:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `retell-ai_list_calls` | List all calls | GET | /v1/calls |
| `retell-ai_get_call` | Get call details | GET | /v1/calls/{callId} |
| `retell-ai_create_call` | Create an outbound call | POST | /v1/calls |
| `retell-ai_end_call` | End an active call | POST | /v1/calls/{callId}/end |
| `retell-ai_list_agents` | List AI agents | GET | /v1/agents |
| `retell-ai_get_agent` | Get agent details | GET | /v1/agents/{agentId} |
| `retell-ai_create_agent` | Create an AI agent | POST | /v1/agents |
| `retell-ai_list_analytics` | Get call analytics | GET | /v1/analytics |
| `retell-ai_get_call_transcript` | Get call transcript | GET | /v1/calls/{callId}/transcript |
| `retell-ai_list_phone_numbers` | List phone numbers | GET | /v1/phone-numbers |

---

## Tool Details

### retell-ai_list_calls

**What it does**: Returns a list of all calls.

**When to use**: View call history, monitor call status.

**Arguments**:
- `limit` (optional): Number of calls (default 50)
- `status` (optional): Filter by status (in-progress, completed, failed)

**Example LLM prompt**: "List all completed calls from this week"

---

### retell-ai_get_call

**What it does**: Gets details of a specific call.

**When to use**: Check call details, duration, and outcome.

**Arguments**:
- `callId` (required): The call ID

**Example LLM prompt**: "Get details for call cal_abc123"

---

### retell-ai_create_call

**What it does**: Initiates an outbound call with an AI agent.

**When to use**: Start a voice call with a customer.

**Arguments**:
- `agentId` (required): AI agent ID to use
- `phoneNumber` (required): Phone number to call
- `metadata` (optional): Custom metadata

**Example LLM prompt**: "Create a call using agent ag_123 to phone number 555-1234"

---

### retell-ai_end_call

**What it does**: Ends an active call.

**When to use**: Terminate a call early.

**Arguments**:
- `callId` (required): The call ID to end

**Example LLM prompt**: "End call cal_abc123"

---

### retell-ai_list_agents

**What it does**: Returns a list of all AI agents.

**When to use**: View available voice agents.

**Arguments**:
- `limit` (optional): Number of agents (default 50)

**Example LLM prompt**: "List all AI agents"

---

### retell-ai_get_agent

**What it does**: Gets details of a specific agent.

**When to use**: Check agent configuration and capabilities.

**Arguments**:
- `agentId` (required): The agent ID

**Example LLM prompt**: "Get details for agent ag_xyz789"

---

### retell-ai_create_agent

**What it does**: Creates a new AI agent.

**When to use**: Set up a new voice agent.

**Arguments**:
- `name` (required): Agent name
- `model` (optional): AI model to use
- `voice` (optional): Voice configuration

**Example LLM prompt**: "Create an AI agent called 'Support Agent'"

---

### retell-ai_list_analytics

**What it does**: Returns call analytics and metrics.

**When to use**: Monitor call volume and performance.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get analytics for the last 7 days"

---

### retell-ai_get_call_transcript

**What it does**: Gets the transcript of a call.

**When to use**: Review conversation content.

**Arguments**:
- `callId` (required): The call ID

**Example LLM prompt**: "Get transcript for call cal_abc123"

---

### retell-ai_list_phone_numbers

**What it does**: Returns all phone numbers.

**When to use**: View your calling numbers.

**Arguments**:
- `limit` (optional): Number of phone numbers (default 50)

**Example LLM prompt**: "List all phone numbers"

---

## Retell AI Notes

- AI agents handle voice conversations autonomously
- Calls can be inbound or outbound
- Transcripts capture full conversation content
- Analytics show call volume and metrics

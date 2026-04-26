# LiveChat Tools

Provider: `livechat_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the LiveChat API. They allow AI agents to manage conversations, visitors, agents, departments, and reports in your LiveChat account.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations--read`, `conversations--write`, `visitors--read`, `agents--read`, `reports--read`, `departments--read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `livechat_more_list_conversations` | List conversations with optional state and date filters | GET | /v3.5/conversations |
| `livechat_more_get_conversation` | Get details of a specific conversation | GET | /v3.5/conversations/{conversation_id} |
| `livechat_more_create_conversation` | Create a new conversation | POST | /v3.5/conversations |
| `livechat_more_list_visitors` | List website visitors | GET | /v3.5/visitors |
| `livechat_more_get_visitor` | Get details of a specific visitor | GET | /v3.5/visitors/{visitor_id} |
| `livechat_more_list_agents` | List agents in the account | GET | /v3.5/agents |
| `livechat_more_get_agent` | Get details of a specific agent | GET | /v3.5/agents/{agent_id} |
| `livechat_more_get_report` | Get chat/agent/goal reports | GET | /v3.5/reports |
| `livechat_more_list_departments` | List configured departments | GET | /v3.5/departments |
| `livechat_more_get_department` | Get details of a specific department | GET | /v3.5/departments/{department_id} |

---

## Tool Details

### livechat_more_list_conversations

**What it does**: Returns a paginated list of conversations filtered by state, date range, or other criteria.

**When to use**: Browse active chats, find pending conversations, or review archived chats.

**Arguments**:
- `state` (optional): `active`, `pending`, `archived` — default `active`
- `date_from` (optional): ISO 8601 start date
- `date_to` (optional): ISO 8601 end date
- `page` (optional): default 1
- `limit` (optional, max 100): default 20

**Example LLM prompt**: "Show me all pending conversations from today"

---

### livechat_more_get_conversation

**What it does**: Gets full details of a specific conversation including messages and participants.

**When to use**: Read the full chat history before responding or taking action.

**Arguments**:
- `conversation_id` (required): The conversation ID

**Example LLM prompt**: "Show me the details of conversation abc123"

---

### livechat_more_create_conversation

**What it does**: Creates a new conversation with a specific user.

**When to use**: Initiate a chat with a visitor or start a new conversation thread.

**Arguments**:
- `user_id` (required): User ID to start the conversation with
- `session_id` (optional): Session ID
- `message` (optional): Initial message with `type` and `content`

**Example LLM prompt**: "Start a new conversation with user john@example.com"

---

### livechat_more_list_visitors

**What it does**: Lists visitors on your website with optional state filtering.

**When to use**: Monitor active users, see who's browsing, or track visitor patterns.

**Arguments**:
- `state` (optional): `online`, `offline`, `all` — default `all`
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "Who is currently browsing the website?"

---

### livechat_more_get_visitor

**What it does**: Gets detailed visitor info including browse history and chat transcript.

**When to use**: Learn about a visitor's behavior before responding or routing their chat.

**Arguments**:
- `visitor_id` (required): The visitor ID

**Example LLM prompt**: "What pages has visitor xyz visited today?"

---

### livechat_more_list_agents

**What it does**: Lists all agents in the account with their status and roles.

**When to use**: See available agents, check who's online, or assign conversations.

**Arguments**:
- `fields` (optional): Array of fields to include
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "List all online agents"

---

### livechat_more_get_agent

**What it does**: Gets detailed agent profile including role and performance stats.

**When to use**: Check an agent's workload, permissions, or performance before routing.

**Arguments**:
- `agent_id` (required): The agent ID

**Example LLM prompt**: "Show me agent john's performance this week"

---

### livechat_more_get_report

**What it does**: Generates reports on chat volume, agent activity, or goal completion.

**When to use**: Analyze performance, track response times, or measure goal achievement.

**Arguments**:
- `type` (required): `chats`, `agents`, or `goals`
- `date_from` (required): Start date (YYYY-MM-DD)
- `date_to` (required): End date (YYYY-MM-DD)
- `agent_id` (optional): Filter by specific agent

**Example LLM prompt**: "Get the chat report for last week"

---

### livechat_more_list_departments

**What it does**: Lists all departments configured for routing chats to specific teams.

**When to use**: See available departments before routing or assigning conversations.

**Arguments**:
- `fields` (optional): Array of fields to include
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "What departments are configured?"

---

### livechat_more_get_department

**What it does**: Gets detailed department info including agents, routing rules, and configuration.

**When to use**: Understand department setup before routing a customer there.

**Arguments**:
- `department_id` (required): The department ID

**Example LLM prompt**: "Show me the Sales department configuration"

---

## LiveChat API Reference

These tools use the LiveChat API. See official docs for full details:
- https://developers.livechat.com/docs/api/
- Rate limits: Varies by plan
- Pagination: Use `page` and `limit` parameters
- All dates: ISO 8601 format (YYYY-MM-DD for reports)

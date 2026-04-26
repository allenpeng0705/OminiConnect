# Livechat Tools

Provider: `livechat` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Livechat API. They allow AI agents to manage real-time customer conversations, visitors, agents, canned responses, and reporting. Livechat is a customer service platform that enables live chat support with features like canned responses and team management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Livechat
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations:r`, `conversations:w`, `messages:r`, `messages:w`, `visitors:r`, `agents:r`, `canned_responses:r`, `reports:r`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `livechat_list_conversations` | List all conversations | GET | /conversations |
| `livechat_get_conversation` | Get conversation details | GET | /conversations/{id} |
| `livechat_send_message` | Send a message | POST | /conversations/{id}/messages |
| `livechat_list_visitors` | List all visitors | GET | /visitors |
| `livechat_get_visitor` | Get visitor details | GET | /visitors/{id} |
| `livechat_list_agents` | List all agents | GET | /agents |
| `livechat_get_agent` | Get agent details | GET | /agents/{id} |
| `livechat_list_canned_responses` | List all canned responses | GET | /canned_responses |
| `livechat_get_canned_response` | Get canned response details | GET | /canned_responses/{id} |
| `livechat_get_report` | Get reporting data | GET | /reports/{type} |

---

## Tool Details

### livechat_list_conversations

**What it does**: Retrieves all conversations from Livechat with optional filtering by status, agent, or date range.

**When to use**: Monitor active chat sessions or review closed conversations.

**Arguments**:
- `status` (optional): Filter by status (open, closed)
- `agent_id` (optional): Filter by agent ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all open conversations from today"

---

### livechat_get_conversation

**What it does**: Gets detailed information about a specific conversation including all messages and participant information.

**When to use**: Review conversation history before following up with a customer.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation ID abc123"

---

### livechat_send_message

**What it does**: Sends a message in an existing Livechat conversation to reply to a visitor.

**When to use**: Respond to a customer question or add a system message.

**Arguments**:
- `conversation_id` (required): Conversation ID
- `body` (required): Message body content
- `type` (optional): Message type (message, system)

**Example LLM prompt**: "Send a message to conversation xyz789 saying 'How can I help you today?'"

---

### livechat_list_visitors

**What it does**: Retrieves all visitors from Livechat with optional filtering by name, email, or country.

**When to use**: See current website visitors or search for past visitors.

**Arguments**:
- `name` (optional): Filter by name
- `email` (optional): Filter by email
- `country` (optional): Filter by country code
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all visitors from the United States"

---

### livechat_get_visitor

**What it does**: Gets detailed information about a specific visitor including their chat history and attributes.

**When to use**: Review visitor profile before starting a chat or during an ongoing conversation.

**Arguments**:
- `id` (required): Visitor ID

**Example LLM prompt**: "Get details for visitor ID v_12345"

---

### livechat_list_agents

**What it does**: Retrieves all agents in Livechat who handle incoming conversations.

**When to use**: See available agents and their online status.

**Arguments**:
- `status` (optional): Filter by status (online, offline, away)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all online agents available for chat"

---

### livechat_get_agent

**What it does**: Gets detailed information about a specific agent including their workload and statistics.

**When to use**: Check agent performance or distribute conversation load.

**Arguments**:
- `id` (required): Agent ID

**Example LLM prompt**: "Get details for agent ID agent_456"

---

### livechat_list_canned_responses

**What it does**: Retrieves all canned responses in Livechat - pre-written templates for common questions.

**When to use**: Browse available quick responses or find specific templates.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all canned responses for shipping questions"

---

### livechat_get_canned_response

**What it does**: Gets detailed information about a specific canned response including its text and trigger conditions.

**When to use**: Review or update a specific canned response template.

**Arguments**:
- `id` (required): Canned response ID

**Example LLM prompt**: "Get details for canned response ID cr_789"

---

### livechat_get_report

**What it does**: Retrieves reporting data from Livechat including chat metrics and agent performance.

**When to use**: Analyze support performance or conversation trends.

**Arguments**:
- `type` (required): Report type (chats, agents, queue)
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)

**Example LLM prompt**: "Get chat report for the last 7 days"

---

## Livechat API Notes

- **Real-time Chat**: Livechat focuses on live customer support conversations
- **Visitor Tracking**: Track website visitors and their browsing behavior
- **Agent Management**: Manage support team availability and workload
- **Canned Responses**: Pre-written templates speed up agent responses
- **Reporting**: Track chat volume, response times, and agent metrics
- **Pagination**: Default per_page is 50, adjust based on your needs

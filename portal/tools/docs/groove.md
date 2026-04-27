# Groove

Provider: `groove` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

Groove is a help desk platform for managing conversations, customers, agents, and macros. This integration allows AI agents to interact with Groove data through OminiConnect for automated customer support and reporting.

## Authentication

**Nango (OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversations.read`, `conversations.write`, `customers.read`, `agents.read`, `macros.read`, `reports.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `groove_list_conversations` | List conversations | GET | /api/v2/conversations |
| `groove_get_conversation` | Get conversation details | GET | /api/v2/conversations/{conversationId} |
| `groove_reply_to_conversation` | Reply to a conversation | POST | /api/v2/conversations/{conversationId}/messages |
| `groove_list_customers` | List customers | GET | /api/v2/customers |
| `groove_get_customer` | Get customer details | GET | /api/v2/customers/{customerId} |
| `groove_list_agents` | List agents | GET | /api/v2/agents |
| `groove_get_agent` | Get agent details | GET | /api/v2/agents/{agentId} |
| `groove_list_macros` | List macros | GET | /api/v2/macros |
| `groove_get_macro` | Get macro details | GET | /api/v2/macros/{macroId} |
| `groove_get_reports` | Get reports | GET | /api/v2/reports |

## Tool Details

### Conversations

#### groove_list_conversations

List conversations in Groove. Filter by status, assignee, mailbox, or tag.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression (e.g., state='open')

#### groove_get_conversation

Get detailed information about a conversation.

**Input Parameters:**
- `conversation_id` (string, required): The conversation ID

#### groove_reply_to_conversation

Reply to a Groove conversation.

**Input Parameters:**
- `conversation_id` (string, required): The conversation ID
- `body` (string, required): Reply message body (text or HTML)
- `type` (string, optional): Message type: reply, note, or forward (default: reply)

### Customers

#### groove_list_customers

List customers in Groove.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression

#### groove_get_customer

Get detailed information about a customer.

**Input Parameters:**
- `customer_id` (string, required): The customer ID

### Agents

#### groove_list_agents

List agents in Groove.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression

#### groove_get_agent

Get detailed information about an agent.

**Input Parameters:**
- `agent_id` (string, required): The agent ID

### Macros

#### groove_list_macros

List macros in Groove.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 20, max 100)
- `filter` (string, optional): Filter expression

#### groove_get_macro

Get detailed information about a macro.

**Input Parameters:**
- `macro_id` (string, required): The macro ID

### Reports

#### groove_get_reports

Get reports and analytics from Groove.

**Input Parameters:**
- `report_type` (string, required): Report type: conversations, team, or inbox
- `start_date` (string, optional): Start date (ISO 8601 format)
- `end_date` (string, optional): End date (ISO 8601 format)

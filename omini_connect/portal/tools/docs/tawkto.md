# Tawk.to Tools

Provider: `tawkto` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

Tawk.to is a free live chat application that enables website owners to monitor and chat with visitors in real-time. These tools wrap the Tawk.to API, allowing AI agents to manage properties, visitors, chats, agents, and automated triggers on behalf of support teams.

## Authentication

**Nango (API Key)**:
- User authenticates via Nango Connect
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `property`, `visitor`, `chat`, `agents`, `departments`, `triggers`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tawkto_list_properties` | List all chat properties (websites) | GET | /property |
| `tawkto_get_property` | Get property details | GET | /property/{propertyId} |
| `tawkto_list_visitors` | List current website visitors | GET | /visitors |
| `tawkto_get_visitor` | Get visitor details | GET | /visitors/{visitorId} |
| `tawkto_list_chats` | List chat sessions | GET | /chats |
| `tawkto_get_chat` | Get chat transcript | GET | /chats/{chatId} |
| `tawkto_list_agents` | List all agents | GET | /agents |
| `tawkto_get_agent` | Get agent details | GET | /agents/{agentId} |
| `tawkto_list_departments` | List departments | GET | /departments |
| `tawkto_list_triggers` | List automated triggers | GET | /triggers |

---

## Tool Details

### tawkto_list_properties

**What it does**: Returns a paginated list of all properties (websites) connected to your Tawk.to account.

**When to use**: See all websites you're monitoring, or find a specific property before checking its visitors or chats.

**Arguments**:
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "Show me all properties in my Tawk.to account"

---

### tawkto_get_property

**What it does**: Returns detailed information about a specific property including its widget settings, chat limits, and current status.

**When to use**: Check property configuration, verify the widget is properly installed, or review limits before taking action.

**Arguments**:
- `propertyId` (required): Property ID

**Example LLM prompt**: "What's the status of property abc123?"

---

### tawkto_list_visitors

**What it does**: Returns a list of currently active visitors on your website(s). Shows real-time browsing activity.

**When to use**: Monitor live traffic, identify high-value visitors for proactive chat, or see which pages are most visited.

**Arguments**:
- `propertyId` (optional): Filter by property ID
- `page` (optional): default 1
- `limit` (optional): default 50

**Example LLM prompt**: "Who is currently browsing our website?"

---

### tawkto_get_visitor

**What it does**: Returns detailed information about a specific visitor including current page, time on site, and chat history.

**When to use**: Get context before starting a conversation, or review past interactions with a returning visitor.

**Arguments**:
- `visitorId` (required): Visitor ID

**Example LLM prompt**: "Show me details for visitor xyz789 — what page are they on?"

---

### tawkto_list_chats

**What it does**: Returns a paginated list of chat sessions, optionally filtered by status, agent, or date range.

**When to use**: Review recent conversations, find chats by a specific agent, or archive old sessions.

**Arguments**:
- `propertyId` (optional): Filter by property
- `status` (optional): `active`, `pending`, `archive`
- `agentId` (optional): Filter by agent
- `startDate` (optional): ISO 8601 date
- `endDate` (optional): ISO 8601 date
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "Show me all archived chats from last week"

---

### tawkto_get_chat

**What it does**: Returns the full transcript and metadata of a specific chat including all messages, agent, and visitor info.

**When to use**: Review a conversation for quality assurance, or get context on a customer's issue.

**Arguments**:
- `chatId` (required): Chat ID

**Example LLM prompt**: "Show me the transcript for chat #456"

---

### tawkto_list_agents

**What it does**: Returns all agents in your Tawk.to account with their status, role, and department.

**When to use**: See who's online, check team availability, or find a specific agent's ID.

**Arguments**:
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "List all agents and their current status"

---

### tawkto_get_agent

**What it does**: Returns detailed information about a specific agent including schedule, performance metrics, and assigned departments.

**When to use**: Review agent performance or check their availability schedule.

**Arguments**:
- `agentId` (required): Agent ID

**Example LLM prompt**: "Show me agent stats for john@company.com"

---

### tawkto_list_departments

**What it does**: Returns all departments configured in your account. Departments route chats to specific teams.

**When to use**: See how agents are organized, or find the right department for a ticket.

**Arguments**:
- `propertyId` (optional): Filter by property
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "What departments are configured for our support team?"

---

### tawkto_list_triggers

**What it does**: Returns all automated triggers. Triggers fire based on conditions (time on page, visitor country, etc.) to start chats automatically.

**When to use**: Review automation rules, see what's configured for proactive chat, or audit your automation strategy.

**Arguments**:
- `propertyId` (optional): Filter by property
- `page` (optional): default 1
- `limit` (optional): default 20

**Example LLM prompt**: "What triggers are active for property abc123?"

---

## Tawk.to API Reference

These tools use the Tawk.to API. See official docs for full details:
- https://developer.tawk.to/
- Rate limits: Varies by plan
- Pagination: Use `page` and `limit` parameters
- All timestamps: ISO 8601 format
# Supportbee Tools

Provider: `supportbee` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Supportbee REST API. They allow AI agents to manage tickets, customers, agents, labels, and team settings in your Supportbee account.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tickets`, `customers`, `agents`, `labels`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `supportbee_list_tickets` | List tickets | GET | /tickets |
| `supportbee_get_ticket` | Get a specific ticket | GET | /tickets/{id} |
| `supportbee_create_ticket` | Create a new ticket | POST | /tickets |
| `supportbee_list_customers` | List customers | GET | /customers |
| `supportbee_get_customer` | Get a specific customer | GET | /customers/{id} |
| `supportbee_list_agents` | List agents | GET | /agents |
| `supportbee_get_agent` | Get a specific agent | GET | /agents/{id} |
| `supportbee_list_labels` | List labels | GET | /labels |
| `supportbee_get_label` | Get a specific label | GET | /labels/{id} |
| `supportbee_get_team_settings` | Get team settings | GET | /team_settings |

---

## Tool Details

### supportbee_list_tickets

**What it does**: Returns a paginated list of tickets filtered by status, assignee, or label. Shows ticket summary including subject, requester, and latest update.

**When to use**: Browse open tickets to see what needs attention or track ticket volume.

**Arguments**:
- `status` (optional): `open`, `closed`, `all` — default `open`
- `assignee_id` (optional): Filter by assigned agent
- `label_id` (optional): Filter by label
- `page` (optional): default 1
- `per_page` (optional): default 20, max 50

**Example LLM prompt**: "Show me all open tickets"

---

### supportbee_get_ticket

**What it does**: Gets full details of a specific ticket including all messages, comments, attachments, and metadata.

**When to use**: Read complete ticket context before responding to a customer or taking action.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Show me ticket #1234"

---

### supportbee_create_ticket

**What it does**: Creates a new ticket in Supportbee. Specify subject, requester email, and optional content.

**When to use**: Log a new customer issue or create a ticket from an incoming email or chat.

**Arguments**:
- `subject` (required): Ticket subject
- `requester_email` (required): Customer email address
- `content` (optional): Initial ticket content
- `assignee_id` (optional): Assign to specific agent
- `label_ids` (optional): Array of label IDs to apply

**Example LLM prompt**: "Create a ticket with subject 'Cannot login' from customer john@example.com"

---

### supportbee_list_customers

**What it does**: Lists all customers in your Supportbee account with their name, email, and ticket statistics.

**When to use**: Search for existing customers or view your customer base.

**Arguments**:
- `query` (optional): Search by email or name
- `page` (optional): default 1
- `per_page` (optional): default 20

**Example LLM prompt**: "Find customer john@example.com"

---

### supportbee_get_customer

**What it does**: Gets full customer profile including contact info, ticket history, and notes.

**When to use**: Get context about a customer before responding to their support request.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Show me customer profile for #5678"

---

### supportbee_list_agents

**What it does**: Lists all agents in your Supportbee team with their name, email, and availability status.

**When to use**: See available support agents or check team composition.

**Arguments**:
- `page` (optional): default 1
- `per_page` (optional): default 20

**Example LLM prompt**: "List all agents on the team"

---

### supportbee_get_agent

**What it does**: Gets details of a specific agent including their profile, current assignments, and working hours.

**When to use**: Check an agent's workload before assigning a ticket.

**Arguments**:
- `id` (required): Agent ID

**Example LLM prompt**: "Show me agent #42"

---

### supportbee_list_labels

**What it does**: Lists all labels in your Supportbee account. Labels categorize tickets for easier filtering and routing.

**When to use**: See available labels before adding them to a ticket.

**Arguments**:
- `page` (optional): default 1
- `per_page` (optional): default 20

**Example LLM prompt**: "What labels do I have?"

---

### supportbee_get_label

**What it does**: Gets details of a specific label including its name, color, and the tickets it's applied to.

**When to use**: Check what tickets use a specific label or understand label usage.

**Arguments**:
- `id` (required): Label ID

**Example LLM prompt**: "Show me label #3"

---

### supportbee_get_team_settings

**What it does**: Gets the team settings for your Supportbee account including working hours, email settings, and notification preferences.

**When to use**: Understand team configuration before managing tickets or schedules.

**Arguments**: None

**Example LLM prompt**: "What are the current team settings?"

---

## Supportbee API Reference

These tools use the Supportbee REST API. See official docs for full details:
- https://supportbee.com/api
- Rate limits: 100 requests/minute for API clients
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format

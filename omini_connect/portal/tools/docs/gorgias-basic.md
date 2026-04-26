# Gorgias Basic Tools

Provider: `gorgias-basic` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Gorgias API. They allow AI agents to manage support tickets, customers, messages, and macros. Gorgias is a helpdesk platform designed for e-commerce businesses.

## Authentication

**Nango Basic Auth**:
- User provides Gorgias username and API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read` (for reading data), `write` (for creating/updating data)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gorgias_list_tickets` | List support tickets | GET | /tickets |
| `gorgias_get_ticket` | Get ticket details | GET | /tickets/{id} |
| `gorgias_create_ticket` | Create a new ticket | POST | /tickets |
| `gorgias_update_ticket` | Update a ticket | PUT | /tickets/{id} |
| `gorgias_list_customers` | List customers | GET | /customers |
| `gorgias_get_customer` | Get customer details | GET | /customers/{id} |
| `gorgias_list_messages` | List messages | GET | /tickets/{ticket_id}/messages |
| `gorgias_create_message` | Create a message | POST | /tickets/{ticket_id}/messages |
| `gorgias_list_macros` | List macros | GET | /macros |
| `gorgias_list_users` | List agents/users | GET | /users |

---

## Tool Details

### gorgias_list_tickets

**What it does**: Lists all support tickets with optional filtering by status.

**When to use**: Browse open tickets, find tickets by status, review pending issues.

**Arguments**:
- `status` (optional): Filter by status (`open`, `pending`, `solved`, `spam`)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20, max 100)

**Example LLM prompt**: "List all open support tickets"

---

### gorgias_get_ticket

**What it does**: Gets detailed information about a specific ticket.

**When to use**: View ticket details, check conversation history, understand customer issue.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Get details for ticket 12345"

---

### gorgias_create_ticket

**What it does**: Creates a new support ticket.

**When to use**: Log a new customer issue, create a support request.

**Arguments**:
- `subject` (required): Ticket subject
- `customer_email` (required): Customer email
- `assignee_email` (optional): Assignee email
- `status` (optional): Status (`open`, `pending`, `solved`)
- `priority` (optional): Priority (`low`, `medium`, `high`, `urgent`)

**Example LLM prompt**: "Create a new ticket for customer john@example.com about login issues"

---

### gorgias_update_ticket

**What it does**: Updates an existing ticket.

**When to use**: Change ticket status, reassign, update priority.

**Arguments**:
- `id` (required): Ticket ID
- `status` (optional): New status
- `assignee_email` (optional): New assignee email
- `priority` (optional): New priority

**Example LLM prompt**: "Update ticket 12345 status to solved"

---

### gorgias_list_customers

**What it does**: Lists all customers in your Gorgias account.

**When to use**: Browse customer list, find customer accounts.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all customers"

---

### gorgias_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: View customer profile, check ticket history.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer 789"

---

### gorgias_list_messages

**What it does**: Lists all messages in a specific ticket.

**When to use**: View conversation thread, see customer and agent replies.

**Arguments**:
- `ticket_id` (required): Ticket ID

**Example LLM prompt**: "Show all messages for ticket 12345"

---

### gorgias_create_message

**What it does**: Creates a new message in a ticket.

**When to use**: Reply to a customer, add internal note.

**Arguments**:
- `ticket_id` (required): Ticket ID
- `body` (required): Message body
- `is_note` (optional): Is this a note (internal)? (default false)

**Example LLM prompt**: "Add a reply to ticket 12345 saying we're investigating"

---

### gorgias_list_macros

**What it does**: Lists all available macros.

**When to use**: Find templated responses for common issues.

**Arguments**: None

**Example LLM prompt**: "List all available macros"

---

### gorgias_list_users

**What it does**: Lists all agents/users in your Gorgias account.

**When to use**: Find available agents, check team members.

**Arguments**: None

**Example LLM prompt**: "List all support agents"

---

## Gorgias API Notes

- **Subdomain**: Your Gorgias account subdomain (e.g., `yourcompany.gorgias.com`)
- **Ticket status flow**: open -> pending -> solved
- **Priority levels**: low, medium, high, urgent
- **Internal notes**: Use `is_note: true` for internal comments not visible to customers

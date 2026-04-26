# Halo PSA Tools

Provider: `halo-psa` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Halo PSA API. They allow AI agents to manage tickets, clients, contacts, invoices, and projects. Halo PSA is a professional services automation platform.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for server-to-server authentication
- Token stored in Nango, accessed via `connection_ref`
- Token URL: https://{hostname}/auth/token
- Auth method: basic
- Scope separator: space

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `halo_list_tickets` | List tickets | GET | /tickets |
| `halo_get_ticket` | Get ticket details | GET | /tickets/{id} |
| `halo_list_clients` | List clients | GET | /clients |
| `halo_get_client` | Get client details | GET | /clients/{id} |
| `halo_list_contacts` | List contacts | GET | /contacts |
| `halo_get_contact` | Get contact details | GET | /contacts/{id} |
| `halo_list_invoices` | List invoices | GET | /invoices |
| `halo_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `halo_list_projects` | List projects | GET | /projects |
| `halo_get_project` | Get project details | GET | /projects/{id} |

---

## Tool Details

### halo_list_tickets

**What it does**: Lists all tickets in Halo PSA.

**When to use**: Browse support tickets.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Page size (default 20)

**Example LLM prompt**: "List all tickets"

---

### halo_get_ticket

**What it does**: Gets detailed information about a specific ticket.

**When to use**: View ticket details and history.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Get ticket with ID 123"

---

### halo_list_clients

**What it does**: Lists all clients in Halo PSA.

**When to use**: Browse client list.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all clients"

---

### halo_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: View client details and contacts.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get client with ID 456"

---

### halo_list_contacts

**What it does**: Lists all contacts in Halo PSA.

**When to use**: Browse contact list.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts for client 456"

---

### halo_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact details.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact with ID 789"

---

### halo_list_invoices

**What it does**: Lists all invoices in Halo PSA.

**When to use**: View invoice list.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all invoices"

---

### halo_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: View invoice details and payment status.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice with ID 101"

---

### halo_list_projects

**What it does**: Lists all projects in Halo PSA.

**When to use**: Browse project list.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all projects"

---

### halo_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details, tasks, and time entries.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project with ID 202"

---

## Halo PSA API Notes

- **API Base URL**: https://{hostname}/api
- **Auth Mode**: OAuth2 Client Credentials
- **Hostname**: Configured via connection config
- **Tickets**: Support and service tickets
- **Clients**: Customer organizations
- **Contacts**: Individual contacts at clients
- **Invoices**: Billing invoices for services
- **Projects**: Professional services projects

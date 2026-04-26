# ConnectWise PSA Tools

Provider: `connectwise-psa` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the ConnectWise PSA API. ConnectWise PSA is a professional services automation platform for IT Managed Service Providers. **Requires ConnectWise PSA credentials.**

## Authentication

**Nango BASIC**:
- User provides their ConnectWise PSA credentials
- Credentials passed via HTTP Basic Authentication
- Base URL: `https://${connectionConfig.hostname}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `connectwise_psa_list_companies` | List companies | GET | /company |
| `connectwise_psa_get_company` | Get company details | GET | /company/{id} |
| `connectwise_psa_list_contacts` | List contacts | GET | /contacts |
| `connectwise_psa_get_contact` | Get contact details | GET | /contacts/{id} |
| `connectwise_psa_list_tickets` | List tickets | GET | /service/tickets |
| `connectwise_psa_get_ticket` | Get ticket details | GET | /service/tickets/{id} |
| `connectwise_psa_create_ticket` | Create a ticket | POST | /service/tickets |
| `connectwise_psa_update_ticket` | Update a ticket | PATCH | /service/tickets/{id} |
| `connectwise_psa_list_time_entries` | List time entries | GET | /time/entries |
| `connectwise_psa_list_projects` | List projects | GET | /project/projects |

---

## Tool Details

### connectwise_psa_list_companies

**What it does**: Lists all companies in ConnectWise PSA.

**When to use**: View all customer companies.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all ConnectWise PSA companies"

---

### connectwise_psa_get_company

**What it does**: Gets details of a specific company.

**When to use**: View company information and contacts.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get company 123 details"

---

### connectwise_psa_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find customer contacts.

**Arguments**:
- `companyId` (optional): Filter by company
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List contacts for company 123"

---

### connectwise_psa_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 456 details"

---

### connectwise_psa_list_tickets

**What it does**: Lists all tickets with optional filters.

**When to use**: View support tickets, track issues.

**Arguments**:
- `status` (optional): Filter by status
- `companyId` (optional): Filter by company
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List open tickets for company 123"

---

### connectwise_psa_get_ticket

**What it does**: Gets details of a specific ticket.

**When to use**: View ticket details and history.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Get ticket 789 details"

---

### connectwise_psa_create_ticket

**What it does**: Creates a new ticket.

**When to use**: Log a new support issue.

**Arguments**:
- `summary` (required): Ticket summary
- `companyId` (required): Company ID
- `priority` (optional): Priority (1-5)

**Example LLM prompt**: "Create a ticket for company 123 with summary 'Server down'"

---

### connectwise_psa_update_ticket

**What it does**: Updates an existing ticket.

**When to use**: Change ticket status or priority.

**Arguments**:
- `id` (required): Ticket ID
- `status` (optional): New status
- `priority` (optional): New priority

**Example LLM prompt**: "Update ticket 789 to status 'In Progress'"

---

### connectwise_psa_list_time_entries

**What it does**: Lists all time entries.

**When to use**: View billable time.

**Arguments**:
- `ticketId` (optional): Filter by ticket
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List time entries for ticket 789"

---

### connectwise_psa_list_projects

**What it does**: Lists all projects.

**When to use**: View project portfolio.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List active projects"

---

## ConnectWise PSA API Notes

- **Hostname**: Your ConnectWise PSA server hostname
- **Companies**: Customer organizations
- **Contacts**: Individual contacts at companies
- **Tickets**: Support and service tickets
- **Time Entries**: Billable time tracked against tickets
- **Projects**: Managed service projects

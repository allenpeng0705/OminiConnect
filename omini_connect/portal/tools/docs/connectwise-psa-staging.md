# ConnectWise PSA (Staging) Tools

Provider: `connectwise-psa-staging` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the ConnectWise PSA Staging API. The staging environment allows testing PSA integrations without affecting production data. **Requires ConnectWise PSA Staging credentials.**

## Authentication

**Nango BASIC**:
- User provides their ConnectWise PSA Staging credentials
- Credentials passed via HTTP Basic Authentication
- Base URL: `https://${connectionConfig.hostname}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `connectwise_psa_staging_list_companies` | List companies | GET | /company |
| `connectwise_psa_staging_get_company` | Get company details | GET | /company/{id} |
| `connectwise_psa_staging_list_contacts` | List contacts | GET | /contacts |
| `connectwise_psa_staging_get_contact` | Get contact details | GET | /contacts/{id} |
| `connectwise_psa_staging_list_tickets` | List tickets | GET | /service/tickets |
| `connectwise_psa_staging_get_ticket` | Get ticket details | GET | /service/tickets/{id} |
| `connectwise_psa_staging_create_ticket` | Create a ticket | POST | /service/tickets |
| `connectwise_psa_staging_update_ticket` | Update a ticket | PATCH | /service/tickets/{id} |
| `connectwise_psa_staging_list_time_entries` | List time entries | GET | /time/entries |
| `connectwise_psa_staging_list_projects` | List projects | GET | /project/projects |

---

## Tool Details

### connectwise_psa_staging_list_companies

**What it does**: Lists all companies in staging.

**When to use**: Test company listing.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test companies in staging"

---

### connectwise_psa_staging_get_company

**What it does**: Gets details of a specific company.

**When to use**: Test company retrieval.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get company 123 details"

---

### connectwise_psa_staging_list_contacts

**What it does**: Lists all contacts.

**When to use**: Test contact listing.

**Arguments**:
- `companyId` (optional): Filter by company
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test contacts"

---

### connectwise_psa_staging_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Test contact retrieval.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 456 details"

---

### connectwise_psa_staging_list_tickets

**What it does**: Lists all tickets.

**When to use**: Test ticket listing.

**Arguments**:
- `status` (optional): Filter by status
- `companyId` (optional): Filter by company
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test tickets"

---

### connectwise_psa_staging_get_ticket

**What it does**: Gets details of a specific ticket.

**When to use**: Test ticket retrieval.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Get ticket 789 details"

---

### connectwise_psa_staging_create_ticket

**What it does**: Creates a new ticket.

**When to use**: Test ticket creation.

**Arguments**:
- `summary` (required): Ticket summary
- `companyId` (required): Company ID
- `priority` (optional): Priority (1-5)

**Example LLM prompt**: "Create a test ticket"

---

### connectwise_psa_staging_update_ticket

**What it does**: Updates an existing ticket.

**When to use**: Test ticket updates.

**Arguments**:
- `id` (required): Ticket ID
- `status` (optional): New status
- `priority` (optional): New priority

**Example LLM prompt**: "Update test ticket 789"

---

### connectwise_psa_staging_list_time_entries

**What it does**: Lists all time entries.

**When to use**: Test time entry listing.

**Arguments**:
- `ticketId` (optional): Filter by ticket
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test time entries"

---

### connectwise_psa_staging_list_projects

**What it does**: Lists all projects.

**When to use**: Test project listing.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test projects"

---

## ConnectWise PSA Staging API Notes

- **Staging Environment**: Isolated test environment
- **No Production Data**: All data is simulated
- **Same API Structure**: Mirror of production API
- **BASIC Auth**: Same authentication as production

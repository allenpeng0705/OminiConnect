# Jobber Tools

Provider: `jobber` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Jobber API. They allow AI agents to manage clients, jobs, quotes, invoices, and visits. **Requires Jobber OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Jobber
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.getjobber.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jobber_list_clients` | List clients | POST | /graphql |
| `jobber_get_client` | Get client details | POST | /graphql |
| `jobber_list_jobs` | List jobs | POST | /graphql |
| `jobber_get_job` | Get job details | POST | /graphql |
| `jobber_list_quotes` | List quotes | POST | /graphql |
| `jobber_get_quote` | Get quote details | POST | /graphql |
| `jobber_list_invoices` | List invoices | POST | /graphql |
| `jobber_get_invoice` | Get invoice details | POST | /graphql |
| `jobber_list_visits` | List visits | POST | /graphql |
| `jobber_get_visit` | Get visit details | POST | /graphql |

---

## Tool Details

### jobber_list_clients

**What it does**: Lists all clients in Jobber.

**When to use**: Find clients, view client list.

**Arguments**:
- `first` (optional): Number of results (default: 20)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all clients in Jobber"

---

### jobber_get_client

**What it does**: Gets details for a specific client.

**When to use**: Get client information, view client details.

**Arguments**:
- `client_id` (required): Client ID

**Example LLM prompt**: "Get details for client abc123"

---

### jobber_list_jobs

**What it does**: Lists all jobs.

**When to use**: View jobs, filter by status.

**Arguments**:
- `status` (optional): Job status filter
- `first` (optional): Number of results (default: 20)

**Example LLM prompt**: "List all jobs in Jobber"

---

### jobber_get_job

**What it does**: Gets details for a specific job.

**When to use**: Get job information, view job details.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job xyz789"

---

### jobber_list_quotes

**What it does**: Lists all quotes.

**When to use**: View quotes, track proposals.

**Arguments**:
- `first` (optional): Number of results (default: 20)

**Example LLM prompt**: "List all quotes in Jobber"

---

### jobber_get_quote

**What it does**: Gets details for a specific quote.

**When to use**: Get quote information.

**Arguments**:
- `quote_id` (required): Quote ID

**Example LLM prompt**: "Get details for quote q1"

---

### jobber_list_invoices

**What it does**: Lists all invoices.

**When to use**: View invoices, track payments.

**Arguments**:
- `first` (optional): Number of results (default: 20)

**Example LLM prompt**: "List all invoices in Jobber"

---

### jobber_get_invoice

**What it does**: Gets details for a specific invoice.

**When to use**: Get invoice information.

**Arguments**:
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice inv1"

---

### jobber_list_visits

**What it does**: Lists all visits.

**When to use**: View visits, track appointments.

**Arguments**:
- `first` (optional): Number of results (default: 20)

**Example LLM prompt**: "List all visits in Jobber"

---

### jobber_get_visit

**What it does**: Gets details for a specific visit.

**When to use**: Get visit information.

**Arguments**:
- `visit_id` (required): Visit ID

**Example LLM prompt**: "Get details for visit v1"

---

## Jobber API Notes

- **GraphQL API**: Jobber uses GraphQL for all API calls
- **Pagination**: Uses cursor-based pagination with `first` and `after`
- **IDs**: String IDs for clients, jobs, quotes, invoices, visits

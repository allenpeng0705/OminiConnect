# FreshBooks Tools

Provider: `freshbooks` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the FreshBooks API. They allow AI agents to manage clients, invoices, expenses, and time entries. FreshBooks is a popular accounting and invoicing platform for small businesses and freelancers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with FreshBooks
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `self`, `clients`, `invoices`, `expenses`, `projects`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freshbooks_list_clients` | List all clients | GET | /users/business/{business_id}/clients |
| `freshbooks_get_client` | Get a specific client | GET | /users/business/{business_id}/clients/{client_id} |
| `freshbooks_create_client` | Create a new client | POST | /users/business/{business_id}/clients |
| `freshbooks_list_invoices` | List all invoices | GET | /users/business/{business_id}/invoices |
| `freshbooks_get_invoice` | Get a specific invoice | GET | /users/business/{business_id}/invoices/{invoice_id} |
| `freshbooks_create_invoice` | Create a new invoice | POST | /users/business/{business_id}/invoices |
| `freshbooks_list_expenses` | List all expenses | GET | /users/business/{business_id}/expenses |
| `freshbooks_get_expense` | Get a specific expense | GET | /users/business/{business_id}/expenses/{expense_id} |
| `freshbooks_list_time_entries` | List all time entries | GET | /users/business/{business_id}/time_entries |
| `freshbooks_get_time_entry` | Get a specific time entry | GET | /users/business/{business_id}/time_entries/{time_entry_id} |

---

## Tool Details

### freshbooks_list_clients

**What it does**: Lists all clients in the FreshBooks account with optional search and pagination.

**When to use**: Find clients, browse client base, search by name or email.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page, max 100 (default: 25)
- `search` (optional): Search by client name or email

**Example LLM prompt**: "List all clients with 'Acme' in the name"

---

### freshbooks_get_client

**What it does**: Gets detailed information about a specific client including contact details and metadata.

**When to use**: Get client details before creating an invoice or viewing their account.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `client_id` (required): Client ID

**Example LLM prompt**: "Get details for client abc-123"

---

### freshbooks_create_client

**What it does**: Creates a new client in FreshBooks with name and optional contact information.

**When to use**: Add new clients to FreshBooks when setting up billing.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `email` (optional): Client email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `organization` (optional): Company/organization name
- `phone` (optional): Phone number
- `billing_address` (optional): Billing address object

**Example LLM prompt**: "Create a new client for Acme Corp with email billing@acme.com"

---

### freshbooks_list_invoices

**What it does**: Lists all invoices in the FreshBooks account with optional filtering.

**When to use**: Find outstanding invoices, see all paid invoices, filter by client or status.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page, max 100 (default: 25)
- `status` (optional): Status filter (`draft`, `sent`, `viewed`, `paid`, `overdue`)

**Example LLM prompt**: "List all unpaid invoices"

---

### freshbooks_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items, totals, and status.

**When to use**: Check invoice details before sending, verify line items, see payment status.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice xyz-456"

---

### freshbooks_create_invoice

**What it does**: Creates a new invoice in FreshBooks with client, line items, and optional dates.

**When to use**: Bill clients for services, create project invoices.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `client_id` (required): Client ID
- `lines` (required): Array of line item objects
- `invoice_number` (optional): Custom invoice number
- `date` (optional): Invoice date (YYYY-MM-DD)
- `due_date` (optional): Due date (YYYY-MM-DD)
- `notes` (optional): Invoice notes

**Example LLM prompt**: "Create an invoice for client abc-123 for $1,500"

---

### freshbooks_list_expenses

**What it does**: Lists all expenses in the FreshBooks account with optional filtering.

**When to use**: Track business expenses, find expenses by category, review spending.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page, max 100 (default: 25)
- `status` (optional): Status filter (`active`, `archived`)

**Example LLM prompt**: "List all expenses from this month"

---

### freshbooks_get_expense

**What it does**: Gets detailed information about a specific expense including vendor, amount, and category.

**When to use**: Review expense details, check category, verify vendor information.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `expense_id` (required): Expense ID

**Example LLM prompt**: "Get details for expense def-789"

---

### freshbooks_list_time_entries

**What it does**: Lists all time entries in the FreshBooks account with optional filtering.

**When to use**: Track time worked on projects, find entries by user or project.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page, max 100 (default: 25)
- `project_id` (optional): Filter by project ID
- `user_id` (optional): Filter by user ID

**Example LLM prompt**: "List all time entries for this week"

---

### freshbooks_get_time_entry

**What it does**: Gets detailed information about a specific time entry including duration, notes, and project.

**When to use**: Review time entry details, check duration, verify project assignment.

**Arguments**:
- `business_id` (required): FreshBooks Business ID
- `time_entry_id` (required): Time Entry ID

**Example LLM prompt**: "Get details for time entry te-101"

---

## FreshBooks API Notes

- **Business ID**: FreshBooks requires a Business ID for all API calls. Found in your FreshBooks account URL.
- **Client IDs**: FreshBooks uses numeric IDs for clients, invoices, and expenses.
- **Line items**: Invoices require line items with description, quantity (`qty`), and `unit_amount`.
- **Status flow**: Invoices progress through statuses: `draft` -> `sent` -> `viewed` -> `paid` -> `overdue`
- **Expense categories**: Predefined categories like Travel, Office Supplies, Meals, etc.
- **Date format**: Use YYYY-MM-DD format for all date parameters.
- **Pagination**: Default page size is 25. Use `page` and `per_page` for pagination.

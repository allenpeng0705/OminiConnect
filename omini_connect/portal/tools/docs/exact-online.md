# Exact Online Tools

Provider: `exact-online` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Exact Online API. They allow AI agents to manage accounts, invoices, contacts, items, and financial data. Exact Online is a cloud-based ERP system for small and medium businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Exact Online
- Token stored in Nango, accessed via `connection_ref`
- Uses region-based subdomain: `https://start.exactonline.{extension}/`
- Divisions are used to separate company data within an account

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `exact_online_list_accounts` | List accounts | GET | /api/v1/{division}/crm/Accounts |
| `exact_online_get_account` | Get account details | GET | /api/v1/{division}/crm/Accounts({id}) |
| `exact_online_list_invoices` | List sales invoices | GET | /api/v1/{division}/salesinvoice/Invoices |
| `exact_online_get_invoice` | Get invoice details | GET | /api/v1/{division}/salesinvoice/Invoices({id}) |
| `exact_online_list_contacts` | List contacts | GET | /api/v1/{division}/crm/Contacts |
| `exact_online_get_contact` | Get contact details | GET | /api/v1/{division}/crm/Contacts({id}) |
| `exact_online_list_items` | List items | GET | /api/v1/{division}/logistics/Items |
| `exact_online_get_item` | Get item details | GET | /api/v1/{division}/logistics/Items({id}) |
| `exact_online_list_journals` | List journals | GET | /api/v1/{division}/financial/Journals |
| `exact_online_get_financial_balance` | Get financial balance | GET | /api/v1/{division}/financial/FinancialBalance |

---

## Tool Details

### exact_online_list_accounts

**What it does**: Lists all accounts (customers and suppliers).

**When to use**: Browse the customer/supplier database.

**Arguments**:
- `division` (optional): Division ID
- `top` (optional): Number of results (default 20)

**Example LLM prompt**: "List all customer accounts"

---

### exact_online_get_account

**What it does**: Gets detailed information about a specific account.

**When to use**: Get account details, view customer info.

**Arguments**:
- `division` (optional): Division ID
- `id` (required): Account ID

**Example LLM prompt**: "Get details for account abc123"

---

### exact_online_list_invoices

**What it does**: Lists all sales invoices.

**When to use**: View outstanding invoices, track sales.

**Arguments**:
- `division` (optional): Division ID
- `top` (optional): Number of results (default 20)

**Example LLM prompt**: "List recent sales invoices"

---

### exact_online_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice details, check invoice status.

**Arguments**:
- `division` (optional): Division ID
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-001"

---

### exact_online_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact database, find people.

**Arguments**:
- `division` (optional): Division ID
- `top` (optional): Number of results (default 20)

**Example LLM prompt**: "List all contacts"

---

### exact_online_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Get contact information, view personal details.

**Arguments**:
- `division` (optional): Division ID
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact john@company.com"

---

### exact_online_list_items

**What it does**: Lists all items (products and services).

**When to use**: Browse product catalog, find items for invoices.

**Arguments**:
- `division` (optional): Division ID
- `top` (optional): Number of results (default 20)

**Example LLM prompt**: "List all products"

---

### exact_online_get_item

**What it does**: Gets details of a specific item.

**When to use**: Check item pricing, view stock levels.

**Arguments**:
- `division` (optional): Division ID
- `id` (required): Item ID

**Example LLM prompt**: "Get details for item P001"

---

### exact_online_list_journals

**What it does**: Lists all journals in the accounting system.

**When to use**: View financial journals, find transaction sources.

**Arguments**:
- `division` (optional): Division ID

**Example LLM prompt**: "List all journals"

---

### exact_online_get_financial_balance

**What it does**: Gets financial balance information.

**When to use**: Check account balances, view financial position.

**Arguments**:
- `division` (optional): Division ID

**Example LLM prompt**: "Get my current financial balance"

---

## Exact Online API Notes

- **Divisions**: Separate company entities within an Exact Online account
- **Accounts**: Customers and suppliers in the CRM
- **Invoices**: Sales invoices with line items
- **Contacts**: People linked to accounts
- **Items**: Products and services for invoicing
- **Journals**: Accounting journals for transactions
- **Rate Limiting**: Uses x-ratelimit headers; implement backoff for 429/503

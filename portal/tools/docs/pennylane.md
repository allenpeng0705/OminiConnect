# Pennylane Tools

Provider: `pennylane` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Pennylane API. They allow AI agents to manage accounts, contacts, invoices, expenses, and items. Pennylane is an accounting and invoicing platform. **Requires Pennylane OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Pennylane
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://app.pennylane.com
- Scope separator: +

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pennylane_list_accounts` | List accounts | GET | /api/v1/accounts |
| `pennylane_get_account` | Get account details | GET | /api/v1/accounts/{accountId} |
| `pennylane_list_contacts` | List contacts | GET | /api/v1/contacts |
| `pennylane_get_contact` | Get contact details | GET | /api/v1/contacts/{contactId} |
| `pennylane_list_invoices` | List invoices | GET | /api/v1/invoices |
| `pennylane_get_invoice` | Get invoice details | GET | /api/v1/invoices/{invoiceId} |
| `pennylane_list_expenses` | List expenses | GET | /api/v1/expenses |
| `pennylane_get_expense` | Get expense details | GET | /api/v1/expenses/{expenseId} |
| `pennylane_list_items` | List items/products | GET | /api/v1/items |
| `pennylane_get_company_info` | Get company information | GET | /api/v1/company |

---

## Tool Details

### pennylane_list_accounts

**What it does**: Lists all accounts in the accounting system.

**When to use**: Browse chart of accounts, find specific account types.

**Arguments**:
- `type` (optional): Filter by type (asset, liability, equity, revenue, expense)

**Example LLM prompt**: "List all expense accounts"

---

### pennylane_get_account

**What it does**: Gets detailed information about a specific account.

**When to use**: Get account balance, account details.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account 12345"

---

### pennylane_list_contacts

**What it does**: Lists all contacts (customers and suppliers).

**When to use**: Browse contact directory, find customers/suppliers.

**Arguments**:
- `type` (optional): Filter by type (customer, supplier)

**Example LLM prompt**: "List all customer contacts"

---

### pennylane_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, payment info.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact 67890"

---

### pennylane_list_invoices

**What it does**: Lists all invoices.

**When to use**: Browse invoice history, track payments.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, paid, cancelled)

**Example LLM prompt**: "Show all unpaid invoices"

---

### pennylane_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: Get invoice details, line items.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### pennylane_list_expenses

**What it does**: Lists all expenses.

**When to use**: Browse expenses, track spending.

**Arguments**:
- `status` (optional): Filter by status (draft, validated)
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show expenses from this month"

---

### pennylane_get_expense

**What it does**: Gets detailed information about a specific expense.

**When to use**: Get expense details, receipts.

**Arguments**:
- `expenseId` (required): Expense ID

**Example LLM prompt**: "Get details for expense 11111"

---

### pennylane_list_items

**What it does**: Lists all items/products in the catalog.

**When to use**: Browse product catalog.

**Arguments**: None

**Example LLM prompt**: "What items are in our catalog?"

---

### pennylane_get_company_info

**What it does**: Gets company/organization information.

**When to use**: Get company details, settings.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Pennylane API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Rate limits**: Apply to API calls — implement backoff for heavy use
- **Date formats**: Use YYYY-MM-DD format

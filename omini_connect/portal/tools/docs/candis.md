# Candis Tools

Provider: `candis` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Candis API. Candis is an accounting platform that helps businesses manage finances, invoices, and contacts. **Requires Candis OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Candis
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.candis.io`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `candis_list_accounts` | List accounts | GET | /api/v1/accounts |
| `candis_get_account` | Get account details | GET | /api/v1/accounts/{id} |
| `candis_list_transactions` | List transactions | GET | /api/v1/transactions |
| `candis_get_transaction` | Get transaction details | GET | /api/v1/transactions/{id} |
| `candis_list_invoices` | List invoices | GET | /api/v1/invoices |
| `candis_get_invoice` | Get invoice details | GET | /api/v1/invoices/{id} |
| `candis_create_invoice` | Create an invoice | POST | /api/v1/invoices |
| `candis_list_contacts` | List contacts | GET | /api/v1/contacts |
| `candis_get_contact` | Get contact details | GET | /api/v1/contacts/{id} |
| `candis_list_bills` | List bills | GET | /api/v1/bills |

---

## Tool Details

### candis_list_accounts

**What it does**: Lists all accounts in the workspace.

**When to use**: View financial accounts for reconciliation.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my Candis accounts"

---

### candis_get_account

**What it does**: Gets details of a specific account.

**When to use**: View account balance and transactions.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get account details for 123"

---

### candis_list_transactions

**What it does**: Lists all transactions with optional filters.

**When to use**: View transaction history, bank reconciliation.

**Arguments**:
- `account_id` (optional): Filter by account
- `from_date` (optional): Start date
- `to_date` (optional): End date
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List transactions for account 123 from last month"

---

### candis_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: View transaction details and categorization.

**Arguments**:
- `id` (required): Transaction ID

**Example LLM prompt**: "Get transaction 456 details"

---

### candis_list_invoices

**What it does**: Lists all invoices.

**When to use**: Track invoice status, find outstanding invoices.

**Arguments**:
- `status` (optional): Filter by invoice status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all pending invoices"

---

### candis_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice items, payment status.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice 789 details"

---

### candis_create_invoice

**What it does**: Creates a new invoice.

**When to use**: Generate an invoice for a client.

**Arguments**:
- `contact_id` (required): Contact ID
- `items` (required): Array of line items
- `due_date` (optional): Payment due date

**Example LLM prompt**: "Create an invoice for contact 123 with two items"

---

### candis_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find customers or vendors.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all my Candis contacts"

---

### candis_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information and transaction history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 456 details"

---

### candis_list_bills

**What it does**: Lists all bills.

**When to use**: Track vendor payments and outstanding bills.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all unpaid bills"

---

## Candis API Notes

- **Accounts**: Financial accounts for tracking money movements
- **Transactions**: Individual money movements linked to accounts
- **Invoices**: Outgoing invoices to clients
- **Bills**: Incoming bills from vendors
- **Contacts**: Customers, vendors, and other business contacts

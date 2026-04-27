# CheckHQ Tools

Provider: `checkhq` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the CheckHQ API. CheckHQ is an accounting and payment platform for businesses. **Requires CheckHQ API key.**

## Authentication

**Nango API_KEY**:
- User provides their CheckHQ API key
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.checkhq.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `checkhq_list_accounts` | List accounts | GET | /api/v1/accounts |
| `checkhq_get_account` | Get account details | GET | /api/v1/accounts/{id} |
| `checkhq_list_transactions` | List transactions | GET | /api/v1/transactions |
| `checkhq_get_transaction` | Get transaction details | GET | /api/v1/transactions/{id} |
| `checkhq_list_invoices` | List invoices | GET | /api/v1/invoices |
| `checkhq_get_invoice` | Get invoice details | GET | /api/v1/invoices/{id} |
| `checkhq_create_invoice` | Create an invoice | POST | /api/v1/invoices |
| `checkhq_list_payments` | List payments | GET | /api/v1/payments |
| `checkhq_get_payment` | Get payment details | GET | /api/v1/payments/{id} |
| `checkhq_list_contacts` | List contacts | GET | /api/v1/contacts |

---

## Tool Details

### checkhq_list_accounts

**What it does**: Lists all accounts in the organization.

**When to use**: View chart of accounts for bookkeeping.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my CheckHQ accounts"

---

### checkhq_get_account

**What it does**: Gets details of a specific account.

**When to use**: View account balance and transactions.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get account 123 details"

---

### checkhq_list_transactions

**What it does**: Lists all transactions.

**When to use**: View transaction history.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List transactions for account 123"

---

### checkhq_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: View transaction details.

**Arguments**:
- `id` (required): Transaction ID

**Example LLM prompt**: "Get transaction 456 details"

---

### checkhq_list_invoices

**What it does**: Lists all invoices.

**When to use**: Track invoice status.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all pending invoices"

---

### checkhq_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice details and line items.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice 789 details"

---

### checkhq_create_invoice

**What it does**: Creates a new invoice.

**When to use**: Generate an invoice for a client.

**Arguments**:
- `contact_id` (required): Contact ID
- `items` (required): Array of line items
- `due_date` (optional): Payment due date

**Example LLM prompt**: "Create an invoice for contact 123"

---

### checkhq_list_payments

**What it does**: Lists all payments.

**When to use**: View received payments.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all payments"

---

### checkhq_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: View payment details.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Get payment 101 details"

---

### checkhq_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find customers and vendors.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all CheckHQ contacts"

---

## CheckHQ API Notes

- **Accounts**: Chart of accounts for double-entry bookkeeping
- **Transactions**: Financial transactions linked to accounts
- **Invoices**: Outgoing invoices to customers
- **Payments**: Money received from customers
- **Contacts**: Customers and vendors

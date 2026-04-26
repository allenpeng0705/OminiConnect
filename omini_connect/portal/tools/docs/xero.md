# Xero Tools

Provider: `xero` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Xero API. They allow AI agents to manage invoices, contacts, accounts, and bank transactions. Xero is a popular cloud accounting platform for small and medium businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Xero
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `accounting`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `xero_list_invoices` | List all invoices | GET | /api.xro/2.0/Invoices |
| `xero_get_invoice` | Get a specific invoice | GET | /api.xro/2.0/Invoices/{invoice_id} |
| `xero_create_invoice` | Create a new invoice | POST | /api.xro/2.0/Invoices |
| `xero_update_invoice` | Update an existing invoice | POST | /api.xro/2.0/Invoices/{invoice_id} |
| `xero_list_contacts` | List all contacts | GET | /api.xro/2.0/Contacts |
| `xero_get_contact` | Get a specific contact | GET | /api.xro/2.0/Contacts/{contact_id} |
| `xero_list_accounts` | List all accounts | GET | /api.xro/2.0/Accounts |
| `xero_get_account` | Get a specific account | GET | /api.xro/2.0/Accounts/{account_id} |
| `xero_list_bank_transactions` | List all bank transactions | GET | /api.xro/2.0/BankTransactions |
| `xero_get_bank_transaction` | Get a specific bank transaction | GET | /api.xro/2.0/BankTransactions/{bank_transaction_id} |

---

## Tool Details

### xero_list_invoices

**What it does**: Lists all invoices in the Xero organization with optional filtering.

**When to use**: Find outstanding invoices, see all paid invoices, filter by contact or status.

**Arguments**:
- `if_modified_since` (optional): Filter by date modified after (YYYY-MM-DD)
- `status` (optional): Status filter (`DRAFT`, `SUBMITTED`, `AUTHORISED`, `PAID`, `VOIDED`)
- `contact_id` (optional): Filter by contact ID
- `page` (optional): Page number for pagination (default: 1)
- `include_archived` (optional): Include archived invoices (default: false)

**Example LLM prompt**: "List all approved invoices from this month"

---

### xero_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items, totals, taxes, and status.

**When to use**: Check invoice details, verify line items, see payment history.

**Arguments**:
- `invoice_id` (required): Invoice ID (GUID)

**Example LLM prompt**: "Get details for invoice abc-123"

---

### xero_create_invoice

**What it does**: Creates a new invoice in Xero with contact, line items, and optional dates.

**When to use**: Bill customers for products or services, create sales invoices.

**Arguments**:
- `Type` (required): Invoice type (`ACCREC` for accounts receivable)
- `Contact` (required): Contact object `{ContactID: 'guid'}`
- `LineItems` (required): Array of line item objects
- `Date` (optional): Invoice date (YYYY-MM-DD)
- `DueDate` (optional): Due date (YYYY-MM-DD)
- `InvoiceNumber` (optional): Custom invoice number
- `Reference` (optional): Invoice reference

**Example LLM prompt**: "Create an invoice for contact xyz-456 for $1,200"

---

### xero_update_invoice

**What it does**: Updates an existing invoice in Xero with new line items, dates, or status.

**When to use**: Modify invoice details, update line items, change dates or status.

**Arguments**:
- `invoice_id` (required): Invoice ID (GUID)
- `LineItems` (optional): Updated line items array
- `Date` (optional): Updated invoice date (YYYY-MM-DD)
- `DueDate` (optional): Updated due date (YYYY-MM-DD)
- `Status` (optional): Status (`DRAFT`, `SUBMITTED`, `AUTHORISED`, `VOIDED`)

**Example LLM prompt**: "Update invoice abc-123 with new line items"

---

### xero_list_contacts

**What it does**: Lists all contacts in the Xero organization.

**When to use**: Find contacts, view all customers or suppliers, search by name.

**Arguments**:
- `if_modified_since` (optional): Filter by date modified after (YYYY-MM-DD)
- `status` (optional): Status filter (`ACTIVE`, `ARCHIVED`)
- `page` (optional): Page number for pagination (default: 1)
- `include_archived` (optional): Include archived contacts (default: false)

**Example LLM prompt**: "List all contacts with 'Acme' in the name"

---

### xero_get_contact

**What it does**: Gets detailed information about a specific contact including name, email, phone, and address.

**When to use**: View contact details before creating an invoice or sending a message.

**Arguments**:
- `contact_id` (required): Contact ID (GUID)

**Example LLM prompt**: "Get details for contact def-789"

---

### xero_list_accounts

**What it does**: Lists all accounts in the Xero organization.

**When to use**: View chart of accounts, find account codes for transactions.

**Arguments**:
- `if_modified_since` (optional): Filter by date modified after (YYYY-MM-DD)

**Example LLM prompt**: "List all bank accounts"

---

### xero_get_account

**What it does**: Gets detailed information about a specific account including type, balance, and metadata.

**When to use**: Check account details, verify account code for transactions.

**Arguments**:
- `account_id` (required): Account ID (GUID)

**Example LLM prompt**: "Get details for account jkl-202"

---

### xero_list_bank_transactions

**What it does**: Lists all bank transactions in the Xero organization.

**When to use**: Track bank deposits and withdrawals, reconcile accounts, find transactions by type.

**Arguments**:
- `if_modified_since` (optional): Filter by date modified after (YYYY-MM-DD)
- `status` (optional): Status filter (`DRAFT`, `AUTHORISED`, `PAID`)
- `type` (optional): Transaction type (`RECEIVE`, `SPEND`, `RECEIVE-PREPAYMENT`, etc.)
- `page` (optional): Page number for pagination (default: 1)

**Example LLM prompt**: "List all bank transactions from this month"

---

### xero_get_bank_transaction

**What it does**: Gets detailed information about a specific bank transaction including amount, date, and line items.

**When to use**: Review transaction details, verify amounts, check associated account.

**Arguments**:
- `bank_transaction_id` (required): Bank Transaction ID (GUID)

**Example LLM prompt**: "Get details for bank transaction txn-303"

---

## Xero API Notes

- **Invoice types**: `ACCREC` (accounts receivable - money owed to you) and `ACCPAY` (accounts payable - money you owe).
- **Status flow**: Invoices progress through statuses: `DRAFT` -> `SUBMITTED` -> `AUTHORISED` -> `PAID` or `VOIDED`.
- **Line items**: Invoices require line items with description, quantity, and unit amount.
- **Account codes**: Each line item typically references an account code for proper categorization.
- **Pagination**: Default page size is 50. Use `page` parameter for pagination.
- **Date format**: Use YYYY-MM-DD format for all date parameters.
- **Contact types**: Contacts can be customers, suppliers, or both.

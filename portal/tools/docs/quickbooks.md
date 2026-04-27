# QuickBooks Tools

Provider: `quickbooks` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the QuickBooks Online API. They allow AI agents to manage customers, invoices, payments, and accounts. QuickBooks Online is a popular accounting and bookkeeping platform for small businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with QuickBooks
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `com.intuit.quickbooks.accounting`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `quickbooks_list_customers` | List all customers | GET | /v3/company/{company_id}/query |
| `quickbooks_get_customer` | Get a specific customer | GET | /v3/company/{company_id}/customer/{customer_id} |
| `quickbooks_create_customer` | Create a new customer | POST | /v3/company/{company_id}/customer |
| `quickbooks_list_invoices` | List all invoices | GET | /v3/company/{company_id}/query |
| `quickbooks_get_invoice` | Get a specific invoice | GET | /v3/company/{company_id}/invoice/{invoice_id} |
| `quickbooks_create_invoice` | Create a new invoice | POST | /v3/company/{company_id}/invoice |
| `quickbooks_list_payments` | List all payments | GET | /v3/company/{company_id}/query |
| `quickbooks_get_payment` | Get a specific payment | GET | /v3/company/{company_id}/payment/{payment_id} |
| `quickbooks_list_accounts` | List all accounts | GET | /v3/company/{company_id}/query |
| `quickbooks_get_account` | Get a specific account | GET | /v3/company/{company_id}/account/{account_id} |

---

## Tool Details

### quickbooks_list_customers

**What it does**: Lists all customers in the QuickBooks Online company using a SQL-like query.

**When to use**: Find customers, browse customer base, search for specific customers by name or email.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `query` (optional): SQL-like query string (default: `SELECT * FROM Customer MAXRESULTS 100`)
- `minorversion` (optional): API minor version

**Example LLM prompt**: "List all customers in QuickBooks"

---

### quickbooks_get_customer

**What it does**: Gets detailed information about a specific customer including contact details, addresses, and metadata.

**When to use**: Get customer details before creating an invoice or viewing their account balance.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `customer_id` (required): Customer ID
- `minorversion` (optional): API minor version

**Example LLM prompt**: "Get details for customer abc-123"

---

### quickbooks_create_customer

**What it does**: Creates a new customer in QuickBooks Online with name and optional contact information.

**When to use**: Add new customers to the accounting system before billing them.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `DisplayName` (required): Customer display name
- `PrimaryEmailAddr` (optional): Email address object `{Address: 'email@example.com'}`
- `PrimaryPhone` (optional): Phone object `{FreeFormNumber: '555-1234'}`
- `GivenName` (optional): First name
- `FamilyName` (optional): Last name
- `BillAddr` (optional): Billing address object
- `ShipAddr` (optional): Shipping address object

**Example LLM prompt**: "Create a new customer for Acme Corp with email billing@acme.com"

---

### quickbooks_list_invoices

**What it does**: Lists all invoices in the QuickBooks Online company using a SQL-like query.

**When to use**: Browse invoices, find invoices by customer or date, track outstanding bills.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `query` (optional): SQL-like query string (default: `SELECT * FROM Invoice MAXRESULTS 100`)
- `minorversion` (optional): API minor version

**Example LLM prompt**: "List all invoices from the last 30 days"

---

### quickbooks_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items, totals, taxes, and payment status.

**When to use**: Check invoice details before sending, verify line items, see payment history.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `invoice_id` (required): Invoice ID
- `minorversion` (optional): API minor version

**Example LLM prompt**: "Get details for invoice inv-456"

---

### quickbooks_create_invoice

**What it does**: Creates a new invoice in QuickBooks Online with customer and line items.

**When to use**: Generate invoices for customers, create billing records.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `CustomerRef` (required): Customer reference object `{value: 'customer_id', name: 'Customer Name'}`
- `Line` (required): Array of line items objects
- `DocNumber` (optional): Invoice number
- `TxnDate` (optional): Transaction date (YYYY-MM-DD)
- `DueDate` (optional): Due date (YYYY-MM-DD)

**Example LLM prompt**: "Create an invoice for Acme Corp with two line items"

---

### quickbooks_list_payments

**What it does**: Lists all payments received in QuickBooks Online.

**When to use**: Track received payments, reconcile bank transactions, find payments by customer.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `query` (optional): SQL-like query string (default: `SELECT * FROM Payment MAXRESULTS 100`)
- `minorversion` (optional): API minor version

**Example LLM prompt**: "List all payments received this month"

---

### quickbooks_get_payment

**What it does**: Gets detailed information about a specific payment including amount, date, and associated invoice.

**When to use**: Verify payment details, see which invoice was paid, check payment method.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `payment_id` (required): Payment ID
- `minorversion` (optional): API minor version

**Example LLM prompt**: "Get details for payment pay-789"

---

### quickbooks_list_accounts

**What it does**: Lists all accounts in the QuickBooks Online company using a SQL-like query.

**When to use**: View chart of accounts, find account codes for transactions.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `query` (optional): SQL-like query string (default: `SELECT * FROM Account MAXRESULTS 100`)
- `minorversion` (optional): API minor version

**Example LLM prompt**: "List all bank accounts"

---

### quickbooks_get_account

**What it does**: Gets detailed information about a specific account including type, balance, and metadata.

**When to use**: Check account details, verify account balance, get account code for categorization.

**Arguments**:
- `company_id` (required): QuickBooks Company ID
- `account_id` (required): Account ID
- `minorversion` (optional): API minor version

**Example LLM prompt**: "Get details for account acc-101"

---

## QuickBooks API Notes

- **Company Context**: Most calls require a company ID. Use `company_id` as a path/query parameter.
- **Query Language**: QuickBooks uses a SQL-like query syntax for the `/query` endpoint.
- **Pagination**: Use `MAXRESULTS` in queries for pagination control.
- **Date Format**: Use `YYYY-MM-DD` for all date parameters.
- **Line Items**: Invoices require `Line` array with description, quantity, and unit price.
- **API Version**: QuickBooks uses v3 of their REST API through Nango.

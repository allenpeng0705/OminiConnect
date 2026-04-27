# Bill (Connect API) Tools

Provider: `bill` | Engine: `nango` | Auth: BILL via Nango

## Overview

These tools wrap the Bill Connect API. They allow AI agents to manage vendors, invoices, and payments for accounts payable automation. Bill (formerly Bill.com) streamlines business payments and spend management.

**Production Environment**: This provider uses the production environment at `https://gateway.prod.bill.com/connect`. For testing, use the `bill-sandbox` provider.

## Authentication

**Nango BILL Auth**:
- User authenticates via Bill Connect OAuth
- Token stored in Nango, accessed via `connection_ref`
- Session token obtained via login endpoint

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bill_list_vendors` | List vendors | GET | /v3/vendors |
| `bill_get_vendor` | Get vendor details | GET | /v3/vendors/{vendorId} |
| `bill_list_invoices` | List invoices | GET | /v3/invoices |
| `bill_get_invoice` | Get invoice details | GET | /v3/invoices/{invoiceId} |
| `bill_create_invoice` | Create an invoice | POST | /v3/invoices |
| `bill_list_payments` | List payments | GET | /v3/payments |
| `bill_get_payment` | Get payment details | GET | /v3/payments/{paymentId} |
| `bill_list_accounts` | List accounts | GET | /v3/accounts |
| `bill_get_account` | Get account details | GET | /v3/accounts/{accountId} |
| `bill_list_organizations` | List organizations | GET | /v3/organizations |

---

## Tool Details

### bill_list_vendors

**What it does**: Lists all vendors in the Bill account.

**When to use**: Browse vendors, find suppliers for invoice creation.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Vendors per page (default 20)

**Example LLM prompt**: "List all vendors in Bill.com"

---

### bill_get_vendor

**What it does**: Gets details for a specific vendor.

**When to use**: Check vendor details, payment terms.

**Arguments**:
- `vendorId` (required): Vendor ID

**Example LLM prompt**: "Get details for vendor V-12345"

---

### bill_list_invoices

**What it does**: Lists all invoices (bills to pay or received).

**When to use**: Track payables, manage payment schedule.

**Arguments**:
- `status` (optional): Filter by status (APPROVED, PAID, etc.)
- `vendorId` (optional): Filter by vendor
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Invoices per page (default 20)

**Example LLM prompt**: "List all unpaid invoices"

---

### bill_get_invoice

**What it does**: Gets details for a specific invoice.

**When to use**: View invoice details, line items, payment status.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### bill_create_invoice

**What it does**: Creates a new invoice (vendor bill).

**When to use**: Record bills from vendors, initiate payment workflow.

**Arguments**:
- `vendorId` (required): Vendor ID
- `invoiceDate` (required): Invoice date (YYYY-MM-DD)
- `dueDate` (required): Due date (YYYY-MM-DD)
- `amount` (required): Invoice amount
- `description` (optional): Invoice description

**Example LLM prompt**: "Create an invoice for vendor V-12345 for $500 due next month"

---

### bill_list_payments

**What it does**: Lists all payments made.

**When to use**: Track payment history, reconcile accounts.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Payments per page (default 20)

**Example LLM prompt**: "List all payments from this month"

---

### bill_get_payment

**What it does**: Gets details for a specific payment.

**When to use**: Check payment status, view payment details.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "Get details for payment P-12345"

---

### bill_list_accounts

**What it does**: Lists all chart of accounts.

**When to use**: View account structure, find GL accounts.

**Arguments**:
- `type` (optional): Filter by account type
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all expense accounts"

---

### bill_get_account

**What it does**: Gets details for a specific account.

**When to use**: Check account details, balance.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account A-123"

---

### bill_list_organizations

**What it does**: Lists all organizations in the account.

**When to use**: Switch between entities, view organizational structure.

**Arguments**: None required

**Example LLM prompt**: "List all organizations"

---

## Bill API Notes

- **Vendor ID**: Format typically V-XXXXX
- **Invoice ID**: Format typically INV-XXXXX
- **Payment Status**: Includes PENDING, PROCESSING, SENT, PAID
- **Organizations**: Can have multiple entities (departments, subsidiaries)
- **Approval Workflow**: Invoices may require approval before payment

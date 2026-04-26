# Bill (Connect API Sandbox) Tools

Provider: `bill-sandbox` | Engine: `nango` | Auth: BILL via Nango

## Overview

These tools wrap the Bill Connect API (Sandbox environment). They allow AI agents to manage vendors, invoices, and payments for accounts payable automation. The sandbox environment is for testing - use `bill` for production.

**Sandbox Environment**: This provider uses the sandbox environment at `https://gateway.stage.bill.com/connect`. For production, use the `bill` provider.

## Authentication

**Nango BILL Auth**:
- User authenticates via Bill Connect OAuth in sandbox
- Token stored in Nango, accessed via `connection_ref`
- Session token obtained via login endpoint

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bill_sandbox_list_vendors` | List vendors | GET | /v3/vendors |
| `bill_sandbox_get_vendor` | Get vendor details | GET | /v3/vendors/{vendorId} |
| `bill_sandbox_list_invoices` | List invoices | GET | /v3/invoices |
| `bill_sandbox_get_invoice` | Get invoice details | GET | /v3/invoices/{invoiceId} |
| `bill_sandbox_create_invoice` | Create an invoice | POST | /v3/invoices |
| `bill_sandbox_list_payments` | List payments | GET | /v3/payments |
| `bill_sandbox_get_payment` | Get payment details | GET | /v3/payments/{paymentId} |
| `bill_sandbox_list_accounts` | List accounts | GET | /v3/accounts |
| `bill_sandbox_get_account` | Get account details | GET | /v3/accounts/{accountId} |
| `bill_sandbox_list_organizations` | List organizations | GET | /v3/organizations |

---

## Tool Details

### bill_sandbox_list_vendors

**What it does**: Lists all vendors in the Bill sandbox account.

**When to use**: Browse vendors in test environment.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Vendors per page (default 20)

**Example LLM prompt**: "List all vendors in sandbox"

---

### bill_sandbox_get_vendor

**What it does**: Gets details for a specific vendor.

**When to use**: Check vendor details for testing.

**Arguments**:
- `vendorId` (required): Vendor ID

**Example LLM prompt**: "Get details for vendor V-12345"

---

### bill_sandbox_list_invoices

**What it does**: Lists all invoices in sandbox.

**When to use**: Test invoice workflows, verify data.

**Arguments**:
- `status` (optional): Filter by status (APPROVED, PAID, etc.)
- `vendorId` (optional): Filter by vendor
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Invoices per page (default 20)

**Example LLM prompt**: "List all invoices in sandbox"

---

### bill_sandbox_get_invoice

**What it does**: Gets details for a specific invoice.

**When to use**: Test invoice retrieval.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### bill_sandbox_create_invoice

**What it does**: Creates a new invoice in sandbox.

**When to use**: Test invoice creation workflow.

**Arguments**:
- `vendorId` (required): Vendor ID
- `invoiceDate` (required): Invoice date (YYYY-MM-DD)
- `dueDate` (required): Due date (YYYY-MM-DD)
- `amount` (required): Invoice amount
- `description` (optional): Invoice description

**Example LLM prompt**: "Create a test invoice for $100"

---

### bill_sandbox_list_payments

**What it does**: Lists all payments in sandbox.

**When to use**: Test payment workflows.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Payments per page (default 20)

**Example LLM prompt**: "List all payments in sandbox"

---

### bill_sandbox_get_payment

**What it does**: Gets details for a specific payment.

**When to use**: Test payment retrieval.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "Get details for payment P-12345"

---

### bill_sandbox_list_accounts

**What it does**: Lists all chart of accounts in sandbox.

**When to use**: Test account queries.

**Arguments**:
- `type` (optional): Filter by account type
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all accounts in sandbox"

---

### bill_sandbox_get_account

**What it does**: Gets details for a specific account.

**When to use**: Test account retrieval.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account A-123"

---

### bill_sandbox_list_organizations

**What it does**: Lists all organizations in sandbox.

**When to use**: Test multi-entity scenarios.

**Arguments**: None required

**Example LLM prompt**: "List all organizations in sandbox"

---

## Bill Sandbox API Notes

- **Sandbox**: Separate test environment from production
- **Test Data**: May include sample vendors, invoices
- **Rate Limits**: Generally more lenient in sandbox
- **No Real Payments**: Payments do not actually process
- **Reset**: Sandbox can be reset to clean state

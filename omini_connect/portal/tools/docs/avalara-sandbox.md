# Avalara (Sandbox) Tools

Provider: `avalara-sandbox` | Engine: `nango` | Auth: BASIC via Nango (Account ID + License Key)

## Overview

These tools wrap the Avalara REST API v2 (Sandbox environment). They allow AI agents to calculate taxes, manage transactions, and handle compliance operations. Avalara is a tax compliance platform that automates indirect tax calculations, filing, and remittance across jurisdictions.

**Sandbox Environment**: This provider uses Avalara's sandbox environment at `https://sandbox-rest.avatax.com/api/v2`. For production, use the `avalara` provider.

## Authentication

**Nango BASIC Auth**:
- User provides Account ID (username) and License Key (password)
- Token stored in Nango, accessed via `connection_ref`
- Avalara client header required (auto-configured)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `avalara_sandbox_ping` | Verify API connectivity | GET | /utilities/subscriptions |
| `avalara_sandbox_list_subscriptions` | List available subscriptions | GET | /utilities/subscriptions |
| `avalara_sandbox_get_tax_rate` | Get tax rate for an address | GET | /taxes/get |
| `avalara_sandbox_calculate_tax` | Calculate tax for a transaction | POST | /taxes/calculate |
| `avalara_sandbox_create_transaction` | Create a transaction | POST | /transactions/create |
| `avalara_sandbox_void_transaction` | Void a transaction | POST | /transactions/{transactionCode}/void |
| `avalara_sandbox_refund_transaction` | Refund a transaction | POST | /transactions/refund |
| `avalara_sandbox_list_companies` | List companies | GET | /companies |
| `avalara_sandbox_get_company` | Get company details | GET | /companies/{companyId} |
| `avalara_sandbox_list_items` | List items | GET | /items |

---

## Tool Details

### avalara_sandbox_ping

**What it does**: Verifies API connectivity and authentication.

**When to use**: Test that the Avalara sandbox connection is working.

**Arguments**: None required

**Example LLM prompt**: "Test my Avalara sandbox connection"

---

### avalara_sandbox_list_subscriptions

**What it does**: Lists all subscriptions available for the account.

**When to use**: Check available Avalara services and features.

**Arguments**: None required

**Example LLM prompt**: "What subscriptions do I have in Avalara"

---

### avalara_sandbox_get_tax_rate

**What it does**: Gets the tax rate for a specific address including jurisdiction breakdown.

**When to use**: Check applicable tax rates before calculating or creating transactions.

**Arguments**:
- `line1` (optional): Street address line 1
- `line2` (optional): Street address line 2
- `city` (optional): City name
- `state` (optional): State or province code
- `postalCode` (required): Postal/ZIP code
- `country` (optional): Country code (default: USA)

**Example LLM prompt**: "What is the tax rate for address 123 Main St, Austin, TX 78701"

---

### avalara_sandbox_calculate_tax

**What it does**: Calculates tax for a transaction without creating it. Used for estimates.

**When to use**: Get tax estimates before finalizing orders or quotes.

**Arguments**:
- `addresses` (required): Array of address objects (shipFrom, shipTo, etc.)
- `lines` (required): Array of line items with quantity and amount
- `type` (optional): Transaction type (default: SalesOrder)

**Example LLM prompt**: "Calculate tax for 2 items at $100 each shipped to Los Angeles, CA"

---

### avalara_sandbox_create_transaction

**What it does**: Creates or posts a transaction (order, invoice) in Avalara.

**When to use**: Record sales, issue invoices, or commit transactions for reporting.

**Arguments**:
- `type` (required): Transaction type (SalesOrder, SalesInvoice, etc.)
- `companyCode` (required): Company code
- `date` (optional): Transaction date
- `customerCode` (optional): Customer identifier
- `addresses` (optional): Address assignments
- `lines` (required): Line items

**Example LLM prompt**: "Create a sales invoice for company ACME with 3 items totaling $500"

---

### avalara_sandbox_void_transaction

**What it does**: Voids an existing transaction.

**When to use**: Cancel or void a transaction that was created in error.

**Arguments**:
- `transactionCode` (required): Transaction code to void
- `voidReasonCode` (optional): Reason code (default: PostFailed)

**Example LLM prompt**: "Void transaction TX-12345"

---

### avalara_sandbox_refund_transaction

**What it does**: Refunds a previously posted transaction.

**When to use**: Process refunds for returned items or cancelled orders.

**Arguments**:
- `transactionCode` (required): Original transaction code
- `refundDate` (optional): Refund date
- `referenceCode` (optional): Reference code

**Example LLM prompt**: "Refund transaction TX-12345"

---

### avalara_sandbox_list_companies

**What it does**: Lists all companies in the account.

**When to use**: Find companies, select target for transactions.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all my Avalara companies"

---

### avalara_sandbox_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: View company settings,nexus, and tax settings.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get details for company ID 12345"

---

### avalara_sandbox_list_items

**What it does**: Lists all items (products and services) in a company.

**When to use**: Browse product catalog, check item tax codes.

**Arguments**:
- `companyId` (optional): Company ID
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all items in my Avalara account"

---

## Avalara API Notes

- **Tax Codes**: Items must have tax codes assigned for accurate calculations
- **Company Code**: Required for most operations - identifies which company to use
- **Transaction Lifecycle**: Orders can be created as drafts and later committed to invoices
- **Jurisdiction Resolution**: Avalara automatically resolves addresses to jurisdictions
- **Sandbox vs Production**: Sandbox uses separate data and has rate limits; test thoroughly before production

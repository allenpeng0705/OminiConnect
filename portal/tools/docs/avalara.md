# Avalara Tools

Provider: `avalara` | Engine: `nango` | Auth: Basic Auth (via Nango)

## Overview

These tools wrap the Avalara AvaTax API v2. They allow AI agents to manage transactions, customers, items, address validation, and tax calculations for the authenticated account.

## Authentication

**Nango (Basic Auth)**:
- User provides Avalara username/password or Account ID/License Key
- Token stored in Nango, accessed via `connection_ref`
- Headers: `x-avalara-client` set automatically

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `avalara_list_transactions` | List transactions in the account | GET | /transactions |
| `avalara_get_transaction` | Get details of a specific transaction | GET | /transactions/{id} |
| `avalara_create_transaction` | Create a new transaction | POST | /transactions/create |
| `avalara_list_customers` | List customers | GET | /customers |
| `avalara_get_customer` | Get details of a specific customer | GET | /customers/{customer_code} |
| `avalara_list_items` | List items in the item catalog | GET | /items |
| `avalara_get_item` | Get details of a specific item | GET | /items/{item_code} |
| `avalara_validate_address` | Validate and normalize an address | POST | /addresses/resolve |
| `avalara_get_tax_rates` | Get tax rates for a location | GET | /taxrates/byaddress |
| `avalara_list_exemptions` | List tax exemptions | GET | /exemptions |

---

## Tool Details

### avalara_list_transactions

**What it does**: List all transactions (invoices, refunds, orders) in the Avalara account.

**When to use**: Review recent transactions, find specific invoices, or audit tax calculations.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `date` (optional): Filter by date (YYYY-MM-DD)
- `status` (optional): Filter by status: posted, saved, voided
- `top` (optional, max 200): default 100
- `skip` (optional): default 0

**Example LLM prompt**: "List all posted transactions from this month"

---

### avalara_get_transaction

**What it does**: Get details of a specific transaction including line items, tax amounts, and status.

**When to use**: Review invoice details, verify tax calculations, or check transaction status.

**Arguments**:
- `id` (required): Transaction ID
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for transaction INV-12345"

---

### avalara_create_transaction

**What it does**: Create a new transaction (invoice, order, refund) and calculate taxes.

**When to use**: Record a sale, create an invoice, or process a refund with automatic tax calculation.

**Arguments**:
- `type` (required): Transaction type: SalesOrder, SalesInvoice, ReturnOrder, ReturnInvoice
- `company_id` (required): Company ID
- `date` (optional): Transaction date (YYYY-MM-DD)
- `currency_code` (optional): Currency code, default USD
- `lines` (optional): Array of line items
- `customer_code` (optional): Customer identifier
- `addresses` (optional): Address information

**Example LLM prompt**: "Create a sales invoice for customer ACME Corp with items item1 and item2"

---

### avalara_list_customers

**What it does**: List all customers in the Avalara account.

**When to use**: Find customers, check exemption status, or manage customer records.

**Arguments**:
- `company_id` (required): Company ID
- `top` (optional, max 200): default 100
- `skip` (optional): default 0

**Example LLM prompt**: "List all customers in my Avalara account"

---

### avalara_get_customer

**What it does**: Get details of a specific customer including contact info and exemption status.

**When to use**: Check customer exemption certificate or update customer information.

**Arguments**:
- `customer_code` (required): Customer code
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for customer ACME001"

---

### avalara_list_items

**What it does**: List all items in the item catalog with SKU, description, and tax code.

**When to use**: Find products, check tax codes, or manage item catalog.

**Arguments**:
- `company_id` (required): Company ID
- `top` (optional, max 200): default 100
- `skip` (optional): default 0

**Example LLM prompt**: "List all items in my item catalog"

---

### avalara_get_item

**What it does**: Get details of a specific item including SKU, description, and tax code.

**When to use**: Check item details or verify tax code before creating transactions.

**Arguments**:
- `item_code` (required): Item code/SKU
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for item SKU-001"

---

### avalara_validate_address

**What it does**: Validate and normalize an address, returning geocoding coordinates.

**When to use**: Verify customer addresses before calculating tax or shipping.

**Arguments**:
- `line1` (required): Street address line 1
- `line2` (optional): Street address line 2
- `city` (required): City
- `region` (required): State/Province code
- `postal_code` (required): ZIP/Postal code
- `country` (optional): Country code, default USA

**Example LLM prompt**: "Validate address 123 Main St, San Francisco, CA 94105"

---

### avalara_get_tax_rates

**What it does**: Get tax rates for a specific address including breakdown by jurisdiction.

**When to use**: Estimate tax rates, verify jurisdiction assignments, or quote prices.

**Arguments**:
- `line1` (required): Street address
- `city` (required): City
- `region` (required): State/Province code
- `postal_code` (required): ZIP/Postal code
- `country` (optional): Country code

**Example LLM prompt**: "Get tax rates for 123 Main St, San Francisco, CA 94105"

---

### avalara_list_exemptions

**What it does**: List all tax exemptions including customer, reason, and validity dates.

**When to use**: Review exemption certificates, check customer eligibility, or audit exemptions.

**Arguments**:
- `company_id` (required): Company ID
- `customer_id` (optional): Filter by customer ID
- `top` (optional, max 200): default 100
- `skip` (optional): default 0

**Example LLM prompt**: "List all exemptions for company ACME"

---

## Avalara API Reference

These tools use the Avalara AvaTax API v2. See official docs for full details:
- https://developer.avalara.com/api-reference/avatax/rest/v2/
- Rate limits: Vary by plan
- Pagination: Use `top` and `skip` parameters
- All dates: ISO 8601 format (UTC)

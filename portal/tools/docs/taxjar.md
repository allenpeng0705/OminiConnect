# Taxjar Tools

Provider: `taxjar` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Taxjar sales tax API. They enable AI agents to manage transactions, nexus locations, address validation, tax rates, reports, and tax categories for sales tax compliance. Taxjar is a leading sales tax automation platform for e-commerce businesses.

## Authentication

**Nango API Key**:
- User authenticates via Nango with Taxjar API key
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `transactions:read`, `transactions:write`, `nexus:read`, `addresses:read`, `rates:read`, `reports:read`, `categories:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `taxjar_list_transactions` | List all transactions | GET | /transactions |
| `taxjar_get_transaction` | Get transaction details | GET | /transactions/{transactionId} |
| `taxjar_create_transaction` | Create a new transaction | POST | /transactions |
| `taxjar_list_nexus` | List all nexus locations | GET | /nexus |
| `taxjar_get_nexus` | Get nexus location details | GET | /nexus/{regionCode} |
| `taxjar_validate_address` | Validate and normalize address | POST | /addresses/validate |
| `taxjar_get_tax_rates` | Get tax rates for location | GET | /rates |
| `taxjar_list_reports` | List tax reports | GET | /reports |
| `taxjar_get_report` | Get report details | GET | /reports/{reportId} |
| `taxjar_list_categories` | List tax categories | GET | /categories |

---

## Tool Details

### taxjar_list_transactions

**What it does**: Retrieves all transactions with optional filtering by date range or entity type.

**When to use**: Get an overview of all transactions, find orders or refunds.

**Arguments**:
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `entity` (optional): Filter by entity type (order, refund)
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all order transactions from March 2026"

---

### taxjar_get_transaction

**What it does**: Gets detailed information about a specific transaction including line items.

**When to use**: Review transaction details, check tax amounts for an order.

**Arguments**:
- `transactionId` (required): Transaction ID
- `entity` (optional): Entity type (order or refund)

**Example LLM prompt**: "Get details for transaction ORDER-123"

---

### taxjar_create_transaction

**What it does**: Creates or syncs a new transaction (order or refund) for sales tax reporting.

**When to use**: Record sales, sync e-commerce orders, log refunds for reporting.

**Arguments**:
- `transaction_id` (required): Unique transaction ID
- `entity` (required): Entity type (order or refund)
- `amount` (required): Transaction amount
- `shipping` (optional): Shipping amount
- `tax_date` (optional): Tax date (YYYY-MM-DD)
- `from_state` (optional): Origin state
- `from_city` (optional): Origin city
- `from_zip` (optional): Origin zip code
- `to_state` (optional): Destination state
- `to_city` (optional): Destination city
- `to_zip` (optional): Destination zip code
- `line_items` (optional): Line items array

**Example LLM prompt**: "Create an order transaction with ID ORDER-456 for $150 shipping to Texas"

---

### taxjar_list_nexus

**What it does**: Retrieves all nexus (taxable presence) locations where you have tax obligations.

**When to use**: View all states where you have nexus, understand tax obligations.

**Arguments**: None

**Example LLM prompt**: "List all nexus locations"

---

### taxjar_get_nexus

**What it does**: Gets detailed information about a specific nexus location.

**When to use**: Check nexus details for a specific state.

**Arguments**:
- `regionCode` (required): Region code (state abbreviation)

**Example LLM prompt**: "Get nexus details for California"

---

### taxjar_validate_address

**What it does**: Validates and normalizes a street address using Taxjar's address validation.

**When to use**: Verify customer addresses at checkout, ensure accurate tax calculation.

**Arguments**:
- `street` (required): Street address
- `city` (required): City
- `state` (required): State
- `zip` (required): Zip code
- `country` (optional): Country code (default US)

**Example LLM prompt**: "Validate address 123 Main St, Austin, TX 78701"

---

### taxjar_get_tax_rates

**What it does**: Gets sales tax rates for a specific location including combined rate breakdown.

**When to use**: Look up tax rates for checkout, estimate tax for orders.

**Arguments**:
- `street` (optional): Street address
- `city` (optional): City
- `state` (required): State
- `zip` (required): Zip code
- `country` (optional): Country code (default US)

**Example LLM prompt**: "Get tax rates for 90210 in California"

---

### taxjar_list_reports

**What it does**: Retrieves available tax reports with optional filtering by date range, state, or type.

**When to use**: View available reports for sales tax filing.

**Arguments**:
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `state` (optional): Filter by state
- `type` (optional): Report type (sales, reconciliation)

**Example LLM prompt**: "List all sales reports for Q1 2026"

---

### taxjar_get_report

**What it does**: Gets detailed information about a specific tax report.

**When to use**: Review report details for tax filing or reconciliation.

**Arguments**:
- `reportId` (required): Report ID

**Example LLM prompt**: "Get details for report rpt_123456"

---

### taxjar_list_categories

**What it does**: Retrieves all tax categories with their descriptions and applicable tax codes.

**When to use**: Find appropriate tax codes for products, understand product taxability.

**Arguments**: None

**Example LLM prompt**: "List all tax categories"

---

## Taxjar API Notes

- **Nexus**: Taxjar tracks where you have sales tax nexus (taxable presence)
- **Product Tax Codes**: Classify products to determine taxability (clothing vs. software)
- **Jurisdictions**: Tax is collected at state, county, city, and special district levels
- **Transaction Sync**: Orders should be synced to Taxjar for accurate reporting
- **Refunds**: Must be created as separate refund transactions for sales tax liability adjustments
- **Reports**: Used for sales tax filing and remittance by jurisdiction
- **Address Validation**: Returns normalized address and lat/long for accurate tax calculation

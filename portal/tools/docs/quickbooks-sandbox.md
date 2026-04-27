# Quickbooks-Sandbox Tools

Provider: `quickbooks-sandbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

QuickBooks Sandbox is a testing environment for QuickBooks Online. These tools allow AI agents to manage customers, invoices, items, and payments in a safe sandbox environment for testing and development.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with QuickBooks Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scope: `com.intuit.quickbooks.accounting`

**Important**: This is the sandbox environment. All operations are test operations and do not affect production data.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `quickbooks-sandbox_list_customers` | List customers | GET | /v3/company/{companyId}/query |
| `quickbooks-sandbox_get_customer` | Get customer details | GET | /v3/company/{companyId}/customer/{customerId} |
| `quickbooks-sandbox_create_customer` | Create a customer | POST | /v3/company/{companyId}/customer |
| `quickbooks-sandbox_list_invoices` | List invoices | GET | /v3/company/{companyId}/query |
| `quickbooks-sandbox_get_invoice` | Get invoice details | GET | /v3/company/{companyId}/invoice/{invoiceId} |
| `quickbooks-sandbox_create_invoice` | Create an invoice | POST | /v3/company/{companyId}/invoice |
| `quickbooks-sandbox_list_items` | List items/products | GET | /v3/company/{companyId}/query |
| `quickbooks-sandbox_get_item` | Get item details | GET | /v3/company/{companyId}/item/{itemId} |
| `quickbooks-sandbox_list_payments` | List payments | GET | /v3/company/{companyId}/query |
| `quickbooks-sandbox_get_company_info` | Get company info | GET | /v3/company/{companyId}/companyinfo/{companyId} |

---

## Tool Details

### quickbooks-sandbox_list_customers

**What it does**: Returns a list of all customers using QuickBooks Query language.

**When to use**: Find customers, search your customer database.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `query` (optional): Query string (default: SELECT * FROM Customer)
- `limit` (optional): Number of results (default 100)

**Example LLM prompt**: "List all customers in QuickBooks sandbox"

---

### quickbooks-sandbox_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Get customer information including balance and contact details.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `customerId` (required): The customer ID

**Example LLM prompt**: "Get details for customer 123"

---

### quickbooks-sandbox_create_customer

**What it does**: Creates a new customer in sandbox.

**When to use**: Add new customers for testing.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `DisplayName` (required): Customer display name
- `GivenName` (optional): First name
- `FamilyName` (optional): Last name
- `PrimaryEmailAddr` (optional): Email as object

**Example LLM prompt**: "Create a test customer named 'Test Company'"

---

### quickbooks-sandbox_list_invoices

**What it does**: Returns a list of all invoices.

**When to use**: View invoice history.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `query` (optional): Query string
- `limit` (optional): Number of results

**Example LLM prompt**: "List all invoices in sandbox"

---

### quickbooks-sandbox_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: Check invoice status and details.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `invoiceId` (required): The invoice ID

**Example LLM prompt**: "Get invoice details for INV-001"

---

### quickbooks-sandbox_create_invoice

**What it does**: Creates a new invoice.

**When to use**: Generate test invoices.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `Line` (required): Array of line items
- `CustomerRef` (required): Customer reference object

**Example LLM prompt**: "Create an invoice for customer 123 with line item 'Service' for $100"

---

### quickbooks-sandbox_list_items

**What it does**: Returns a list of all products and services.

**When to use**: View available items for invoicing.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `query` (optional): Query string

**Example LLM prompt**: "List all items in QuickBooks sandbox"

---

### quickbooks-sandbox_get_item

**What it does**: Gets details of a specific item.

**When to use**: Check item price and description.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `itemId` (required): The item ID

**Example LLM prompt**: "Get details for item 456"

---

### quickbooks-sandbox_list_payments

**What it does**: Returns a list of all payments received.

**When to use**: View payment history.

**Arguments**:
- `companyId` (required): QuickBooks company ID
- `query` (optional): Query string

**Example LLM prompt**: "List all payments in sandbox"

---

### quickbooks-sandbox_get_company_info

**What it does**: Gets company information and settings.

**When to use**: Get company name, address, and settings.

**Arguments**:
- `companyId` (required): QuickBooks company ID

**Example LLM prompt**: "Get company info for my sandbox"

---

## QuickBooks Sandbox Notes

- This is a TEST environment - data does not affect production
- Company ID is required for all operations
- Uses QuickBooks Query Language (QQL) for queries
- All monetary values are in the smallest currency unit (cents)

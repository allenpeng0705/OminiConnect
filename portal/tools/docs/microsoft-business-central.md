# Microsoft Business Central Tools

Provider: `microsoft-business-central` | Engine: `nango` | Auth: OAUTH2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Microsoft Business Central API. They allow AI agents to manage companies, customers, vendors, and invoices in ERP. **Requires Business Central Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client credentials via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.businesscentral.dynamics.com/v2.0/${connectionConfig.tenantId}/${connectionConfig.environmentName}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `business_central_list_companies` | List companies | GET | /api/v2.0/companies |
| `business_central_get_company` | Get company details | GET | /api/v2.0/companies/{companyId} |
| `business_central_list_customers` | List customers | GET | /api/v2.0/customers |
| `business_central_get_customer` | Get customer details | GET | /api/v2.0/customers/{customerId} |
| `business_central_list_vendors` | List vendors | GET | /api/v2.0/vendors |
| `business_central_get_vendor` | Get vendor details | GET | /api/v2.0/vendors/{vendorId} |
| `business_central_list_invoices` | List sales invoices | GET | /api/v2.0/salesInvoices |
| `business_central_get_invoice` | Get invoice details | GET | /api/v2.0/salesInvoices/{invoiceId} |
| `business_central_list_items` | List items | GET | /api/v2.0/items |
| `business_central_get_item` | Get item details | GET | /api/v2.0/items/{itemId} |

---

## Tool Details

### business_central_list_companies

**What it does**: Lists all companies in Business Central.

**When to use**: Find company IDs, understand organization.

**Arguments**: None

**Example LLM prompt**: "List all companies in Business Central"

---

### business_central_get_company

**What it does**: Gets details of a specific company.

**When to use**: Check company information.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get details for company 12345"

---

### business_central_list_customers

**What it does**: Lists all customers in Business Central.

**When to use**: Browse customers, find customer IDs.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 20)

**Example LLM prompt**: "List all customers in City ABC"

---

### business_central_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Check customer details, view balance.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer 12345"

---

### business_central_list_vendors

**What it does**: Lists all vendors in Business Central.

**When to use**: Browse vendors, find vendor IDs.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 20)

**Example LLM prompt**: "List all vendors"

---

### business_central_get_vendor

**What it does**: Gets details of a specific vendor.

**When to use**: Check vendor details, view payables.

**Arguments**:
- `vendorId` (required): Vendor ID

**Example LLM prompt**: "Get details for vendor 12345"

---

### business_central_list_invoices

**What it does**: Lists all sales invoices in Business Central.

**When to use**: Track invoices, manage receivables.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 20)

**Example LLM prompt**: "List all unpaid invoices"

---

### business_central_get_invoice

**What it does**: Gets details of a specific sales invoice.

**When to use**: Check invoice details, view line items.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice 12345"

---

### business_central_list_items

**What it does**: Lists all items (products/services) in Business Central.

**When to use**: Browse inventory, find item IDs.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 20)

**Example LLM prompt**: "List all items in category Electronics"

---

### business_central_get_item

**What it does**: Gets details of a specific item.

**When to use**: Check item details, view pricing.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Get details for item 12345"

---

## Business Central Notes

- **Client Credentials**: Machine-to-machine auth
- **Companies**: Multiple companies in single tenant
- **Customers/Vendors**: AR/AP management
- **Items**: Products and services catalog
- **OData**: Standard REST API with OData queries

# NetSuite (TBA) Tools

Provider: `netsuite-tba` | Engine: `nango` | Auth: Token-Based Authentication via Nango

## Overview

These tools wrap the NetSuite SuiteTalks REST API using Token-Based Authentication. They allow AI agents to manage customers, invoices, items, vendors, purchase orders, employees, and departments. **Requires NetSuite TBA authentication.**

## Authentication

**Token-Based Authentication (TBA)**:
- Uses NetSuite access tokens instead of OAuth
- Requires Account ID, Consumer Key/Secret, Token ID/Secret
- Same API endpoints as OAuth variant
- Base URL: `https://{accountId}.suitetalk.api.netsuite.com/services/rest/record/v1`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `netsuite_list_customers` | List customers | GET | /customer |
| `netsuite_get_customer` | Get customer details | GET | /customer/{id} |
| `netsuite_list_invoices` | List invoices | GET | /invoice |
| `netsuite_get_invoice` | Get invoice details | GET | /invoice/{id} |
| `netsuite_list_items` | List items/products | GET | /item |
| `netsuite_list_vendors` | List vendors | GET | /vendor |
| `netsuite_list_purchase_orders` | List purchase orders | GET | /purchaseOrder |
| `netsuite_list_employees` | List employees | GET | /employee |
| `netsuite_list_departments` | List departments | GET | /department |
| `netsuite_get_server_time` | Get server time | GET | /authentication |

---

## Tool Details

### netsuite_list_customers

**What it does**: Lists all customers in NetSuite.

**When to use**: Browse customer database, find customers by name.

**Arguments**:
- `limit` (optional): Number of records (default 10, max 1000)
- `offset` (optional): Record offset for pagination

**Example LLM prompt**: "List the first 50 customers in NetSuite"

---

### netsuite_get_customer

**What it does**: Gets detailed information for a specific customer.

**When to use**: View customer details, contact info, transaction history.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer ID 12345"

---

### netsuite_list_invoices

**What it does**: Lists all invoices in NetSuite.

**When to use**: Browse invoices, track billing.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset
- `status` (optional): Filter by status

**Example LLM prompt**: "List all open invoices"

---

### netsuite_get_invoice

**What it does**: Gets detailed information for a specific invoice.

**When to use**: View invoice details, line items, payment status.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-2024-001"

---

### netsuite_list_items

**What it does**: Lists all items/products in NetSuite.

**When to use**: Browse inventory, find products.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset
- `type` (optional): Filter by item type

**Example LLM prompt**: "List all inventory items"

---

### netsuite_list_vendors

**What it does**: Lists all vendors in NetSuite.

**When to use**: Browse vendor database, find suppliers.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset

**Example LLM prompt**: "List all approved vendors"

---

### netsuite_list_purchase_orders

**What it does**: Lists all purchase orders in NetSuite.

**When to use**: Track procurement, manage orders.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending purchase orders"

---

### netsuite_list_employees

**What it does**: Lists all employees in NetSuite.

**When to use**: Browse employee directory.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset

**Example LLM prompt**: "List all active employees"

---

### netsuite_list_departments

**What it does**: Lists all departments in NetSuite.

**When to use**: View organizational structure.

**Arguments**:
- `limit` (optional): Number of records
- `offset` (optional): Record offset

**Example LLM prompt**: "List all departments"

---

### netsuite_get_server_time

**What it does**: Gets the current server time from NetSuite.

**When to use**: Sync time, check connectivity.

**Arguments**: None

**Example LLM prompt**: "What is the current NetSuite server time"

---

## NetSuite API Notes

- **Authentication**: Uses Token-Based Authentication (TBA) instead of OAuth
- **Account ID**: NetSuite account identifier (e.g., `TSTDRV123456`)
- **Pagination**: Use limit and offset for paginated results
- **REST API**: Uses SuiteTalks REST API (not SOAP)
- **Rate limits**: Implement backoff for heavy API usage

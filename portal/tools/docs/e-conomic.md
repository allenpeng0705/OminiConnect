# e-conomic Tools

Provider: `e-conomic` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the e-conomic API. They allow AI agents to manage customers, invoices, products, orders, and accounts. e-conomic is a Danish cloud-based accounting platform.

## Authentication

**Nango BASIC**:
- User provides private token and agreement grant token via Nango
- Token stored in Nango, accessed via `connection_ref`
- Passed in headers for authentication

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `e_conomic_list_customers` | List customers | GET | /customers |
| `e_conomic_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `e_conomic_list_invoices` | List invoices | GET | /invoices |
| `e_conomic_get_invoice` | Get invoice details | GET | /invoices/{invoice_id} |
| `e_conomic_list_products` | List products | GET | /products |
| `e_conomic_get_product` | Get product details | GET | /products/{product_id} |
| `e_conomic_list_orders` | List orders | GET | /orders |
| `e_conomic_get_order` | Get order details | GET | /orders/{order_id} |
| `e_conomic_list_accounts` | List accounts | GET | /accounts |
| `e_conomic_get_account` | Get account details | GET | /accounts/{account_id} |

---

## Tool Details

### e_conomic_list_customers

**What it does**: Lists all customers with optional email filtering.

**When to use**: Browse customer database, find customers, manage customer relationships.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `email` (optional): Filter by email address

**Example LLM prompt**: "List all customers"

---

### e_conomic_get_customer

**What it does**: Gets detailed customer information.

**When to use**: Check customer details, verify customer balance, review customer data.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer c-123"

---

### e_conomic_list_invoices

**What it does**: Lists all invoices with optional customer filtering.

**When to use**: View invoice list, track outstanding invoices, find invoices by customer.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `customer_id` (optional): Filter by customer ID

**Example LLM prompt**: "List all invoices for customer c-123"

---

### e_conomic_get_invoice

**What it does**: Gets detailed invoice information.

**When to use**: Check invoice details, verify line items, review payment status.

**Arguments**:
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice i-456"

---

### e_conomic_list_products

**What it does**: Lists all products in the inventory.

**When to use**: Browse product catalog, find products, manage inventory.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all products"

---

### e_conomic_get_product

**What it does**: Gets detailed product information.

**When to use**: Check product details, verify pricing, review product settings.

**Arguments**:
- `product_id` (required): Product ID

**Example LLM prompt**: "Get details for product p-789"

---

### e_conomic_list_orders

**What it does**: Lists all orders with optional customer filtering.

**When to use**: View sales orders, track order status, find orders by customer.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `customer_id` (optional): Filter by customer ID

**Example LLM prompt**: "List all orders for customer c-123"

---

### e_conomic_get_order

**What it does**: Gets detailed order information.

**When to use**: Check order details, verify order lines, track delivery status.

**Arguments**:
- `order_id` (required): Order ID

**Example LLM prompt**: "Get details for order o-101"

---

### e_conomic_list_accounts

**What it does**: Lists all accounts in the chart of accounts.

**When to use**: Browse account structure, find specific accounts, understand financial layout.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all accounts"

---

### e_conomic_get_account

**What it does**: Gets detailed account information.

**When to use**: Check account details, verify account balance, review account settings.

**Arguments**:
- `account_id` (required): Account ID

**Example LLM prompt**: "Get details for account a-202"

---

## e-conomic API Notes

- **Accounting Platform**: Cloud-based accounting for Danish businesses
- **Customers**: Business customers and contacts
- **Invoices**: Sales invoices with line items and payment terms
- **Products**: Items in the product catalog with pricing
- **Orders**: Sales orders for products and services
- **Accounts**: Chart of accounts for double-entry bookkeeping
- **Private Token**: Used as username for authentication
- **Agreement Grant Token**: Used as password for authentication

# Fortnox Tools

Provider: `fortnox` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Fortnox API. They allow AI agents to interact with invoices, customers, suppliers, products, and employees in the Fortnox accounting platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `invoices:read`, `invoices:write`, `customers:read`, `suppliers:read`, `products:read`, `employees:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fortnox_list_invoices` | List all invoices | GET | /invoices |
| `fortnox_get_invoice` | Get details of a specific invoice | GET | /invoices/{id} |
| `fortnox_create_invoice` | Create a new invoice | POST | /invoices |
| `fortnox_list_customers` | List all customers | GET | /customers |
| `fortnox_get_customer` | Get details of a specific customer | GET | /customers/{id} |
| `fortnox_list_suppliers` | List all suppliers | GET | /suppliers |
| `fortnox_get_supplier` | Get details of a specific supplier | GET | /suppliers/{id} |
| `fortnox_list_products` | List all products | GET | /products |
| `fortnox_get_product` | Get details of a specific product | GET | /products/{id} |
| `fortnox_list_employees` | List all employees | GET | /employees |

---

## Tool Details

### fortnox_list_invoices

**What it does**: Returns a paginated list of invoices with their status, customer, and amount.

**When to use**: Track payments, monitor outstanding invoices, or review billing history.

**Arguments**:
- `status` (optional): Filter by status: draft, printed, sent, paid, overdue
- `customer_id` (optional): Filter by customer ID
- `date_from` (optional): Filter by invoice date from (YYYY-MM-DD)
- `date_to` (optional): Filter by invoice date to (YYYY-MM-DD)
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Show me all overdue invoices from this month"

---

### fortnox_get_invoice

**What it does**: Get details of a specific invoice including line items, totals, tax, and payment status.

**When to use**: Review invoice details before payment or check billing information.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice inv123"

---

### fortnox_create_invoice

**What it does**: Creates a new invoice for a customer with line items, pricing, and automatic tax calculation.

**When to use**: Generate invoices programmatically or automate billing workflows.

**Arguments**:
- `customer_id` (required): Customer ID
- `invoice_date` (optional): Invoice date (YYYY-MM-DD)
- `due_date` (optional): Payment due date (YYYY-MM-DD)
- `lines` (optional): Array of invoice line items with article_id, quantity, price
- `remarks` (optional): Invoice remarks/notes

**Example LLM prompt**: "Create an invoice for customer cust456 with two line items: 5 units of product prod1 at $100 each"

---

### fortnox_list_customers

**What it does**: Returns a paginated list of customers with their contact information and balances.

**When to use**: Browse customer database or search for specific customers.

**Arguments**:
- `search` (optional): Search by name, email, or customer number
- `price_list` (optional): Filter by price list
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Search for customers with 'Acme' in their name"

---

### fortnox_get_customer

**What it does**: Get details of a specific customer including contact info, address, payment terms, and balance.

**When to use**: Look up customer details before creating invoices or checking credit status.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer cust789"

---

### fortnox_list_suppliers

**What it does**: Returns a paginated list of suppliers with their contact information and balances.

**When to use**: Browse supplier database or find specific vendors.

**Arguments**:
- `search` (optional): Search by name or supplier number
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all suppliers"

---

### fortnox_get_supplier

**What it does**: Get details of a specific supplier including contact info, address, and payment terms.

**When to use**: Look up supplier details before creating purchase orders or checking payment status.

**Arguments**:
- `id` (required): Supplier ID

**Example LLM prompt**: "Get details for supplier sup123"

---

### fortnox_list_products

**What it does**: Returns a paginated list of products and articles with pricing and stock levels.

**When to use**: Browse inventory, check stock levels, or find specific products.

**Arguments**:
- `search` (optional): Search by name, article number, or description
- `warehouse_id` (optional): Filter by warehouse ID
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "Search for products with 'widget' in the name"

---

### fortnox_get_product

**What it does**: Get details of a specific product including pricing, stock levels, and unit of measure.

**When to use**: Check product availability or get detailed pricing information.

**Arguments**:
- `id` (required): Product ID

**Example LLM prompt**: "Get details for product prod456"

---

### fortnox_list_employees

**What it does**: Returns a paginated list of employees with their departments and employment details.

**When to use**: Browse employee directory or find specific staff members.

**Arguments**:
- `search` (optional): Search by name or employee ID
- `department` (optional): Filter by department
- `per_page` (optional): Results per page (default 50)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all employees in the sales department"

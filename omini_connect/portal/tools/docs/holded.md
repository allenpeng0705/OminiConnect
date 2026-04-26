# Holded Tools

Provider: `holded` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Holded API. They allow AI agents to manage contacts, invoices, products, expenses, and projects. Holded is an online accounting and invoicing platform for SMBs.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in `Key` header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.holded.com/api

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `holded_list_contacts` | List contacts | GET | /invoicing/v1/contacts |
| `holded_get_contact` | Get contact details | GET | /invoicing/v1/contacts/{id} |
| `holded_list_invoices` | List invoices | GET | /invoicing/v1/invoices |
| `holded_get_invoice` | Get invoice details | GET | /invoicing/v1/invoices/{id} |
| `holded_list_products` | List products | GET | /invoicing/v1/products |
| `holded_get_product` | Get product details | GET | /invoicing/v1/products/{id} |
| `holded_list_expenses` | List expenses | GET | /invoicing/v1/expenses |
| `holded_get_expense` | Get expense details | GET | /invoicing/v1/expenses/{id} |
| `holded_list_treasury` | List treasury movements | GET | /invoicing/v1/treasury |
| `holded_list_projects` | List projects | GET | /invoicing/v1/projects |

---

## Tool Details

### holded_list_contacts

**What it does**: Lists all contacts in Holded.

**When to use**: Browse contact directory.

**Arguments**:
- `type` (optional): Filter by type (`customer`, `vendor`)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all customer contacts"

---

### holded_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact details and invoice history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact with ID abc123"

---

### holded_list_invoices

**What it does**: Lists all invoices in Holded.

**When to use**: Browse invoice list.

**Arguments**:
- `contact_id` (optional): Filter by contact ID
- `status` (optional): Filter by status (`draft`, `sent`, `paid`)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all paid invoices"

---

### holded_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: View invoice details and payment status.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice with ID inv456"

---

### holded_list_products

**What it does**: Lists all products in Holded.

**When to use**: Browse product catalog.

**Arguments**: None

**Example LLM prompt**: "List all products"

---

### holded_get_product

**What it does**: Gets detailed information about a specific product.

**When to use**: View product details and inventory.

**Arguments**:
- `id` (required): Product ID

**Example LLM prompt**: "Get product with ID prd789"

---

### holded_list_expenses

**What it does**: Lists all expenses in Holded.

**When to use**: Browse expense list.

**Arguments**:
- `contact_id` (optional): Filter by contact ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all expenses"

---

### holded_get_expense

**What it does**: Gets detailed information about a specific expense.

**When to use**: View expense details and receipt.

**Arguments**:
- `id` (required): Expense ID

**Example LLM prompt**: "Get expense with ID exp101"

---

### holded_list_treasury

**What it does**: Lists all treasury movements in Holded.

**When to use**: View cash flow movements.

**Arguments**: None

**Example LLM prompt**: "List all treasury movements"

---

### holded_list_projects

**What it does**: Lists all projects in Holded.

**When to use**: Browse project list.

**Arguments**: None

**Example LLM prompt**: "List all projects"

---

## Holded API Notes

- **API Base URL**: https://api.holded.com/api
- **Auth Mode**: API Key in header (`Key`)
- **Contacts**: Customers and vendors
- **Invoices**: Sales and purchase invoices
- **Products**: Product catalog with pricing
- **Expenses**: Business expenses with receipts
- **Treasury**: Cash and bank movements
- **Projects**: Project tracking and billing

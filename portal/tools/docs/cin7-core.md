# Cin7 Core Tools

Provider: `cin7-core` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Cin7 Core API. Cin7 Core is an inventory and order management platform for product businesses. **Requires Cin7 Core API key and Account ID.**

## Authentication

**Nango API_KEY**:
- User provides their Cin7 Core API key and Account ID
- Token passed via `api-auth-applicationkey` header
- Account ID passed via `api-auth-accountid` header
- Base URL: `https://inventory.dearsystems.com/externalapi`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cin7_core_list_products` | List products | GET | /v2/product |
| `cin7_core_get_product` | Get product details | GET | /v2/product/{id} |
| `cin7_core_list_orders` | List orders | GET | /v2/salesorder |
| `cin7_core_get_order` | Get order details | GET | /v2/salesorder/{id} |
| `cin7_core_create_order` | Create an order | POST | /v2/salesorder |
| `cin7_core_list_contacts` | List contacts | GET | /v2/contact |
| `cin7_core_get_contact` | Get contact details | GET | /v2/contact/{id} |
| `cin7_core_list_inventory` | List inventory | GET | /v2/stockitem |
| `cin7_core_get_inventory_item` | Get inventory item | GET | /v2/stockitem/{id} |
| `cin7_core_list_sales_orders` | List sales orders | GET | /v2/salesorder |

---

## Tool Details

### cin7_core_list_products

**What it does**: Lists all products in the inventory.

**When to use**: Browse product catalog.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Cin7 products"

---

### cin7_core_get_product

**What it does**: Gets details of a specific product.

**When to use**: View product information and pricing.

**Arguments**:
- `id` (required): Product ID

**Example LLM prompt**: "Get product 123 details"

---

### cin7_core_list_orders

**What it does**: Lists all orders.

**When to use**: View order history and status.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all pending orders"

---

### cin7_core_get_order

**What it does**: Gets details of a specific order.

**When to use**: View order details and line items.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get order 456 details"

---

### cin7_core_create_order

**What it does**: Creates a new sales order.

**When to use**: Create a new order for a customer.

**Arguments**:
- `contact_id` (required): Contact ID
- `lines` (required): Array of order line items

**Example LLM prompt**: "Create an order for contact 123"

---

### cin7_core_list_contacts

**What it does**: Lists all contacts.

**When to use**: Find customers and vendors.

**Arguments**:
- `type` (optional): Filter by contact type
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Cin7 contacts"

---

### cin7_core_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact 789 details"

---

### cin7_core_list_inventory

**What it does**: Lists all inventory items.

**When to use**: Check stock levels.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all inventory items"

---

### cin7_core_get_inventory_item

**What it does**: Gets details of a specific inventory item.

**When to use**: View stock levels and locations.

**Arguments**:
- `id` (required): Stock item ID

**Example LLM prompt**: "Get inventory item 101 details"

---

### cin7_core_list_sales_orders

**What it does**: Lists all sales orders.

**When to use**: View all sales orders.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all sales orders"

---

## Cin7 Core API Notes

- **Account ID**: Required header for API authentication
- **Products**: Items in the product catalog
- **Sales Orders**: Customer orders for products
- **Inventory**: Stock items and quantities
- **Contacts**: Customers and vendors

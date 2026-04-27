# eBay Sandbox Tools

Provider: `ebay-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the eBay Sandbox API for testing purposes. They allow AI agents to manage inventory items, orders, users, categories, and transactions in a sandbox environment.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with eBay Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Uses sandbox environment (api.sandbox.ebay.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ebay_sandbox_list_items` | List items for sale | GET | /sell/inventory/v1/inventory_item |
| `ebay_sandbox_get_item` | Get item details | GET | /sell/inventory/v1/inventory_item/{sku} |
| `ebay_sandbox_list_orders` | List orders | GET | /sell/fulfillment/v1/order |
| `ebay_sandbox_get_order` | Get order details | GET | /sell/fulfillment/v1/order/{order_id} |
| `ebay_sandbox_list_users` | List users | GET | /user |
| `ebay_sandbox_get_user` | Get user details | GET | /user/{user_id} |
| `ebay_sandbox_list_categories` | List categories | GET | /commerce/category/v1/tree |
| `ebay_sandbox_get_category` | Get category details | GET | /commerce/category/v1/category/{category_id} |
| `ebay_sandbox_list_transactions` | List transactions | GET | /sell/fulfillment/v1/transactions |
| `ebay_sandbox_get_transaction` | Get transaction details | GET | /sell/fulfillment/v1/transactions/{transaction_id} |

---

## Tool Details

### ebay_sandbox_list_items

**What it does**: Lists all inventory items in sandbox.

**When to use**: Test item listing, develop against mock data.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all sandbox inventory items"

---

### ebay_sandbox_get_item

**What it does**: Gets detailed inventory item information in sandbox.

**When to use**: Test item retrieval, verify data structure.

**Arguments**:
- `sku` (required): Item SKU

**Example LLM prompt**: "Get details for sandbox item with SKU item-123"

---

### ebay_sandbox_list_orders

**What it does**: Lists all orders in sandbox.

**When to use**: Test order listing, simulate order flows.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `order_state` (optional): Filter by ACTIVE, COMPLETED, or CANCELLED

**Example LLM prompt**: "List all sandbox orders"

---

### ebay_sandbox_get_order

**What it does**: Gets detailed order information in sandbox.

**When to use**: Test order retrieval, verify order data.

**Arguments**:
- `order_id` (required): Order ID

**Example LLM prompt**: "Get details for sandbox order 123456789"

---

### ebay_sandbox_list_users

**What it does**: Lists users in sandbox.

**When to use**: Test user listing, develop user features.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all sandbox users"

---

### ebay_sandbox_get_user

**What it does**: Gets detailed user information in sandbox.

**When to use**: Test user retrieval, verify user data.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for sandbox user u-123"

---

### ebay_sandbox_list_categories

**What it does**: Lists the eBay category tree in sandbox.

**When to use**: Test category browsing, develop category features.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all sandbox categories"

---

### ebay_sandbox_get_category

**What it does**: Gets detailed category information in sandbox.

**When to use**: Test category retrieval, verify category data.

**Arguments**:
- `category_id` (required): Category ID

**Example LLM prompt**: "Get details for sandbox category 267"

---

### ebay_sandbox_list_transactions

**What it does**: Lists all transactions in sandbox.

**When to use**: Test transaction listing, develop transaction features.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all sandbox transactions"

---

### ebay_sandbox_get_transaction

**What it does**: Gets detailed transaction information in sandbox.

**When to use**: Test transaction retrieval, verify transaction data.

**Arguments**:
- `transaction_id` (required): Transaction ID

**Example LLM prompt**: "Get details for sandbox transaction t-789"

---

## eBay Sandbox API Notes

- **Sandbox Only**: This environment is for testing and development
- **Same API Structure**: Identical endpoints to production eBay API
- **Sandbox URLs**: Uses api.sandbox.ebay.com instead of api.ebay.com
- **Sandbox URLs Auth**: Uses auth.sandbox.ebay.com for OAuth
- **Mock Data**: Contains synthetic or test data
- **Inventory Items**: Test product listings
- **Orders**: Simulated sales orders
- **Users**: Test buyer and seller accounts
- **Categories**: Same category structure as production
- **Transactions**: Simulated payment transactions

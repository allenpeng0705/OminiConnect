# eBay Tools

Provider: `ebay` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the eBay API. They allow AI agents to manage inventory items, orders, users, categories, and transactions. eBay is an e-commerce marketplace platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with eBay
- Token stored in Nango, accessed via `connection_ref`
- Supports refresh token flow

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ebay_list_items` | List items for sale | GET | /sell/inventory/v1/inventory_item |
| `ebay_get_item` | Get item details | GET | /sell/inventory/v1/inventory_item/{sku} |
| `ebay_list_orders` | List orders | GET | /sell/fulfillment/v1/order |
| `ebay_get_order` | Get order details | GET | /sell/fulfillment/v1/order/{order_id} |
| `ebay_list_users` | List users | GET | /user |
| `ebay_get_user` | Get user details | GET | /user/{user_id} |
| `ebay_list_categories` | List categories | GET | /commerce/category/v1/tree |
| `ebay_get_category` | Get category details | GET | /commerce/category/v1/category/{category_id} |
| `ebay_list_transactions` | List transactions | GET | /sell/fulfillment/v1/transactions |
| `ebay_get_transaction` | Get transaction details | GET | /sell/fulfillment/v1/transactions/{transaction_id} |

---

## Tool Details

### ebay_list_items

**What it does**: Lists all inventory items with pagination.

**When to use**: Browse product inventory, find items by SKU, manage listings.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all inventory items"

---

### ebay_get_item

**What it does**: Gets detailed inventory item information.

**When to use**: Check item details, verify listing status, review inventory levels.

**Arguments**:
- `sku` (required): Item SKU

**Example LLM prompt**: "Get details for item with SKU item-123"

---

### ebay_list_orders

**What it does**: Lists all orders with optional state filtering.

**When to use**: View sales orders, track fulfillment status, find pending orders.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `order_state` (optional): Filter by ACTIVE, COMPLETED, or CANCELLED

**Example LLM prompt**: "List all active orders"

---

### ebay_get_order

**What it does**: Gets detailed order information.

**When to use**: Check order details, verify buyer information, review line items.

**Arguments**:
- `order_id` (required): Order ID

**Example LLM prompt**: "Get details for order 123456789"

---

### ebay_list_users

**What it does**: Lists users (for the authenticated OAuth context).

**When to use**: View user account information, check seller profile.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List eBay users"

---

### ebay_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user profile, verify seller ratings, review account details.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-123"

---

### ebay_list_categories

**What it does**: Lists the eBay category tree.

**When to use**: Browse categories, find appropriate category for listing, understand category structure.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all eBay categories"

---

### ebay_get_category

**What it does**: Gets detailed category information.

**When to use**: Check category details, understand category requirements, find subcategories.

**Arguments**:
- `category_id` (required): Category ID

**Example LLM prompt**: "Get details for category 267"

---

### ebay_list_transactions

**What it does**: Lists all transactions.

**When to use**: View transaction history, track payments, find specific transactions.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all transactions"

---

### ebay_get_transaction

**What it does**: Gets detailed transaction information.

**When to use**: Check transaction details, verify payment status, review transaction line items.

**Arguments**:
- `transaction_id` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction t-789"

---

## eBay API Notes

- **E-commerce Marketplace**: Platform for buying and selling goods
- **Inventory Items**: Product listings with SKU and quantities
- **Orders**: Sales orders from buyers
- **Users**: Buyer and seller accounts
- **Categories**: Product category taxonomy for listings
- **Transactions**: Individual payment transactions
- **OAuth Flow**: Standard OAuth2 with refresh tokens
- **Production API**: Uses api.ebay.com for production

# Rippling-shop-app Tools

Provider: `rippling-shop-app` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Rippling Shop App is an e-commerce platform for employee shopping and procurement. These tools allow AI agents to manage products, orders, customers, and inventory.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Rippling Shop App
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `products:read`, `products:write`, `orders:read`, `orders:write`, `customers:read`, `inventory:read`, `inventory:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rippling-shop-app_list_products` | List all products | GET | /v1/products |
| `rippling-shop-app_get_product` | Get product details | GET | /v1/products/{productId} |
| `rippling-shop-app_create_product` | Create a product | POST | /v1/products |
| `rippling-shop-app_list_orders` | List all orders | GET | /v1/orders |
| `rippling-shop-app_get_order` | Get order details | GET | /v1/orders/{orderId} |
| `rippling-shop-app_create_order` | Create an order | POST | /v1/orders |
| `rippling-shop-app_list_customers` | List customers | GET | /v1/customers |
| `rippling-shop-app_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `rippling-shop-app_list_inventory` | List inventory | GET | /v1/inventory |
| `rippling-shop-app_update_inventory` | Update inventory | PUT | /v1/inventory/{productId} |

---

## Tool Details

### rippling-shop-app_list_products

**What it does**: Returns a list of all products.

**When to use**: Browse available products in the shop.

**Arguments**:
- `limit` (optional): Number of products (default 50)

**Example LLM prompt**: "List all products"

---

### rippling-shop-app_get_product

**What it does**: Gets details of a specific product.

**When to use**: Get product information, price, and description.

**Arguments**:
- `productId` (required): The product ID

**Example LLM prompt**: "Get details for product prd_abc123"

---

### rippling-shop-app_create_product

**What it does**: Creates a new product.

**When to use**: Add new products to the shop.

**Arguments**:
- `name` (required): Product name
- `description` (optional): Product description
- `price` (required): Product price
- `sku` (optional): Stock keeping unit

**Example LLM prompt**: "Create a product called 'Office Chair' for $299"

---

### rippling-shop-app_list_orders

**What it does**: Returns a list of all orders.

**When to use**: View order history and status.

**Arguments**:
- `limit` (optional): Number of orders (default 50)
- `status` (optional): Filter by status (pending, processing, shipped, delivered)

**Example LLM prompt**: "List all pending orders"

---

### rippling-shop-app_get_order

**What it does**: Gets details of a specific order.

**When to use**: Get order details, items, and shipping info.

**Arguments**:
- `orderId` (required): The order ID

**Example LLM prompt**: "Get details for order ord_xyz789"

---

### rippling-shop-app_create_order

**What it does**: Creates a new order.

**When to use**: Place a new order for a customer.

**Arguments**:
- `customerId` (required): Customer ID
- `items` (required): Array of order items
- `shippingAddress` (optional): Shipping address

**Example LLM prompt**: "Create an order for customer cust_123 with item prd_456"

---

### rippling-shop-app_list_customers

**What it does**: Returns a list of all customers.

**When to use**: View customer database.

**Arguments**:
- `limit` (optional): Number of customers (default 50)

**Example LLM prompt**: "List all customers"

---

### rippling-shop-app_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Get customer information and order history.

**Arguments**:
- `customerId` (required): The customer ID

**Example LLM prompt**: "Get details for customer cust_123"

---

### rippling-shop-app_list_inventory

**What it does**: Returns inventory items and quantities.

**When to use**: Check stock levels.

**Arguments**:
- `limit` (optional): Number of items (default 50)

**Example LLM prompt**: "List all inventory"

---

### rippling-shop-app_update_inventory

**What it does**: Updates inventory quantity.

**When to use**: Adjust stock levels.

**Arguments**:
- `productId` (required): The product ID
- `quantity` (required): New quantity

**Example LLM prompt**: "Update inventory for product prd_456 to 50 units"

---

## Rippling Shop App Notes

- Products have SKUs and pricing
- Orders link customers to products
- Inventory tracks stock quantities
- Customers have order history

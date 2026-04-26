# ECU360 Production Tools

Provider: `ecu360-production` | Engine: `nango` | Auth: Two-Step via Nango (Production)

## Overview

These tools wrap the ECU360 Production API. They allow AI agents to manage products, orders, customers, inventory, and shipments. ECU360 is an e-commerce and supply chain management platform.

## Authentication

**Nango Two-Step (Production)**:
- Uses API key, username, and password for authentication
- Token stored in Nango, accessed via `connection_ref`
- Uses production environment (apim.ecu360.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ecu360_production_list_products` | List products | GET | /products |
| `ecu360_production_get_product` | Get product details | GET | /products/{product_id} |
| `ecu360_production_list_orders` | List orders | GET | /orders |
| `ecu360_production_get_order` | Get order details | GET | /orders/{order_id} |
| `ecu360_production_list_customers` | List customers | GET | /customers |
| `ecu360_production_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `ecu360_production_list_inventories` | List inventory | GET | /inventory |
| `ecu360_production_get_inventory` | Get inventory details | GET | /inventory/{inventory_id} |
| `ecu360_production_list_shipments` | List shipments | GET | /shipments |
| `ecu360_production_get_shipment` | Get shipment details | GET | /shipments/{shipment_id} |

---

## Tool Details

### ecu360_production_list_products

**What it does**: Lists all products in production.

**When to use**: Browse product catalog, find products, manage inventory.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all products in production"

---

### ecu360_production_get_product

**What it does**: Gets detailed product information.

**When to use**: Check product details, verify product data, review pricing.

**Arguments**:
- `product_id` (required): Product ID

**Example LLM prompt**: "Get details for product p-123"

---

### ecu360_production_list_orders

**What it does**: Lists all orders in production with optional status filtering.

**When to use**: View order list, track order status, find pending orders.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `status` (optional): Filter by pending, shipped, or delivered

**Example LLM prompt**: "List all pending orders"

---

### ecu360_production_get_order

**What it does**: Gets detailed order information.

**When to use**: Check order details, verify order items, review delivery status.

**Arguments**:
- `order_id` (required): Order ID

**Example LLM prompt**: "Get details for order o-456"

---

### ecu360_production_list_customers

**What it does**: Lists all customers in production.

**When to use**: Browse customer database, find customers, manage relationships.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all customers"

---

### ecu360_production_get_customer

**What it does**: Gets detailed customer information.

**When to use**: Check customer details, verify customer data, review purchase history.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer c-789"

---

### ecu360_production_list_inventories

**What it does**: Lists all inventory records.

**When to use**: Check stock levels, track inventory, manage warehouse.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all inventory"

---

### ecu360_production_get_inventory

**What it does**: Gets detailed inventory information.

**When to use**: Check stock levels, verify inventory data, track quantities.

**Arguments**:
- `inventory_id` (required): Inventory ID

**Example LLM prompt**: "Get details for inventory i-101"

---

### ecu360_production_list_shipments

**What it does**: Lists all shipments.

**When to use**: View shipment list, track deliveries, find pending shipments.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all shipments"

---

### ecu360_production_get_shipment

**What it does**: Gets detailed shipment information.

**When to use**: Check shipment status, verify delivery details, track package.

**Arguments**:
- `shipment_id` (required): Shipment ID

**Example LLM prompt**: "Get details for shipment s-202"

---

## ECU360 Production API Notes

- **Production Environment**: Uses the live production API (apim.ecu360.com)
- **Test Variant**: ecu360 uses the test environment
- **Products**: E-commerce product catalog
- **Orders**: Customer purchase orders
- **Customers**: Customer account management
- **Inventory**: Warehouse stock tracking
- **Shipments**: Order fulfillment and delivery tracking
- **Two-Step Auth**: Uses API key, username, and password flow

# Shopify Tools

Provider: `shopify` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Shopify REST Admin API (2024-01). They allow AI agents to manage products, orders, customers, and fulfillments in a Shopify store. Shopify is the #1 e-commerce platform globally.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Shopify
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_products`, `write_products`, `read_orders`, `write_orders`, `read_customers`, `read_fulfillments`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `shopify_list_orders` | List orders | GET | /admin/api/2024-01/orders.json |
| `shopify_get_order` | Get order details | GET | /admin/api/2024-01/orders/{order_id}.json |
| `shopify_create_order` | Create an order | POST | /admin/api/2024-01/orders.json |
| `shopify_list_products` | List products | GET | /admin/api/2024-01/products.json |
| `shopify_get_product` | Get product details | GET | /admin/api/2024-01/products/{product_id}.json |
| `shopify_create_product` | Create a product | POST | /admin/api/2024-01/products.json |
| `shopify_list_customers` | List customers | GET | /admin/api/2024-01/customers.json |
| `shopify_get_customer` | Get customer details | GET | /admin/api/2024-01/customers/{customer_id}.json |
| `shopify_list_fulfillments` | List fulfillments | GET | /admin/api/2024-01/orders/{order_id}/fulfillments.json |
| `shopify_get_fulfillment` | Get fulfillment details | GET | /admin/api/2024-01/orders/{order_id}/fulfillments/{fulfillment_id}.json |

---

## Tool Details

### shopify_list_orders

**What it does**: Lists all orders in a Shopify store with optional status and date filtering.

**When to use**: Track orders, find orders needing fulfillment, check financial status.

**Arguments**:
- `status` (optional): Filter by status: open, closed, cancelled, any
- `created_at_min` (optional): ISO 8601 date-time to filter orders created after this time
- `limit` (optional): Maximum number of orders to return (default 50, max 250)

**Example LLM prompt**: "List all paid orders from the last 7 days"

---

### shopify_get_order

**What it does**: Gets detailed information about a specific order including line items, customer, and shipping.

**When to use**: Check order details, view line items, check shipping address.

**Arguments**:
- `order_id` (required): Unique identifier of the order
- `fields` (optional): Comma-separated list of fields to include in response

**Example LLM prompt**: "Get details for order 54321"

---

### shopify_create_order

**What it does**: Creates a new order in the Shopify store with line items and shipping details.

**When to use**: Manually create orders, place orders for customers.

**Arguments**:
- `line_items` (required): Array of line item objects with variant_id and quantity
- `customer_id` (optional): ID of the customer placing the order
- `shipping_address` (optional): Shipping address object with address fields
- `financial_status` (optional): pending, authorized, paid

**Example LLM prompt**: "Create an order with 2 units of product variant 123 for customer 456"

---

### shopify_list_products

**What it does**: Lists all products in a Shopify store with filtering by status, type, or collection.

**When to use**: Browse catalog, find products by status, check inventory levels.

**Arguments**:
- `status` (optional): Filter by status: active, archived, draft
- `product_type` (optional): Filter by product type
- `collection_id` (optional): Filter by collection ID
- `limit` (optional): Maximum number of products to return (default 50, max 250)

**Example LLM prompt**: "List all active products in the store"

---

### shopify_get_product

**What it does**: Gets detailed information about a specific product including variants, images, and SEO.

**When to use**: Check full product details, view variants, check inventory.

**Arguments**:
- `product_id` (required): Unique identifier of the product
- `fields` (optional): Comma-separated list of fields to include in response

**Example LLM prompt**: "Get details for product 12345"

---

### shopify_create_product

**What it does**: Creates a new product in the Shopify store with variants, pricing, and media.

**When to use**: Add new products to the catalog.

**Arguments**:
- `title` (required): Product title
- `body_html` (optional): Product description in HTML format
- `vendor` (optional): Vendor name
- `product_type` (optional): Product type category
- `variants` (optional): Array of variant objects with price and inventory

**Example LLM prompt**: "Create a new product called 'Blue T-Shirt' with price $29.99 and size variants"

---

### shopify_list_customers

**What it does**: Lists all customers in a Shopify store with filtering by tags or email.

**When to use**: Look up customers, find customers by email or tags.

**Arguments**:
- `email` (optional): Filter by customer email address
- `tags` (optional): Filter by customer tags (comma-separated)
- `limit` (optional): Maximum number of customers to return (default 50, max 250)

**Example LLM prompt**: "List all customers with tag 'VIP'"

---

### shopify_get_customer

**What it does**: Gets detailed information about a specific customer including orders and addresses.

**When to use**: Get customer profile, view addresses, check order history.

**Arguments**:
- `customer_id` (required): Unique identifier of the customer
- `fields` (optional): Comma-separated list of fields to include in response

**Example LLM prompt**: "Get details for customer 67890"

---

### shopify_list_fulfillments

**What it does**: Lists all fulfillment orders for an order, showing shipment tracking information.

**When to use**: Check fulfillment status, track shipments.

**Arguments**:
- `order_id` (required): Unique identifier of the order

**Example LLM prompt**: "List fulfillments for order 54321"

---

### shopify_get_fulfillment

**What it does**: Gets detailed information about a specific fulfillment including tracking events.

**When to use**: Track shipment, get tracking info, check fulfillment details.

**Arguments**:
- `order_id` (required): Unique identifier of the order
- `fulfillment_id` (required): Unique identifier of the fulfillment

**Example LLM prompt**: "Get fulfillment details for order 54321 fulfillment 111"

---

## Shopify API Notes

- **API Version**: Uses `2024-01` API version
- **Rate Limits**: 40 requests per second for most endpoints
- **Inventory**: Use `inventory_item_id` and `location_id` for inventory management
- **Images**: Products can have multiple images via `images` array
- **Variants**: Products can have multiple variants (size, color, etc.)
- **Fulfillments**: Linked to orders, contain tracking information
- **Webhooks**: Shopify supports webhooks for real-time order/product updates (future expansion)

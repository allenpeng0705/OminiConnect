# Shopify More Tools

Provider: `shopify_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Shopify Admin API. They allow AI agents to manage orders, products, customers, and fulfillments in a Shopify store. This is an extended set of Shopify tools for comprehensive store management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Shopify
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_orders`, `write_orders`, `read_products`, `write_products`, `read_customers`, `read_fulfillments`, `write_fulfillments`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `shopify_more_list_orders` | List orders | GET | /admin/api/2024-01/orders.json |
| `shopify_more_get_order` | Get order details | GET | /admin/api/2024-01/orders/{order_id}.json |
| `shopify_more_create_order` | Create a new order | POST | /admin/api/2024-01/orders.json |
| `shopify_more_list_fulfillments` | List fulfillments | GET | /admin/api/2024-01/orders/{order_id}/fulfillments.json |
| `shopify_more_create_fulfillment` | Create a fulfillment | POST | /admin/api/2024-01/orders/{order_id}/fulfillments.json |
| `shopify_more_list_products` | List products | GET | /admin/api/2024-01/products.json |
| `shopify_more_get_product` | Get product details | GET | /admin/api/2024-01/products/{product_id}.json |
| `shopify_more_create_product` | Create a new product | POST | /admin/api/2024-01/products.json |
| `shopify_more_list_customers` | List customers | GET | /admin/api/2024-01/customers.json |
| `shopify_more_get_customer` | Get customer details | GET | /admin/api/2024-01/customers/{customer_id}.json |

---

## Tool Details

### shopify_more_list_orders

**What it does**: Lists all orders in the Shopify store. Supports filtering by status, fulfillment status, and date range.

**When to use**: Browse orders, track order status, find orders by date or fulfillment state.

**Arguments**:
- `status` (optional): Filter by status (any, open, closed, cancelled, pending)
- `fulfillment_status` (optional): Filter by fulfillment (shipped, partial, unshipped, unfulfilled)
- `limit` (optional): Max results (default 50, max 250)
- `created_at_min` (optional): Created after this date (ISO 8601)

**Example LLM prompt**: "List all open orders from the last week"

---

### shopify_more_get_order

**What it does**: Gets detailed information about a specific order including line items, shipping address, and payment details.

**When to use**: Check full order details, verify line items, get shipping information.

**Arguments**:
- `order_id` (required): The order ID

**Example LLM prompt**: "Get details for order 12345"

---

### shopify_more_create_order

**What it does**: Creates a new order in the Shopify store with line items, shipping address, and payment details.

**When to use**: Create orders programmatically, process manual orders.

**Arguments**:
- `line_items` (required): Line items array with variant_id and quantity
- `email` (optional): Customer email
- `shipping_address` (optional): Shipping address object
- `financial_status` (optional): Payment status (pending, paid, partially_paid)

**Example LLM prompt**: "Create an order for customer@example.com with one unit of variant 456"

---

### shopify_more_list_fulfillments

**What it does**: Lists all fulfillments for an order. Fulfillments represent shipments and tracking information.

**When to use**: Check fulfillment status, track shipments, view tracking details.

**Arguments**:
- `order_id` (required): The order ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all fulfillments for order 12345"

---

### shopify_more_create_fulfillment

**What it does**: Creates a fulfillment for an order with tracking numbers and customer notifications.

**When to use**: Mark orders as shipped, add tracking information.

**Arguments**:
- `order_id` (required): The order ID
- `tracking_number` (optional): Tracking number for the shipment
- `tracking_company` (optional): Tracking company name
- `notify_customer` (optional): Send notification email (default true)

**Example LLM prompt**: "Create a fulfillment for order 12345 with tracking number ABC123"

---

### shopify_more_list_products

**What it does**: Lists all products in the Shopify store. Supports filtering by collection, product type, and vendor.

**When to use**: Browse catalog, find products by category or vendor.

**Arguments**:
- `collection_id` (optional): Filter by collection ID
- `product_type` (optional): Filter by product type
- `vendor` (optional): Filter by vendor
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all products from vendor Acme Co"

---

### shopify_more_get_product

**What it does**: Gets detailed information about a specific product including variants, images, and metafields.

**When to use**: Check full product details, view variants, get inventory levels.

**Arguments**:
- `product_id` (required): The product ID

**Example LLM prompt**: "Get details for product 67890"

---

### shopify_more_create_product

**What it does**: Creates a new product in Shopify with title, description, variants, and images.

**When to use**: Add new products to the catalog.

**Arguments**:
- `title` (required): Product title
- `body_html` (optional): Product description (HTML)
- `vendor` (optional): Product vendor
- `product_type` (optional): Product type
- `variants` (optional): Product variants array

**Example LLM prompt**: "Create a new product called 'Blue T-Shirt' with price $29.99"

---

### shopify_more_list_customers

**What it does**: Lists all customers in the Shopify store. Supports filtering by tags and marketing acceptance.

**When to use**: Browse customer base, find customers by tags.

**Arguments**:
- `tags` (optional): Filter by tags (comma-separated)
- `accepts_marketing` (optional): Filter by marketing acceptance
- `limit` (optional): Max results (default 50, max 250)
- `created_at_min` (optional): Created after this date (ISO 8601)

**Example LLM prompt**: "List all customers who accept marketing"

---

### shopify_more_get_customer

**What it does**: Gets detailed information about a specific customer including orders, addresses, and metafields.

**When to use**: Get customer profile, view order history, check customer metafields.

**Arguments**:
- `customer_id` (required): The customer ID

**Example LLM prompt**: "Get details for customer 54321"

---

## Shopify API Notes

- **API Version**: Uses Shopify Admin API 2024-01
- **Rate Limits**: 40 requests per second for standard access
- **Metafields**: Products and customers support custom metafields
- **Fulfillments**: Multiple fulfillments per order are supported
- **Variants**: Products can have multiple variants with different prices/inventory
- **Webhooks**: Shopify supports webhooks for real-time updates (future expansion)

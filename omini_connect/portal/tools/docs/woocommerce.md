# WooCommerce Tools

Provider: `woocommerce` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the WooCommerce REST API. They allow AI agents to manage products, orders, customers, and coupons in a WooCommerce store. WooCommerce is a popular WordPress-based e-commerce platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with WooCommerce
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `woocommerce_list_orders` | List orders | GET | /wp-json/wc/v3/orders |
| `woocommerce_get_order` | Get order details | GET | /wp-json/wc/v3/orders/{id} |
| `woocommerce_create_order` | Create a new order | POST | /wp-json/wc/v3/orders |
| `woocommerce_list_products` | List products | GET | /wp-json/wc/v3/products |
| `woocommerce_get_product` | Get product details | GET | /wp-json/wc/v3/products/{id} |
| `woocommerce_create_product` | Create a new product | POST | /wp-json/wc/v3/products |
| `woocommerce_list_customers` | List customers | GET | /wp-json/wc/v3/customers |
| `woocommerce_get_customer` | Get customer details | GET | /wp-json/wc/v3/customers/{id} |
| `woocommerce_list_coupons` | List coupons | GET | /wp-json/wc/v3/coupons |
| `woocommerce_get_coupon` | Get coupon details | GET | /wp-json/wc/v3/coupons/{id} |

---

## Tool Details

### woocommerce_list_orders

**What it does**: Lists all orders in the WooCommerce store. Supports filtering by status, customer, and date range.

**When to use**: Browse orders, track order status, find orders by customer or date.

**Arguments**:
- `status` (optional): Filter by status (pending, processing, completed, on-hold, cancelled, refunded, failed)
- `customer` (optional): Filter by customer ID
- `after` (optional): Created after this date (ISO 8601)
- `before` (optional): Created before this date (ISO 8601)
- `per_page` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "List all processing orders from the last week"

---

### woocommerce_get_order

**What it does**: Gets detailed information about a specific order including line items, shipping, and payment details.

**When to use**: Check order details, verify line items, get shipping information.

**Arguments**:
- `id` (required): The order ID

**Example LLM prompt**: "Get details for order 12345"

---

### woocommerce_create_order

**What it does**: Creates a new order in WooCommerce with line items, shipping address, and payment details.

**When to use**: Create orders programmatically, process manual orders.

**Arguments**:
- `payment_method` (optional): Payment method ID
- `billing` (optional): Billing address object
- `shipping` (optional): Shipping address object
- `line_items` (required): Line items array with product_id and quantity
- `status` (optional): Order status (default: pending)

**Example LLM prompt**: "Create an order for customer@example.com with one unit of product 123"

---

### woocommerce_list_products

**What it does**: Lists all products in the WooCommerce store. Supports filtering by category, tag, and status.

**When to use**: Browse catalog, find products by category.

**Arguments**:
- `category` (optional): Filter by category ID
- `tag` (optional): Filter by tag ID
- `status` (optional): Product status (publish, draft, pending, private)
- `per_page` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "List all products in category 5"

---

### woocommerce_get_product

**What it does**: Gets detailed information about a specific product including variations, images, and attributes.

**When to use**: Check full product details, view variations, get inventory.

**Arguments**:
- `id` (required): The product ID

**Example LLM prompt**: "Get details for product 67890"

---

### woocommerce_create_product

**What it does**: Creates a new product in WooCommerce with name, description, prices, and images.

**When to use**: Add new products to the catalog.

**Arguments**:
- `name` (required): Product name
- `type` (optional): Product type (simple, grouped, external, variable)
- `regular_price` (optional): Regular price
- `description` (optional): Product description
- `short_description` (optional): Short description
- `categories` (optional): Categories array with id

**Example LLM prompt**: "Create a new product called 'Blue T-Shirt' with price $29.99"

---

### woocommerce_list_customers

**What it does**: Lists all customers in WooCommerce. Supports filtering by role and date registered.

**When to use**: Browse customer base, find customers by registration date.

**Arguments**:
- `role` (optional): Filter by role (customer, subscriber)
- `after` (optional): Created after this date (ISO 8601)
- `before` (optional): Created before this date (ISO 8601)
- `per_page` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "List all customers created this month"

---

### woocommerce_get_customer

**What it does**: Gets detailed information about a specific customer including orders, addresses, and billing info.

**When to use**: Get customer profile, view order history.

**Arguments**:
- `id` (required): The customer ID

**Example LLM prompt**: "Get details for customer 54321"

---

### woocommerce_list_coupons

**What it does**: Lists all coupons in the WooCommerce store. Supports filtering by discount type.

**When to use**: Browse available coupons, check coupon types.

**Arguments**:
- `discount_type` (optional): Filter by type (percent, fixed_cart, fixed_product)
- `limit` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "List all percentage discount coupons"

---

### woocommerce_get_coupon

**What it does**: Gets detailed information about a specific coupon including usage restrictions and limits.

**When to use**: Check coupon details, usage limits, expiration.

**Arguments**:
- `id` (required): The coupon ID

**Example LLM prompt**: "Get details for coupon SAVE20"

---

## WooCommerce API Notes

- **API Version**: Uses WooCommerce REST API v3
- **Rate Limits**: Varies by hosting environment
- **Stock**: Enable inventory tracking when creating products
- **Images**: Products can have multiple images
- **Categories**: Products can belong to multiple categories
- **Coupons**: Support for percentage, fixed cart, and fixed product discounts
- **Webhooks**: WooCommerce supports webhooks for real-time updates (future expansion)

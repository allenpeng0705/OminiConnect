# BigCommerce Tools

Provider: `bigcommerce` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the BigCommerce API. They allow AI agents to manage products, orders, customers, categories, and coupons in a BigCommerce store. BigCommerce is a leading SaaS e-commerce platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with BigCommerce
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_products`, `create_products`, `read_orders`, `read_customers`, `read_catalog`, `read_marketing`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bigcommerce_list_orders` | List orders | GET | /api/storefront/orders |
| `bigcommerce_get_order` | Get order details | GET | /api/storefront/orders/{order_id} |
| `bigcommerce_list_products` | List products | GET | /api/storefront/products |
| `bigcommerce_get_product` | Get product details | GET | /api/storefront/products/{product_id} |
| `bigcommerce_create_product` | Create a new product | POST | /api/v3/catalog/products |
| `bigcommerce_list_customers` | List customers | GET | /api/storefront/customers |
| `bigcommerce_get_customer` | Get customer details | GET | /api/storefront/customers/{customer_id} |
| `bigcommerce_list_categories` | List categories | GET | /api/storefront/categories |
| `bigcommerce_get_category` | Get category details | GET | /api/storefront/categories/{category_id} |
| `bigcommerce_list_coupons` | List coupons | GET | /api/v3/marketing/coupons |

---

## Tool Details

### bigcommerce_list_orders

**What it does**: Lists all orders in the BigCommerce store. Supports filtering by status, payment method, and date range.

**When to use**: Track orders, find orders needing attention, check financial status.

**Arguments**:
- `status_id` (optional): Filter by order status ID
- `min_date_created` (optional): Created after this date (ISO 8601)
- `max_date_created` (optional): Created before this date (ISO 8601)
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all orders from the last 30 days"

---

### bigcommerce_get_order

**What it does**: Gets detailed information about a specific order including products, shipping, and payment details.

**When to use**: Check order details, verify line items, get shipping information.

**Arguments**:
- `order_id` (required): The order ID

**Example LLM prompt**: "Get details for order 12345"

---

### bigcommerce_list_products

**What it does**: Lists all products in the BigCommerce store. Supports filtering by category, brand, and visibility.

**When to use**: Browse catalog, find products by category or brand.

**Arguments**:
- `category_id` (optional): Filter by category ID
- `brand_id` (optional): Filter by brand ID
- `is_visible` (optional): Filter by visibility
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all visible products in category 10"

---

### bigcommerce_get_product

**What it does**: Gets detailed information about a specific product including variants, images, and custom fields.

**When to use**: Check full product details, view variants, get inventory.

**Arguments**:
- `product_id` (required): The product ID

**Example LLM prompt**: "Get details for product 67890"

---

### bigcommerce_create_product

**What it does**: Creates a new product in BigCommerce with name, description, price, and inventory settings.

**When to use**: Add new products to the catalog.

**Arguments**:
- `name` (required): Product name
- `type` (optional): Product type (physical, digital)
- `price` (optional): Product price
- `description` (optional): Product description
- `sku` (optional): Stock keeping unit
- `inventory_level` (optional): Current inventory level

**Example LLM prompt**: "Create a new product called 'Blue T-Shirt' with price $29.99"

---

### bigcommerce_list_customers

**What it does**: Lists all customers in BigCommerce. Supports filtering by email, company, and date created.

**When to use**: Browse customer base, find customers by email or company.

**Arguments**:
- `email` (optional): Filter by email address
- `company` (optional): Filter by company name
- `min_date_created` (optional): Created after this date (ISO 8601)
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all customers from company Acme Inc"

---

### bigcommerce_get_customer

**What it does**: Gets detailed information about a specific customer including addresses and attributes.

**When to use**: Get customer profile, view addresses.

**Arguments**:
- `customer_id` (required): The customer ID

**Example LLM prompt**: "Get details for customer 54321"

---

### bigcommerce_list_categories

**What it does**: Lists all product categories in BigCommerce. Supports filtering by parent category.

**When to use**: Browse category structure, find subcategories.

**Arguments**:
- `parent_id` (optional): Filter by parent category ID
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all top-level categories"

---

### bigcommerce_get_category

**What it does**: Gets detailed information about a specific category including parent and subcategories.

**When to use**: Get category details, check parent category.

**Arguments**:
- `category_id` (required): The category ID

**Example LLM prompt**: "Get details for category 15"

---

### bigcommerce_list_coupons

**What it does**: Lists all coupons in the BigCommerce store. Supports filtering by type.

**When to use**: Browse available coupons, check coupon types.

**Arguments**:
- `type` (optional): Filter by type (percentage, fixed, free_shipping, fixed_total)
- `limit` (optional): Max results (default 50, max 250)

**Example LLM prompt**: "List all percentage discount coupons"

---

## BigCommerce API Notes

- **API Version**: Uses BigCommerce REST API (Storefront API and Catalog API)
- **Rate Limits**: 250 requests per minute for most endpoints
- **Products**: Support for variants, modifiers, and complex pricing
- **Images**: Products can have multiple images
- **Channels**: Products can be assigned to multiple sales channels
- **Coupons**: Support for percentage, fixed amount, free shipping, and fixed total discounts
- **Webhooks**: BigCommerce supports webhooks for real-time updates (future expansion)

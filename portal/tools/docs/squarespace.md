# Squarespace Tools

Provider: `squarespace` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Squarespace API. They allow AI agents to manage sites, pages, products, orders, and inventory in a Squarespace store. Squarespace is a popular website building and e-commerce platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Squarespace
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `website:read`, `website:write`, `commerce:read`, `commerce:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `squarespace_list_sites` | List all sites | GET | /sites |
| `squarespace_get_site` | Get site details | GET | /sites/{site_id} |
| `squarespace_list_pages` | List pages in a site | GET | /sites/{site_id}/pages |
| `squarespace_get_page` | Get page details | GET | /sites/{site_id}/pages/{page_id} |
| `squarespace_list_products` | List products | GET | /sites/{site_id}/products |
| `squarespace_get_product` | Get product details | GET | /sites/{site_id}/products/{product_id} |
| `squarespace_list_orders` | List orders | GET | /sites/{site_id}/orders |
| `squarespace_get_order` | Get order details | GET | /sites/{site_id}/orders/{order_id} |
| `squarespace_update_inventory` | Update inventory | PATCH | /sites/{site_id}/products/{product_id}/inventory |
| `squarespace_list_inventory` | List inventory | GET | /sites/{site_id}/inventory |

---

## Tool Details

### squarespace_list_sites

**What it does**: Lists all Squarespace websites associated with your account.

**When to use**: View all sites, find sites by pagination.

**Arguments**:
- `limit` (optional): Maximum number of sites to return (default 20, max 100)
- `cursor` (optional): Pagination cursor for retrieving next page of results

**Example LLM prompt**: "List all sites in my Squarespace account"

---

### squarespace_get_site

**What it does**: Gets detailed information about a specific Squarespace website.

**When to use**: Check site settings, view site configuration.

**Arguments**:
- `site_id` (required): Unique identifier of the site

**Example LLM prompt**: "Get details for site abc-123"

---

### squarespace_list_pages

**What it does**: Lists all pages within a Squarespace website including main pages and subpages.

**When to use**: Browse site structure, find specific pages, filter by folder.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `folder_id` (optional): Filter by folder ID

**Example LLM prompt**: "List all pages in site abc-123"

---

### squarespace_get_page

**What it does**: Gets detailed content and settings of a specific page including SEO data.

**When to use**: Get page details, check page content and settings.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `page_id` (required): Unique identifier of the page

**Example LLM prompt**: "Get page details for page xyz-456 in site abc-123"

---

### squarespace_list_products

**What it does**: Lists all products in a Squarespace online store including variants and inventory.

**When to use**: Browse product catalog, check inventory levels.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `limit` (optional): Maximum number of products to return (default 20)
- `cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all products in site abc-123"

---

### squarespace_get_product

**What it does**: Gets detailed information about a specific product including variants, pricing, and media.

**When to use**: Check product details, view variants, check pricing.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `product_id` (required): Unique identifier of the product

**Example LLM prompt**: "Get product details for product prod-789 in site abc-123"

---

### squarespace_list_orders

**What it does**: Lists all orders from a Squarespace commerce store with status filtering.

**When to use**: Track orders, find orders by status, check order history.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `status` (optional): Filter by status (pending, fulfilled, cancelled)
- `limit` (optional): Maximum number of orders to return (default 20)
- `cursor` (optional): Pagination cursor for next page

**Example LLM prompt**: "List all orders with status pending in site abc-123"

---

### squarespace_get_order

**What it does**: Gets detailed information about a specific order including line items and shipping.

**When to use**: Check order details, view customer info, check shipping.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `order_id` (required): Unique identifier of the order

**Example LLM prompt**: "Get order details for order ord-111 in site abc-123"

---

### squarespace_update_inventory

**What it does**: Updates inventory levels for a specific product or variant.

**When to use**: Adjust stock levels after sales, restock items.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `product_id` (required): Unique identifier of the product
- `variant_id` (optional): Variant ID for variant-specific inventory
- `quantity` (required): New inventory quantity

**Example LLM prompt**: "Update inventory for product prod-789 to 50 units in site abc-123"

---

### squarespace_list_inventory

**What it does**: Lists inventory status for all products including low stock alerts.

**When to use**: Check stock levels, find products below threshold.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `low_stock_threshold` (optional): Filter to show only products below this stock level

**Example LLM prompt**: "List all products with inventory below 10 in site abc-123"

---

## Squarespace API Notes

- **API Version**: Uses Squarespace Commerce API
- **Rate Limits**: Varies by plan tier
- **Products**: Support for variants (size, color, etc.)
- **Images**: Products can have multiple images
- **Inventory**: Two ways to update: adjustment (relative) or set (absolute)
- **Pages**: CMS pages are separate from commerce products
- **Webhooks**: Squarespace supports webhooks for real-time updates (future expansion)

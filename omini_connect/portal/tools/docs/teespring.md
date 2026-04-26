# Teespring Provider Documentation

## Overview

Teespring is a merchandise platform that allows you to create, design, and sell custom products. It handles production, shipping, and customer service, making it easy to launch and manage your own branded merchandise store.

## Authentication

Teespring uses API keys for authentication. Include your API key in the `X-API-Key` header:
```
X-API-Key: YOUR_TEESPRING_API_KEY
```

## Base URL

```
https://api.teespring.com
```

## Rate Limits

- Default: 100 requests per minute
- Write operations: 50 requests per minute

## Tools

### Campaigns

#### list_campaigns
Retrieve a list of all marketing campaigns with performance metrics.

**Endpoint:** `GET /v2/campaigns`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of campaigns per page (default 20) |
| status | string | No | Filter by status: draft, published, archived |

#### get_campaign
Retrieve detailed information about a specific campaign including design and settings.

**Endpoint:** `GET /v2/campaigns/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Campaign ID |

#### create_campaign
Create a new campaign with title, design, products, and pricing.

**Endpoint:** `POST /v2/campaigns`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| title | string | Yes | Campaign title |
| design_id | string | No | Design ID to apply to products |
| products | array | Yes | Array of product types and variants to include |
| pricing | object | No | Pricing configuration per product |
| start_date | string | No | Campaign start date (ISO 8601 format) |

---

### Products

#### list_products
Retrieve all products available in the Teespring catalog.

**Endpoint:** `GET /v2/products`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of products per page (default 50) |
| category | string | No | Filter by product category |

#### get_product
Retrieve detailed information about a specific product including variants and sizing.

**Endpoint:** `GET /v2/products/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Product ID |

---

### Orders

#### list_orders
Retrieve orders with filtering by campaign, status, and date range.

**Endpoint:** `GET /v2/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of orders per page (default 50) |
| campaign_id | string | No | Filter by specific campaign ID |
| status | string | No | Filter by status: pending, processing, shipped, delivered, canceled |
| from_date | string | No | Start date filter (ISO 8601 format) |
| to_date | string | No | End date filter (ISO 8601 format) |

#### get_order
Retrieve detailed information about a specific order including items and shipping.

**Endpoint:** `GET /v2/orders/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Order ID |

---

### Designs

#### list_designs
Retrieve all designs uploaded to your Teespring account.

**Endpoint:** `GET /v2/designs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of designs per page (default 50) |

#### get_design
Retrieve detailed information about a specific design including artwork and layers.

**Endpoint:** `GET /v2/designs/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Design ID |

---

### Analytics

#### get_analytics
Retrieve analytics data including sales, revenue, and conversion metrics.

**Endpoint:** `GET /v2/analytics`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| campaign_id | string | No | Specific campaign ID for analytics (optional for global) |
| from_date | string | No | Start date for analytics period (ISO 8601 format) |
| to_date | string | No | End date for analytics period (ISO 8601 format) |
| metrics | array | No | Specific metrics to retrieve: sales, revenue, views, conversions |

## Webhooks

Teespring supports webhooks for the following events:
- `campaign_published`
- `order_created`
- `order_shipped`
- `order_delivered`

## Additional Resources

- [Teespring API Documentation](https://developers.teespring.com/)
- [Product Catalog](https://teespring.com/catalog)
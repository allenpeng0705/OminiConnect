# Redbubble Provider Documentation

## Overview

Redbubble is an artwork marketplace where artists can upload their designs and sell them on various products like t-shirts, stickers, phone cases, and home decor. Redbubble handles printing, shipping, and customer service while you earn royalties on each sale.

## Authentication

Redbubble uses OAuth 2.0 for authentication. Include the access token in the `Authorization` header:
```
Authorization: Bearer YOUR_REDBUBBLE_ACCESS_TOKEN
```

## Base URL

```
https://api.redbubble.com
```

## Rate Limits

- Default: 100 requests per minute
- Write operations: 20 requests per minute

## Tools

### Works

#### list_works
Retrieve all artwork works uploaded to your Redbubble account.

**Endpoint:** `GET /v2/works`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| per_page | integer | No | Number of works per page (default 25, max 100) |
| tags | string | No | Filter works by tags (comma-separated) |

#### get_work
Retrieve detailed information about a specific artwork work.

**Endpoint:** `GET /v2/works/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Work ID |

#### upload_work
Upload a new artwork file to your Redbubble account.

**Endpoint:** `POST /v2/works`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| title | string | Yes | Title for the artwork |
| file | string | Yes | Base64-encoded image file or URL to file |
| tags | array | No | Array of tags for discoverability |
| description | string | No | Description of the artwork |

---

### Products

#### list_products
Retrieve all product types available in the Redbubble catalog.

**Endpoint:** `GET /v2/products`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| per_page | integer | No | Number of products per page (default 50) |

#### get_product
Retrieve detailed information about a specific product type.

**Endpoint:** `GET /v2/products/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Product ID |

---

### Orders

#### list_orders
Retrieve orders with filtering by status and date range.

**Endpoint:** `GET /v2/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| per_page | integer | No | Number of orders per page (default 50) |
| status | string | No | Filter by status: pending, production, shipped, delivered, returned |
| from_date | string | No | Start date filter (ISO 8601 format) |
| to_date | string | No | End date filter (ISO 8601 format) |

#### get_order
Retrieve detailed information about a specific order.

**Endpoint:** `GET /v2/orders/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Order ID |

---

### Statistics

#### get_sales_stats
Retrieve sales statistics and revenue data for your account or specific works.

**Endpoint:** `GET /v2/stats/sales`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| from_date | string | No | Start date for stats period (ISO 8601 format) |
| to_date | string | No | End date for stats period (ISO 8601 format) |
| work_id | string | No | Filter stats for specific work ID |
| group_by | string | No | Group results by: day, month, work, product |

---

### Collections

#### list_collections
Retrieve all collections of works organized on your account.

**Endpoint:** `GET /v2/collections`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| per_page | integer | No | Number of collections per page (default 25) |

#### get_collection
Retrieve detailed information about a specific collection.

**Endpoint:** `GET /v2/collections/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Collection ID |

## Webhooks

Redbubble supports webhooks for the following events:
- `work_uploaded`
- `work_updated`
- `order_created`
- `order_shipped`
- `order_completed`

## Additional Resources

- [Redbubble API Documentation](https://developers.redbubble.com/)
- [Artist Resources](https://www.redbubble.com/help/artists)
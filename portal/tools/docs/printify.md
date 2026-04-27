# Printify Provider Documentation

## Overview

Printify is a print-on-demand platform that connects you with a network of print providers worldwide. It allows you to create custom products and automatically route orders to the best-suited provider based on location and capabilities.

## Authentication

Printify uses OAuth 2.0 for authentication. Include the access token in the `Authorization` header:
```
Authorization: Bearer YOUR_PRINTIFY_ACCESS_TOKEN
```

## Base URL

```
https://api.printify.com
```

## Rate Limits

- Default: 300 requests per minute
- Burst: 600 requests per minute for short periods

## Tools

### Products

#### list_products
Retrieve a list of all products in your Printify catalog with variant details.

**Endpoint:** `GET /v1/catalog/products`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of products per page (default 50, max 100) |
| search | string | No | Search term to filter products by name |

#### get_product
Retrieve detailed information about a specific product including variants and blueprints.

**Endpoint:** `GET /v1/catalog/products/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Product ID |

---

### Orders

#### list_orders
Retrieve a list of orders with filtering options for status and date range.

**Endpoint:** `GET /v1/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of orders per page (default 50, max 100) |
| status | string | No | Filter by status: pending, printing, shipped, delivered, canceled |
| from_date | string | No | Start date filter (ISO 8601 format) |
| to_date | string | No | End date filter (ISO 8601 format) |

#### get_order
Retrieve detailed information about a specific order including line items and shipping.

**Endpoint:** `GET /v1/orders/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Order ID |

#### create_order
Create a new order by specifying items, shipping address, and print provider.

**Endpoint:** `POST /v1/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| line_items | array | Yes | Array of items with product_id, variant_id, and quantity |
| shipping_address | object | Yes | Shipping address details |
| shipping_provider | string | No | Shipping provider ID |
| external_id | string | No | External order reference ID |

---

### Shipments

#### list_shipments
Retrieve a list of shipments with tracking information for shipped orders.

**Endpoint:** `GET /v1/shipments`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of shipments per page (default 50) |

#### get_shipment
Retrieve detailed tracking information for a specific shipment.

**Endpoint:** `GET /v1/shipments/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Shipment ID |

---

### Returns

#### list_returns
Retrieve a list of return requests with their status and details.

**Endpoint:** `GET /v1/returns`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of returns per page (default 50) |
| status | string | No | Filter by status: pending, approved, rejected, received |

#### get_return
Retrieve detailed information about a specific return request.

**Endpoint:** `GET /v1/returns/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Return ID |

---

### Catalog

#### get_product_catalog
Browse the full Printify product catalog with blueprints and available providers.

**Endpoint:** `GET /v1/catalog`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer | No | Page number for pagination (default 1) |
| limit | integer | No | Number of products per page (default 50) |
| category | string | No | Filter by product category (e.g., t-shirts, hoodies) |

## Webhooks

Printify supports webhooks for the following events:
- `order_created`
- `order_status_changed`
- `shipment_created`
- `return_created`

## Additional Resources

- [Printify API Documentation](https://developers.printify.com/)
- [Provider Network Map](https://printify.com/providers)
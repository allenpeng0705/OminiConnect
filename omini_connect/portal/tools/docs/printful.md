# Printful Provider Documentation

## Overview

Printful is a print-on-demand platform that allows you to create and sell custom branded products without upfront inventory costs. Products are printed and shipped on-demand when orders come in.

## Authentication

Printful uses API tokens for authentication. Include your API token in the `Authorization` header:
```
Authorization: Bearer YOUR_PRINTFUL_API_TOKEN
```

## Base URL

```
https://api.printful.com
```

## Rate Limits

- Default: 200 requests per minute
- Webhook endpoints: 100 requests per minute

## Tools

### Products

#### list_products
Retrieve a paginated list of all products in your Printful catalog.

**Endpoint:** `GET /v2/products`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Number of products to return (default 20, max 100) |
| offset | integer | No | Number of products to skip for pagination |
| status | string | No | Filter by product status: active, archived |

#### get_product
Retrieve detailed information about a specific product by ID.

**Endpoint:** `GET /v2/products/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Product ID |

---

### Orders

#### list_orders
Retrieve a list of orders with optional filtering by status and date range.

**Endpoint:** `GET /v2/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Number of orders to return (default 20, max 100) |
| offset | integer | No | Number of orders to skip for pagination |
| status | string | No | Filter by status: pending, processing, on_hold, shipped, canceled |
| from | string | No | Start date filter (ISO 8601 format) |
| to | string | No | End date filter (ISO 8601 format) |

#### get_order
Retrieve detailed information about a specific order by ID.

**Endpoint:** `GET /v2/orders/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Order ID |

#### create_order
Create a new order with recipient info, items, and shipping option.

**Endpoint:** `POST /v2/orders`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| recipient | object | Yes | Recipient information (name, address, email, phone) |
| items | array | Yes | Array of items to order with variant ID and quantity |
| shipping | string | No | Shipping method identifier |
| external_id | string | No | External order reference ID |

---

### Files

#### list_files
Retrieve a list of all uploaded files (designs, images) in your account.

**Endpoint:** `GET /v2/files`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Number of files to return (default 20, max 100) |
| offset | integer | No | Number of files to skip for pagination |

#### get_file
Retrieve detailed information about a specific file by ID.

**Endpoint:** `GET /v2/files/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | File ID |

---

### Shipping

#### list_shipping_rates
Calculate available shipping options for a given address and items.

**Endpoint:** `GET /v2/shipping_rates`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | object | Yes | Recipient address (name, address1, city, state_code, country_code, zip) |
| items | array | Yes | Array of items with variant_id and quantity |

#### get_shipping_rate
Get specific shipping rate details for a particular shipping option.

**Endpoint:** `GET /v2/shipping_rates/{id}`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | Yes | Shipping rate ID |

---

### Inventory

#### list_inventory
Retrieve inventory levels for all products and variants.

**Endpoint:** `GET /v2/inventory`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| limit | integer | No | Number of inventory records to return (default 20, max 100) |
| offset | integer | No | Number of records to skip for pagination |
| product_id | string | No | Filter by specific product ID |

## Webhooks

Printful supports webhooks for the following events:
- `order_created`
- `order_updated`
- `order_canceled`
- `order_shipped`
- `product_updated`
- `file_updated`

## Additional Resources

- [Printful API Documentation](https://www.printful.com/docs/api)
- [API Status](https://status.printful.com/)
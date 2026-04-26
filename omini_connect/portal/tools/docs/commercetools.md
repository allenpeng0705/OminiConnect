# commercetools Integration

## Overview

commercetools is a composable e-commerce platform that provides APIs for managing products, orders, customers, carts, and channels. This integration enables AI agents to interact with the commercetools Composable Commerce API.

## Authentication

commercetools uses OAuth 2.0 client credentials authentication. Configure the following credentials:

- **Client ID**: OAuth client identifier
- **Client Secret**: OAuth client secret
- **Project Key**: The project key for your commercetools instance
- **API URL**: Your commercetools API endpoint (e.g., `https://api.commercetools.com`)

## Available Tools

### Products

| Tool | Description |
|------|-------------|
| `commercetools_list_products` | List all products in a project with optional filtering |
| `commercetools_get_product` | Get details of a specific product by ID |
| `commercetools_create_product` | Create a new product with name, description, and variants |

### Orders

| Tool | Description |
|------|-------------|
| `commercetools_list_orders` | List all orders with optional filtering by customer, state, or date |
| `commercetools_get_order` | Get details of a specific order by ID |

### Customers

| Tool | Description |
|------|-------------|
| `commercetools_list_customers` | List all customers with optional filtering |
| `commercetools_get_customer` | Get details of a specific customer by ID |

### Carts

| Tool | Description |
|------|-------------|
| `commercetools_list_carts` | List all carts with optional filtering |
| `commercetools_get_cart` | Get details of a specific cart by ID |

### Channels

| Tool | Description |
|------|-------------|
| `commercetools_list_channels` | List all distribution channels |

## API Base URL

```
https://api.commercetools.com/{projectKey}
```

## Common Use Cases

### Listing Products

```json
{
  "projectKey": "my-project",
  "limit": 20,
  "offset": 0,
  "where": "categories(id=\"cat-123\")",
  "sort": "name.en asc"
}
```

### Getting an Order

```json
{
  "projectKey": "my-project",
  "id": "order-abc-123"
}
```

### Creating a Product

```json
{
  "projectKey": "my-project",
  "name": {"en": "Premium Widget"},
  "slug": {"en": "premium-widget"},
  "productType": {"id": "product-type-id"},
  "description": {"en": "A high-quality premium widget"}
}
```

## Scopes

The following OAuth scopes are used by these tools:

- `view_products` - Read product data
- `manage_products` - Create and modify products
- `view_orders` - Read order data
- `view_customers` - Read customer data
- `view_carts` - Read cart data
- `view_channels` - Read channel data

## Rate Limits

commercetools implements rate limiting on API requests. The integration handles pagination automatically to stay within rate limits.

## Error Handling

Common error responses:

- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Invalid or expired OAuth token
- `403 Forbidden` - Insufficient permissions for the requested operation
- `404 Not Found` - The requested resource does not exist
- `429 Too Many Requests` - Rate limit exceeded

## Resources

- [commercetools API Documentation](https://docs.commercetools.com/api/)
- [commercetools Authentication](https://docs.commercetools.com/api/authorization)
- [Commercetools Composable Commerce](https://commercetools.com/)

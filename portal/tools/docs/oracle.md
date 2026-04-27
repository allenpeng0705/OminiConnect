# Oracle Integration

Oracle Cloud ERP provides comprehensive enterprise resource planning capabilities including customer management, order processing, product inventory, invoicing, and shipment tracking.

## Authentication

Oracle Cloud uses OAuth 2.0 for authentication. Configure the following credentials:

- **Client ID**: Your Oracle Cloud application client ID
- **Client Secret**: Your Oracle Cloud application client secret
- **Tenant ID**: Your Oracle Cloud tenant identifier
- **Base URL**: `https://{tenantId}.oraclecloud.com`

## API Version

Current API version: v4

## Available Tools

### Customers

| Tool | Name | Description |
|------|------|-------------|
| `oracle_list_customers` | List Customers | Retrieve a list of all customers in the Oracle ERP system |
| `oracle_get_customer` | Get Customer | Retrieve detailed information for a specific customer by ID |

### Orders

| Tool | Name | Description |
|------|------|-------------|
| `oracle_list_orders` | List Orders | Retrieve a list of all orders in the Oracle ERP system |
| `oracle_get_order` | Get Order | Retrieve detailed information for a specific order by ID |
| `oracle_create_order` | Create Order | Create a new order in the Oracle ERP system |

### Products

| Tool | Name | Description |
|------|------|-------------|
| `oracle_list_products` | List Products | Retrieve a list of all products in the Oracle ERP catalog |
| `oracle_get_product` | Get Product | Retrieve detailed information for a specific product by ID |

### Invoices

| Tool | Name | Description |
|------|------|-------------|
| `oracle_list_invoices` | List Invoices | Retrieve a list of all invoices in the Oracle ERP system |
| `oracle_get_invoice` | Get Invoice | Retrieve detailed information for a specific invoice by ID |

### Shipments

| Tool | Name | Description |
|------|------|-------------|
| `oracle_list_shipments` | List Shipments | Retrieve a list of all shipments in the Oracle ERP system |

## Rate Limits

- Default rate limit: 1000 requests per minute
- Burst limit: 100 requests per second

## Error Handling

Oracle API errors return standard HTTP status codes with error details in the response body:

| Status Code | Meaning |
|-------------|---------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or expired credentials |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

## Example Usage

### List Customers

```json
{
  "tool": "oracle_list_customers",
  "parameters": {
    "limit": 50,
    "search": "acme"
  }
}
```

### Create Order

```json
{
  "tool": "oracle_create_order",
  "parameters": {
    "customer_id": "CUST-12345",
    "line_items": [
      {
        "product_id": "PROD-001",
        "quantity": 5,
        "unit_price": 29.99
      }
    ],
    "notes": "Urgent delivery requested"
  }
}
```

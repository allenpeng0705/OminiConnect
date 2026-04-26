# Sap-odata-basic Tools

Provider: `sap-odata-basic` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

SAP OData Basic is an OData API for SAP systems with basic authentication. These tools allow AI agents to manage products, orders, and customers.

## Authentication

**Nango Basic Auth**:
- User authenticates via Nango Connect with SAP OData Basic
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `products:read`, `products:write`, `orders:read`, `orders:write`, `customers:read`, `customers:write`, `metadata:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-odata-basic_list_products` | List all products | GET | /v1/products |
| `sap-odata-basic_get_product` | Get product details | GET | /v1/products/{productId} |
| `sap-odata-basic_create_product` | Create a product | POST | /v1/products |
| `sap-odata-basic_list_orders` | List all orders | GET | /v1/orders |
| `sap-odata-basic_get_order` | Get order details | GET | /v1/orders/{orderId} |
| `sap-odata-basic_create_order` | Create an order | POST | /v1/orders |
| `sap-odata-basic_list_customers` | List all customers | GET | /v1/customers |
| `sap-odata-basic_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `sap-odata-basic_create_customer` | Create a customer | POST | /v1/customers |
| `sap-odata-basic_get_metadata` | Get service metadata | GET | /$metadata |

---

## Tool Details

### sap-odata-basic_list_products

**What it does**: Returns a list of all products.

**When to use**: View product catalog.

**Arguments**:
- `limit` (optional): Number of products (default 50)
- `category` (optional): Filter by category

**Example LLM prompt**: "List all products"

---

### sap-odata-basic_get_product

**What it does**: Gets details of a specific product.

**When to use**: Get product information.

**Arguments**:
- `productId` (required): The product ID

**Example LLM prompt**: "Get details for product prod_abc123"

---

### sap-odata-basic_create_product

**What it does**: Creates a new product.

**When to use**: Add new products to catalog.

**Arguments**:
- `name` (required): Product name
- `description` (optional): Product description
- `price` (optional): Product price
- `category` (optional): Product category

**Example LLM prompt**: "Create a product called 'Widget' for $29.99"

---

### sap-odata-basic_list_orders

**What it does**: Returns a list of all orders.

**When to use**: View order history.

**Arguments**:
- `limit` (optional): Number of orders (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending orders"

---

### sap-odata-basic_get_order

**What it does**: Gets details of a specific order.

**When to use**: Get order information.

**Arguments**:
- `orderId` (required): The order ID

**Example LLM prompt**: "Get details for order ord_xyz789"

---

### sap-odata-basic_create_order

**What it does**: Creates a new order.

**When to use**: Place new orders.

**Arguments**:
- `customerId` (required): Customer ID
- `orderDate` (optional): Order date
- `lines` (optional): Order lines

**Example LLM prompt**: "Create an order for customer cust_123"

---

### sap-odata-basic_list_customers

**What it does**: Returns a list of all customers.

**When to use**: View customer directory.

**Arguments**:
- `limit` (optional): Number of customers (default 50)

**Example LLM prompt**: "List all customers"

---

### sap-odata-basic_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Get customer information.

**Arguments**:
- `customerId` (required): The customer ID

**Example LLM prompt**: "Get details for customer cust_abc123"

---

### sap-odata-basic_create_customer

**What it does**: Creates a new customer.

**When to use**: Add new customers.

**Arguments**:
- `name` (required): Customer name
- `email` (optional): Customer email
- `phone` (optional): Customer phone

**Example LLM prompt**: "Create a customer called 'Acme Corp' with email info@acme.com"

---

### sap-odata-basic_get_metadata

**What it does**: Returns OData service metadata.

**When to use**: Understand available entities.

**Arguments**: None

**Example LLM prompt**: "Get the service metadata"

---

## SAP OData Basic Notes

- Products are items in the catalog
- Orders track customer purchases
- Customers are business entities
- OData metadata describes the service schema

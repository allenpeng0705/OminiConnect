# Sap-odata-oauth2-cc Tools

Provider: `sap-odata-oauth2-cc` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

SAP OData OAuth2 Client Credentials is an OData API for SAP systems with OAuth2 client credentials authentication. These tools allow AI agents to manage products, orders, and customers.

## Authentication

**Nango OAuth2 Client Credentials**:
- Application authenticates via Nango with OAuth2 client credentials flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `products:read`, `products:write`, `orders:read`, `orders:write`, `customers:read`, `customers:write`, `metadata:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-odata-oauth2-cc_list_products` | List all products | GET | /v1/products |
| `sap-odata-oauth2-cc_get_product` | Get product details | GET | /v1/products/{productId} |
| `sap-odata-oauth2-cc_create_product` | Create a product | POST | /v1/products |
| `sap-odata-oauth2-cc_list_orders` | List all orders | GET | /v1/orders |
| `sap-odata-oauth2-cc_get_order` | Get order details | GET | /v1/orders/{orderId} |
| `sap-odata-oauth2-cc_create_order` | Create an order | POST | /v1/orders |
| `sap-odata-oauth2-cc_list_customers` | List all customers | GET | /v1/customers |
| `sap-odata-oauth2-cc_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `sap-odata-oauth2-cc_create_customer` | Create a customer | POST | /v1/customers |
| `sap-odata-oauth2-cc_get_metadata` | Get service metadata | GET | /$metadata |

---

## Tool Details

### sap-odata-oauth2-cc_list_products

**What it does**: Returns a list of all products.

**When to use**: View product catalog.

**Arguments**:
- `limit` (optional): Number of products (default 50)
- `category` (optional): Filter by category

**Example LLM prompt**: "List all products"

---

### sap-odata-oauth2-cc_get_product

**What it does**: Gets details of a specific product.

**When to use**: Get product information.

**Arguments**:
- `productId` (required): The product ID

**Example LLM prompt**: "Get details for product prod_abc123"

---

### sap-odata-oauth2-cc_create_product

**What it does**: Creates a new product.

**When to use**: Add new products to catalog.

**Arguments**:
- `name` (required): Product name
- `description` (optional): Product description
- `price` (optional): Product price
- `category` (optional): Product category

**Example LLM prompt**: "Create a product called 'Widget' for $29.99"

---

### sap-odata-oauth2-cc_list_orders

**What it does**: Returns a list of all orders.

**When to use**: View order history.

**Arguments**:
- `limit` (optional): Number of orders (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending orders"

---

### sap-odata-oauth2-cc_get_order

**What it does**: Gets details of a specific order.

**When to use**: Get order information.

**Arguments**:
- `orderId` (required): The order ID

**Example LLM prompt**: "Get details for order ord_xyz789"

---

### sap-odata-oauth2-cc_create_order

**What it does**: Creates a new order.

**When to use**: Place new orders.

**Arguments**:
- `customerId` (required): Customer ID
- `orderDate` (optional): Order date
- `lines` (optional): Order lines

**Example LLM prompt**: "Create an order for customer cust_123"

---

### sap-odata-oauth2-cc_list_customers

**What it does**: Returns a list of all customers.

**When to use**: View customer directory.

**Arguments**:
- `limit` (optional): Number of customers (default 50)

**Example LLM prompt**: "List all customers"

---

### sap-odata-oauth2-cc_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Get customer information.

**Arguments**:
- `customerId` (required): The customer ID

**Example LLM prompt**: "Get details for customer cust_abc123"

---

### sap-odata-oauth2-cc_create_customer

**What it does**: Creates a new customer.

**When to use**: Add new customers.

**Arguments**:
- `name` (required): Customer name
- `email` (optional): Customer email
- `phone` (optional): Customer phone

**Example LLM prompt**: "Create a customer called 'Acme Corp' with email info@acme.com"

---

### sap-odata-oauth2-cc_get_metadata

**What it does**: Returns OData service metadata.

**When to use**: Understand available entities.

**Arguments**: None

**Example LLM prompt**: "Get the service metadata"

---

## SAP OData OAuth2 CC Notes

- Products are items in the catalog
- Orders track customer purchases
- Customers are business entities
- OData metadata describes the service schema
- OAuth2 client credentials is machine-to-machine auth

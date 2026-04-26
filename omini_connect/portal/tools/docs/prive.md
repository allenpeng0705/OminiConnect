# Prive Tools

Provider: `prive` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Prive API. They allow AI agents to manage subscriptions, customers, orders, products, and invoices. Prive is an e-commerce subscription platform. **Requires Prive API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://subs.api.tryprive.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `prive_list_subscriptions` | List subscriptions | GET | /subscriptions |
| `prive_get_subscription` | Get subscription details | GET | /subscriptions/{subscriptionId} |
| `prive_list_customers` | List customers | GET | /customers |
| `prive_get_customer` | Get customer details | GET | /customers/{customerId} |
| `prive_list_orders` | List orders | GET | /orders |
| `prive_get_order` | Get order details | GET | /orders/{orderId} |
| `prive_list_products` | List products | GET | /products |
| `prive_get_product` | Get product details | GET | /products/{productId} |
| `prive_list_invoices` | List invoices | GET | /invoices |
| `prive_get_invoice` | Get invoice details | GET | /invoices/{invoiceId} |

---

## Tool Details

### prive_list_subscriptions

**What it does**: Lists all subscriptions.

**When to use**: Browse subscription base.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active subscriptions"

---

### prive_get_subscription

**What it does**: Gets detailed information about a specific subscription.

**When to use**: Get subscription details, billing info.

**Arguments**:
- `subscriptionId` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription 12345"

---

### prive_list_customers

**What it does**: Lists all customers.

**When to use**: Browse customer directory.

**Arguments**: None

**Example LLM prompt**: "List all customers"

---

### prive_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: Get customer profile, order history.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer 67890"

---

### prive_list_orders

**What it does**: Lists all orders.

**When to use**: Browse order history.

**Arguments**:
- `status` (optional): Filter by status
- `customerId` (optional): Filter by customer

**Example LLM prompt**: "List all orders for customer 67890"

---

### prive_get_order

**What it does**: Gets detailed information about a specific order.

**When to use**: Get order details, items.

**Arguments**:
- `orderId` (required): Order ID

**Example LLM prompt**: "Get details for order 11111"

---

### prive_list_products

**What it does**: Lists all products.

**When to use**: Browse product catalog.

**Arguments**: None

**Example LLM prompt**: "List all products"

---

### prive_get_product

**What it does**: Gets detailed information about a specific product.

**When to use**: Get product details, pricing.

**Arguments**:
- `productId` (required): Product ID

**Example LLM prompt**: "Get details for product 22222"

---

### prive_list_invoices

**What it does**: Lists all invoices.

**When to use**: Browse invoice history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all invoices"

---

### prive_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: Get invoice details.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-33333"

---

## Prive API Notes

- **API Key**: Uses Bearer token for authentication
- **E-commerce**: Subscription and order management
- **Rate limits**: Apply to API calls

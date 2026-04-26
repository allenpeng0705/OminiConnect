# Gumroad Tools

Provider: `gumroad` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Gumroad API. They allow AI agents to manage products, sales, customers, subscribers, and webhooks. Gumroad is an e-commerce platform for digital products and subscriptions.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Gumroad
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://gumroad.com/oauth/authorize
- Token URL: https://api.gumroad.com/oauth/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gumroad_list_products` | List products | GET | /v1/products |
| `gumroad_get_product` | Get product details | GET | /v1/products/{product_id} |
| `gumroad_list_sales` | List sales | GET | /v1/sales |
| `gumroad_get_sale` | Get sale details | GET | /v1/sales/{sale_id} |
| `gumroad_list_customers` | List customers | GET | /v1/customers |
| `gumroad_get_customer` | Get customer details | GET | /v1/customers/{customer_id} |
| `gumroad_list_subscribers` | List subscribers | GET | /v1/subscribers |
| `gumroad_get_subscriber` | Get subscriber details | GET | /v1/subscribers/{subscriber_id} |
| `gumroad_list_webhooks` | List webhooks | GET | /v1/webhooks |
| `gumroad_create_webhook` | Create a webhook | POST | /v1/webhooks |

---

## Tool Details

### gumroad_list_products

**What it does**: Lists all products in your Gumroad account.

**When to use**: Browse your product catalog.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all products"

---

### gumroad_get_product

**What it does**: Gets detailed information about a specific product.

**When to use**: View product details and sales stats.

**Arguments**:
- `product_id` (required): Product ID

**Example LLM prompt**: "Get product abc123"

---

### gumroad_list_sales

**What it does**: Lists all sales in your Gumroad account.

**When to use**: View sales history and revenue.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all sales for product abc123"

---

### gumroad_get_sale

**What it does**: Gets detailed information about a specific sale.

**When to use**: View sale details and customer info.

**Arguments**:
- `sale_id` (required): Sale ID

**Example LLM prompt**: "Get sale xyz789"

---

### gumroad_list_customers

**What it does**: Lists all customers in your Gumroad account.

**When to use**: Browse customer list.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all customers"

---

### gumroad_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: View customer profile and purchase history.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get customer def456"

---

### gumroad_list_subscribers

**What it does**: Lists all subscribers for your Gumroad products.

**When to use**: View subscription status.

**Arguments**:
- `product_id` (optional): Filter by product ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all subscribers"

---

### gumroad_get_subscriber

**What it does**: Gets detailed information about a specific subscriber.

**When to use**: View subscription details and billing.

**Arguments**:
- `subscriber_id` (required): Subscriber ID

**Example LLM prompt**: "Get subscriber ghi789"

---

### gumroad_list_webhooks

**What it does**: Lists all configured webhooks in your Gumroad account.

**When to use**: View webhook configuration.

**Arguments**: None

**Example LLM prompt**: "List all webhooks"

---

### gumroad_create_webhook

**What it does**: Creates a new webhook for Gumroad events.

**When to use**: Set up event notifications.

**Arguments**:
- `url` (required): Webhook URL
- `events` (optional): Event types to subscribe

**Example LLM prompt**: "Create a webhook for new sales"

---

## Gumroad API Notes

- **API Base URL**: https://api.gumroad.com
- **Auth Mode**: OAuth2
- **Products**: Digital products or subscriptions
- **Sales**: One-time purchases
- **Customers**: Buyers with purchase history
- **Subscribers**: Recurring subscription holders
- **Webhooks**: Event notifications for sales, subscriptions, etc.

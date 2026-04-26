# Recharge Tools

Provider: `recharge` | Engine: `nango` | Auth: OAuth/API_KEY via Nango

## Overview

These tools wrap the Recharge API. They allow AI agents to manage subscriptions, customers, orders, and products for subscription-based businesses. Recharge is a leading subscription billing platform for Shopify and other e-commerce platforms.

## Authentication

**Nango OAuth or API Key**:
- User authenticates via Nango Connect with Recharge
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_subscriptions`, `write_subscriptions`, `read_customers`, `write_customers`, `read_products`, `read_orders`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recharge_list_subscriptions` | List subscriptions | GET | /subscriptions |
| `recharge_get_subscription` | Get subscription details | GET | /subscriptions/{subscription_id} |
| `recharge_create_subscription` | Create a subscription | POST | /subscriptions |
| `recharge_update_subscription` | Update a subscription | PUT | /subscriptions/{subscription_id} |
| `recharge_cancel_subscription` | Cancel a subscription | POST | /subscriptions/{subscription_id}/cancel |
| `recharge_list_customers` | List customers | GET | /customers |
| `recharge_get_customer` | Get customer details | GET | /customers/{customer_id} |
| `recharge_list_orders` | List orders | GET | /orders |
| `recharge_get_order` | Get order details | GET | /orders/{order_id} |
| `recharge_list_products` | List products | GET | /products |

---

## Tool Details

### recharge_list_subscriptions

**What it does**: Lists all subscriptions with optional filtering by customer, status, or product.

**When to use**: Find active subscriptions, see paused accounts, check subscription counts.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `cursor` (optional): Pagination cursor for next page
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (`active`, `paused`, `canceled`)

**Example LLM prompt**: "List all active subscriptions for customer ABC123"

---

### recharge_get_subscription

**What it does**: Gets detailed information about a specific subscription.

**When to use**: Check subscription details before updating or canceling.

**Arguments**:
- `subscription_id` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription SUB-456"

---

### recharge_create_subscription

**What it does**: Creates a new subscription for a customer.

**When to use**: Set up new recurring orders, start subscription plans.

**Arguments**:
- `customer_id` (required): Customer ID
- `address_id` (required): Shipping address ID
- `product_id` (required): Product ID
- `variant_id` (optional): Variant ID
- `quantity` (optional): Number of units
- `charge_interval_frequency` (optional): Days between charges
- `order_interval_frequency` (optional): Days between orders
- `next_charge_date` (optional): Next charge date (YYYY-MM-DD)

**Example LLM prompt**: "Create a subscription for customer ABC123 with product PROD-001"

---

### recharge_update_subscription

**What it does**: Updates an existing subscription's details.

**When to use**: Change quantity, pause/resume, update next charge date.

**Arguments**:
- `subscription_id` (required): Subscription ID
- `quantity` (optional): New quantity
- `status` (optional): New status (`active`, `paused`, `canceled`)
- `next_charge_date` (optional): New next charge date
- `charge_interval_frequency` (optional): New charge interval

**Example LLM prompt**: "Update subscription SUB-456 to have quantity of 3"

---

### recharge_cancel_subscription

**What it does**: Cancels an existing subscription.

**When to use**: Customer requests cancellation, failed payments.

**Arguments**:
- `subscription_id` (required): Subscription ID
- `cancellation_email` (optional): Send cancellation email (boolean)
- `reason` (optional): Cancellation reason

**Example LLM prompt**: "Cancel subscription SUB-456"

---

### recharge_list_customers

**What it does**: Lists all customers in the Recharge account.

**When to use**: Find customers, view customer base, search by email.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `cursor` (optional): Pagination cursor
- `email` (optional): Filter by email address

**Example LLM prompt**: "List all customers with email containing @example.com"

---

### recharge_get_customer

**What it does**: Gets detailed information about a specific customer including their subscriptions and payment methods.

**When to use**: Check customer details, view subscription history.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer CUST-789"

---

### recharge_list_orders

**What it does**: Lists all orders with optional filtering by customer, status, or date range.

**When to use**: Track orders, view order history, find orders by status.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `cursor` (optional): Pagination cursor
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (`pending`, `paid`, etc.)

**Example LLM prompt**: "List all paid orders from the last week"

---

### recharge_get_order

**What it does**: Gets detailed information about a specific order including line items and shipping info.

**When to use**: Check order details, verify order contents.

**Arguments**:
- `order_id` (required): Order ID

**Example LLM prompt**: "Get details for order ORD-123"

---

### recharge_list_products

**What it does**: Lists all products in the Recharge product catalog.

**When to use**: Browse available products, check catalog for subscription offerings.

**Arguments**:
- `limit` (optional): Max results (default 100)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all available products"

---

## Recharge API Notes

- **Subscriptions**: Core entity linking customers to products with recurring billing
- **Customers**: Can have multiple addresses and payment methods
- **Products**: Can have multiple variants with different prices
- **Orders**: Generated automatically from subscriptions at charge time
- **Webhooks**: Recharge supports webhooks for subscription lifecycle events (future expansion)

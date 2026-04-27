# Stripe Additional Tools

Provider: `stripe_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Stripe API for invoices, subscriptions, customers, and payment methods. They enable AI agents to manage billing, subscriptions, and customer payment data for e-commerce and SaaS platforms.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Stripe
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `invoices:read`, `invoices:write`, `subscriptions:read`, `subscriptions:write`, `customers:read`, `customers:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `stripe_more_list_invoices` | List all invoices | GET | /v1/invoices |
| `stripe_more_get_invoice` | Get invoice details | GET | /v1/invoices/{id} |
| `stripe_more_create_invoice` | Create a new invoice | POST | /v1/invoices |
| `stripe_more_list_subscriptions` | List all subscriptions | GET | /v1/subscriptions |
| `stripe_more_get_subscription` | Get subscription details | GET | /v1/subscriptions/{id} |
| `stripe_more_create_subscription` | Create a new subscription | POST | /v1/subscriptions |
| `stripe_more_list_customers` | List all customers | GET | /v1/customers |
| `stripe_more_get_customer` | Get customer details | GET | /v1/customers/{id} |
| `stripe_more_create_customer` | Create a new customer | POST | /v1/customers |
| `stripe_more_list_payment_methods` | List payment methods | GET | /v1/payment_methods |

---

## Tool Details

### stripe_more_list_invoices

**What it does**: Retrieves all invoices from Stripe with optional filtering by customer, status, or due date.

**When to use**: Get an overview of all invoices for billing reconciliation or customer follow-up.

**Arguments**:
- `customer` (optional): Filter by customer ID
- `status` (optional): Filter by status (draft, open, paid, void)
- `due_date_from` (optional): Due date from (Unix timestamp)
- `due_date_to` (optional): Due date to (Unix timestamp)
- `limit` (optional): Number of results (default 100)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all open invoices for customer cus_123"

---

### stripe_more_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items and payment status.

**When to use**: Review invoice details before sending or following up on payment.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice inv_456"

---

### stripe_more_create_invoice

**What it does**: Creates a new invoice in Stripe with customer, items, and billing details.

**When to use**: Create an invoice for a customer subscription or one-time charge.

**Arguments**:
- `customer` (required): Customer ID
- `collection_method` (optional): Collection method (charge_automatically, send_invoice)
- `days_until_due` (optional): Days until due for send_invoice
- `description` (optional): Invoice description
- `metadata` (optional): Metadata key-value pairs

**Example LLM prompt**: "Create an invoice for customer cus_123 for $500 due in 30 days"

---

### stripe_more_list_subscriptions

**What it does**: Retrieves all subscriptions with optional filtering by customer, status, or plan.

**When to use**: Track active subscriptions and revenue.

**Arguments**:
- `customer` (optional): Filter by customer ID
- `status` (optional): Filter by status (active, trialing, past_due, canceled)
- `price` (optional): Filter by price ID
- `limit` (optional): Number of results (default 100)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all active subscriptions for customer cus_123"

---

### stripe_more_get_subscription

**What it does**: Gets detailed information about a specific subscription including plan and billing cycle.

**When to use**: Review subscription details before making changes or cancellations.

**Arguments**:
- `id` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription sub_789"

---

### stripe_more_create_subscription

**What it does**: Creates a new subscription for a customer with items, pricing, and trial options.

**When to use**: Set up a new recurring billing relationship.

**Arguments**:
- `customer` (required): Customer ID
- `items` (required): Array of subscription items with price data
- `collection_method` (optional): Collection method (charge_automatically, send_invoice)
- `trial_period_days` (optional): Number of trial days
- `cancel_at` (optional): Cancel at Unix timestamp
- `metadata` (optional): Metadata key-value pairs

**Example LLM prompt**: "Create a subscription for customer cus_123 with price price_abc at $99/month"

---

### stripe_more_list_customers

**What it does**: Retrieves all customers with optional filtering by email, name, or creation date.

**When to use**: Get an overview of all customers for CRM or marketing purposes.

**Arguments**:
- `email` (optional): Filter by email
- `created_from` (optional): Created from (Unix timestamp)
- `created_to` (optional): Created to (Unix timestamp)
- `limit` (optional): Number of results (default 100)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all customers created this month"

---

### stripe_more_get_customer

**What it does**: Gets detailed information about a specific customer including contact info and payment methods.

**When to use**: Review customer details before transactions or updates.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer cus_123"

---

### stripe_more_create_customer

**What it does**: Creates a new customer in Stripe with name, email, and optional description.

**When to use**: Add a new customer for billing or subscription purposes.

**Arguments**:
- `email` (optional): Email address
- `name` (optional): Customer name
- `phone` (optional): Phone number
- `description` (optional): Customer description
- `metadata` (optional): Metadata key-value pairs

**Example LLM prompt**: "Create a new customer for John Doe with email john@example.com"

---

### stripe_more_list_payment_methods

**What it does**: Lists all payment methods for a customer in Stripe.

**When to use**: View stored payment methods for a customer before processing charges.

**Arguments**:
- `customer` (optional): Customer ID to filter by
- `type` (optional): Filter by type (card, us_bank_account, etc)
- `limit` (optional): Number of results (default 100)

**Example LLM prompt**: "List all payment methods for customer cus_123"

---

## Stripe API Notes

- **Billing Model**: Stripe uses invoices for billing cycles, subscriptions for recurring revenue
- **Customer Hierarchy**: Customers can have multiple payment methods and subscriptions
- **Payment Methods**: Support for cards, bank accounts, and other payment types
- **Pagination**: Uses cursor-based pagination with `starting_after`
- **Idempotency**: POST requests support idempotency keys for safe retries
- **Metadata**: Use metadata to store custom data on any object
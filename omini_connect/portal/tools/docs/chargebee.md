# Chargebee Tools

Provider: `chargebee` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Chargebee subscription billing API. They enable AI agents to manage subscriptions, customers, invoices, and plans for subscription-based businesses. Chargebee is a subscription billing and revenue management platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Chargebee
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `subscriptions:read`, `subscriptions:write`, `customers:read`, `invoices:read`, `plans:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `chargebee_list_subscriptions` | List all subscriptions | GET | /subscriptions |
| `chargebee_get_subscription` | Get subscription details | GET | /subscriptions/{id} |
| `chargebee_create_subscription` | Create a new subscription | POST | /subscriptions |
| `chargebee_cancel_subscription` | Cancel a subscription | POST | /subscriptions/{id}/cancel |
| `chargebee_list_customers` | List all customers | GET | /customers |
| `chargebee_get_customer` | Get customer details | GET | /customers/{id} |
| `chargebee_list_invoices` | List all invoices | GET | /invoices |
| `chargebee_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `chargebee_list_plans` | List all plans | GET | /plans |
| `chargebee_get_plan` | Get plan details | GET | /plans/{id} |

---

## Tool Details

### chargebee_list_subscriptions

**What it does**: Retrieves all subscriptions with optional filtering by status, customer, or plan.

**When to use**: Get an overview of all subscriptions, find active or cancelled subscriptions.

**Arguments**:
- `status` (optional): Filter by status (active, cancelled, paused, future, etc)
- `customer_id` (optional): Filter by customer ID
- `plan_id` (optional): Filter by plan ID
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all active subscriptions for plan premium"

---

### chargebee_get_subscription

**What it does**: Gets detailed information about a specific subscription including plan, status, and billing dates.

**When to use**: Review subscription details, check subscription state or terms.

**Arguments**:
- `id` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription sub_123456"

---

### chargebee_create_subscription

**What it does**: Creates a new subscription for a customer with a specific plan.

**When to use**: Onboard new customers, start new subscription plans.

**Arguments**:
- `customer_id` (required): Customer ID
- `plan_id` (required): Plan ID
- `billing_cycles` (optional): Number of billing cycles
- `start_date` (optional): Start date as Unix timestamp
- `trial_end` (optional): Trial end as Unix timestamp
- `auto_collection` (optional): Auto collection (on, off, default on)

**Example LLM prompt**: "Create a subscription for customer cust_789 on the pro plan"

---

### chargebee_cancel_subscription

**What it does**: Cancels an existing subscription with optional timing.

**When to use**: Handle customer cancellations, end subscriptions at term end.

**Arguments**:
- `id` (required): Subscription ID
- `end_of_term` (optional): Cancel at end of current term (default false)
- `cancellation_reason` (optional): Reason for cancellation
- `cancel_at` (optional): Cancel at specific Unix timestamp

**Example LLM prompt**: "Cancel subscription sub_123 at the end of the current term"

---

### chargebee_list_customers

**What it does**: Retrieves all customers with optional filtering by email or company.

**When to use**: Browse customer base, find customers by email or company.

**Arguments**:
- `email` (optional): Filter by email address
- `company` (optional): Filter by company name
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all customers from company Acme Inc"

---

### chargebee_get_customer

**What it does**: Gets detailed information about a specific customer including billing info.

**When to use**: Get customer profile, billing address, payment method details.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer cust_456"

---

### chargebee_list_invoices

**What it does**: Retrieves all invoices with optional filtering by customer, status, or date range.

**When to use**: View billing history, find unpaid invoices, check payment status.

**Arguments**:
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (paid, pending, voided, etc)
- `from_date` (optional): From date as Unix timestamp
- `to_date` (optional): To date as Unix timestamp
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all paid invoices for customer cust_789 from January 2026"

---

### chargebee_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items.

**When to use**: View invoice details, check charges, see tax amounts.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice inv_321"

---

### chargebee_list_plans

**What it does**: Retrieves all plans with optional filtering by status.

**When to use**: View available plans, find plan IDs for subscription creation.

**Arguments**:
- `status` (optional): Filter by status (active, archived)
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all active plans"

---

### chargebee_get_plan

**What it does**: Gets detailed information about a specific plan including pricing.

**When to use**: Check plan details, view pricing and features.

**Arguments**:
- `id` (required): Plan ID

**Example LLM prompt**: "Get details for plan pro_monthly"

---

## Chargebee API Notes

- **Subscription Status**: Common statuses include `active`, `cancelled`, `paused`, `future`, `trial`
- **Invoice Status**: `paid`, `pending`, `voided`, `draft`, `not_sent`
- **Pagination**: Use `limit` and `offset` for paginating through results
- **Billing Cycles**: Specify number of cycles for term-based subscriptions
- **Auto Collection**: Controls whether payment is automatically collected at renewal

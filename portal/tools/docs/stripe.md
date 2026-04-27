# Stripe Tools

Provider: `stripe` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Stripe REST API. They allow AI agents to manage customers, charges, subscriptions, and invoices. Stripe is the dominant payment processor for modern SaaS and e-commerce.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Stripe
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_only`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `stripe_list_charges` | List charges for the account | GET | /v1/charges |
| `stripe_get_charge` | Get details of a specific charge | GET | /v1/charges/{id} |
| `stripe_create_charge` | Create a new charge | POST | /v1/charges |
| `stripe_list_customers` | List customers | GET | /v1/customers |
| `stripe_get_customer` | Get details of a specific customer | GET | /v1/customers/{id} |
| `stripe_create_customer` | Create a new customer | POST | /v1/customers |
| `stripe_list_subscriptions` | List subscriptions | GET | /v1/subscriptions |
| `stripe_get_subscription` | Get details of a specific subscription | GET | /v1/subscriptions/{id} |
| `stripe_list_invoices` | List invoices | GET | /v1/invoices |
| `stripe_get_invoice` | Get details of a specific invoice | GET | /v1/invoices/{id} |

---

## Tool Details

### stripe_list_charges

**What it does**: Returns a paginated list of charges with details like amount, currency, status, and customer.

**When to use**: Review payment history, find charges by customer, or check charge statuses.

**Arguments**:
- `customer` (optional): Filter by customer ID
- `created` (optional): Filter by creation date (gte, lte, gt, lt)
- `limit` (optional): Number of results per page (max 100, default 10)
- `starting_after` (optional): Cursor for pagination (object ID)

**Example LLM prompt**: "List all charges for customer cus_abc123"

---

### stripe_get_charge

**What it does**: Get details of a specific charge including amount, currency, payment method, and status.

**When to use**: Check charge status, verify payment success, or review failure reasons.

**Arguments**:
- `id` (required): Charge ID

**Example LLM prompt**: "Get details for charge ch_xyz789"

---

### stripe_create_charge

**What it does**: Create a new charge for a customer. Specify amount, currency, and optional payment method details.

**When to use**: Process a one-time payment from a customer.

**Arguments**:
- `amount` (required): Amount in smallest currency unit (e.g., cents for USD)
- `currency` (required): Three-letter ISO currency code (default: usd)
- `customer` (optional): Customer ID to charge
- `description` (optional): Description of the charge
- `payment_method` (optional): Payment method ID to use

**Example LLM prompt**: "Create a $50 charge in USD for customer cus_abc123"

---

### stripe_list_customers

**What it does**: Returns a paginated list of customers with email, name, and metadata.

**When to use**: Find existing customers, list recent signups, or search by email.

**Arguments**:
- `email` (optional): Filter by email address
- `created` (optional): Filter by creation date
- `limit` (optional): Number of results per page (max 100, default 10)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all customers with email john@example.com"

---

### stripe_get_customer

**What it does**: Get details of a specific customer including email, name, address, and default payment method.

**When to use**: Check customer profile before creating charges or subscriptions.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer cus_abc123"

---

### stripe_create_customer

**What it does**: Create a new customer with email, name, and optional metadata. Required before recurring billing.

**When to use**: Register new customers before taking payments or setting up subscriptions.

**Arguments**:
- `email` (optional): Customer email address
- `name` (optional): Customer full name
- `phone` (optional): Customer phone number
- `description` (optional): Customer description
- `metadata` (optional): Key-value pairs for custom data

**Example LLM prompt**: "Create a customer for john@example.com named John Smith"

---

### stripe_list_subscriptions

**What it does**: List subscriptions filtered by customer, status, or plan.

**When to use**: Find active subscriptions, see canceled ones, or check for past due.

**Arguments**:
- `customer` (optional): Filter by customer ID
- `status` (optional): Filter by status (active, past_due, canceled, all)
- `limit` (optional): Number of results per page (max 100, default 10)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all active subscriptions for customer cus_abc123"

---

### stripe_get_subscription

**What it does**: Get details of a specific subscription including plan, status, current period, and cancel date.

**When to use**: Check subscription details before upgrading, downgrading, or canceling.

**Arguments**:
- `id` (required): Subscription ID

**Example LLM prompt**: "Get subscription sub_xyz789 details"

---

### stripe_list_invoices

**What it does**: List invoices for the account or a specific customer. Returns pending, open, and paid invoices.

**When to use**: View billing history, find outstanding invoices, or track payment status.

**Arguments**:
- `customer` (optional): Filter by customer ID
- `status` (optional): Filter by status (draft, open, paid, uncollectible, void)
- `limit` (optional): Number of results per page (max 100, default 10)
- `starting_after` (optional): Cursor for pagination

**Example LLM prompt**: "List all open invoices for customer cus_abc123"

---

### stripe_get_invoice

**What it does**: Get details of a specific invoice including line items, amount due, tax, and payment status.

**When to use**: Review invoice details before payment or for customer inquiries.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get invoice inv_xyz789 details"

---

## Stripe API Notes

- **Amounts are in smallest unit**: $10.00 USD = `1000` (cents). EUR 10.00 = `1000` (cents).
- **Currency codes**: Always lowercase three-letter ISO codes (`usd`, `eur`, `gbp`)
- **Charges vs Payment Intents**: Charges are the simpler API; Payment Intents are for more complex flows
- **Customers**: Always create a customer first for subscriptions and recurring payments
- **IDs**: Stripe IDs are prefixed with type (e.g., `cus_...`, `ch_...`, `sub_...`, `inv_...`)
- **Pagination**: Use `starting_after` cursor for pagination through results
- **Test mode**: Use `sk_test_...` keys for testing

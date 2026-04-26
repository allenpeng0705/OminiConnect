# PayPal Tools

Provider: `paypal` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the PayPal API. They enable AI agents to manage orders, payments, subscriptions, and invoices for PayPal merchants.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with PayPal
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_only`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paypal_list_orders` | List orders | GET | /v2/checkout/orders |
| `paypal_get_order` | Get details of a specific order | GET | /v2/checkout/orders/{id} |
| `paypal_create_order` | Create a new order | POST | /v2/checkout/orders |
| `paypal_capture_order` | Capture payment for an order | POST | /v2/checkout/orders/{id}/capture |
| `paypal_list_payments` | List authorized payments | GET | /v2/payments/authorizations |
| `paypal_get_payment` | Get details of a specific payment | GET | /v2/payments/authorizations/{id} |
| `paypal_list_subscriptions` | List subscriptions | GET | /v1/billing/subscriptions |
| `paypal_get_subscription` | Get details of a specific subscription | GET | /v1/billing/subscriptions/{id} |
| `paypal_create_subscription` | Create a new subscription | POST | /v1/billing/subscriptions |
| `paypal_list_invoices` | List invoices | GET | /v2/invoicing/invoices |

---

## Tool Details

### paypal_list_orders

**What it does**: List orders from PayPal. Returns order details with status, amounts, and payer information.

**When to use**: Get an overview of all orders for tracking or reporting.

**Arguments**:
- `page_size` (optional): Number of results per page (max 100, default 100)
- `after` (optional): Cursor for forward pagination
- `before` (optional): Cursor for backward pagination
- `start_time` (optional): Filter by start time (ISO 8601)
- `end_time` (optional): Filter by end time (ISO 8601)

**Example LLM prompt**: "List all orders from the last week"

---

### paypal_get_order

**What it does**: Get details of a specific PayPal order including status, purchase units, and payer info.

**When to use**: Check order status or review order details for customer inquiries.

**Arguments**:
- `id` (required): Order ID

**Example LLM prompt**: "Get details for order 5O190127TN364715T"

---

### paypal_create_order

**What it does**: Create a new PayPal order with amount and intent (CAPTURE or AUTHORIZE). Returns order ID for checkout.

**When to use**: Set up a new payment for processing through PayPal checkout.

**Arguments**:
- `intent` (required): Payment intent: CAPTURE or AUTHORIZE (default CAPTURE)
- `purchase_units` (required): Array of purchase units with amount and description

**Example LLM prompt**: "Create a new order for $100 USD with CAPTURE intent"

---

### paypal_capture_order

**What it does**: Capture payment for an order that was authorized. Completes the payment flow after buyer approval.

**When to use**: Complete a payment after the buyer approves the order on PayPal.

**Arguments**:
- `id` (required): Order ID to capture
- `idempotency_id` (optional): Idempotency ID to prevent duplicate captures

**Example LLM prompt**: "Capture payment for order 5O190127TN364715T"

---

### paypal_list_payments

**What it does**: List authorized payments (not yet captured) for the PayPal account.

**When to use**: Track authorized payments that are pending capture.

**Arguments**:
- `page_size` (optional): Number of results per page (max 100, default 100)
- `start_time` (optional): Filter by start time (ISO 8601)
- `end_time` (optional): Filter by end time (ISO 8601)

**Example LLM prompt**: "List all pending authorized payments"

---

### paypal_get_payment

**What it does**: Get details of a specific authorized payment including status, amount, and payer information.

**When to use**: Review authorized payment details before capturing.

**Arguments**:
- `id` (required): Authorization ID

**Example LLM prompt**: "Get details for authorization 5O190127TN364715T"

---

### paypal_list_subscriptions

**What it does**: List PayPal subscriptions. Returns subscription details with status, plan, and billing info.

**When to use**: Track active subscriptions, view canceled ones, or monitor billing status.

**Arguments**:
- `status` (optional): Filter by status (ACTIVE, SUSPENDED, CANCELED, EXPIRED)
- `plan_id` (optional): Filter by plan ID
- `page_size` (optional): Number of results per page (max 100, default 100)
- `start_time` (optional): Filter by start time (ISO 8601)
- `end_time` (optional): Filter by end time (ISO 8601)

**Example LLM prompt**: "List all active subscriptions"

---

### paypal_get_subscription

**What it does**: Get details of a specific subscription including plan, status, billing cycles, and next billing date.

**When to use**: Check subscription details before upgrading, downgrading, or canceling.

**Arguments**:
- `id` (required): Subscription ID

**Example LLM prompt**: "Get subscription details for I-BW452GLDU68J"

---

### paypal_create_subscription

**What it does**: Create a new PayPal subscription with plan ID and subscriber info. Triggers billing agreement.

**When to use**: Set up recurring billing for a customer.

**Arguments**:
- `plan_id` (required): Subscription plan ID
- `subscriber` (optional): Subscriber object with email_address and name
- `start_time` (optional): Billing cycle start time (ISO 8601)
- `quantity` (optional): Quantity to subscribe
- `application_context` (optional): Application context with return_url and cancel_url

**Example LLM prompt**: "Create a subscription for plan P-5ML4181245553462MN"

---

### paypal_list_invoices

**What it does**: List invoices from PayPal. Returns invoice details with amounts, status, and payment terms.

**When to use**: View billing history, find outstanding invoices, or track payment status.

**Arguments**:
- `page_size` (optional): Number of results per page (max 100, default 100)
- `start_time` (optional): Filter by start time (ISO 8601)
- `end_time` (optional): Filter by end time (ISO 8601)
- `status` (optional): Filter by status (DRAFT, SENT, PAID, MARKED_AS_PAID, CANCELED)

**Example LLM prompt**: "List all sent invoices from this month"

---

## PayPal API Notes

- **Checkout Flow**: Create Order -> Redirect to PayPal -> Buyer Approves -> Capture Order
- **Intent Modes**: CAPTURE (immediate capture) vs AUTHORIZE (capture later)
- **Subscription Plans**: Pre-configured billing plans with pricing and frequency
- **Invoice Statuses**: DRAFT -> SENT -> PAID (or CANCELED)
- **ISO 8601 Dates**: All timestamps use ISO 8601 format
- **Pagination**: Uses cursor-based pagination with after/before cursors

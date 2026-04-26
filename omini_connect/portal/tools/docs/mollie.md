# Mollie Tools

Provider: `mollie` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Mollie API. They allow AI agents to manage payments, refunds, customers, and subscriptions. **Requires Mollie OAuth2.**

## Authentication

**Nango OAUTH2**:
- User authenticates via Nango Connect with Mollie
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.mollie.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mollie_list_payments` | List all payments | GET | /v2/payments |
| `mollie_get_payment` | Get payment details | GET | /v2/payments/{paymentId} |
| `mollie_create_payment` | Create a payment | POST | /v2/payments |
| `mollie_list_refunds` | List refunds | GET | /v2/refunds |
| `mollie_get_refund` | Get refund details | GET | /v2/refunds/{refundId} |
| `mollie_create_refund` | Create a refund | POST | /v2/payments/{paymentId}/refunds |
| `mollie_list_customers` | List customers | GET | /v2/customers |
| `mollie_get_customer` | Get customer details | GET | /v2/customers/{customerId} |
| `mollie_list_subscriptions` | List subscriptions | GET | /v2/customers/{customerId}/subscriptions |
| `mollie_get_subscription` | Get subscription details | GET | /v2/customers/{customerId}/subscriptions/{subscriptionId} |

---

## Tool Details

### mollie_list_payments

**What it does**: Lists all payments in Mollie.

**When to use**: View payment history, find payments.

**Arguments**:
- `status` (optional): Filter by status (pending, paid, failed, etc.)
- `customer_id` (optional): Filter by customer ID
- `from_date` (optional): From date (ISO 8601)
- `to_date` (optional): To date (ISO 8601)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all pending payments"

---

### mollie_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: Check payment status, details.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "Get details for payment TR-12345"

---

### mollie_create_payment

**What it does**: Creates a new payment in Mollie.

**When to use**: Generate payment links, process payments.

**Arguments**:
- `amount` (required): Payment amount with currency and value
- `description` (required): Payment description
- `redirect_url` (optional): Redirect URL after payment
- `customer_id` (optional): Customer ID

**Example LLM prompt**: "Create a payment for 100 EUR for order 12345"

---

### mollie_list_refunds

**What it does**: Lists all refunds in Mollie.

**When to use**: Track refunds, manage disputes.

**Arguments**:
- `payment_id` (optional): Filter by payment ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all refunds from this month"

---

### mollie_get_refund

**What it does**: Gets details of a specific refund.

**When to use**: Check refund status.

**Arguments**:
- `refundId` (required): Refund ID

**Example LLM prompt**: "Get details for refund RF-12345"

---

### mollie_create_refund

**What it does**: Creates a refund for a payment.

**When to use**: Process refunds, handle returns.

**Arguments**:
- `paymentId` (required): Payment ID
- `amount` (optional): Refund amount (full if not specified)
- `description` (optional): Refund description

**Example LLM prompt**: "Refund 50 EUR for payment TR-12345"

---

### mollie_list_customers

**What it does**: Lists all customers in Mollie.

**When to use**: Manage customer base, find customers.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all customers"

---

### mollie_get_customer

**What it does**: Gets details of a specific customer.

**When to use**: Check customer info, payment methods.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer CS-12345"

---

### mollie_list_subscriptions

**What it does**: Lists all subscriptions for a customer.

**When to use**: View recurring payments, manage subscriptions.

**Arguments**:
- `customerId` (required): Customer ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List subscriptions for customer CS-12345"

---

### mollie_get_subscription

**What it does**: Gets details of a specific subscription.

**When to use**: Check subscription details, next payment.

**Arguments**:
- `customerId` (required): Customer ID
- `subscriptionId` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription SB-12345"

---

## Mollie Notes

- **Payment processor**: Dutch payment provider
- **Payments**: One-time and recurring
- **Refunds**: Full or partial
- **Customers**: Saved customer profiles
- **Subscriptions**: Recurring payment mandates

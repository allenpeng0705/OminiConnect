# Checkout.com Tools

Provider: `checkout-com` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Checkout.com API. Checkout.com is a payment processing platform for businesses to accept and manage payments globally. **Requires Checkout.com OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for server-to-server integration
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.checkout.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `checkout_com_list_payments` | List payments | GET | /payments |
| `checkout_com_get_payment` | Get payment details | GET | /payments/{id} |
| `checkout_com_create_payment` | Create a payment | POST | /payments |
| `checkout_com_capture_payment` | Capture a payment | POST | /payments/{id}/captures |
| `checkout_com_void_payment` | Void a payment | POST | /payments/{id}/voids |
| `checkout_com_list_refunds` | List refunds | GET | /payments/{paymentId}/refunds |
| `checkout_com_get_refund` | Get refund details | GET | /refunds/{id} |
| `checkout_com_list_chargebacks` | List chargebacks | GET | /chargebacks |
| `checkout_com_get_chargeback` | Get chargeback details | GET | /chargebacks/{id} |
| `checkout_com_list_transactions` | List transactions | GET | /transactions |

---

## Tool Details

### checkout_com_list_payments

**What it does**: Lists all payments with cursor-based pagination.

**When to use**: View payment history, find specific payments.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List my Checkout.com payments"

---

### checkout_com_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: View payment status, card details, amounts.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Get payment pay_123 details"

---

### checkout_com_create_payment

**What it does**: Creates a new payment.

**When to use**: Process a payment for a customer.

**Arguments**:
- `amount` (required): Amount in minor currency units (cents)
- `currency` (required): Currency code (e.g., USD, EUR)
- `reference` (optional): Your payment reference
- `payment_type` (optional): Payment type

**Example LLM prompt**: "Create a payment for $100 in USD"

---

### checkout_com_capture_payment

**What it does**: Captures an authorized payment.

**When to use**: Capture funds after order fulfillment.

**Arguments**:
- `id` (required): Payment ID
- `amount` (optional): Amount to capture (full if not specified)

**Example LLM prompt**: "Capture payment pay_123"

---

### checkout_com_void_payment

**What it does**: Voids an authorized payment.

**When to use**: Cancel an order before capture.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Void payment pay_123"

---

### checkout_com_list_refunds

**What it does**: Lists all refunds for a payment.

**When to use**: View refund history for a payment.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "List refunds for payment pay_123"

---

### checkout_com_get_refund

**What it does**: Gets details of a specific refund.

**When to use**: View refund status and details.

**Arguments**:
- `id` (required): Refund ID

**Example LLM prompt**: "Get refund ref_456 details"

---

### checkout_com_list_chargebacks

**What it does**: Lists all chargebacks.

**When to use**: Track disputed payments.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all chargebacks"

---

### checkout_com_get_chargeback

**What it does**: Gets details of a specific chargeback.

**When to use**: View chargeback reason and evidence status.

**Arguments**:
- `id` (required): Chargeback ID

**Example LLM prompt**: "Get chargeback cb_789 details"

---

### checkout_com_list_transactions

**What it does**: Lists all transactions.

**When to use**: View all financial activity.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all transactions"

---

## Checkout.com API Notes

- **Client Credentials**: Uses OAuth2 client credentials flow
- **Amounts**: Always in minor currency units (e.g., cents for USD)
- **Payment Lifecycle**: Authorized -> Captured -> Settled
- **Chargebacks**: Disputed payments from customers

# Checkout.com (Sandbox) Tools

Provider: `checkout-com-sandbox` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Checkout.com Sandbox API. The sandbox environment allows testing payment integration without processing real transactions. **Requires Checkout.com Sandbox OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for sandbox integration
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.sandbox.checkout.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `checkout_com_sandbox_list_payments` | List payments | GET | /payments |
| `checkout_com_sandbox_get_payment` | Get payment details | GET | /payments/{id} |
| `checkout_com_sandbox_create_payment` | Create a payment | POST | /payments |
| `checkout_com_sandbox_capture_payment` | Capture a payment | POST | /payments/{id}/captures |
| `checkout_com_sandbox_void_payment` | Void a payment | POST | /payments/{id}/voids |
| `checkout_com_sandbox_list_refunds` | List refunds | GET | /payments/{paymentId}/refunds |
| `checkout_com_sandbox_get_refund` | Get refund details | GET | /refunds/{id} |
| `checkout_com_sandbox_list_chargebacks` | List chargebacks | GET | /chargebacks |
| `checkout_com_sandbox_get_chargeback` | Get chargeback details | GET | /chargebacks/{id} |
| `checkout_com_sandbox_list_transactions` | List transactions | GET | /transactions |

---

## Tool Details

### checkout_com_sandbox_list_payments

**What it does**: Lists all payments in sandbox.

**When to use**: View test payment history.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List test payments in sandbox"

---

### checkout_com_sandbox_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: Verify payment processing in tests.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Get payment pay_123 details"

---

### checkout_com_sandbox_create_payment

**What it does**: Creates a new payment.

**When to use**: Test payment creation flows.

**Arguments**:
- `amount` (required): Amount in minor currency units
- `currency` (required): Currency code
- `reference` (optional): Payment reference

**Example LLM prompt**: "Create a test payment for $100"

---

### checkout_com_sandbox_capture_payment

**What it does**: Captures an authorized payment.

**When to use**: Test capture flow.

**Arguments**:
- `id` (required): Payment ID
- `amount` (optional): Amount to capture

**Example LLM prompt**: "Capture test payment pay_123"

---

### checkout_com_sandbox_void_payment

**What it does**: Voids an authorized payment.

**When to use**: Test void flow.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Void test payment pay_123"

---

### checkout_com_sandbox_list_refunds

**What it does**: Lists all refunds for a payment.

**When to use**: Test refund listing.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "List refunds for payment pay_123"

---

### checkout_com_sandbox_get_refund

**What it does**: Gets details of a specific refund.

**When to use**: Verify refund details in tests.

**Arguments**:
- `id` (required): Refund ID

**Example LLM prompt**: "Get refund ref_456 details"

---

### checkout_com_sandbox_list_chargebacks

**What it does**: Lists all chargebacks.

**When to use**: Test chargeback handling.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test chargebacks"

---

### checkout_com_sandbox_get_chargeback

**What it does**: Gets details of a specific chargeback.

**When to use**: Verify chargeback details in tests.

**Arguments**:
- `id` (required): Chargeback ID

**Example LLM prompt**: "Get chargeback cb_789 details"

---

### checkout_com_sandbox_list_transactions

**What it does**: Lists all transactions.

**When to use**: View all test financial activity.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all test transactions"

---

## Checkout.com Sandbox API Notes

- **Sandbox Environment**: Isolated test environment
- **No Real Money**: Transactions are simulated only
- **Same API**: Mirror of production API structure
- **Client Credentials**: Uses OAuth2 client credentials flow

# PayPal Sandbox Tools

Provider: `paypal-sandbox` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the PayPal Sandbox API. They allow AI agents to manage payments, payouts, invoices, subscriptions, and webhooks in a sandbox environment. **Requires PayPal Sandbox OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with PayPal Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api-m.sandbox.paypal.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paypal_sandbox_list_payments` | List payments | GET | /v1/payments/payments |
| `paypal_sandbox_get_payment` | Get payment details | GET | /v1/payments/payments/{paymentId} |
| `paypal_sandbox_list_payouts` | List payouts | GET | /v1/payouts |
| `paypal_sandbox_get_payout` | Get payout details | GET | /v1/payouts/{payoutBatchId} |
| `paypal_sandbox_list_invoices` | List invoices | GET | /v1/invoicing/invoices |
| `paypal_sandbox_get_invoice` | Get invoice details | GET | /v1/invoicing/invoices/{invoiceId} |
| `paypal_sandbox_list_subscriptions` | List subscriptions | GET | /v1/billing/subscriptions |
| `paypal_sandbox_get_subscription` | Get subscription details | GET | /v1/billing/subscriptions/{subscriptionId} |
| `paypal_sandbox_list_webhooks` | List webhooks | GET | /v1/notifications/webhooks |
| `paypal_sandbox_get_user_info` | Get user information | GET | /v1/identity/oauth2/userinfo |

---

## Tool Details

### paypal_sandbox_list_payments

**What it does**: Lists payments in the PayPal Sandbox environment.

**When to use**: Review payment transactions, check payment status.

**Arguments**:
- `status` (optional): Filter by status (approved, failed, etc.)
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show recent payments in sandbox"

---

### paypal_sandbox_get_payment

**What it does**: Gets detailed information about a specific payment.

**When to use**: Review payment details, transaction info.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "Get details for payment PAY-12345"

---

### paypal_sandbox_list_payouts

**What it does**: Lists payouts in the PayPal Sandbox environment.

**When to use**: Review payout transactions.

**Arguments**:
- `status` (optional): Filter by status (pending, success, failed)

**Example LLM prompt**: "Show pending payouts"

---

### paypal_sandbox_get_payout

**What it does**: Gets detailed information about a specific payout.

**When to use**: Review payout details.

**Arguments**:
- `payoutBatchId` (required): Payout batch ID

**Example LLM prompt**: "Get details for payout BATCH-67890"

---

### paypal_sandbox_list_invoices

**What it does**: Lists invoices in the PayPal Sandbox environment.

**When to use**: Review invoice history.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, paid)

**Example LLM prompt**: "Show all paid invoices"

---

### paypal_sandbox_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: Review invoice details.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### paypal_sandbox_list_subscriptions

**What it does**: Lists subscriptions in the PayPal Sandbox environment.

**When to use**: Review subscription status.

**Arguments**:
- `status` (optional): Filter by status (active, suspended, cancelled)

**Example LLM prompt**: "Show all active subscriptions"

---

### paypal_sandbox_get_subscription

**What it does**: Gets detailed information about a specific subscription.

**When to use**: Review subscription details.

**Arguments**:
- `subscriptionId` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription I-SB12345"

---

### paypal_sandbox_list_webhooks

**What it does**: Lists webhooks configured in the PayPal Sandbox.

**When to use**: Review webhook configuration.

**Arguments**: None

**Example LLM prompt**: "What webhooks are configured?"

---

### paypal_sandbox_get_user_info

**What it does**: Gets user information from PayPal Sandbox.

**When to use**: Get user profile, account info.

**Arguments**: None

**Example LLM prompt**: "Get my PayPal user info"

---

## PayPal Sandbox API Notes

- **Sandbox Environment**: Uses sandbox APIs for testing
- **OAuth2**: Requires user authentication via OAuth flow
- **URL-based Scopes**: PayPal uses URL patterns for scopes
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format

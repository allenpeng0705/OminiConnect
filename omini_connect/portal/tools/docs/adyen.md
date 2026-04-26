# Adyen Tools

Provider: `adyen` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Adyen payment platform API. They enable AI agents to manage payments, shoppers, platform accounts, webhooks, and settlement reports for global payment processing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Adyen
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `payments:read`, `payments:write`, `accounts:read`, `webhooks:read`, `webhooks:write`, `reports:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `adyen_list_payments` | List all payments | GET | /v1/payments |
| `adyen_get_payment` | Get payment details | GET | /v1/payments/{id} |
| `adyen_create_payment` | Create a new payment | POST | /v1/payments |
| `adyen_get_payment_methods` | Get available payment methods | POST | /v1/paymentMethods |
| `adyen_list_accounts` | List platform accounts | GET | /v1/accounts |
| `adyen_get_account` | Get account details | GET | /v1/accounts/{id} |
| `adyen_list_webhooks` | List webhook configurations | GET | /v1/webhooks |
| `adyen_get_webhook` | Get webhook details | GET | /v1/webhooks/{id} |
| `adyen_create_webhook` | Create a new webhook | POST | /v1/webhooks |
| `adyen_get_report` | Get settlement reports | POST | /v1/reports |

---

## Tool Details

### adyen_list_payments

**What it does**: Retrieves all payments with optional filtering by merchant account, status, or date range.

**When to use**: Get an overview of all payment transactions for reconciliation or reporting.

**Arguments**:
- `merchantAccount` (optional): Merchant account name
- `status` (optional): Filter by status (received, pending, refused, etc)
- `fromCreationDate` (optional): From creation date (YYYY-MM-DD HH:mm:ss)
- `toCreationDate` (optional): To creation date (YYYY-MM-DD HH:mm:ss)
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all pending payments for merchant MyMerchantAccount"

---

### adyen_get_payment

**What it does**: Gets detailed information about a specific payment including PSP reference and result.

**When to use**: Review payment details for customer inquiries or dispute investigation.

**Arguments**:
- `id` (required): Payment ID (PSP reference)

**Example LLM prompt**: "Get details for payment 123456789"

---

### adyen_create_payment

**What it does**: Creates a new payment with amount, currency, and payment method details.

**When to use**: Process a payment from a customer's payment method or saved card.

**Arguments**:
- `amount` (required): Amount object with value and currency
- `reference` (required): Your order/reference ID
- `merchantAccount` (required): Merchant account name
- `paymentMethod` (required): Payment method details
- `returnUrl` (optional): Return URL for redirect payments
- `shopperEmail` (optional): Shopper email address
- `shopperReference` (optional): Shopper ID for recurring payments

**Example LLM prompt**: "Create a payment for 100 EUR for customer with reference order_123"

---

### adyen_get_payment_methods

**What it does**: Gets available payment methods for a merchant account, amount, and country.

**When to use**: Display available payment options to customers at checkout.

**Arguments**:
- `amount` (required): Amount object with value and currency
- `merchantAccount` (required): Merchant account name
- `countryCode` (optional): Country code (e.g., US, GB)
- `channel` (optional): Channel (ios, android, web)

**Example LLM prompt**: "Get payment methods for 100 USD in the US for web channel"

---

### adyen_list_accounts

**What it does**: Lists all platform accounts for marketplaces using Adyen.

**When to use**: Manage multiple merchant accounts in a platform structure.

**Arguments**:
- `pageNumber` (optional): Page number (default 1)
- `pageSize` (optional): Page size (default 100)

**Example LLM prompt**: "List all platform accounts"

---

### adyen_get_account

**What it does**: Gets detailed information about a specific platform account including balances.

**When to use**: Review account status and available balance.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get details for account account_789"

---

### adyen_list_webhooks

**What it does**: Lists all webhook configurations for a merchant account.

**When to use**: Monitor webhook setup for payment and event notifications.

**Arguments**:
- `merchantAccount` (optional): Merchant account name

**Example LLM prompt**: "List all webhooks for merchant MyMerchantAccount"

---

### adyen_get_webhook

**What it does**: Gets detailed information about a specific webhook configuration.

**When to use**: Review webhook settings before modification or debugging.

**Arguments**:
- `id` (required): Webhook ID

**Example LLM prompt**: "Get details for webhook webhook_456"

---

### adyen_create_webhook

**What it does**: Creates a new webhook configuration for merchant events.

**When to use**: Set up real-time event notifications for payment status changes.

**Arguments**:
- `merchantAccount` (required): Merchant account name
- `url` (required): Webhook URL
- `active` (optional): Whether webhook is active
- `eventTypes` (required): Event types to subscribe to
- `communicationFormat` (optional): Format (json, xml, default json)

**Example LLM prompt**: "Create a webhook for events payment_cancelled and payment_refunded at https://api.example.com/adyen-webhook"

---

### adyen_get_report

**What it does**: Gets settlement or financial reports from Adyen.

**When to use**: Download reports for accounting, reconciliation, or tax purposes.

**Arguments**:
- `reportType` (required): Type of report (settlement, payment, etc)
- `merchantAccount` (required): Merchant account name
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get settlement report for MyMerchantAccount for April 2026"

---

## Adyen API Notes

- **PSP Reference**: Adyen's unique identifier for payments (used as payment ID)
- **Marketplace Platform**: Adyen supports platform/marketplace account structures
- **Payment Lifecycle**: received -> pending -> refused/captured -> settled/refunded
- **Webhook Events**: Real-time notifications for payment, account, and report events
- **Communication Format**: JSON or XML for webhook delivery
- **Event Types**: Configure specific events to receive (payments, accounts, reports, etc)
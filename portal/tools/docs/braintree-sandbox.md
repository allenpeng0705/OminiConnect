# Braintree (Sandbox) Tools

Provider: `braintree-sandbox` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Braintree API (Sandbox environment). They allow AI agents to manage transactions, customers, and subscriptions for payment processing. The sandbox environment is for testing - use `braintree` for production.

**Sandbox Environment**: This provider uses `https://api.sandbox.braintreegateway.com`. For production, use the `braintree` provider.

## Authentication

**Nango OAuth2**:
- User authenticates via Braintree OAuth
- Token stored in Nango, accessed via `connection_ref`
- Merchant ID required for operations

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `braintree_sandbox_list_transactions` | List transactions | GET | /v1/transactions |
| `braintree_sandbox_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `braintree_sandbox_create_transaction` | Create transaction | POST | /v1/transactions |
| `braintree_sandbox_void_transaction` | Void transaction | PUT | /v1/transactions/{transactionId}/void |
| `braintree_sandbox_refund_transaction` | Refund transaction | PUT | /v1/transactions/{transactionId}/refund |
| `braintree_sandbox_list_customers` | List customers | GET | /v1/customers |
| `braintree_sandbox_get_customer` | Get customer details | GET | /v1/customers/{customerId} |
| `braintree_sandbox_create_customer` | Create customer | POST | /v1/customers |
| `braintree_sandbox_list_subscriptions` | List subscriptions | GET | /v1/subscriptions |
| `braintree_sandbox_get_subscription` | Get subscription details | GET | /v1/subscriptions/{subscriptionId} |

---

## Tool Details

### braintree_sandbox_list_transactions

**What it does**: Lists all transactions in sandbox.

**When to use**: View transaction history, find specific payments.

**Arguments**:
- `merchantAccountId` (optional): Filter by merchant account
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all transactions in sandbox"

---

### braintree_sandbox_get_transaction

**What it does**: Gets details for a specific transaction.

**When to use**: Check payment details, status.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction T-123"

---

### braintree_sandbox_create_transaction

**What it does**: Creates a new transaction (sale or authorization).

**When to use**: Process payments, authorize holds.

**Arguments**:
- `amount` (required): Transaction amount
- `paymentMethodNonce` (required): Nonce from client SDK
- `merchantAccountId` (optional): Merchant account ID
- `orderId` (optional): Order ID

**Example LLM prompt**: "Create a transaction for $100 with nonce abc123"

---

### braintree_sandbox_void_transaction

**What it does**: Voids an existing transaction.

**When to use**: Cancel authorization, stop payment.

**Arguments**:
- `transactionId` (required): Transaction ID to void

**Example LLM prompt**: "Void transaction T-123"

---

### braintree_sandbox_refund_transaction

**What it does**: Refunds a transaction.

**When to use**: Process refunds for returned items.

**Arguments**:
- `transactionId` (required): Transaction ID to refund
- `amount` (optional): Refund amount (full if not specified)

**Example LLM prompt**: "Refund $50 of transaction T-123"

---

### braintree_sandbox_list_customers

**What it does**: Lists all customers in sandbox.

**When to use**: View customer database.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all customers"

---

### braintree_sandbox_get_customer

**What it does**: Gets details for a specific customer.

**When to use**: View customer profile, payment methods.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer C-456"

---

### braintree_sandbox_create_customer

**What it does**: Creates a new customer.

**When to use**: Add new customers to the system.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create customer John Doe with email john@example.com"

---

### braintree_sandbox_list_subscriptions

**What it does**: Lists all subscriptions in sandbox.

**When to use**: View recurring billing.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active subscriptions"

---

### braintree_sandbox_get_subscription

**What it does**: Gets details for a specific subscription.

**When to use**: Check subscription status, billing info.

**Arguments**:
- `subscriptionId` (required): Subscription ID

**Example LLM prompt**: "Get details for subscription S-789"

---

## Braintree Sandbox API Notes

- **Sandbox**: Separate test environment from production
- **Test Cards**: Use Braintree test card numbers for testing
- **No Real Money**: Transactions do not process real payments
- **Merchant Account**: Required for transaction operations
- **Nonces**: Client-side payment method tokens

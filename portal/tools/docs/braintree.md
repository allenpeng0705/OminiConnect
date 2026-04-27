# Braintree Tools

Provider: `braintree` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Braintree payment gateway API. They enable AI agents to manage transactions, customers, merchant accounts, and webhooks for payment processing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Braintree
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read_only`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `braintree_list_transactions` | List transactions | GET | /v1/transactions |
| `braintree_get_transaction` | Get details of a specific transaction | GET | /v1/transactions/{id} |
| `braintree_create_transaction` | Create a new transaction | POST | /v1/transactions |
| `braintree_void_transaction` | Void an existing transaction | POST | /v1/transactions/{id}/void |
| `braintree_list_customers` | List customers | GET | /v1/customers |
| `braintree_get_customer` | Get details of a specific customer | GET | /v1/customers/{id} |
| `braintree_create_customer` | Create a new customer | POST | /v1/customers |
| `braintree_list_merchants` | List merchant accounts | GET | /v1/merchant_accounts |
| `braintree_get_merchant` | Get details of a specific merchant | GET | /v1/merchant_accounts/{id} |
| `braintree_list_webhooks` | List webhook event types | GET | /v1/webhook_notifications/available |

---

## Tool Details

### braintree_list_transactions

**What it does**: Returns a collection of transactions with amounts, statuses, and customer info.

**When to use**: Get an overview of all payment transactions for reconciliation or reporting.

**Arguments**:
- `merchant_account_id` (optional): Filter by merchant account ID
- `customer_id` (optional): Filter by customer ID
- `status` (optional): Filter by status (authorized, submitted_for_settlement, settled, voided)
- `order_id` (optional): Filter by order ID
- `page_size` (optional): Number of results per page (max 100, default 50)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all settled transactions from the last week"

---

### braintree_get_transaction

**What it does**: Get details of a specific transaction including amount, status, payment method, and gateway response.

**When to use**: Review transaction details for customer disputes or fulfillment checks.

**Arguments**:
- `id` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction f4k3r3f4k3r"

---

### braintree_create_transaction

**What it does**: Create a new transaction for collecting payment. Specify amount, payment method, and options.

**When to use**: Process a payment from a customer's stored payment method or one-time nonce.

**Arguments**:
- `amount` (required): Transaction amount
- `payment_method_nonce` (required): One-time use payment method nonce from client
- `customer_id` (optional): Existing customer ID to associate
- `merchant_account_id` (optional): Merchant account ID for multi-currency
- `order_id` (optional): Order ID for tracking
- `description` (optional): Transaction description
- `customer` (optional): Inline customer object with first_name, last_name, email

**Example LLM prompt**: "Charge $99.99 to customer using nonce nonce_abc123"

---

### braintree_void_transaction

**What it does**: Void an authorized transaction before it is submitted for settlement. Cannot void settled transactions.

**When to use**: Cancel a transaction before it settles to refund the customer.

**Arguments**:
- `id` (required): Transaction ID to void

**Example LLM prompt**: "Void transaction f4k3r3f4k3r"

---

### braintree_list_customers

**What it does**: Returns paginated list with names, emails, and company info.

**When to use**: Get an overview of all customers for CRM or marketing purposes.

**Arguments**:
- `first_name` (optional): Filter by first name
- `last_name` (optional): Filter by last name
- `email` (optional): Filter by email
- `company` (optional): Filter by company name
- `page_size` (optional): Number of results per page (default 50)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all customers from Acme Corp"

---

### braintree_get_customer

**What it does**: Get details of a specific customer including payment methods, addresses, and custom fields.

**When to use**: Review customer details before processing transactions.

**Arguments**:
- `id` (required): Customer ID

**Example LLM prompt**: "Get details for customer 12345"

---

### braintree_create_customer

**What it does**: Create a new customer with contact info and payment methods. Required for recurring billing.

**When to use**: Add a new customer for payment processing.

**Arguments**:
- `first_name` (optional): Customer first name
- `last_name` (optional): Customer last name
- `email` (optional): Customer email address
- `phone` (optional): Customer phone number
- `company` (optional): Customer company name
- `website` (optional): Customer website URL

**Example LLM prompt**: "Create a new customer for Jane Smith at jane@example.com"

---

### braintree_list_merchants

**What it does**: List all merchant accounts associated with the gateway. Returns merchant IDs, currencies, and status.

**When to use**: View available merchant accounts for multi-merchant processing.

**Arguments**:
- `page_size` (optional): Number of results per page (default 50)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all merchant accounts"

---

### braintree_get_merchant

**What it does**: Get details of a specific merchant account including currency, status, and takings limits.

**When to use**: Review merchant account details for multi-currency processing.

**Arguments**:
- `id` (required): Merchant account ID

**Example LLM prompt**: "Get details for merchant account master_merchant"

---

### braintree_list_webhooks

**What it does**: List webhook notification event types available for subscription. Use for configuring webhook endpoints.

**When to use**: Review available webhook event types for integration monitoring.

**Arguments**: None

**Example LLM prompt**: "List all available webhook event types"

---

## Braintree API Notes

- **Transaction Lifecycle**: authorized -> submitted_for_settlement -> settled -> voided/refunded
- **Merchant Accounts**: Support for multiple merchants and currencies
- **Payment Method Nonces**: Client-side tokenized payment methods for secure transmission
- **Service Fees**: Support for marketplace-style split payments
- **Webhook Events**: Real-time notifications for transaction and customer events
- **Pagination**: Uses page-based pagination with page_size and page parameters

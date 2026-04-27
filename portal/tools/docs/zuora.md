# Zuora Tools

Provider: `zuora` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Zuora billing API. They enable AI agents to manage subscriptions, accounts, invoices, and orders for subscription-based businesses. Zuora is an enterprise billing and subscription management platform supporting complex pricing models.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zuora
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `Billing:Read`, `Billing:Create`, `Billing:Edit`, `Accounting:Read`, `Orders:Read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zuora_list_subscriptions` | List all subscriptions | GET | /v1/subscriptions |
| `zuora_get_subscription` | Get subscription details | GET | /v1/subscriptions/{subscriptionKey} |
| `zuora_create_subscription` | Create a new subscription | POST | /v1/subscriptions |
| `zuora_update_subscription` | Update a subscription | PUT | /v1/subscriptions/{subscriptionKey} |
| `zuora_list_accounts` | List all accounts | GET | /v1/accounts |
| `zuora_get_account` | Get account details | GET | /v1/accounts/{accountKey} |
| `zuora_list_invoices` | List all invoices | GET | /v1/invoices |
| `zuora_get_invoice` | Get invoice details | GET | /v1/invoices/{invoiceNumber} |
| `zuora_list_orders` | List all orders | GET | /v1/orders |
| `zuora_get_order` | Get order details | GET | /v1/orders/{orderId} |

---

## Tool Details

### zuora_list_subscriptions

**What it does**: Retrieves all subscriptions with optional filtering by account, status, or term type.

**When to use**: Get an overview of all subscriptions, find active or expired subscriptions.

**Arguments**:
- `accountKey` (optional): Filter by account key
- `status` (optional): Filter by status (Active, Cancelled, Expired, etc)
- `termType` (optional): Filter by term type (TERMED, EVERGREEN)
- `pageSize` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active subscriptions with EVERGREEN term type"

---

### zuora_get_subscription

**What it does**: Gets detailed information about a specific subscription including terms and billing info.

**When to use**: Review subscription details, check subscription state or terms.

**Arguments**:
- `subscriptionKey` (required): Subscription ID or number

**Example LLM prompt**: "Get details for subscription SUB-123456"

---

### zuora_create_subscription

**What it does**: Creates a new subscription for an account with product and pricing details.

**When to use**: Onboard new customers, start new subscription plans.

**Arguments**:
- `accountKey` (required): Account number or ID
- `subscriptionName` (optional): Subscription name
- `terms` (optional): Subscription terms object
- `initialTerm` (optional): Initial term with start date and term length
- `autoRenew` (optional): Auto renew flag (default true)

**Example LLM prompt**: "Create a subscription for account ACC-123 with auto renew enabled"

---

### zuora_update_subscription

**What it does**: Updates an existing subscription with new terms or settings.

**When to use**: Modify subscription terms, change auto renew settings.

**Arguments**:
- `subscriptionKey` (required): Subscription ID or number
- `autoRenew` (optional): Auto renew flag
- `renewalTerm` (optional): Renewal term settings
- `contractEffectiveDate` (optional): Contract effective date (YYYY-MM-DD)

**Example LLM prompt**: "Update subscription SUB-123456 to disable auto renew"

---

### zuora_list_accounts

**What it does**: Retrieves all accounts with optional filtering by status, balance, or sales rep.

**When to use**: Get an overview of all accounts, find accounts by status or sales rep.

**Arguments**:
- `status` (optional): Filter by status (Active, Cancelled, etc)
- `balance` (optional): Filter by balance amount
- `salesRep` (optional): Filter by sales rep name
- `pageSize` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active accounts with balance greater than 0"

---

### zuora_get_account

**What it does**: Gets detailed information about a specific account including billing info.

**When to use**: Check account details, view billing information.

**Arguments**:
- `accountKey` (required): Account number or ID

**Example LLM prompt**: "Get details for account ACC-123"

---

### zuora_list_invoices

**What it does**: Retrieves all invoices with optional filtering by account, status, or due date.

**When to use**: View billing history, find posted or draft invoices.

**Arguments**:
- `accountKey` (optional): Filter by account key
- `status` (optional): Filter by status (Draft, Posted, Cancelled, etc)
- `dueDate` (optional): Due date filter
- `pageSize` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all posted invoices from March 2026"

---

### zuora_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items.

**When to use**: View invoice details, check charges, see tax amounts.

**Arguments**:
- `invoiceNumber` (required): Invoice number

**Example LLM prompt**: "Get details for invoice INV-789"

---

### zuora_list_orders

**What it does**: Retrieves all orders with optional filtering by account, status, or order date.

**When to use**: View order history, find pending or activated orders.

**Arguments**:
- `accountKey` (optional): Filter by account key
- `status` (optional): Filter by status (Pending, Activated, Cancelled, etc)
- `orderDate` (optional): Order date filter (YYYY-MM-DD)
- `pageSize` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all pending orders from Q1 2026"

---

### zuora_get_order

**What it does**: Gets detailed information about a specific order including order items.

**When to use**: Review order details, check order items and status.

**Arguments**:
- `orderId` (required): Order ID

**Example LLM prompt**: "Get details for order ORD-456"

---

## Zuora API Notes

- **Account Key**: Primary identifier for account (usually account number)
- **Subscription Key**: Can be subscription ID or subscription number
- **Term Types**: `TERMED` has fixed term with renewal; `EVERGREEN` auto-renews
- **Invoice Status**: Draft invoices can be edited; Posted invoices are finalized
- **Complex Billing**: Zuora supports complex pricing models including usage-based billing
- **Orders**: Zuora Orders is a separate object from subscriptions, representing the sales order

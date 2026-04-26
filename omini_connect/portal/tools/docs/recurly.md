# Recurly Tools

Provider: `recurly` | Engine: `nango` | Auth: OAuth/API_KEY via Nango

## Overview

These tools wrap the Recurly subscription management API. They enable AI agents to manage subscriptions, accounts, invoices, and plans for subscription-based businesses. Recurly is a leading subscription management platform with built-in billing, revenue recognition, and reporting.

## Authentication

**Nango OAuth or API Key**:
- User authenticates via Nango Connect with Recurly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `subscriptions:read`, `subscriptions:write`, `accounts:read`, `invoices:read`, `plans:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recurly_list_subscriptions` | List all subscriptions | GET | /subscriptions |
| `recurly_get_subscription` | Get subscription details | GET | /subscriptions/{uuid} |
| `recurly_create_subscription` | Create a new subscription | POST | /subscriptions |
| `recurly_cancel_subscription` | Cancel a subscription | PUT | /subscriptions/{uuid}/cancel |
| `recurly_list_accounts` | List all accounts | GET | /accounts |
| `recurly_get_account` | Get account details | GET | /accounts/{account_code} |
| `recurly_list_invoices` | List all invoices | GET | /invoices |
| `recurly_get_invoice` | Get invoice details | GET | /invoices/{invoice_id} |
| `recurly_list_plans` | List all plans | GET | /plans |
| `recurly_get_plan` | Get plan details | GET | /plans/{plan_code} |

---

## Tool Details

### recurly_list_subscriptions

**What it does**: Retrieves all subscriptions with optional filtering by account, state, or plan.

**When to use**: Get an overview of all subscriptions, find active or cancelled subscriptions.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `state` (optional): Filter by state (active, cancelled, expired, etc)
- `plan_id` (optional): Filter by plan ID
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all active subscriptions for account ACC-123"

---

### recurly_get_subscription

**What it does**: Gets detailed information about a specific subscription including plan, state, and billing information.

**When to use**: Review subscription details, check subscription state or terms.

**Arguments**:
- `uuid` (required): Subscription UUID

**Example LLM prompt**: "Get details for subscription 12345678-1234-1234-1234-123456789012"

---

### recurly_create_subscription

**What it does**: Creates a new subscription for an account with a specific plan.

**When to use**: Onboard new customers, start new subscription plans.

**Arguments**:
- `account_id` (required): Account ID or code
- `plan_code` (required): Plan code
- `currency` (required): Currency code (e.g., USD)
- `unit_amount` (optional): Unit amount
- `quantity` (optional): Quantity (default 1)
- `trial_ends_at` (optional): Trial end date (YYYY-MM-DD)
- `start_date` (optional): Start date (YYYY-MM-DD)

**Example LLM prompt**: "Create a subscription for account CUST-001 with plan BASIC in USD"

---

### recurly_cancel_subscription

**What it does**: Cancels an existing Recurly subscription.

**When to use**: Handle customer cancellations, end subscriptions.

**Arguments**:
- `uuid` (required): Subscription UUID

**Example LLM prompt**: "Cancel subscription 12345678-1234-1234-1234-123456789012"

---

### recurly_list_accounts

**What it does**: Retrieves all accounts with optional filtering by email, username, or company.

**When to use**: Browse account base, find accounts by email or company.

**Arguments**:
- `email` (optional): Filter by email address
- `username` (optional): Filter by username
- `company` (optional): Filter by company name
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all accounts from company Acme Inc"

---

### recurly_get_account

**What it does**: Gets detailed information about a specific account including billing info.

**When to use**: Get account profile, billing address, payment method details.

**Arguments**:
- `account_code` (required): Account code

**Example LLM prompt**: "Get details for account ACC-123"

---

### recurly_list_invoices

**What it does**: Retrieves all invoices with optional filtering by account or state.

**When to use**: View billing history, find paid or open invoices.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `state` (optional): Filter by state (paid, open, voided, etc)
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all paid invoices for account ACC-123"

---

### recurly_get_invoice

**What it does**: Gets detailed information about a specific invoice including line items.

**When to use**: View invoice details, check line item charges, see tax amounts.

**Arguments**:
- `invoice_id` (required): Invoice ID or UUID

**Example LLM prompt**: "Get details for invoice r1234567890"

---

### recurly_list_plans

**What it does**: Retrieves all plans with optional filtering by state.

**When to use**: View available plans, find plan codes for subscription creation.

**Arguments**:
- `state` (optional): Filter by state (active, retired)
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all active plans"

---

### recurly_get_plan

**What it does**: Gets detailed information about a specific plan including pricing and features.

**When to use**: Check plan details, view pricing and billing intervals.

**Arguments**:
- `plan_code` (required): Plan code

**Example LLM prompt**: "Get details for plan BASIC"

---

## Recurly API Notes

- **Account Codes**: Must be unique; used as primary identifier
- **Subscription UUID**: Recurly uses UUIDs for subscription identification
- **Plans**: Define billing intervals and pricing; linked to subscriptions
- **Invoices**: Auto-generated based on subscription billing cycles
- **States**: Subscriptions and invoices have distinct state machines
- **Dunning**: Recurly handles failed payment retry logic automatically

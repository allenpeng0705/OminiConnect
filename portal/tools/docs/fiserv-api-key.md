# Fiserv API Key Tools

Provider: `fiserv-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fiserv API using API Key authentication. They allow AI agents to manage accounts, transactions, and payments. Fiserv is a payment and banking platform.

## Authentication

**Nango API_KEY**:
- User provides API key via Nango Connect
- Key is passed in the Authorization header
- Key stored in Nango, accessed via `connection_ref`
- Uses production Fiserv endpoint

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fiserv_api_key_get_balance` | Get account balance | GET | /api/v1/accounts/balance |
| `fiserv_api_key_list_transactions` | List transactions | GET | /api/v1/transactions |
| `fiserv_api_key_get_transaction` | Get transaction details | GET | /api/v1/transactions/{id} |
| `fiserv_api_key_create_payment` | Create a payment | POST | /api/v1/payments |
| `fiserv_api_key_get_payment` | Get payment details | GET | /api/v1/payments/{id} |
| `fiserv_api_key_list_accounts` | List accounts | GET | /api/v1/accounts |
| `fiserv_api_key_get_account` | Get account details | GET | /api/v1/accounts/{id} |
| `fiserv_api_key_list_payments` | List payments | GET | /api/v1/payments |
| `fiserv_api_key_cancel_payment` | Cancel a payment | POST | /api/v1/payments/{id}/cancel |
| `fiserv_api_key_get_merchant_info` | Get merchant info | GET | /api/v1/merchant |

---

## Tool Details

### fiserv_api_key_get_balance

**What it does**: Gets account balance information.

**When to use**: Check available balance.

**Arguments**:
- `account_id` (optional): Account ID

**Example LLM prompt**: "Get account balance"

---

### fiserv_api_key_list_transactions

**What it does**: Lists all transactions with optional date filtering.

**When to use**: View transaction history.

**Arguments**:
- `account_id` (optional): Account ID
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "List transactions from the last month"

---

### fiserv_api_key_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: View transaction details.

**Arguments**:
- `id` (required): Transaction ID

**Example LLM prompt**: "Get transaction abc123"

---

### fiserv_api_key_create_payment

**What it does**: Creates a new payment.

**When to use**: Initiate a payment.

**Arguments**:
- `amount` (required): Payment amount
- `currency` (optional): Currency code
- `recipient_id` (optional): Recipient ID

**Example LLM prompt**: "Create a payment of 100 USD"

---

### fiserv_api_key_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: View payment status.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Get payment xyz789"

---

### fiserv_api_key_list_accounts

**What it does**: Lists all accounts.

**When to use**: Browse account list.

**Arguments**: None

**Example LLM prompt**: "List all accounts"

---

### fiserv_api_key_get_account

**What it does**: Gets details of a specific account.

**When to use**: View account information.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get account def456"

---

### fiserv_api_key_list_payments

**What it does**: Lists all payments.

**When to use**: Browse payment history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all payments"

---

### fiserv_api_key_cancel_payment

**What it does**: Cancels a pending payment.

**When to use**: Stop a payment before it processes.

**Arguments**:
- `id` (required): Payment ID

**Example LLM prompt**: "Cancel payment xyz789"

---

### fiserv_api_key_get_merchant_info

**What it does**: Gets merchant information.

**When to use**: View merchant profile.

**Arguments**: None

**Example LLM prompt**: "Get merchant information"

---

## Fiserv API Key Notes

- **API Key vs OAuth**: Same API, different auth methods
- **Accounts**: Bank accounts and payment accounts
- **Transactions**: Individual payment transactions
- **Payments**: Fund transfers and payment requests
- **Merchant**: Merchant profile information
- **Production**: Uses prod.emea.api.fiservapps.com endpoint

# Mercury Tools

Provider: `mercury` | Engine: `nango` | Auth: OAUTH2 via Nango

## Overview

These tools wrap the Mercury API. They allow AI agents to manage accounts, transactions, payments, and invoices for business banking. **Requires Mercury OAuth2.**

## Authentication

**Nango OAUTH2**:
- User authenticates via Nango Connect with Mercury
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.mercury.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mercury_list_accounts` | List all accounts | GET | /v1/accounts |
| `mercury_get_account` | Get account details | GET | /v1/accounts/{accountId} |
| `mercury_list_transactions` | List transactions | GET | /v1/transactions |
| `mercury_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `mercury_list_payments` | List payments | GET | /v1/payments |
| `mercury_get_payment` | Get payment details | GET | /v1/payments/{paymentId} |
| `mercury_list_invoices` | List invoices | GET | /v1/invoices |
| `mercury_get_invoice` | Get invoice details | GET | /v1/invoices/{invoiceId} |
| `mercury_create_transfer` | Create a transfer | POST | /v1/transfers |
| `mercury_list_webhooks` | List webhooks | GET | /v1/webhooks |

---

## Tool Details

### mercury_list_accounts

**What it does**: Lists all accounts in Mercury.

**When to use**: View account balances, find account IDs.

**Arguments**: None

**Example LLM prompt**: "List all my Mercury accounts"

---

### mercury_get_account

**What it does**: Gets details of a specific Mercury account.

**When to use**: Check account balance, view account settings.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account ACC-12345"

---

### mercury_list_transactions

**What it does**: Lists all transactions in Mercury.

**When to use**: View transaction history, reconcile accounts.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `from_date` (optional): From date (ISO 8601)
- `to_date` (optional): To date (ISO 8601)
- `status` (optional): Filter by status (pending, posted)
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all transactions from January 2024"

---

### mercury_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: Investigate a transaction, get receipt details.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction TXN-12345"

---

### mercury_list_payments

**What it does**: Lists all payments in Mercury.

**When to use**: Track outgoing payments, view payment history.

**Arguments**:
- `from_date` (optional): From date
- `to_date` (optional): To date
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all payments from last month"

---

### mercury_get_payment

**What it does**: Gets details of a specific payment.

**When to use**: Check payment status, get payment details.

**Arguments**:
- `paymentId` (required): Payment ID

**Example LLM prompt**: "Get details for payment PAY-12345"

---

### mercury_list_invoices

**What it does**: Lists all invoices in Mercury.

**When to use**: Manage invoices, track payments.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, paid)
- `client_id` (optional): Filter by client ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all unpaid invoices"

---

### mercury_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: View invoice details, check payment status.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### mercury_create_transfer

**What it does**: Creates a transfer between Mercury accounts.

**When to use**: Move funds between accounts.

**Arguments**:
- `from_account_id` (required): Source account ID
- `to_account_id` (required): Destination account ID
- `amount` (required): Transfer amount
- `currency` (optional): Currency code (default USD)

**Example LLM prompt**: "Transfer $1000 from ACC-12345 to ACC-67890"

---

### mercury_list_webhooks

**What it does**: Lists all configured webhooks in Mercury.

**When to use**: Manage webhook configurations.

**Arguments**: None

**Example LLM prompt**: "List all Mercury webhooks"

---

## Mercury Notes

- **Accounts**: Bank accounts (checking, savings)
- **Transactions**: All account activity
- **Payments**: Outgoing wire transfers
- **Invoices**: Client invoices
- **Modern banking**: Mercury is a fintech bank for startups

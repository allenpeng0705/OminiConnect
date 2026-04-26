# Schwab Tools

Provider: `schwab` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Charles Schwab is a brokerage platform for investing and trading. These tools allow AI agents to manage accounts, view portfolios, check balances, and place trades.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Schwab
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `accounts:read`, `positions:read`, `portfolio:read`, `transactions:read`, `marketdata:read`, `orders:read`, `orders:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `schwab_list_accounts` | List all accounts | GET | /v1/accounts |
| `schwab_get_account` | Get account details | GET | /v1/accounts/{accountId} |
| `schwab_list_positions` | List all positions | GET | /v1/accounts/{accountId}/positions |
| `schwab_get_portfolio` | Get portfolio details | GET | /v1/accounts/{accountId}/portfolio |
| `schwab_list_transactions` | List all transactions | GET | /v1/accounts/{accountId}/transactions |
| `schwab_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `schwab_get_quotes` | Get stock quotes | GET | /v1/marketdata/quotes |
| `schwab_list_orders` | List all orders | GET | /v1/accounts/{accountId}/orders |
| `schwab_place_order` | Place a trade order | POST | /v1/accounts/{accountId}/orders |
| `schwab_get_balances` | Get account balances | GET | /v1/accounts/{accountId}/balances |

---

## Tool Details

### schwab_list_accounts

**What it does**: Returns a list of all brokerage accounts.

**When to use**: View all linked accounts.

**Arguments**:
- `limit` (optional): Number of accounts (default 50)

**Example LLM prompt**: "List all my accounts"

---

### schwab_get_account

**What it does**: Gets details of a specific account.

**When to use**: Get account information.

**Arguments**:
- `accountId` (required): The account ID

**Example LLM prompt**: "Get details for account acct_abc123"

---

### schwab_list_positions

**What it does**: Returns a list of all positions in an account.

**When to use**: View holdings.

**Arguments**:
- `accountId` (required): The account ID
- `limit` (optional): Number of positions (default 50)

**Example LLM prompt**: "List all positions in my brokerage account"

---

### schwab_get_portfolio

**What it does**: Gets portfolio details and allocations.

**When to use**: View portfolio breakdown.

**Arguments**:
- `accountId` (required): The account ID

**Example LLM prompt**: "Get portfolio details for acct_123"

---

### schwab_list_transactions

**What it does**: Returns a list of all transactions.

**When to use**: View transaction history.

**Arguments**:
- `accountId` (required): The account ID
- `limit` (optional): Number of transactions (default 50)
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "List all transactions for this month"

---

### schwab_get_transaction

**What it does**: Gets details of a specific transaction.

**When to use**: Get transaction details.

**Arguments**:
- `transactionId` (required): The transaction ID

**Example LLM prompt**: "Get details for transaction txn_xyz789"

---

### schwab_get_quotes

**What it does**: Gets stock quotes for symbols.

**When to use**: Check current prices.

**Arguments**:
- `symbols` (required): Stock symbols (array)

**Example LLM prompt**: "Get quotes for AAPL, GOOGL, MSFT"

---

### schwab_list_orders

**What it does**: Returns a list of all orders.

**When to use**: View pending and executed orders.

**Arguments**:
- `accountId` (required): The account ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all open orders"

---

### schwab_place_order

**What it does**: Places a new trade order.

**When to use**: Execute a trade.

**Arguments**:
- `accountId` (required): The account ID
- `symbol` (required): Stock symbol
- `quantity` (required): Number of shares
- `side` (required): Buy or sell
- `orderType` (optional): Order type

**Example LLM prompt**: "Buy 100 shares of AAPL"

---

### schwab_get_balances

**What it does**: Gets account balances.

**When to use**: Check cash and portfolio value.

**Arguments**:
- `accountId` (required): The account ID

**Example LLM prompt**: "Get balances for account acct_123"

---

## Schwab Notes

- Accounts are individual brokerage accounts
- Positions are holdings in securities
- Transactions include buys, sells, dividends
- Orders are trade requests
- Quotes provide real-time market data

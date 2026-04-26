# Brex Tools

Provider: `brex` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Brex API. They allow AI agents to manage transactions, accounts, cards, and employees. Brex is a business banking platform offering corporate cards, expense management, and treasury services.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Brex
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `transactions:read`, `accounts:read`, `cards:read`, `cards:write`, `employees:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `brex_list_transactions` | List all transactions | GET | /v2/transactions |
| `brex_get_transaction` | Get a specific transaction | GET | /v2/transactions/{transaction_id} |
| `brex_list_accounts` | List all accounts | GET | /v2/accounts |
| `brex_get_account` | Get a specific account | GET | /v2/accounts/{account_id} |
| `brex_list_cards` | List all cards | GET | /v2/cards |
| `brex_get_card` | Get a specific card | GET | /v2/cards/{card_id} |
| `brex_list_employees` | List employees | GET | /v2/employees |
| `brex_get_employee` | Get a specific employee | GET | /v2/employees/{employee_id} |
| `brex_create_card` | Create a new card | POST | /v2/cards |
| `brex_set_spending_limit` | Set spending limit on a card | PUT | /v2/cards/{card_id}/spending_limit |

---

## Tool Details

### brex_list_transactions

**What it does**: Lists all transactions across accounts. Returns amounts, descriptions, dates, and status.

**When to use**: Monitor spending, reconcile accounts, or track expenses.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `status` (optional): Filter by status (posted, pending, settled)
- `start_date` (optional): Start date in YYYY-MM-DD format
- `end_date` (optional): End date in YYYY-MM-DD format
- `limit` (optional): Maximum results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Show me all transactions from the last month"

---

### brex_get_transaction

**What it does**: Gets details of a specific transaction including amounts, dates, merchant info, and status.

**When to use**: Get full transaction details before categorizing or flagging.

**Arguments**:
- `transaction_id` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction txn-123"

---

### brex_list_accounts

**What it does**: Lists all Brex accounts including checking, savings, and credit accounts.

**When to use**: Understand available funds and account balances.

**Arguments**:
- `type` (optional): Filter by account type (checking, savings, credit)
- `limit` (optional): Maximum results
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all our Brex accounts"

---

### brex_get_account

**What it does**: Gets details of a specific account including balance, type, currency, and status.

**When to use**: Check account balance or verify account status.

**Arguments**:
- `account_id` (required): Account ID

**Example LLM prompt**: "What is the current balance of our main checking account"

---

### brex_list_cards

**What it does**: Lists all Brex cards including virtual and physical cards.

**When to use**: Manage corporate cards or see all issued cards.

**Arguments**:
- `account_id` (optional): Filter by account ID
- `status` (optional): Filter by status (active, paused, cancelled)
- `limit` (optional): Maximum results
- `offset` (optional): Pagination offset

**Example LLM prompt**: "Show all active Brex cards"

---

### brex_get_card

**What it does**: Gets details of a specific card including masked card number, spending limit, status, and linked account.

**When to use**: Review card details or check spending limits.

**Arguments**:
- `card_id` (required): Card ID

**Example LLM prompt**: "Get details for card card-456"

---

### brex_list_employees

**What it does**: Lists all employees in the Brex account with roles and spending permissions.

**When to use**: Manage corporate spenders and their access levels.

**Arguments**:
- `role` (optional): Filter by role (admin, member, viewer)
- `status` (optional): Filter by status (active, inactive)
- `limit` (optional): Maximum results
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all active administrators on Brex"

---

### brex_get_employee

**What it does**: Gets details of a specific employee including profile, spending limits, and card associations.

**When to use**: Review employee spending permissions.

**Arguments**:
- `employee_id` (required): Employee ID

**Example LLM prompt**: "What cards and spending limits does employee emp-789 have"

---

### brex_create_card

**What it does**: Creates a new Brex card for an employee with spending limits, card type, and linked account.

**When to use**: Issue new corporate cards to employees.

**Arguments**:
- `employee_id` (required): Employee ID to associate with the card
- `account_id` (optional): Account ID to link the card to
- `card_type` (optional): Card type (virtual, physical, default virtual)
- `spending_limit` (optional): Spending limit amount
- `currency` (optional): Currency code (default USD)

**Example LLM prompt**: "Create a virtual card for John with a $5000 monthly limit"

---

### brex_set_spending_limit

**What it does**: Updates the spending limit on an existing Brex card.

**When to use**: Adjust employee spending limits based on role changes or requests.

**Arguments**:
- `card_id` (required): Card ID
- `limit` (required): Spending limit amount
- `interval` (optional): Limit interval (per_transaction, daily, monthly, default monthly)

**Example LLM prompt**: "Increase the spending limit on card card-101 to $10,000 per month"

---

## Brex API Notes

- **Account Types**: checking, savings, and credit accounts supported
- **Transaction Status**: posted -> pending -> settled lifecycle
- **Card Types**: Virtual cards for online purchases, physical cards for in-person spending
- **Spending Limits**: Set per card with per_transaction, daily, or monthly intervals
- **Currency**: Brex supports multiple currencies
- **Employee Roles**: admin (full access), member (spending only), viewer (read-only)

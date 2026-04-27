# Brex (API Key) Tools

Provider: `brex-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Brex API. They allow AI agents to manage transactions, cards, users, and accounts for business banking. Brex is a financial platform offering business accounts, corporate cards, and expense management.

## Authentication

**Nango API_KEY**:
- User provides Brex API token
- Token stored in Nango, accessed via `connection_ref`
- Bearer token in Authorization header

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `brex_get_account_info` | Get account info | GET | /v1/account_info |
| `brex_list_transactions` | List transactions | GET | /v1/transactions |
| `brex_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `brex_list_cards` | List cards | GET | /v1/cards |
| `brex_get_card` | Get card details | GET | /v1/cards/{cardId} |
| `brex_list_users` | List team members | GET | /v1/users |
| `brex_get_user` | Get user details | GET | /v1/users/{userId} |
| `brex_list_accounts` | List accounts | GET | /v1/accounts |
| `brex_get_balance` | Get account balance | GET | /v1/accounts/{accountId}/balance |
| `brex_get_limits` | Get card limits | GET | /v1/cards/{cardId}/limits |

---

## Tool Details

### brex_get_account_info

**What it does**: Gets Brex account information.

**When to use**: Verify account, check company info.

**Arguments**: None required

**Example LLM prompt**: "Get my Brex account info"

---

### brex_list_transactions

**What it does**: Lists all transactions in Brex.

**When to use**: View transaction history, reconcile expenses.

**Arguments**:
- `accountId` (optional): Filter by account ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all transactions from this month"

---

### brex_get_transaction

**What it does**: Gets details for a specific transaction.

**When to use**: View transaction details, check status.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction T-123"

---

### brex_list_cards

**What it does**: Lists all Brex cards.

**When to use**: View corporate cards, check card status.

**Arguments**:
- `userId` (optional): Filter by user ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active cards"

---

### brex_get_card

**What it does**: Gets details for a specific card.

**When to use**: Check card details, spending.

**Arguments**:
- `cardId` (required): Card ID

**Example LLM prompt**: "Get details for card C-456"

---

### brex_list_users

**What it does**: Lists all team members in Brex.

**When to use**: View team, manage access.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all team members"

---

### brex_get_user

**What it does**: Gets details for a specific user.

**When to use**: Check user details, permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user U-789"

---

### brex_list_accounts

**What it does**: Lists all Brex accounts.

**When to use**: View all business accounts.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all Brex accounts"

---

### brex_get_balance

**What it does**: Gets balance for a specific account.

**When to use**: Check account balance, cash flow.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get balance for account A-100"

---

### brex_get_limits

**What it does**: Gets spending limits for a card.

**When to use**: Check card limits, spending capacity.

**Arguments**:
- `cardId` (required): Card ID

**Example LLM prompt**: "Get limits for card C-456"

---

## Brex API Notes

- **Accounts**: Business banking accounts (checking, savings)
- **Cards**: Corporate cards assigned to users
- **Transactions**: All financial activity on accounts/cards
- **Users**: Team members with Brex access
- **Limits**: Per-card spending limits and restrictions

# Pleo Tools

Provider: `pleo` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Pleo API. They allow AI agents to manage users, cards, transactions, expenses, and receipts. Pleo is a company spending management platform. **Requires Pleo OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Pleo
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://{apiSubdomain}.pleo.io
- Requires subdomain and apiSubdomain in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pleo_list_users` | List users | GET | /v1/users |
| `pleo_get_user` | Get user details | GET | /v1/users/{userId} |
| `pleo_list_cards` | List cards | GET | /v1/cards |
| `pleo_get_card` | Get card details | GET | /v1/cards/{cardId} |
| `pleo_list_transactions` | List transactions | GET | /v1/transactions |
| `pleo_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `pleo_list_expenses` | List expenses | GET | /v1/expenses |
| `pleo_get_expense` | Get expense details | GET | /v1/expenses/{expenseId} |
| `pleo_list_receipts` | List receipts | GET | /v1/receipts |
| `pleo_get_company` | Get company info | GET | /v1/company |

---

## Tool Details

### pleo_list_users

**What it does**: Lists all users in the organization.

**When to use**: Browse team members.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### pleo_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, spending limits.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### pleo_list_cards

**What it does**: Lists all cards in the organization.

**When to use**: Browse company cards.

**Arguments**:
- `userId` (optional): Filter by user
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active cards"

---

### pleo_get_card

**What it does**: Gets detailed information about a specific card.

**When to use**: Get card details, limits.

**Arguments**:
- `cardId` (required): Card ID

**Example LLM prompt**: "Get details for card 67890"

---

### pleo_list_transactions

**What it does**: Lists all transactions.

**When to use**: Browse spending transactions.

**Arguments**:
- `cardId` (optional): Filter by card
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show transactions from this month"

---

### pleo_get_transaction

**What it does**: Gets detailed information about a specific transaction.

**When to use**: Get transaction details.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction 11111"

---

### pleo_list_expenses

**What it does**: Lists all expenses.

**When to use**: Browse expense reports.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending expenses"

---

### pleo_get_expense

**What it does**: Gets detailed information about a specific expense.

**When to use**: Get expense details.

**Arguments**:
- `expenseId` (required): Expense ID

**Example LLM prompt**: "Get details for expense 22222"

---

### pleo_list_receipts

**What it does**: Lists all receipts.

**When to use**: Browse receipt documents.

**Arguments**:
- `expenseId` (optional): Filter by expense

**Example LLM prompt**: "List receipts for expense 22222"

---

### pleo_get_company

**What it does**: Gets company information.

**When to use**: Get company settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

## Pleo API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Spending Management**: Cards, transactions, expenses
- **Rate limits**: Apply to API calls

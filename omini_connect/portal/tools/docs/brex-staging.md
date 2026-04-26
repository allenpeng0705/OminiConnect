# Brex (Staging) Tools

Provider: `brex-staging` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Brex API (Staging environment). They allow AI agents to manage transactions, cards, users, and accounts for business banking. The staging environment is for testing - use `brex` or `brex-api-key` for production.

**Staging Environment**: This provider uses `https://platform.staging.brexapps.com`. For production, use the `brex` or `brex-api-key` provider.

## Authentication

**Nango OAuth2**:
- User authenticates via Brex OAuth in staging
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `brex_staging_get_account_info` | Get account info | GET | /v1/account_info |
| `brex_staging_list_transactions` | List transactions | GET | /v1/transactions |
| `brex_staging_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `brex_staging_list_cards` | List cards | GET | /v1/cards |
| `brex_staging_get_card` | Get card details | GET | /v1/cards/{cardId} |
| `brex_staging_list_users` | List team members | GET | /v1/users |
| `brex_staging_get_user` | Get user details | GET | /v1/users/{userId} |
| `brex_staging_list_accounts` | List accounts | GET | /v1/accounts |
| `brex_staging_get_balance` | Get account balance | GET | /v1/accounts/{accountId}/balance |
| `brex_staging_get_limits` | Get card limits | GET | /v1/cards/{cardId}/limits |

---

## Tool Details

### brex_staging_get_account_info

**What it does**: Gets Brex staging account information.

**When to use**: Test account info retrieval.

**Arguments**: None required

**Example LLM prompt**: "Get my Brex staging account info"

---

### brex_staging_list_transactions

**What it does**: Lists all transactions in Brex staging.

**When to use**: Test transaction queries.

**Arguments**:
- `accountId` (optional): Filter by account ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all staging transactions"

---

### brex_staging_get_transaction

**What it does**: Gets details for a specific transaction.

**When to use**: Test transaction retrieval.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction T-123"

---

### brex_staging_list_cards

**What it does**: Lists all Brex cards in staging.

**When to use**: Test card queries.

**Arguments**:
- `userId` (optional): Filter by user ID
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all staging cards"

---

### brex_staging_get_card

**What it does**: Gets details for a specific card.

**When to use**: Test card retrieval.

**Arguments**:
- `cardId` (required): Card ID

**Example LLM prompt**: "Get details for card C-456"

---

### brex_staging_list_users

**What it does**: Lists all team members in Brex staging.

**When to use**: Test user queries.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all staging users"

---

### brex_staging_get_user

**What it does**: Gets details for a specific user.

**When to use**: Test user retrieval.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user U-789"

---

### brex_staging_list_accounts

**What it does**: Lists all Brex accounts in staging.

**When to use**: Test account queries.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all staging accounts"

---

### brex_staging_get_balance

**What it does**: Gets balance for a specific account.

**When to use**: Test balance queries.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get balance for account A-100"

---

### brex_staging_get_limits

**What it does**: Gets spending limits for a card.

**When to use**: Test limit queries.

**Arguments**:
- `cardId` (required): Card ID

**Example LLM prompt**: "Get limits for card C-456"

---

## Brex Staging API Notes

- **Staging**: Separate test environment from production
- **Test Data**: May include sample transactions and cards
- **No Real Money**: Financial operations don't process real money
- **OAuth2**: Uses OAuth authentication like production
- **Same Endpoints**: API structure matches production

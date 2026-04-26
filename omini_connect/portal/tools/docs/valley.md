# Valley Tools

Provider: `valley` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Valley is a banking platform for businesses. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `valley_list_accounts` | List all bank accounts | GET | /accounts |
| `valley_get_account` | Get account details | GET | /accounts/{id} |
| `valley_list_transactions` | List all transactions | GET | /transactions |
| `valley_get_transaction` | Get transaction details | GET | /transactions/{id} |
| `valley_create_transfer` | Create a transfer | POST | /transfers |
| `valley_list_payments` | List all payments | GET | /payments |
| `valley_create_payment` | Create a payment | POST | /payments |
| `valley_list_contacts` | List saved contacts | GET | /contacts |
| `valley_create_contact` | Create a contact | POST | /contacts |
| `valley_get_balance` | Get account balance | GET | /accounts/{id}/balance |

---

## Tool Details

### valley_list_accounts

**What it does**: List all bank accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_get_account

**What it does**: Get account details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_list_transactions

**What it does**: List all transactions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_get_transaction

**What it does**: Get transaction details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_create_transfer

**What it does**: Create a transfer

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_list_payments

**What it does**: List all payments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_create_payment

**What it does**: Create a payment

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_list_contacts

**What it does**: List saved contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_create_contact

**What it does**: Create a contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### valley_get_balance

**What it does**: Get account balance

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.joinvalley.co/api/`
- Docs: https://nango.dev/docs/integrations/all/valley

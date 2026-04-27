# Valley (API Key) Tools

Provider: `valley-api-key` | Engine: `nango` | Auth: API Key via Nango

## Overview

Valley is a banking platform for businesses with API key auth. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Valley (API Key) API key
- Key stored securely in Nango

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

- Base URL: `https://api.joinvalley.co/api`
- Docs: https://nango.dev/docs/integrations/all/valley-api-key

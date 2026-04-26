# Wise (API Key) Tools

Provider: `wise-api-key` | Engine: `nango` | Auth: API Key via Nango

## Overview

Wise is a global payments platform for international transfers. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Wise (API Key) API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wise_list_profiles` | List all user profiles | GET | /v1/profiles |
| `wise_get_profile` | Get profile details | GET | /v1/profiles/{id} |
| `wise_list_balances` | List all account balances | GET | /v1/profiles/{profile_id}/balances |
| `wise_list_accounts` | List all accounts | GET | /v1/accounts |
| `wise_create_account` | Create a new account | POST | /v1/accounts |
| `wise_list_transfers` | List all transfers | GET | /v1/transfers |
| `wise_create_transfer` | Create a new transfer | POST | /v1/transfers |
| `wise_get_quote` | Get a transfer quote | POST | /v1/quotes |
| `wise_list_recipients` | List all recipients | GET | /v1/recipients |
| `wise_create_recipient` | Create a new recipient | POST | /v1/recipients |

---

## Tool Details

### wise_list_profiles

**What it does**: List all user profiles

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_get_profile

**What it does**: Get profile details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_list_balances

**What it does**: List all account balances

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_list_accounts

**What it does**: List all accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_create_account

**What it does**: Create a new account

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_list_transfers

**What it does**: List all transfers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_create_transfer

**What it does**: Create a new transfer

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_get_quote

**What it does**: Get a transfer quote

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_list_recipients

**What it does**: List all recipients

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wise_create_recipient

**What it does**: Create a new recipient

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.wise.com`
- Docs: https://nango.dev/docs/integrations/all/wise-api-key

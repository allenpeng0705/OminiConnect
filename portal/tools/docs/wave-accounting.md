# Wave Accounting Tools

Provider: `wave-accounting` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Wave Accounting is a free accounting software for small businesses. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wave Accounting
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wave_list_accounts` | List all accounts | POST | /graphql |
| `wave_get_account` | Get account details | POST | /graphql |
| `wave_list_invoices` | List all invoices | POST | /graphql |
| `wave_get_invoice` | Get invoice details | POST | /graphql |
| `wave_create_invoice` | Create a new invoice | POST | /graphql |
| `wave_list_customers` | List all customers | POST | /graphql |
| `wave_get_customer` | Get customer details | POST | /graphql |
| `wave_create_customer` | Create a new customer | POST | /graphql |
| `wave_list_transactions` | List all transactions | POST | /graphql |
| `wave_list_vendors` | List all vendors | POST | /graphql |

---

## Tool Details

### wave_list_accounts

**What it does**: List all accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_get_account

**What it does**: Get account details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_get_invoice

**What it does**: Get invoice details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_create_invoice

**What it does**: Create a new invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_list_customers

**What it does**: List all customers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_get_customer

**What it does**: Get customer details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_create_customer

**What it does**: Create a new customer

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_list_transactions

**What it does**: List all transactions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wave_list_vendors

**What it does**: List all vendors

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://gql.waveapps.com`
- Docs: https://nango.dev/docs/integrations/all/wave-accounting

# Twinfield Tools

Provider: `twinfield` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Twinfield is a cloud accounting software for businesses. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Twinfield
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twinfield_list_customers` | List all customers/debtors | GET | /api/v1/customers |
| `twinfield_get_customer` | Get customer details | GET | /api/v1/customers/{id} |
| `twinfield_list_suppliers` | List all suppliers/creditors | GET | /api/v1/suppliers |
| `twinfield_get_supplier` | Get supplier details | GET | /api/v1/suppliers/{id} |
| `twinfield_list_invoices` | List all invoices | GET | /api/v1/invoices |
| `twinfield_create_invoice` | Create a new invoice | POST | /api/v1/invoices |
| `twinfield_list_transactions` | List all transactions | GET | /api/v1/transactions |
| `twinfield_get_balance` | Get account balance | GET | /api/v1/accounts/{id}/balance |
| `twinfield_list_dimensions` | List all dimensions | GET | /api/v1/dimensions |
| `twinfield_create_transaction` | Create a new transaction | POST | /api/v1/transactions |

---

## Tool Details

### twinfield_list_customers

**What it does**: List all customers/debtors

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_get_customer

**What it does**: Get customer details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_list_suppliers

**What it does**: List all suppliers/creditors

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_get_supplier

**What it does**: Get supplier details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_create_invoice

**What it does**: Create a new invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_list_transactions

**What it does**: List all transactions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_get_balance

**What it does**: Get account balance

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_list_dimensions

**What it does**: List all dimensions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twinfield_create_transaction

**What it does**: Create a new transaction

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.cluster}.twinfield.com`
- Docs: https://nango.dev/docs/integrations/all/twinfield

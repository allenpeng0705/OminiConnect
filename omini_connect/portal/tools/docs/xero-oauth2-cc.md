# Xero (OAuth2 CC) Tools

Provider: `xero-oauth2-cc` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

Xero is a cloud accounting platform for small businesses. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `xero_list_tenants` | List all connected tenants | GET | /connections |
| `xero_list_invoices` | List all invoices | GET | /api.xro/2.0/Invoices |
| `xero_create_invoice` | Create a new invoice | POST | /api.xro/2.0/Invoices |
| `xero_list_contacts` | List all contacts | GET | /api.xro/2.0/Contacts |
| `xero_get_contact` | Get contact details | GET | /api.xro/2.0/Contacts/{id} |
| `xero_list_bank_transactions` | List all bank transactions | GET | /api.xro/2.0/BankTransactions |
| `xero_list_bank_transfers` | List all bank transfers | GET | /api.xro/2.0/BankTransfers |
| `xero_list_accounts` | List all accounts | GET | /api.xro/2.0/Accounts |
| `xero_list_tracking_categories` | List tracking categories | GET | /api.xro/2.0/TrackingCategories |
| `xero_get_organisations` | Get organisation details | GET | /api.xro/2.0/Organisations |

---

## Tool Details

### xero_list_tenants

**What it does**: List all connected tenants

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_create_invoice

**What it does**: Create a new invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_bank_transactions

**What it does**: List all bank transactions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_bank_transfers

**What it does**: List all bank transfers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_accounts

**What it does**: List all accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_list_tracking_categories

**What it does**: List tracking categories

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### xero_get_organisations

**What it does**: Get organisation details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.xero.com`
- Docs: https://nango.dev/docs/integrations/all/xero-oauth2-cc

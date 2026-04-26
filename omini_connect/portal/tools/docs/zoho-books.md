# Zoho Books Tools

Provider: `zoho-books` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Books is an online accounting software. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Books
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_books_list_invoices` | List all invoices | GET | /invoices |
| `zoho_books_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `zoho_books_create_invoice` | Create a new invoice | POST | /invoices |
| `zoho_books_list_contacts` | List all contacts | GET | /contacts |
| `zoho_books_get_contact` | Get contact details | GET | /contacts/{id} |
| `zoho_books_create_contact` | Create a new contact | POST | /contacts |
| `zoho_books_list_bills` | List all bills | GET | /bills |
| `zoho_books_get_bill` | Get bill details | GET | /bills/{id} |
| `zoho_books_list_accounts` | List all accounts | GET | /chartofaccounts |
| `zoho_books_get_balance` | Get organization balance | GET | /organizations/{org_id}/balance |

---

## Tool Details

### zoho_books_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_get_invoice

**What it does**: Get invoice details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_create_invoice

**What it does**: Create a new invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_list_bills

**What it does**: List all bills

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_get_bill

**What it does**: Get bill details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_list_accounts

**What it does**: List all accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_books_get_balance

**What it does**: Get organization balance

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://www.zohoapis.com/books/v3`
- Docs: https://nango.dev/docs/integrations/all/zoho-books

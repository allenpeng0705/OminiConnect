# Zoho Invoice Tools

Provider: `zoho-invoice` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Invoice is an online invoicing software. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Invoice
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_invoice_list_invoices` | List all invoices | GET | /invoices |
| `zoho_invoice_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `zoho_invoice_create_invoice` | Create a new invoice | POST | /invoices |
| `zoho_invoice_email_invoice` | Email an invoice to customer | POST | /invoices/{id}/email |
| `zoho_invoice_list_contacts` | List all contacts | GET | /contacts |
| `zoho_invoice_get_contact` | Get contact details | GET | /contacts/{id} |
| `zoho_invoice_create_contact` | Create a new contact | POST | /contacts |
| `zoho_invoice_list_estimates` | List all estimates | GET | /estimates |
| `zoho_invoice_create_estimate` | Create a new estimate | POST | /estimates |
| `zoho_invoice_list_recurring_invoices` | List all recurring invoices | GET | /recurringinvoices |

---

## Tool Details

### zoho_invoice_list_invoices

**What it does**: List all invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_get_invoice

**What it does**: Get invoice details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_create_invoice

**What it does**: Create a new invoice

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_email_invoice

**What it does**: Email an invoice to customer

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_list_estimates

**What it does**: List all estimates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_create_estimate

**What it does**: Create a new estimate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_invoice_list_recurring_invoices

**What it does**: List all recurring invoices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://www.zohoapis.com/invoice/v2`
- Docs: https://nango.dev/docs/integrations/all/zoho-invoice

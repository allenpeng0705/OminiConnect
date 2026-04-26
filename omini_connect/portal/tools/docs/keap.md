# Keap Tools

Provider: `keap` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Keap API. They allow AI agents to manage contacts, companies, invoices, products, and notes. **Requires Keap OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Keap
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.infusionsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `keap_list_contacts` | List contacts | GET | /rest/v1/contacts |
| `keap_get_contact` | Get contact details | GET | /rest/v1/contacts/{contact_id} |
| `keap_list_companies` | List companies | GET | /rest/v1/companies |
| `keap_get_company` | Get company details | GET | /rest/v1/companies/{company_id} |
| `keap_list_invoices` | List invoices | GET | /rest/v1/invoices |
| `keap_get_invoice` | Get invoice details | GET | /rest/v1/invoices/{invoice_id} |
| `keap_list_products` | List products | GET | /rest/v1/products |
| `keap_get_product` | Get product details | GET | /rest/v1/products/{product_id} |
| `keap_list_tags` | List tags | GET | /rest/v1/tags |
| `keap_list_notes` | List notes | GET | /rest/v1/notes |

---

## Tool Details

### keap_list_contacts

**What it does**: Lists all contacts in Keap.

**When to use**: Find contacts, view contact list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all contacts in Keap"

---

### keap_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information, view contact profile.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### keap_list_companies

**What it does**: Lists all companies.

**When to use**: View companies, find business accounts.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all companies in Keap"

---

### keap_get_company

**What it does**: Gets details for a specific company.

**When to use**: Get company information.

**Arguments**:
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for company 67890"

---

### keap_list_invoices

**What it does**: Lists all invoices.

**When to use**: View invoices, track payments.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all invoices in Keap"

---

### keap_get_invoice

**What it does**: Gets details for a specific invoice.

**When to use**: Get invoice information.

**Arguments**:
- `invoice_id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice inv1"

---

### keap_list_products

**What it does**: Lists all products.

**When to use**: View products, find catalog items.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all products in Keap"

---

### keap_get_product

**What it does**: Gets details for a specific product.

**When to use**: Get product information.

**Arguments**:
- `product_id` (required): Product ID

**Example LLM prompt**: "Get details for product p1"

---

### keap_list_tags

**What it does**: Lists all tags.

**When to use**: View tags, organize contacts.

**Arguments**: None

**Example LLM prompt**: "List all tags in Keap"

---

### keap_list_notes

**What it does**: Lists all notes.

**When to use**: View notes, find contact notes.

**Arguments**:
- `contact_id` (optional): Filter by contact ID
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all notes for contact 12345"

---

## Keap API Notes

- **CRM**: Customer relationship management for small businesses
- **IDs**: Numeric IDs for contacts, companies, invoices, products
- **Notes**: Associated with contacts
- **Tags**: Categorize contacts

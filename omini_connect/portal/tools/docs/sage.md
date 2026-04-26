# Sage Tools

Provider: `sage` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Sage is an accounting and financial management platform. These tools allow AI agents to manage contacts, invoices, payments, products, and account balances.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Sage
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:read`, `contacts:write`, `invoices:read`, `invoices:write`, `payments:read`, `payments:write`, `products:read`, `balance:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sage_list_contacts` | List all contacts | GET | /v3/contacts |
| `sage_get_contact` | Get contact details | GET | /v3/contacts/{contactId} |
| `sage_create_contact` | Create a contact | POST | /v3/contacts |
| `sage_list_invoices` | List all invoices | GET | /v3/invoices |
| `sage_get_invoice` | Get invoice details | GET | /v3/invoices/{invoiceId} |
| `sage_create_invoice` | Create an invoice | POST | /v3/invoices |
| `sage_list_payments` | List all payments | GET | /v3/payments |
| `sage_create_payment` | Record a payment | POST | /v3/payments |
| `sage_list_products` | List all products | GET | /v3/products |
| `sage_get_balance` | Get account balance | GET | /v3/account-balance |

---

## Tool Details

### sage_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: View customers and suppliers.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)
- `type` (optional): Contact type (customer, supplier)

**Example LLM prompt**: "List all customer contacts"

---

### sage_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Get contact information and payment details.

**Arguments**:
- `contactId` (required): The contact ID

**Example LLM prompt**: "Get details for contact cnt_abc123"

---

### sage_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new customers or suppliers.

**Arguments**:
- `name` (required): Contact name
- `contactType` (optional): Type (customer or supplier, default customer)
- `email` (optional): Email address

**Example LLM prompt**: "Create a customer contact for 'Acme Corp'"

---

### sage_list_invoices

**What it does**: Returns a list of all invoices.

**When to use**: View invoice history.

**Arguments**:
- `limit` (optional): Number of invoices (default 50)
- `status` (optional): Filter by status (draft, sent, paid)

**Example LLM prompt**: "List all unpaid invoices"

---

### sage_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: Get invoice total and line items.

**Arguments**:
- `invoiceId` (required): The invoice ID

**Example LLM prompt**: "Get details for invoice inv_xyz789"

---

### sage_create_invoice

**What it does**: Creates a new invoice.

**When to use**: Bill customers for products or services.

**Arguments**:
- `contactId` (required): Contact ID
- `reference` (optional): Invoice reference
- `lineItems` (optional): Array of line items

**Example LLM prompt**: "Create an invoice for contact cnt_123"

---

### sage_list_payments

**What it does**: Returns a list of all payments.

**When to use**: View payment history.

**Arguments**:
- `limit` (optional): Number of payments (default 50)

**Example LLM prompt**: "List all recent payments"

---

### sage_create_payment

**What it does**: Records a payment received.

**When to use**: Mark an invoice as paid.

**Arguments**:
- `invoiceId` (required): Invoice ID
- `amount` (required): Payment amount
- `date` (optional): Payment date

**Example LLM prompt**: "Record payment of $500 for invoice inv_123"

---

### sage_list_products

**What it does**: Returns a list of all products.

**When to use**: View product catalog.

**Arguments**:
- `limit` (optional): Number of products (default 50)

**Example LLM prompt**: "List all products"

---

### sage_get_balance

**What it does**: Returns account balance information.

**When to use**: Check available balance.

**Arguments**: None

**Example LLM prompt**: "What's my account balance?"

---

## Sage Notes

- Contacts are customers and suppliers
- Invoices track billing to contacts
- Payments record money received
- Products are items for sale
- Balance shows available funds

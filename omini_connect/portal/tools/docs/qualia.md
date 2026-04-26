# Qualia Tools

Provider: `qualia` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Qualia API. They allow AI agents to manage transactions, orders, documents, contacts, and products. Qualia is a platform for legal and real estate transactions. **Requires Qualia Basic Authentication.**

## Authentication

**Nango Basic Auth**:
- Uses username and password for Basic authentication
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://{subdomain}.qualia.io/api
- Requires subdomain in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `qualia_list_transactions` | List transactions | GET | /transactions |
| `qualia_get_transaction` | Get transaction details | GET | /transactions/{transactionId} |
| `qualia_list_orders` | List orders | GET | /orders |
| `qualia_get_order` | Get order details | GET | /orders/{orderId} |
| `qualia_list_documents` | List documents | GET | /documents |
| `qualia_get_document` | Get document details | GET | /documents/{documentId} |
| `qualia_list_contacts` | List contacts | GET | /contacts |
| `qualia_get_contact` | Get contact details | GET | /contacts/{contactId} |
| `qualia_list_products` | List products | GET | /products |
| `qualia_get_company` | Get company info | GET | /company |

---

## Tool Details

### qualia_list_transactions

**What it does**: Lists all transactions.

**When to use**: Browse transaction history.

**Arguments**:
- `status` (optional): Filter by status
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "List all transactions from this month"

---

### qualia_get_transaction

**What it does**: Gets detailed information about a specific transaction.

**When to use**: Get transaction details.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction 12345"

---

### qualia_list_orders

**What it does**: Lists all orders.

**When to use**: Browse order history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all orders"

---

### qualia_get_order

**What it does**: Gets detailed information about a specific order.

**When to use**: Get order details.

**Arguments**:
- `orderId` (required): Order ID

**Example LLM prompt**: "Get details for order 67890"

---

### qualia_list_documents

**What it does**: Lists all documents.

**When to use**: Browse document repository.

**Arguments**:
- `transactionId` (optional): Filter by transaction

**Example LLM prompt**: "List documents for transaction 12345"

---

### qualia_get_document

**What it does**: Gets detailed information about a specific document.

**When to use**: Get document details.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "Get details for document 11111"

---

### qualia_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact directory.

**Arguments**: None

**Example LLM prompt**: "List all contacts"

---

### qualia_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact 22222"

---

### qualia_list_products

**What it does**: Lists all products.

**When to use**: Browse product catalog.

**Arguments**: None

**Example LLM prompt**: "List all products"

---

### qualia_get_company

**What it does**: Gets company information.

**When to use**: Get company settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

## Qualia API Notes

- **Basic Auth**: Uses username/password for authentication
- **Legal/Real Estate**: Transaction management
- **Rate limits**: Apply to API calls

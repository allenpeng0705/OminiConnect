# DATEV Tools

Provider: `datev` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the DATEV API. They allow AI agents to manage clients, accounts, vouchers, contacts, and documents. DATEV is a German accounting and tax software provider used by accountants and businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with DATEV
- Token stored in Nango, accessed via `connection_ref`
- Default scope: `openid`
- Uses OpenID Connect with basic auth for token requests

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `datev_list_clients` | List client accounts | GET | /v1/clients |
| `datev_get_client` | Get client details | GET | /v1/clients/{client_id} |
| `datev_list_accounts` | List accounting accounts | GET | /v1/clients/{client_id}/accounts |
| `datev_get_account` | Get account details | GET | /v1/clients/{client_id}/accounts/{account_id} |
| `datev_list_vouchers` | List vouchers/entries | GET | /v1/clients/{client_id}/vouchers |
| `datev_get_voucher` | Get voucher details | GET | /v1/clients/{client_id}/vouchers/{voucher_id} |
| `datev_list_contacts` | List contacts | GET | /v1/clients/{client_id}/contacts |
| `datev_get_contact` | Get contact details | GET | /v1/clients/{client_id}/contacts/{contact_id} |
| `datev_list_documents` | List documents | GET | /v1/clients/{client_id}/documents |
| `datev_get_document` | Get document details | GET | /v1/clients/{client_id}/documents/{document_id} |

---

## Tool Details

### datev_list_clients

**What it does**: Lists all clients/accounting offices accessible to the user.

**When to use**: Browse available clients, select client for operations, manage client access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all DATEV clients"

---

### datev_get_client

**What it does**: Gets detailed client information.

**When to use**: Review client details, check client configuration, verify client access.

**Arguments**:
- `client_id` (required): Client ID

**Example LLM prompt**: "Get details for client c-123"

---

### datev_list_accounts

**What it does**: Lists all accounting accounts for a specific client.

**When to use**: View chart of accounts, find specific accounts, review account structure.

**Arguments**:
- `client_id` (required): Client ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all accounts for client c-123"

---

### datev_get_account

**What it does**: Gets detailed account information including balance.

**When to use**: Check account balance, review account transactions, verify account settings.

**Arguments**:
- `client_id` (required): Client ID
- `account_id` (required): Account ID

**Example LLM prompt**: "Get details for account 4000 for client c-123"

---

### datev_list_vouchers

**What it does**: Lists all accounting vouchers (entries) for a client.

**When to use**: View transaction history, find specific entries, audit accounting records.

**Arguments**:
- `client_id` (required): Client ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all vouchers for client c-123 from January 2024"

---

### datev_get_voucher

**What it does**: Gets detailed voucher information including line items.

**When to use**: Review specific transaction, check voucher postings, verify accounting entries.

**Arguments**:
- `client_id` (required): Client ID
- `voucher_id` (required): Voucher ID

**Example LLM prompt**: "Get details for voucher v-456 for client c-123"

---

### datev_list_contacts

**What it does**: Lists all contacts (vendors and customers) for a client.

**When to use**: Browse contact database, find vendors/customers, manage business relationships.

**Arguments**:
- `client_id` (required): Client ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all contacts for client c-123"

---

### datev_get_contact

**What it does**: Gets detailed contact information.

**When to use**: Review vendor/customer details, check contact information, verify business partner.

**Arguments**:
- `client_id` (required): Client ID
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact ct-789 for client c-123"

---

### datev_list_documents

**What it does**: Lists all documents for a client.

**When to use**: Browse attached documents, find supporting files, manage document storage.

**Arguments**:
- `client_id` (required): Client ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all documents for client c-123"

---

### datev_get_document

**What it does**: Gets detailed document information.

**When to use**: Check document metadata, verify document attachments, retrieve document.

**Arguments**:
- `client_id` (required): Client ID
- `document_id` (required): Document ID

**Example LLM prompt**: "Get details for document d-101 for client c-123"

---

## DATEV API Notes

- **DATEV**: German accounting and tax software for accountants and businesses
- **Clients**: Accounting offices or business entities managed in DATEV
- **Accounts**: Chart of accounts for double-entry bookkeeping (German SKR)
- **Vouchers**: Accounting entries with debit/credit postings
- **Contacts**: Business partners - vendors, customers, and other entities
- **Documents**: Attached files and supporting documentation
- **Scope**: Default scope is `openid` for authentication

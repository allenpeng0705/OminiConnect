# DocuSign Tools

Provider: `docusign` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the DocuSign eSignature REST API v2.1. They allow AI agents to create, send, and manage electronic signatures on documents. DocuSign is the industry-leading platform for digital document signing and contract workflows.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with DocuSign
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `signature`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `docusign_list_envelopes` | List envelopes with filtering | GET | /v2.1/accounts/{accountId}/envelopes |
| `docusign_get_envelope` | Get envelope details | GET | /v2.1/accounts/{accountId}/envelopes/{envelopeId} |
| `docusign_create_envelope` | Create an envelope | POST | /v2.1/accounts/{accountId}/envelopes |
| `docusign_send_envelope` | Send envelope for signature | PUT | /v2.1/accounts/{accountId}/envelopes/{envelopeId} |
| `docusign_list_documents` | List documents in envelope | GET | /v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents |
| `docusign_get_document` | Get document details | GET | /v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId} |
| `docusign_list_recipients` | List envelope recipients | GET | /v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients |
| `docusign_get_recipient` | Get recipient details | GET | /v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId} |
| `docusign_list_templates` | List templates | GET | /v2.1/accounts/{accountId}/templates |
| `docusign_get_template` | Get template details | GET | /v2.1/accounts/{accountId}/templates/{templateId} |

---

## Tool Details

### docusign_list_envelopes

**What it does**: Lists envelopes with optional filtering by status, date range, or folder.

**When to use**: Find envelopes by status (sent, completed, voided), search sent documents, track signature requests.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `status` (optional): Filter by status (`created`, `sent`, `delivered`, `completed`, `declined`, `voided`)
- `from_date` (optional): Start date (ISO 8601)
- `to_date` (optional): End date (ISO 8601)
- `folder` (optional): Filter by folder (`inbox`, `sent`, `drafts`)
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset (default 0)

**Example LLM prompt**: "List all envelopes sent in the last 30 days that are still pending signature"

---

### docusign_get_envelope

**What it does**: Gets detailed information about a specific envelope including status and recipients.

**When to use**: Check envelope status, see who has signed, view document details.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "Get details for envelope abc-123-def-456 to check signature status"

---

### docusign_create_envelope

**What it does**: Creates a new envelope definition with documents and recipients.

**When to use**: Prepare a new document for signature, define who needs to sign and in what order.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `email_subject` (required): Email subject
- `template_id` (optional): Template ID to use
- `documents` (optional): Array of document objects
- `recipients` (optional): Recipients with signers

**Example LLM prompt**: "Create an envelope with subject 'Please sign your NDA' and attach the document from our OneDrive"

---

### docusign_send_envelope

**What it does**: Sends an existing envelope draft for signature.

**When to use**: Send out a prepared envelope, release documents for signing.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID to send

**Example LLM prompt**: "Send envelope abc-123 to all recipients now"

---

### docusign_list_documents

**What it does**: Lists all documents in an envelope with their IDs and names.

**When to use**: See what documents are included in an envelope before sending or after receiving.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "List all documents in envelope abc-123"

---

### docusign_get_document

**What it does**: Downloads or retrieves a specific document from an envelope.

**When to use**: Get the actual document content for review or storage.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID
- `document_id` (required): Document ID

**Example LLM prompt**: "Download the signed NDA document from envelope abc-123"

---

### docusign_list_recipients

**What it does**: Lists all recipients for an envelope including signers, cc, and witness.

**When to use**: See who needs to sign, check recipient status, verify signature order.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "List all recipients for envelope abc-123 and their signing status"

---

### docusign_get_recipient

**What it does**: Gets detailed information about a specific recipient including signature status and timing.

**When to use**: Check if a specific person has signed, view when they received and signed the document.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `envelope_id` (required): Envelope ID
- `recipient_id` (required): Recipient ID

**Example LLM prompt**: "Check if john@example.com has signed the NDA yet"

---

### docusign_list_templates

**What it does**: Lists all templates available in the account for creating envelopes.

**When to use**: Find pre-made templates for common documents like NDAs, contracts, or agreements.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `search_text` (optional): Search templates by name
- `folder_id` (optional): Filter by folder ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all templates that contain 'NDA' in the name"

---

### docusign_get_template

**What it does**: Gets detailed information about a template including structure and recipients.

**When to use**: Understand template fields and requirements before creating an envelope from it.

**Arguments**:
- `account_id` (required): DocuSign account ID
- `template_id` (required): Template ID

**Example LLM prompt**: "Get the structure of template template-456 to see what fields need to be filled"
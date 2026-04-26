# BoldSign Tools

Provider: `boldsign` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the BoldSign API. They allow AI agents to create, send, and manage electronic signatures on documents. BoldSign is an e-signature platform for businesses to collect signatures on contracts and agreements.

## Authentication

**Nango OAuth2**:
- User authenticates via BoldSign OAuth
- Token stored in Nango, accessed via `connection_ref`
- Scopes for document and template management

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `boldsign_list_documents` | List documents | GET | /v1/documents |
| `boldsign_get_document` | Get document details | GET | /v1/documents/{documentId} |
| `boldsign_create_document` | Create a document for signing | POST | /v1/documents |
| `boldsign_send_document` | Send document for signature | POST | /v1/documents/{documentId}/send |
| `boldsign_list_templates` | List templates | GET | /v1/templates |
| `boldsign_get_template` | Get template details | GET | /v1/templates/{templateId} |
| `boldsign_list_signers` | List signers | GET | /v1/signers |
| `boldsign_get_signer` | Get signer details | GET | /v1/signers/{signerId} |
| `boldsign_list_teams` | List teams | GET | /v1/teams |
| `boldsign_get_settings` | Get team settings | GET | /v1/settings |

---

## Tool Details

### boldsign_list_documents

**What it does**: Lists all documents in BoldSign.

**When to use**: Browse documents, find specific ones.

**Arguments**:
- `status` (optional): Filter by status (draft, sent, signed, etc.)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Documents per page (default 20)

**Example LLM prompt**: "List all pending documents for signature"

---

### boldsign_get_document

**What it does**: Gets details for a specific document.

**When to use**: Check document status, view signers.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "Get details for document D-123"

---

### boldsign_create_document

**What it does**: Creates a new document for signing.

**When to use**: Prepare a new document for e-signature.

**Arguments**:
- `title` (required): Document title
- `subject` (optional): Email subject
- `signers` (required): Array of signer objects
- `files` (required): Array of file URLs or base64 content

**Example LLM prompt**: "Create a document 'Contract' with signer email@example.com"

---

### boldsign_send_document

**What it does**: Sends an existing document for signature.

**When to use**: Initiate signing workflow for draft documents.

**Arguments**:
- `documentId` (required): Document ID
- `signers` (required): Array of signer objects

**Example LLM prompt**: "Send document D-123 to john@company.com"

---

### boldsign_list_templates

**What it does**: Lists all templates in BoldSign.

**When to use**: Find reusable document templates.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Templates per page (default 20)

**Example LLM prompt**: "List all templates"

---

### boldsign_get_template

**What it does**: Gets details for a specific template.

**When to use**: View template structure, required fields.

**Arguments**:
- `templateId` (required): Template ID

**Example LLM prompt**: "Get details for template T-456"

---

### boldsign_list_signers

**What it does**: Lists all signers in BoldSign.

**When to use**: View signer database, find contacts.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Signers per page (default 20)

**Example LLM prompt**: "List all signers"

---

### boldsign_get_signer

**What it does**: Gets details for a specific signer.

**When to use**: View signer information, signing history.

**Arguments**:
- `signerId` (required): Signer ID

**Example LLM prompt**: "Get details for signer S-789"

---

### boldsign_list_teams

**What it does**: Lists all teams in BoldSign.

**When to use**: View organizational structure.

**Arguments**: None required

**Example LLM prompt**: "List all teams"

---

### boldsign_get_settings

**What it does**: Gets team settings for BoldSign.

**When to use**: Check account configuration.

**Arguments**: None required

**Example LLM prompt**: "Get my team settings"

---

## BoldSign API Notes

- **Documents**: Created from files or templates for signing
- **Signers**: Recipients who need to sign documents
- **Templates**: Reusable document formats with pre-defined fields
- **Status Flow**: draft -> sent -> viewed -> signed -> completed
- **E-Signatures**: Legally binding electronic signatures

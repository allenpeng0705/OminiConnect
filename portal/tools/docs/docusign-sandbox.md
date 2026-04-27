# DocuSign Sandbox Tools

Provider: `docusign-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the DocuSign Sandbox API for testing purposes. They allow AI agents to manage envelopes, documents, recipients, templates, and users. DocuSign is an electronic signature platform.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with DocuSign Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Uses sandbox environment (account-d.docusign.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `docusign_sandbox_list_envelopes` | List envelopes | GET | /v2.1/envelopes |
| `docusign_sandbox_get_envelope` | Get envelope details | GET | /v2.1/envelopes/{envelope_id} |
| `docusign_sandbox_create_envelope` | Create an envelope | POST | /v2.1/envelopes |
| `docusign_sandbox_list_documents` | List documents | GET | /v2.1/envelopes/{envelope_id}/documents |
| `docusign_sandbox_get_document` | Get document details | GET | /v2.1/envelopes/{envelope_id}/documents/{document_id} |
| `docusign_sandbox_list_recipients` | List recipients | GET | /v2.1/envelopes/{envelope_id}/recipients |
| `docusign_sandbox_get_recipient` | Get recipient details | GET | /v2.1/envelopes/{envelope_id}/recipients/{recipient_id} |
| `docusign_sandbox_list_templates` | List templates | GET | /v2.1/templates |
| `docusign_sandbox_get_template` | Get template details | GET | /v2.1/templates/{template_id} |
| `docusign_sandbox_list_users` | List users | GET | /v2.1/users |

---

## Tool Details

### docusign_sandbox_list_envelopes

**What it does**: Lists all envelopes in the sandbox.

**When to use**: View sent documents, track signature status, find envelopes.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `status` (optional): Filter by created, sent, delivered, signed, or completed

**Example LLM prompt**: "List all completed envelopes"

---

### docusign_sandbox_get_envelope

**What it does**: Gets detailed envelope information.

**When to use**: Check envelope status, review signing progress, get envelope details.

**Arguments**:
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "Get details for envelope e-123"

---

### docusign_sandbox_create_envelope

**What it does**: Creates and sends a new envelope.

**When to use**: Send documents for signature, initiate signing workflows, request signatures.

**Arguments**:
- `email_subject` (required): Email subject
- `documents` (optional): Array of documents to include
- `recipients` (optional): Array of recipients

**Example LLM prompt**: "Create an envelope with subject 'Please sign' for recipient john@example.com"

---

### docusign_sandbox_list_documents

**What it does**: Lists all documents in an envelope.

**When to use**: View envelope contents, find attached files, check document list.

**Arguments**:
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "List all documents in envelope e-123"

---

### docusign_sandbox_get_document

**What it does**: Gets detailed document information.

**When to use**: Check document details, verify document content, get document metadata.

**Arguments**:
- `envelope_id` (required): Envelope ID
- `document_id` (required): Document ID

**Example LLM prompt**: "Get details for document d-456 in envelope e-123"

---

### docusign_sandbox_list_recipients

**What it does**: Lists all recipients of an envelope.

**When to use**: View signer list, check recipient status, track who needs to sign.

**Arguments**:
- `envelope_id` (required): Envelope ID

**Example LLM prompt**: "List all recipients of envelope e-123"

---

### docusign_sandbox_get_recipient

**What it does**: Gets detailed recipient information.

**When to use**: Check recipient status, verify signing progress, get recipient details.

**Arguments**:
- `envelope_id` (required): Envelope ID
- `recipient_id` (required): Recipient ID

**Example LLM prompt**: "Get details for recipient r-789 in envelope e-123"

---

### docusign_sandbox_list_templates

**What it does**: Lists all templates in the sandbox account.

**When to use**: Browse available templates, find reusable documents, view template library.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all templates"

---

### docusign_sandbox_get_template

**What it does**: Gets detailed template information.

**When to use**: Review template details, check template settings, understand template structure.

**Arguments**:
- `template_id` (required): Template ID

**Example LLM prompt**: "Get details for template t-101"

---

### docusign_sandbox_list_users

**What it does**: Lists all users in the sandbox account.

**When to use**: View account users, find specific users, manage user access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all sandbox users"

---

## DocuSign Sandbox API Notes

- **Sandbox Only**: This environment is for testing and development
- **Same API Structure**: Identical endpoints to production DocuSign API
- **Sandbox URLs**: Uses account-d.docusign.com for OAuth and demo.docusign.net for API
- **Mock Data**: Contains synthetic or test data
- **Envelopes**: Containers for documents sent for signature
- **Recipients**: Signers and approvers of documents
- **Templates**: Reusable document patterns for common signatures
- **Post Connection Script**: Runs additional setup after OAuth connection

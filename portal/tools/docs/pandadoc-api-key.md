# Pandadoc (API Key) Tools

Provider: `pandadoc-api-key` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Pandadoc API for document management and e-signatures. They allow AI agents to manage documents, templates, folders, and track document analytics. **Requires Pandadoc API key authentication.**

## Authentication

**API Key**:
- User provides Pandadoc API key
- Key passed via `Authorization: API-Key` header
- Base URL: `https://api.pandadoc.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pandadoc_list_documents` | List documents | GET | /public/v1/documents |
| `pandadoc_get_document` | Get document details | GET | /public/v1/documents/{id} |
| `pandadoc_create_document` | Create document | POST | /public/v1/documents |
| `pandadoc_send_document` | Send document | POST | /public/v1/documents/{id}/send |
| `pandadoc_list_templates` | List templates | GET | /public/v1/templates |
| `pandadoc_get_template` | Get template details | GET | /public/v1/templates/{id} |
| `pandadoc_list_folders` | List folders | GET | /public/v1/folders |
| `pandadoc_get_member` | Get member details | GET | /public/v1/members/current |
| `pandadoc_list_webhooks` | List webhooks | GET | /public/v1/webhooks |
| `pandadoc_get_analytics` | Get document analytics | GET | /public/v1/documents/{id}/analytics |

---

## Tool Details

### pandadoc_list_documents

**What it does**: Lists all documents in Pandadoc.

**When to use**: Browse documents, find contracts.

**Arguments**:
- `limit` (optional): Number of documents (default 20)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending signature documents"

---

### pandadoc_get_document

**What it does**: Gets detailed information for a specific document.

**When to use**: View document status, check details.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get details for document ABC123"

---

### pandadoc_create_document

**What it does**: Creates a new document in Pandadoc.

**When to use**: Create contracts, new documents.

**Arguments**:
- `name` (required): Document name
- `template_id` (optional): Template ID to use

**Example LLM prompt**: "Create a new document called 'Service Agreement'"

---

### pandadoc_send_document

**What it does**: Sends a document for signature.

**When to use**: Request signatures, send for review.

**Arguments**:
- `id` (required): Document ID
- `recipient` (required): Recipient email
- `message` (optional): Optional message

**Example LLM prompt**: "Send document ABC123 to john@company.com"

---

### pandadoc_list_templates

**What it does**: Lists all templates in Pandadoc.

**When to use**: Browse templates, find reusable documents.

**Arguments**: None

**Example LLM prompt**: "List all available templates"

---

### pandadoc_get_template

**What it does**: Gets detailed information for a specific template.

**When to use**: View template fields, usage.

**Arguments**:
- `id` (required): Template ID

**Example LLM prompt**: "Get details for template T123"

---

### pandadoc_list_folders

**What it does**: Lists all folders in Pandadoc.

**When to use**: Organize documents, find folders.

**Arguments**: None

**Example LLM prompt**: "List all folders"

---

### pandadoc_get_member

**What it does**: Gets details for the current member.

**When to use**: View account info, check subscription.

**Arguments**: None

**Example LLM prompt**: "Get my account information"

---

### pandadoc_list_webhooks

**What it does**: Lists all webhooks in Pandadoc.

**When to use**: View integrations, manage notifications.

**Arguments**: None

**Example LLM prompt**: "List all configured webhooks"

---

### pandadoc_get_analytics

**What it does**: Gets document analytics and tracking data.

**When to use**: Track views, monitor engagement.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get analytics for document ABC123"

---

## Pandadoc API Key Notes

- **API Key**: 32 character hexadecimal string
- **E-signatures**: Document signing platform
- **Templates**: Reusable document formats
- **Status**: draft, pending, sent, viewed, completed
- **Same API endpoints**: As OAuth2 variant, only auth differs

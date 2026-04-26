# Dropbox Sign Tools

Provider: `dropbox-sign` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Dropbox Sign (formerly HelloSign) API. They allow AI agents to manage signature requests, templates, team users, and documents. Dropbox Sign is an electronic signature platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dropbox Sign
- Token stored in Nango, accessed via `connection_ref`
- Supports refresh token flow

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dropbox_sign_list_signature_requests` | List signature requests | GET | /signature_requests |
| `dropbox_sign_get_signature_request` | Get request details | GET | /signature_requests/{signature_request_id} |
| `dropbox_sign_create_signature_request` | Create a signature request | POST | /signature_requests/send |
| `dropbox_sign_list_templates` | List templates | GET | /templates |
| `dropbox_sign_get_template` | Get template details | GET | /templates/{template_id} |
| `dropbox_sign_list_users` | List team users | GET | /teams |
| `dropbox_sign_get_user` | Get user details | GET | /teams/{user_id} |
| `dropbox_sign_list_bulk_send_jobs` | List bulk send jobs | GET | /bulk_send_jobs |
| `dropbox_sign_get_bulk_send_job` | Get bulk send details | GET | /bulk_send_jobs/{job_id} |
| `dropbox_sign_list_documents` | List documents | GET | /documents |

---

## Tool Details

### dropbox_sign_list_signature_requests

**What it does**: Lists all signature requests with optional status filtering.

**When to use**: View pending signatures, track signature status, find documents needing signatures.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `status` (optional): Filter by pending, signed, declined, or expired

**Example LLM prompt**: "List all pending signature requests"

---

### dropbox_sign_get_signature_request

**What it does**: Gets detailed signature request information.

**When to use**: Check request details, verify signers, track signature progress.

**Arguments**:
- `signature_request_id` (required): Signature Request ID

**Example LLM prompt**: "Get details for signature request sr-123"

---

### dropbox_sign_create_signature_request

**What it does**: Creates and sends a new signature request.

**When to use**: Send documents for signature, initiate signing workflows, request signatures.

**Arguments**:
- `title` (required): Request title
- `subject` (optional): Email subject
- `message` (optional): Email message
- `signers` (required): Array of signer objects with email and name
- `file_url` (optional): URL of file to sign

**Example LLM prompt**: "Create a signature request titled 'Employment Contract' for john@example.com"

---

### dropbox_sign_list_templates

**What it does**: Lists all templates in the Dropbox Sign account.

**When to use**: Browse available templates, find reusable signature forms, view template library.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all templates"

---

### dropbox_sign_get_template

**What it does**: Gets detailed template information.

**When to use**: Review template details, check template fields, understand template usage.

**Arguments**:
- `template_id` (required): Template ID

**Example LLM prompt**: "Get details for template t-456"

---

### dropbox_sign_list_users

**What it does**: Lists all users on the Dropbox Sign team.

**When to use**: View team members, find team administrators, manage team access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all team users"

---

### dropbox_sign_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user details, verify user permissions, review user activity.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-789"

---

### dropbox_sign_list_bulk_send_jobs

**What it does**: Lists all bulk send jobs.

**When to use**: View bulk sending operations, track mass signature requests, find bulk jobs.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all bulk send jobs"

---

### dropbox_sign_get_bulk_send_job

**What it does**: Gets detailed bulk send job information.

**When to use**: Check bulk send progress, verify bulk job status, review bulk job results.

**Arguments**:
- `job_id` (required): Bulk Send Job ID

**Example LLM prompt**: "Get details for bulk send job bj-101"

---

### dropbox_sign_list_documents

**What it does**: Lists all completed and archived documents.

**When to use**: Browse signed documents, find completed signatures, archive search.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all documents"

---

## Dropbox Sign API Notes

- **Formerly HelloSign**: Dropbox Sign is the new brand name
- **Signature Requests**: Documents sent for electronic signature
- **Templates**: Reusable signature forms with predefined fields
- **Bulk Send**: Send the same document to multiple recipients at once
- **Team Users**: Organization members with Dropbox Sign access
- **OAuth URL**: Uses app.hellosign.com for OAuth
- **Rate Limiting**: API has rate limits with retry-after header

# SignNow Tools

Provider: `signnow` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the SignNow API. They allow AI agents to interact with SignNow functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_documents` | List all documents | GET | /documents |
| `get_document` | Get document details | GET | /documents/{id} |
| `upload_document` | Upload a document | POST | /documents |
| `download_document` | Download document PDF | GET | /documents/{id}/download |
| `create_signature_request` | Create signature request | POST | /signature_requests |
| `list_signature_requests` | List signature requests | GET | /signature_requests |
| `get_signature_request` | Get signature request status | GET | /signature_requests/{id} |
| `cancel_signature_request` | Cancel a signature request | POST | /signature_requests/{id}/cancel |
| `create_folder` | Create a folder | POST | /folders |
| `list_folders` | List all folders | GET | /folders |

---

## Tool Details

### list_documents

**What it does**: List all documents

**When to use**: Use this tool when you need to list all documents.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list documents to..."

---

### get_document

**What it does**: Get document details

**When to use**: Use this tool when you need to get document details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get document to..."

---

### upload_document

**What it does**: Upload a document

**When to use**: Use this tool when you need to upload a document.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload document to..."

---

### download_document

**What it does**: Download document PDF

**When to use**: Use this tool when you need to download document pdf.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use download document to..."

---

### create_signature_request

**What it does**: Create signature request

**When to use**: Use this tool when you need to create signature request.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create signature request to..."

---

### list_signature_requests

**What it does**: List signature requests

**When to use**: Use this tool when you need to list signature requests.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list signature requests to..."

---

### get_signature_request

**What it does**: Get signature request status

**When to use**: Use this tool when you need to get signature request status.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get signature request to..."

---

### cancel_signature_request

**What it does**: Cancel a signature request

**When to use**: Use this tool when you need to cancel a signature request.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use cancel signature request to..."

---

### create_folder

**What it does**: Create a folder

**When to use**: Use this tool when you need to create a folder.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create folder to..."

---

### list_folders

**What it does**: List all folders

**When to use**: Use this tool when you need to list all folders.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list folders to..."

---

## SignNow API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://api.signnow.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits

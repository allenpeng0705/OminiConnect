# DocuWare Tools

Provider: `docuware` | Engine: `nango` | Auth: Two-Step via Nango

## Overview

These tools wrap the DocuWare API. They allow AI agents to manage cabinets, documents, folders, and indexes. DocuWare is an enterprise document management platform.

## Authentication

**Nango Two-Step**:
- Uses a multi-step authentication flow with username/password
- Token stored in Nango, accessed via `connection_ref`
- Domain configured per connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `docuware_list_cabinets` | List cabinets | GET | /Cabinets |
| `docuware_get_cabinet` | Get cabinet details | GET | /Cabinets/{cabinet_id} |
| `docuware_list_documents` | List documents | GET | /Cabinets/{cabinet_id}/Documents |
| `docuware_get_document` | Get document details | GET | /Cabinets/{cabinet_id}/Documents/{document_id} |
| `docuware_search_documents` | Search documents | POST | /Cabinets/{cabinet_id}/Documents/Search |
| `docuware_list_folders` | List folders | GET | /Cabinets/{cabinet_id}/Folders |
| `docuware_get_folder` | Get folder details | GET | /Cabinets/{cabinet_id}/Folders/{folder_id} |
| `docuware_list_indexes` | List index fields | GET | /Cabinets/{cabinet_id}/Indexes |
| `docuware_create_document` | Upload a document | POST | /Cabinets/{cabinet_id}/Documents |
| `docuware_get_user` | Get user details | GET | /UserInfo |

---

## Tool Details

### docuware_list_cabinets

**What it does**: Lists all DocuWare cabinets (document repositories).

**When to use**: Browse available document storage, select cabinet for operations.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all DocuWare cabinets"

---

### docuware_get_cabinet

**What it does**: Gets detailed cabinet information.

**When to use**: Check cabinet configuration, verify cabinet access, review cabinet settings.

**Arguments**:
- `cabinet_id` (required): Cabinet ID

**Example LLM prompt**: "Get details for cabinet c-123"

---

### docuware_list_documents

**What it does**: Lists all documents in a specific cabinet.

**When to use**: Browse documents, find files in cabinet, navigate document storage.

**Arguments**:
- `cabinet_id` (required): Cabinet ID
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "List all documents in cabinet c-123"

---

### docuware_get_document

**What it does**: Gets detailed document information.

**When to use**: Check document metadata, verify document properties, get document for download.

**Arguments**:
- `cabinet_id` (required): Cabinet ID
- `document_id` (required): Document ID

**Example LLM prompt**: "Get details for document d-456 in cabinet c-123"

---

### docuware_search_documents

**What it does**: Searches documents using query expressions.

**When to use**: Find specific documents, search by metadata, locate files quickly.

**Arguments**:
- `cabinet_id` (required): Cabinet ID
- `query` (required): Search query expression
- `limit` (optional): Max results (default 20, max 100)

**Example LLM prompt**: "Search for documents with keyword 'invoice' in cabinet c-123"

---

### docuware_list_folders

**What it does**: Lists all folders in a cabinet.

**When to use**: Browse folder structure, navigate document hierarchy, find specific folders.

**Arguments**:
- `cabinet_id` (required): Cabinet ID

**Example LLM prompt**: "List all folders in cabinet c-123"

---

### docuware_get_folder

**What it does**: Gets detailed folder information.

**When to use**: Check folder contents, verify folder structure, get folder metadata.

**Arguments**:
- `cabinet_id` (required): Cabinet ID
- `folder_id` (required): Folder ID

**Example LLM prompt**: "Get details for folder f-789 in cabinet c-123"

---

### docuware_list_indexes

**What it does**: Lists all index fields for a cabinet.

**When to use**: View searchable fields, understand document schema, prepare searches.

**Arguments**:
- `cabinet_id` (required): Cabinet ID

**Example LLM prompt**: "List all index fields for cabinet c-123"

---

### docuware_create_document

**What it does**: Uploads a new document to a cabinet.

**When to use**: Add new documents, store files, archive documents.

**Arguments**:
- `cabinet_id` (required): Cabinet ID
- `file_name` (required): File name
- `content` (optional): Document content (base64 encoded)

**Example LLM prompt**: "Upload a new document named 'report.pdf' to cabinet c-123"

---

### docuware_get_user

**What it does**: Gets current user information.

**When to use**: Verify authentication, check user permissions, get user profile.

**Arguments**: None

**Example LLM prompt**: "Get my DocuWare user info"

---

## DocuWare API Notes

- **Two-Step Auth**: Multi-phase authentication flow with OpenID Connect
- **Cabinets**: Top-level document repositories in DocuWare
- **Documents**: Individual files stored in cabinets
- **Folders**: Hierarchical organization within cabinets
- **Indexes**: Configurable metadata fields for document search
- **Search**: Query-based document search using index fields
- **Domain**: Configured per connection for multi-tenant deployment

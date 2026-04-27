# Ragieai Tools

Provider: `ragieai` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Ragie.ai provides document parsing and retrieval augmented generation (RAG) services. These tools allow AI agents to manage documents, collections, search document content, and monitor usage.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Ragie
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `documents:read`, `documents:write`, `tasks:read`, `collections:read`, `collections:write`, `search:read`, `usage:read`, `webhooks:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ragieai_list_documents` | List all documents | GET | /documents |
| `ragieai_get_document` | Get document details | GET | /documents/{documentId} |
| `ragieai_delete_document` | Delete a document | DELETE | /documents/{documentId} |
| `ragieai_list_tasks` | List processing tasks | GET | /tasks |
| `ragieai_get_task` | Get task status | GET | /tasks/{taskId} |
| `ragieai_list_collections` | List collections | GET | /collections |
| `ragieai_create_collection` | Create a collection | POST | /collections |
| `ragieai_search` | Search documents | POST | /search |
| `ragieai_get_usage` | Get API usage stats | GET | /usage |
| `ragieai_list_webhooks` | List webhooks | GET | /webhooks |

---

## Tool Details

### ragieai_list_documents

**What it does**: Returns a paginated list of all documents.

**When to use**: View all processed documents, find documents by listing.

**Arguments**:
- `limit` (optional): Number of documents (default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all documents in Ragie"

---

### ragieai_get_document

**What it does**: Gets details of a specific document including status and metadata.

**When to use**: Check document processing status, get document information.

**Arguments**:
- `documentId` (required): The document ID

**Example LLM prompt**: "Get details for document doc_abc123"

---

### ragieai_delete_document

**What it does**: Deletes a document and all its associated data.

**When to use**: Remove unwanted documents.

**Arguments**:
- `documentId` (required): The document ID to delete

**Example LLM prompt**: "Delete document doc_abc123"

---

### ragieai_list_tasks

**What it does**: Returns a list of document processing tasks.

**When to use**: Monitor document processing progress.

**Arguments**:
- `limit` (optional): Number of tasks (default 20)
- `status` (optional): Filter by status (pending, processing, completed, failed)

**Example LLM prompt**: "List all processing tasks"

---

### ragieai_get_task

**What it does**: Gets status and details of a specific task.

**When to use**: Check if a document has finished processing.

**Arguments**:
- `taskId` (required): The task ID

**Example LLM prompt**: "Get status for task task_xyz789"

---

### ragieai_list_collections

**What it does**: Returns a list of all document collections.

**When to use**: Organize documents into groups.

**Arguments**:
- `limit` (optional): Number of collections (default 20)

**Example LLM prompt**: "List all collections"

---

### ragieai_create_collection

**What it does**: Creates a new collection for organizing documents.

**When to use**: Group related documents together.

**Arguments**:
- `name` (required): Collection name
- `description` (optional): Collection description

**Example LLM prompt**: "Create a collection called 'Legal Documents'"

---

### ragieai_search

**What it does**: Searches through documents using natural language.

**When to use**: Find specific information across all documents.

**Arguments**:
- `query` (required): Search query
- `limit` (optional): Number of results (default 10)
- `collectionId` (optional): Limit search to a specific collection

**Example LLM prompt**: "Find documents about contract termination"

---

### ragieai_get_usage

**What it does**: Returns API usage statistics and quotas.

**When to use**: Check usage limits and billing information.

**Arguments**: None

**Example LLM prompt**: "Get my API usage statistics"

---

### ragieai_list_webhooks

**What it does**: Returns a list of configured webhooks.

**When to use**: See webhook configurations.

**Arguments**:
- `limit` (optional): Number of webhooks (default 20)

**Example LLM prompt**: "List all configured webhooks"

---

## Ragie API Notes

- Document IDs are string identifiers
- Tasks track document processing progress
- Collections help organize documents for better retrieval
- Search supports natural language queries
- Webhooks can notify external systems of events

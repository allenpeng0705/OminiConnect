# Sanity Tools

Provider: `sanity` | Engine: `nango` | Auth: API Token (via Nango)

## Overview

These tools wrap the Sanity API. They allow AI agents to manage datasets, documents, and users on a Sanity project.

## Authentication

**Nango (required)**:
- User authenticates via Nango Connect with API token
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projects:read`, `projects:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sanity_list_datasets` | List all datasets in the project | GET | /projects/{project_id}/datasets |
| `sanity_get_dataset` | Get dataset configuration and stats | GET | /projects/{project_id}/datasets/{dataset} |
| `sanity_list_documents` | Query documents in a dataset | GET | /data/projects/{project_id}/documents |
| `sanity_get_document` | Get a specific document by ID | GET | /documents/{document_id} |
| `sanity_create_document` | Create a new document | POST | /documents/{project_id}/{dataset} |
| `sanity_update_document` | Update an existing document | PUT | /documents/{project_id}/{dataset}/{document_id} |
| `sanity_delete_document` | Delete a document | DELETE | /documents/{project_id}/{dataset}/{document_id} |
| `sanity_list_users` | List project users | GET | /users |
| `sanity_get_user` | Get user details | GET | /users/{identifier} |
| `sanity_get_projection` | Get a projected document view | GET | /data/proj/{project_id}/{dataset}/{document_id} |

---

## Tool Details

### sanity_list_datasets

**What it does**: List all datasets in the Sanity project. Each dataset is a separate content repository.

**When to use**: Before querying or managing documents, know which datasets exist.

**Arguments**:
- `project_id` (required): Sanity project ID

**Example LLM prompt**: "List all datasets in my Sanity project"

---

### sanity_get_dataset

**What it does**: Get configuration and statistics for a specific dataset.

**When to use**: Check dataset details before working with documents.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)

**Example LLM prompt**: "Get statistics for the production dataset"

---

### sanity_list_documents

**What it does**: Query documents in a dataset. Filter by type or other criteria.

**When to use**: Search and browse documents. Filter by document type to find specific content.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)
- `type` (optional): Document type filter
- `limit` (optional): Number of results (max 1000, default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "Show me all posts in the production dataset"

---

### sanity_get_document

**What it does**: Get a specific document by ID including all fields and metadata.

**When to use**: Read full document content before editing or deleting.

**Arguments**:
- `document_id` (required): Document ID (starts with id://)
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)

**Example LLM prompt**: "Get the document with ID id://abc123"

---

### sanity_create_document

**What it does**: Create a new document in a dataset with the specified type and fields.

**When to use**: Add new content to the dataset.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)
- `type` (required): Document type (e.g., post, author)
- `body` (required): Document fields as key-value pairs

**Example LLM prompt**: "Create a new post titled 'Getting Started' with body 'Hello world'"

---

### sanity_update_document

**What it does**: Update an existing document. Uses revision ID for optimistic locking.

**When to use**: Modify document fields. Include revision ID to prevent conflicts.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)
- `document_id` (required): Document ID
- `body` (required): Updated field values
- `ifRevisionID` (optional): Expected revision ID for optimistic locking

**Example LLM prompt**: "Update the post abc123 to add a new tag"

---

### sanity_delete_document

**What it does**: Delete a document from a dataset by ID. This action is irreversible.

**When to use**: Remove unwanted documents.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)
- `document_id` (required): Document ID

**Example LLM prompt**: "Delete the document abc123"

---

### sanity_list_users

**What it does**: List all users in the Sanity project with email, name, and role.

**When to use**: Find project collaborators or check user roles.

**Arguments**:
- `project_id` (required): Sanity project ID

**Example LLM prompt**: "List all users in the project"

---

### sanity_get_user

**What it does**: Get details of a specific user by email or ID.

**When to use**: Check user details before assigning roles or permissions.

**Arguments**:
- `identifier` (required): User email or ID

**Example LLM prompt**: "Get details for user@example.com"

---

### sanity_get_projection

**What it does**: Get a projected (flattened) view of a document that resolves references.

**When to use**: Get the full resolved document including nested referenced data.

**Arguments**:
- `project_id` (required): Sanity project ID
- `dataset` (optional): Dataset name (default production)
- `document_id` (required): Document ID

**Example LLM prompt**: "Get the projected view of document abc123 with all references resolved"

---

## Sanity API Reference

These tools use the Sanity API. See official docs for full details:
- https://www.sanity.io/docs/http-api
- Rate limits: Depends on plan
- GROQ: Sanity's query language for filtering documents
- All dates: ISO 8601 format

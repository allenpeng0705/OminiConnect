# Google Docs Tools

Provider: `google-docs` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Docs API. They allow AI agents to manage documents, content, comments, revisions, and styles. **Requires Google OAuth2 with Docs permissions.**

## Authentication

**Nango OAUTH2 (Google Docs)**:
- User authenticates via OAuth2 with Docs scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://docs.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_docs_list_documents` | List documents | GET | /documents |
| `google_docs_get_document` | Get document details | GET | /documents/{documentId} |
| `google_docs_create_document` | Create document | POST | /documents |
| `google_docs_batch_update` | Batch update document | POST | /documents/{documentId}:batchUpdate |
| `google_docs_get_content` | Get document content | GET | /documents/{documentId} |
| `google_docs_create_comment` | Create comment | POST | /documents/{documentId}/comments |
| `google_docs_list_comments` | List comments | GET | /documents/{documentId}/comments |
| `google_docs_list_revisions` | List revisions | GET | /documents/{documentId}/revisions |
| `google_docs_get_revision` | Get revision | GET | /documents/{documentId}/revisions/{revisionId} |
| `google_docs_list_styles` | List styles | GET | /documents/{documentId}/styles |

---

## Tool Details

### google_docs_list_documents

**What it does**: Lists documents in Google Docs.

**When to use**: Browse user's documents.

**Arguments**: None

**Example LLM prompt**: "List my Google Docs"

---

### google_docs_get_document

**What it does**: Gets detailed information about a document.

**When to use**: Get document metadata.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "Get details for document abc123"

---

### google_docs_create_document

**What it does**: Creates a new Google Doc.

**When to use**: Create new documents.

**Arguments**:
- `title` (required): Document title

**Example LLM prompt**: "Create a new document titled 'My Report'"

---

### google_docs_batch_update

**What it does**: Applies batch updates to a document.

**When to use**: Make multiple changes at once.

**Arguments**:
- `documentId` (required): Document ID
- `requests` (required): Array of update requests

**Example LLM prompt**: "Insert text at the beginning of document abc123"

---

### google_docs_get_content

**What it does**: Gets full content of a document.

**When to use**: Read document content.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "Get content of document abc123"

---

### google_docs_create_comment

**What it does**: Creates a comment on a document.

**When to use**: Add comments to documents.

**Arguments**:
- `documentId` (required): Document ID
- `content` (required): Comment text
- `anchor` (optional): Text anchor for location

**Example LLM prompt**: "Add a comment 'Please review this section' to document abc123"

---

### google_docs_list_comments

**What it does**: Lists comments on a document.

**When to use**: View document comments.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "List comments on document abc123"

---

### google_docs_list_revisions

**What it does**: Lists revision history of a document.

**When to use**: See document edit history.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "List revisions for document abc123"

---

### google_docs_get_revision

**What it does**: Gets a specific revision of a document.

**When to use**: View document at a point in time.

**Arguments**:
- `documentId` (required): Document ID
- `revisionId` (required): Revision ID

**Example LLM prompt**: "Get revision xyz789 of document abc123"

---

### google_docs_list_styles

**What it does**: Lists defined styles in a document.

**When to use**: See available formatting styles.

**Arguments**:
- `documentId` (required): Document ID

**Example LLM prompt**: "List styles in document abc123"

---

## Google Docs API Notes

- **Document ID**: Found in the document URL
- **Batch updates**: Use requests array for multiple changes
- **Comments**: Anchored to specific text ranges
- **Revisions**: Historical versions of document
- **Styles**: Heading 1, Normal, etc.

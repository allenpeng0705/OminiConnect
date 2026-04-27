# Readwise-Reader Tools

Provider: `readwise-reader` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Readwise Reader is a read-later app for saving and reading articles, PDFs, and other documents. These tools allow AI agents to manage documents, highlights, folders, and reading progress in Reader.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Readwise Reader
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `documents:read`, `documents:write`, `highlights:read`, `notes:write`, `folders:read`, `folders:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `readwise-reader_list_documents` | List all documents | GET | /v3/documents |
| `readwise-reader_get_document` | Get document details | GET | /v3/documents/{documentId} |
| `readwise-reader_list_highlights` | List document highlights | GET | /v3/documents/{documentId}/highlights |
| `readwise-reader_create_note` | Create a note | POST | /v3/documents/{documentId}/notes |
| `readwise-reader_archive_document` | Archive a document | POST | /v3/documents/{documentId}/archive |
| `readwise-reader_list_folders` | List folders | GET | /v3/folders |
| `readwise-reader_create_folder` | Create a folder | POST | /v3/folders |
| `readwise-reader_move_document` | Move document to folder | POST | /v3/documents/{documentId}/move |
| `readwise-reader_get_reading_progress` | Get reading progress | GET | /v3/documents/{documentId}/progress |
| `readwise-reader_export_reader` | Export reader data | GET | /v3/export |

---

## Tool Details

### readwise-reader_list_documents

**What it does**: Returns a list of all saved documents.

**When to use**: Browse your reading list, find unread articles.

**Arguments**:
- `folderId` (optional): Filter by folder
- `limit` (optional): Number of documents (default 50)
- `unread` (optional): Filter to unread only

**Example LLM prompt**: "List all unread documents"

---

### readwise-reader_get_document

**What it does**: Gets details of a specific document.

**When to use**: Get document title, URL, and reading status.

**Arguments**:
- `documentId` (required): The document ID

**Example LLM prompt**: "Get details for document doc_abc123"

---

### readwise-reader_list_highlights

**What it does**: Lists all highlights from a document.

**When to use**: See highlights saved from a specific article.

**Arguments**:
- `documentId` (required): The document ID

**Example LLM prompt**: "List all highlights from doc_abc123"

---

### readwise-reader_create_note

**What it does**: Creates a note on a document.

**When to use**: Add thoughts or annotations to documents.

**Arguments**:
- `documentId` (required): The document ID
- `content` (required): Note content
- `highlightId` (optional): Attach to specific highlight

**Example LLM prompt**: "Add a note to doc_abc123 saying 'Important article'"

---

### readwise-reader_archive_document

**What it does**: Archives a document in Reader.

**When to use**: Mark document as read/done.

**Arguments**:
- `documentId` (required): The document ID

**Example LLM prompt**: "Archive document doc_abc123"

---

### readwise-reader_list_folders

**What it does**: Returns a list of all folders.

**When to use**: View folder structure for organization.

**Arguments**: None

**Example LLM prompt**: "List all my folders"

---

### readwise-reader_create_folder

**What it does**: Creates a new folder.

**When to use**: Organize documents into topics.

**Arguments**:
- `name` (required): Folder name
- `parentId` (optional): For nested folders

**Example LLM prompt**: "Create a folder called 'AI Research'"

---

### readwise-reader_move_document

**What it does**: Moves a document to a different folder.

**When to use**: Reorganize your reading library.

**Arguments**:
- `documentId` (required): The document ID
- `folderId` (required): Destination folder

**Example LLM prompt**: "Move doc_abc123 to folder 'Tech Articles'"

---

### readwise-reader_get_reading_progress

**What it does**: Gets reading progress for a document.

**When to use**: See how far you've read in a document.

**Arguments**:
- `documentId` (required): The document ID

**Example LLM prompt**: "Get reading progress for doc_abc123"

---

### readwise-reader_export_reader

**What it does**: Exports all Reader data.

**When to use**: Backup or migrate your reading data.

**Arguments**:
- `format` (optional): Export format (json, csv)

**Example LLM prompt**: "Export all my reader data as JSON"

---

## Readwise Reader Notes

- Reader supports articles, PDFs, and other document types
- Folders can be nested for deeper organization
- Archives remove documents from the main reading list
- Progress tracking shows how much of a document has been read

# Craft Tools

Provider: `craft` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Craft API. They allow AI agents to manage documents, folders, users, and sharing. Craft is a document collaboration platform focused on beautiful, flexible content creation.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Craft
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `docs:read`, `docs:write`, `folders:read`, `folders:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `craft_list_docs` | List documents | GET | /api/v1/docs |
| `craft_get_doc` | Get document details | GET | /api/v1/docs/{id} |
| `craft_create_doc` | Create a new document | POST | /api/v1/docs |
| `craft_list_folders` | List folders | GET | /api/v1/folders |
| `craft_get_folder` | Get folder details | GET | /api/v1/folders/{id} |
| `craft_create_folder` | Create a new folder | POST | /api/v1/folders |
| `craft_list_users` | List users | GET | /api/v1/users |
| `craft_get_user` | Get user details | GET | /api/v1/users/{id} |
| `craft_export_doc` | Export a document | POST | /api/v1/docs/{id}/export |
| `craft_share_doc` | Share a document | POST | /api/v1/docs/{id}/share |

---

## Tool Details

### craft_list_docs

**What it does**: Lists all documents with optional filtering by folder, status, or keyword.

**When to use**: Browse documents, find content by keyword, see recent documents.

**Arguments**:
- `folder_id` (optional): Filter by folder ID
- `status` (optional): Filter by status (draft, published, archived)
- `keyword` (optional): Search keyword in title or content
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all documents in folder abc-123"

---

### craft_get_doc

**What it does**: Gets detailed information about a specific document including title, content, author, and timestamps.

**When to use**: Read document content, understand authorship and history.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get details for document xyz-456"

---

### craft_create_doc

**What it does**: Creates a new document with title, content, and folder association.

**When to use**: Create new content, start a new document in a specific folder.

**Arguments**:
- `title` (required): Document title
- `content` (optional): Document content (Markdown or HTML)
- `folder_id` (optional): Parent folder ID
- `status` (optional): Initial status (draft, published)

**Example LLM prompt**: "Create a new document titled 'Meeting Notes' in folder abc-123"

---

### craft_list_folders

**What it does**: Lists all folders with optional filtering by parent folder or name.

**When to use**: Browse folder structure, find documents in a location.

**Arguments**:
- `parent_id` (optional): Filter by parent folder ID
- `name` (optional): Filter by folder name
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all folders in parent folder abc-123"

---

### craft_get_folder

**What it does**: Gets detailed information about a specific folder including name, parent, and document count.

**When to use**: Understand folder structure, see how many documents are inside.

**Arguments**:
- `id` (required): Folder ID

**Example LLM prompt**: "Get details for folder xyz-456"

---

### craft_create_folder

**What it does**: Creates a new folder with name and optional parent folder.

**When to use**: Organize documents, create new folder structure.

**Arguments**:
- `name` (required): Folder name
- `parent_id` (optional): Parent folder ID

**Example LLM prompt**: "Create a new folder named 'Project Documentation' in parent folder abc-123"

---

### craft_list_users

**What it does**: Lists all users with optional filtering by role or status.

**When to use**: Find team members, see who has access.

**Arguments**:
- `role` (optional): Filter by role
- `start` (optional): Pagination start
- `limit` (optional): Max results (default 25)

**Example LLM prompt**: "List all users with editor role"

---

### craft_get_user

**What it does**: Gets detailed information about a specific user including name, email, and role.

**When to use**: Check user details, verify permissions.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user uuu-999"

---

### craft_export_doc

**What it does**: Exports a document in a specific format (PDF, Markdown, HTML).

**When to use**: Generate a PDF version, extract content as Markdown, get HTML version.

**Arguments**:
- `id` (required): Document ID
- `format` (required): Export format (pdf, markdown, html)

**Example LLM prompt**: "Export document xyz-456 as PDF"

---

### craft_share_doc

**What it does**: Shares a document with specific users or makes it publicly accessible.

**When to use**: Collaborate with team members, publish content publicly.

**Arguments**:
- `id` (required): Document ID
- `user_ids` (optional): User IDs to share with
- `public` (optional): Make document publicly accessible
- `permissions` (optional): Permission level (view, edit)

**Example LLM prompt**: "Share document xyz-456 with user uuu-999 with edit permissions"

---

## Craft API Notes

- **Documents**: The main content unit in Craft. Support Markdown/HTML content, folders, and sharing.
- **Folders**: Organizational units that contain documents. Can be nested in parent folders.
- **Users**: Team members with roles (owner, editor, viewer) and permissions.
- **Sharing**: Documents can be shared with specific users or made publicly accessible.
- **Export Formats**: PDF for print-ready output, Markdown for plain text, HTML for web embedding.
- **Status**: Documents can be draft, published, or archived.
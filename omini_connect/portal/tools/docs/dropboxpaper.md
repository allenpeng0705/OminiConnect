# Dropbox Paper Tools

Provider: `dropboxpaper` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Dropbox Paper API. They allow AI agents to manage Paper docs, folders, members, and comments; create and edit content; and collaborate with teammates. Dropbox Paper is a collaborative document editor that combines editing, commenting, and sharing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dropbox Paper
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `files.metadata.read`, `files.content.read`, `files.content.write`, `files.sharing.read`, `files.sharing.write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dropboxpaper_list_docs` | List all docs | GET | /api/1.0/paper/docs |
| `dropboxpaper_get_doc` | Get doc details | GET | /api/1.0/paper/docs/{docId} |
| `dropboxpaper_create_doc` | Create a new doc | POST | /api/1.0/paper/docs |
| `dropboxpaper_list_folders` | List folders | GET | /api/1.0/paper/folders |
| `dropboxpaper_get_folder` | Get folder details | GET | /api/1.0/paper/folders/{folderId} |
| `dropboxpaper_list_members` | List doc members | GET | /api/1.0/paper/docs/{docId}/members |
| `dropboxpaper_get_member` | Get member details | GET | /api/1.0/paper/docs/{docId}/members/{memberId} |
| `dropboxpaper_add_comment` | Add a comment to a doc | POST | /api/1.0/paper/docs/{docId}/comments |
| `dropboxpaper_list_invites` | List pending invites | GET | /api/1.0/paper/docs/{docId}/invites |
| `dropboxpaper_create_invite` | Create an invite | POST | /api/1.0/paper/docs/{docId}/invites |

---

## Tool Details

### dropboxpaper_list_docs

**What it does**: Lists all Paper docs in the workspace. Returns doc metadata including title, owner, and dates.

**When to use**: See all available documents, find docs to edit or read.

**Arguments**:
- `limit` (optional): Max docs to return (default 100)

**Example LLM prompt**: "List all my Paper docs"

---

### dropboxpaper_get_doc

**What it does**: Gets detailed information about a specific Paper doc including title, content, and sharing settings.

**When to use**: Get doc details before editing, sharing, or viewing members.

**Arguments**:
- `docId` (required): Doc ID

**Example LLM prompt**: "Get details for doc abc123"

---

### dropboxpaper_create_doc

**What it does**: Creates a new Paper doc with title and optional initial content in Markdown format.

**When to use**: Create new documents, start new projects, initialize content.

**Arguments**:
- `title` (required): Doc title
- `content` (optional): Initial content in Markdown
- `folderId` (optional): Folder ID to create doc in

**Example LLM prompt**: "Create a new doc called 'Q2 Strategy' in the Product folder"

---

### dropboxpaper_list_folders

**What it does**: Lists all folders in the workspace. Returns folder metadata and hierarchy.

**When to use**: Organize docs, find where documents are stored, navigate workspace structure.

**Arguments**:
- `limit` (optional): Max folders to return (default 100)

**Example LLM prompt**: "List all folders in my workspace"

---

### dropboxpaper_get_folder

**What it does**: Gets detailed information about a specific folder including name and contained docs.

**When to use**: See what docs are in a folder, understand workspace organization.

**Arguments**:
- `folderId` (required): Folder ID

**Example LLM prompt**: "Get details for folder abc123 and list its contents"

---

### dropboxpaper_list_members

**What it does**: Lists all members of a Paper doc. Returns member info including email and permission level.

**When to use**: See who has access to a document, check collaborators.

**Arguments**:
- `docId` (required): Doc ID

**Example LLM prompt**: "List all members of doc abc123"

---

### dropboxpaper_get_member

**What it does**: Gets detailed information about a specific member including name, email, and permission level.

**When to use**: Get member details, check permissions, verify access levels.

**Arguments**:
- `docId` (required): Doc ID
- `memberId` (required): Member ID or email

**Example LLM prompt**: "Get details for member john@company.com in doc abc123"

---

### dropboxpaper_add_comment

**What it does**: Adds a comment to a Paper doc at a specific position or as a general comment.

**When to use**: Provide feedback, ask questions, start discussions on documents.

**Arguments**:
- `docId` (required): Doc ID
- `content` (required): Comment content in Markdown
- `position` (optional): Position in document for anchored comment

**Example LLM prompt**: "Add a comment to doc abc123 saying 'This section needs more detail'"

---

### dropboxpaper_list_invites

**What it does**: Lists all pending invites for a Paper doc. Returns invite details and status.

**When to use**: Check pending invitations, see who has been invited but not yet accepted.

**Arguments**:
- `docId` (required): Doc ID

**Example LLM prompt**: "List all pending invites for doc abc123"

---

### dropboxpaper_create_invite

**What it does**: Creates an invite to a Paper doc for a specific email address with permission level.

**When to use**: Share documents with teammates, grant access to collaborators.

**Arguments**:
- `docId` (required): Doc ID
- `inviteeEmail` (required): Email address to invite
- `permissionLevel` (optional): Permission level (`view`, `edit`, `comment`) - default `view`

**Example LLM prompt**: "Invite sarah@company.com to doc abc123 with edit access"

---

## Dropbox Paper API Notes

- **Docs**: Main documents that contain content and collaborators
- **Folders**: Organize docs hierarchically in the workspace
- **Members**: Users who have accepted access to a doc
- **Invites**: Pending invitations that have not yet been accepted
- **Permissions**: `view` (read only), `edit` (can modify), `comment` (can comment)
- **Content Format**: Markdown is used for doc content and comments
- **Folder Location**: Docs can be created in specific folders using folderId
- **Comments**: Can be anchored to specific positions in the document or be general comments
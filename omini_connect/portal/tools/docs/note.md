# Note Tools

Provider: `note` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap a generic Note API. They allow AI agents to create, read, update, and delete notes; organize content into notebooks; apply tags for categorization; search through notes; and share notes with others. This is a generic note-taking API that can be adapted for various note-taking platforms.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with their note service
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `notes:read`, `notes:write`, `notebooks:read`, `notebooks:write`, `tags:read`, `tags:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `note_list_notes` | List notes | GET | /notes |
| `note_get_note` | Get note details | GET | /notes/{note_id} |
| `note_create_note` | Create a note | POST | /notes |
| `note_update_note` | Update a note | PUT | /notes/{note_id} |
| `note_delete_note` | Delete a note | DELETE | /notes/{note_id} |
| `note_list_notebooks` | List notebooks | GET | /notebooks |
| `note_get_notebook` | Get notebook details | GET | /notebooks/{notebook_id} |
| `note_list_tags` | List tags | GET | /tags |
| `note_search_notes` | Search notes | GET | /notes/search |
| `note_share_note` | Share a note | POST | /notes/{note_id}/share |

---

## Tool Details

### note_list_notes

**What it does**: Lists notes with optional filters for notebook, tag, or status.

**When to use**: Browse notes, find notes in a specific notebook or with a specific tag.

**Arguments**:
- `notebook_id` (optional): Filter by notebook ID
- `tag_id` (optional): Filter by tag ID
- `status` (optional): Filter by status (active, archived)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all notes in the Work notebook"

---

### note_get_note

**What it does**: Gets detailed information about a specific note.

**When to use**: Read note content before editing or deleting.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "Get the note with ID 12345"

---

### note_create_note

**What it does**: Creates a new note with title, content, and metadata.

**When to use**: Save new information, capture ideas, create notes in specific notebooks.

**Arguments**:
- `title` (required): Note title
- `content` (required): Note content
- `notebook_id` (optional): Notebook ID to create note in
- `tags` (optional): Array of tag IDs or names

**Example LLM prompt**: "Create a note titled Meeting Notes in the Work notebook with content about the quarterly review"

---

### note_update_note

**What it does**: Updates an existing note's title, content, or metadata.

**When to use**: Edit notes, update information, reorganize with different tags.

**Arguments**:
- `note_id` (required): Note ID
- `title` (optional): New note title
- `content` (optional): New note content
- `tags` (optional): Array of tag IDs or names

**Example LLM prompt**: "Update note 12345 to add the important tag"

---

### note_delete_note

**What it does**: Permanently deletes a note.

**When to use**: Remove unwanted or duplicate notes. This cannot be undone.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "Delete note 99999"

---

### note_list_notebooks

**What it does**: Lists all notebooks available in the account.

**When to use**: See available notebooks, understand organization structure.

**Arguments**: None

**Example LLM prompt**: "List all my notebooks"

---

### note_get_notebook

**What it does**: Gets detailed information about a specific notebook.

**When to use**: Get notebook details before creating notes or understanding organization.

**Arguments**:
- `notebook_id` (required): Notebook ID

**Example LLM prompt**: "Get details for notebook 12345"

---

### note_list_tags

**What it does**: Lists all tags available in the account.

**When to use**: See available tags for organizing notes.

**Arguments**: None

**Example LLM prompt**: "What tags are available?"

---

### note_search_notes

**What it does**: Searches notes by keyword across title and content.

**When to use**: Find specific information across all notes.

**Arguments**:
- `query` (required): Search query
- `notebook_id` (optional): Limit search to notebook
- `tag` (optional): Limit search to tag
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Search for notes containing 'budget'"

---

### note_share_note

**What it does**: Shares a note by creating a public link or sharing with specific users.

**When to use**: Collaborate on notes, share information with teammates.

**Arguments**:
- `note_id` (required): Note ID
- `share_type` (required): Type of share ('public', 'user', 'email')
- `user_ids` (optional): User IDs to share with
- `emails` (optional): Emails to share with

**Example LLM prompt**: "Share note 12345 as public link"

---

## Note API Notes

- **Notebooks**: Primary organization unit for notes
- **Tags**: Cross-reference notes across notebooks
- **Status**: Notes can be active or archived
- **Content**: Varies by implementation - could be plain text, markdown, or rich text
- **Relationships**: Notes can belong to one notebook but have multiple tags
- **Sharing**: Various sharing options depending on platform support

# Granola Tools

Provider: `granola` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Granola API. They allow AI agents to manage notes, collections, and collaborate with team members. Granola is a knowledge base and note-taking platform.

## Authentication

**Nango API Key**:
- User provides Granola API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Token format: `grn_*`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `granola_list_notes` | List notes | GET | /v1/notes |
| `granola_get_note` | Get note details | GET | /v1/notes/{id} |
| `granola_create_note` | Create a note | POST | /v1/notes |
| `granola_update_note` | Update a note | PUT | /v1/notes/{id} |
| `granola_delete_note` | Delete a note | DELETE | /v1/notes/{id} |
| `granola_list_collections` | List collections | GET | /v1/collections |
| `granola_get_collection` | Get collection details | GET | /v1/collections/{id} |
| `granola_list_tags` | List tags | GET | /v1/tags |
| `granola_search_notes` | Search notes | GET | /v1/notes/search |
| `granola_list_collaborators` | List collaborators | GET | /v1/collaborators |

---

## Tool Details

### granola_list_notes

**What it does**: Lists all notes in your Granola account.

**When to use**: Browse notes, find notes in a collection.

**Arguments**:
- `collection_id` (optional): Filter by collection ID
- `page` (optional): Page number (default 1)
- `page_size` (optional): Page size (default 20)

**Example LLM prompt**: "List all notes in collection abc123"

---

### granola_get_note

**What it does**: Gets detailed information about a specific note.

**When to use**: Read full note content, view metadata.

**Arguments**:
- `id` (required): Note ID

**Example LLM prompt**: "Get note with ID xyz789"

---

### granola_create_note

**What it does**: Creates a new note in Granola.

**When to use**: Create a new note or document.

**Arguments**:
- `title` (required): Note title
- `content` (optional): Note content
- `collection_id` (optional): Collection ID

**Example LLM prompt**: "Create a note titled 'Meeting Notes' with content 'Discussed project timeline'"

---

### granola_update_note

**What it does**: Updates an existing note.

**When to use**: Edit note content, move to different collection.

**Arguments**:
- `id` (required): Note ID
- `title` (optional): New title
- `content` (optional): New content
- `collection_id` (optional): New collection ID

**Example LLM prompt**: "Update note xyz789 to add more content"

---

### granola_delete_note

**What it does**: Deletes a note from Granola.

**When to use**: Remove unwanted or outdated notes.

**Arguments**:
- `id` (required): Note ID

**Example LLM prompt**: "Delete note xyz789"

---

### granola_list_collections

**What it does**: Lists all collections in your Granola account.

**When to use**: Browse collection structure, find specific collections.

**Arguments**: None

**Example LLM prompt**: "List all collections"

---

### granola_get_collection

**What it does**: Gets detailed information about a specific collection.

**When to use**: View collection settings and notes.

**Arguments**:
- `id` (required): Collection ID

**Example LLM prompt**: "Get collection with ID abc123"

---

### granola_list_tags

**What it does**: Lists all tags in your Granola account.

**When to use**: View available tags, organize notes.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### granola_search_notes

**What it does**: Searches for notes matching a query.

**When to use**: Find specific content across all notes.

**Arguments**:
- `q` (required): Search query

**Example LLM prompt**: "Search for notes containing 'project timeline'"

---

### granola_list_collaborators

**What it does**: Lists all collaborators in your Granola account.

**When to use**: View team members, check permissions.

**Arguments**: None

**Example LLM prompt**: "List all collaborators"

---

## Granola API Notes

- **API Base URL**: https://public-api.granola.ai
- **Token Format**: `grn_*`
- **Collections**: Organize notes into folders/categories
- **Tags**: Label notes for easy filtering
- **Collaborators**: Team members with access to notes
- **Pagination**: Use page and page_size for list endpoints

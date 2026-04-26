# Evernote Tools

Provider: `evernote` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Evernote API. They allow AI agents to read and write notes; organize content into notebooks; apply tags for categorization; search through the note archive; and manage resources attached to notes. Evernote is a popular note-taking app that stores information in notebooks with rich text content.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Evernote
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read notes`, `write notes`, `read notebooks`, `read tags`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `evernote_list_notes` | List notes | GET | /notes |
| `evernote_get_note` | Get note details | GET | /notes/{guid} |
| `evernote_create_note` | Create a note | POST | /notes |
| `evernote_update_note` | Update a note | PUT | /notes/{guid} |
| `evernote_list_notebooks` | List notebooks | GET | /notebooks |
| `evernote_get_notebook` | Get notebook details | GET | /notebooks/{guid} |
| `evernote_list_tags` | List tags | GET | /tags |
| `evernote_get_tag` | Get tag details | GET | /tags/{guid} |
| `evernote_search_notes` | Search notes | GET | /notes/search |
| `evernote_list_resources` | List resources | GET | /notes/{guid}/resources |

---

## Tool Details

### evernote_list_notes

**What it does**: Lists notes in a specific notebook or all notes if no notebook specified.

**When to use**: Browse notes in a specific notebook, see notes with a specific tag.

**Arguments**:
- `notebookGuid` (optional): Filter by notebook GUID
- `tagGuid` (optional): Filter by tag GUID
- `limit` (optional): Max results (default 50)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all notes in my Work notebook"

---

### evernote_get_note

**What it does**: Gets full content and metadata of a specific Evernote note.

**When to use**: Read note content, see attached resources, understand note details.

**Arguments**:
- `guid` (required): Note GUID
- `withContent` (optional): Include full note content (default true)
- `withResourcesData` (optional): Include resource data (default false)

**Example LLM prompt**: "Get the note with guid abc-123 about meeting notes"

---

### evernote_create_note

**What it does**: Creates a new note in a specific notebook with title, content, and tags.

**When to use**: Save new information, capture ideas, log meeting notes.

**Arguments**:
- `title` (required): Note title
- `content` (required): Note content in ENML format
- `notebookGuid` (optional): Notebook GUID to create note in
- `tagNames` (optional): Array of tag names to apply

**Example LLM prompt**: "Create a new note titled Meeting Notes in my Work notebook"

---

### evernote_update_note

**What it does**: Updates an existing note's title, content, or tags.

**When to use**: Edit notes, add information, change tags.

**Arguments**:
- `guid` (required): Note GUID
- `title` (optional): New note title
- `content` (optional): New note content in ENML format
- `tagNames` (optional): Array of tag names to apply

**Example LLM prompt**: "Update note abc-123 to add the project management tag"

---

### evernote_list_notebooks

**What it does**: Lists all Evernote notebooks in the user's account.

**When to use**: See available notebooks, understand organization structure.

**Arguments**: None

**Example LLM prompt**: "List all my Evernote notebooks"

---

### evernote_get_notebook

**What it does**: Gets detailed information about a specific notebook.

**When to use**: Get notebook details before creating or listing notes.

**Arguments**:
- `guid` (required): Notebook GUID

**Example LLM prompt**: "Get details for notebook with guid 12345-6789"

---

### evernote_list_tags

**What it does**: Lists all tags in the user's Evernote account.

**When to use**: See available tags for organizing notes.

**Arguments**: None

**Example LLM prompt**: "What tags do I have in Evernote?"

---

### evernote_get_tag

**What it does**: Gets details about a specific Evernote tag.

**When to use**: Understand tag details before applying to notes.

**Arguments**:
- `guid` (required): Tag GUID

**Example LLM prompt**: "Get details for tag with guid xyz-789"

---

### evernote_search_notes

**What it does**: Searches notes using Evernote's search query syntax.

**When to use**: Find notes by keyword, search within specific notebooks or tags.

**Arguments**:
- `query` (required): Search query (Evernote syntax)
- `notebookGuid` (optional): Limit search to a specific notebook
- `tagGuid` (optional): Filter by tag
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "Search for notes containing 'budget' in my Work notebook"

---

### evernote_list_resources

**What it does**: Lists all resources (attachments) in a note.

**When to use**: See images, files, or other attachments in a note.

**Arguments**:
- `guid` (required): Note GUID

**Example LLM prompt**: "List all resources in note abc-123"

---

## Evernote API Notes

- **ENML format**: Note content must be in Evernote's ENML format (like HTML)
- **GUIDs**: All resources are identified by GUIDs, not human-readable names
- **Search syntax**: Evernote supports operators like `intitle:`, `notebook:`, `tag:`, `created:`, `updated:`
- **Notebooks vs Tags**: Notebooks organize notes spatially; tags label notes across notebooks
- **Resources**: Notes can contain attachments (images, files) as resources
- **Shared notebooks**: Notes can be shared via Evernote's sharing system

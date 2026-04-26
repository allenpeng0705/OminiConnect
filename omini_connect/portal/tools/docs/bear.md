# Bear Tools

Provider: `bear` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Bear API. They allow AI agents to create, read, update, and delete notes; organize content with tags; search through the note archive; and manage resources attached to notes. Bear is a popular markdown note-taking app for iOS and macOS with a clean, focused writing experience.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Bear
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bear_list_notes` | List notes | GET | /notes |
| `bear_get_note` | Get note details | GET | /notes/{note_id} |
| `bear_create_note` | Create a note | POST | /notes |
| `bear_update_note` | Update a note | PUT | /notes/{note_id} |
| `bear_delete_note` | Delete a note | DELETE | /notes/{note_id} |
| `bear_list_tags` | List tags | GET | /tags |
| `bear_get_tag` | Get tag details | GET | /tags/{tag_id} |
| `bear_search_notes` | Search notes | GET | /notes/search |
| `bear_list_resources` | List resources | GET | /notes/{note_id}/resources |
| `bear_get_resource` | Get resource details | GET | /resources/{resource_id} |

---

## Tool Details

### bear_list_notes

**What it does**: Lists notes with optional filters for tag, search term, or archive status.

**When to use**: Browse notes by tag, see recent notes, find notes by keyword.

**Arguments**:
- `tag` (optional): Filter by tag name
- `search` (optional): Search term to filter notes
- `archived` (optional): Include archived notes (default false)
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all notes with the work tag"

---

### bear_get_note

**What it does**: Gets full content and metadata of a specific Bear note.

**When to use**: Read note content, understand note details before editing.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "Get note abc-123"

---

### bear_create_note

**What it does**: Creates a new note with title, content, and tags.

**When to use**: Save new information, capture ideas, create meeting notes.

**Arguments**:
- `title` (required): Note title
- `content` (required): Note content (supports markdown)
- `tags` (optional): Array of tag names
- `color` (optional): Note color (red, orange, yellow, green, blue, purple, pink, gray)

**Example LLM prompt**: "Create a note titled Project Ideas with content about the new app features and tags work,planning"

---

### bear_update_note

**What it does**: Updates an existing note's title, content, or tags.

**When to use**: Edit notes, update information, reorganize with tags.

**Arguments**:
- `note_id` (required): Note ID
- `title` (optional): New note title
- `content` (optional): New note content
- `tags` (optional): Array of tag names
- `color` (optional): New note color

**Example LLM prompt**: "Update note abc-123 to add the important tag and change color to red"

---

### bear_delete_note

**What it does**: Deletes a Bear note. This moves it to archive first, then permanent trash after 30 days.

**When to use**: Remove unwanted notes from main view.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "Delete note xyz-789"

---

### bear_list_tags

**What it does**: Lists all tags in the user's Bear account.

**When to use**: See available tags, understand organization structure.

**Arguments**: None

**Example LLM prompt**: "What tags do I have in Bear?"

---

### bear_get_tag

**What it does**: Gets details about a specific tag including note count.

**When to use**: Understand tag usage before filtering.

**Arguments**:
- `tag_id` (required): Tag identifier (name or ID)

**Example LLM prompt**: "Get details for the work tag"

---

### bear_search_notes

**What it does**: Searches notes by keyword across title and content.

**When to use**: Find specific information across all notes.

**Arguments**:
- `term` (required): Search term
- `tag` (optional): Limit search to specific tag
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "Search for notes containing 'budget'"

---

### bear_list_resources

**What it does**: Lists all resources (images, files) in a note.

**When to use**: See attachments in a note.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "List all resources in note abc-123"

---

### bear_get_resource

**What it does**: Gets details about a specific resource (image, file attachment).

**When to use**: Get details about an attachment before downloading.

**Arguments**:
- `resource_id` (required): Resource ID

**Example LLM prompt**: "Get details for resource xyz-456"

---

## Bear API Notes

- **Markdown support**: Bear notes use markdown formatting
- **Tags**: Use # tagname in notes or specify via API. Tags are hierarchical (work/projects/tasks)
- **Colors**: Notes can have color coding for visual organization
- **Archive vs Delete**: Deleting moves to archive; archive is permanent trash after 30 days
- **Search**: Searches both title and content of notes
- **Resources**: Notes can contain images, files, or other attachments

# Readwise Tools

Provider: `readwise` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Readwise is a reading comprehension and knowledge management app that helps users retain what they read. These tools allow AI agents to manage highlights, books, tags, and reading statistics.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Readwise
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `highlights:read`, `highlights:write`, `books:read`, `tags:read`, `tags:write`, `stats:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `readwise_list_highlights` | List all highlights | GET | /v2/highlights |
| `readwise_get_highlight` | Get highlight details | GET | /v2/highlights/{highlightId} |
| `readwise_list_books` | List all books | GET | /v2/books |
| `readwise_get_book` | Get book details | GET | /v2/books/{bookId} |
| `readwise_create_highlight` | Create a highlight | POST | /v2/highlights |
| `readwise_list_tags` | List all tags | GET | /v2/tags |
| `readwise_create_tag` | Create a tag | POST | /v2/tags |
| `readwise_associate_highlight` | Associate tag with highlight | POST | /v2/highlights/{highlightId}/tags |
| `readwise_get_stats` | Get reading statistics | GET | /v2/stats |
| `readwise_export_highlights` | Export highlights | GET | /v2/export |

---

## Tool Details

### readwise_list_highlights

**What it does**: Returns a paginated list of all highlights.

**When to use**: Browse your highlights, find highlights by book or tag.

**Arguments**:
- `bookId` (optional): Filter by book ID
- `tagId` (optional): Filter by tag ID
- `limit` (optional): Number of highlights (default 100)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all highlights from 'Atomic Habits'"

---

### readwise_get_highlight

**What it does**: Gets details of a specific highlight.

**When to use**: Get highlight text, note, and location.

**Arguments**:
- `highlightId` (required): The highlight ID

**Example LLM prompt**: "Get details for highlight hl_abc123"

---

### readwise_list_books

**What it does**: Returns a list of all books in your library.

**When to use**: View your reading library.

**Arguments**:
- `limit` (optional): Number of books (default 100)
- `source` (optional): Filter by source (kindle, pocket, etc.)

**Example LLM prompt**: "List all my books"

---

### readwise_get_book

**What it does**: Gets details of a specific book.

**When to use**: Get book info, author, and highlight count.

**Arguments**:
- `bookId` (required): The book ID

**Example LLM prompt**: "Get details for book bk_xyz789"

---

### readwise_create_highlight

**What it does**: Creates a new highlight for a book.

**When to use**: Save important passages manually.

**Arguments**:
- `bookId` (required): The book ID
- `text` (required): Highlight text
- `location` (optional): Location in book
- `note` (optional): Personal note

**Example LLM prompt**: "Create a highlight for book bk_123 with text 'This is important'"

---

### readwise_list_tags

**What it does**: Returns a list of all tags.

**When to use**: View available tags for organization.

**Arguments**: None

**Example LLM prompt**: "List all my tags"

---

### readwise_create_tag

**What it does**: Creates a new tag.

**When to use**: Create tags for organizing highlights.

**Arguments**:
- `name` (required): Tag name

**Example LLM prompt**: "Create a tag called 'Important'"

---

### readwise_associate_highlight

**What it does**: Associates a tag with an existing highlight.

**When to use**: Organize highlights with tags.

**Arguments**:
- `highlightId` (required): The highlight ID
- `tagId` (required): The tag ID

**Example LLM prompt**: "Tag highlight hl_123 with tag 'Key Insight'"

---

### readwise_get_stats

**What it does**: Returns reading statistics and insights.

**When to use**: Track reading habits and progress.

**Arguments**: None

**Example LLM prompt**: "Get my reading statistics"

---

### readwise_export_highlights

**What it does**: Exports highlights to specified format.

**When to use**: Backup or share highlights.

**Arguments**:
- `format` (optional): Export format (json, csv, html)
- `bookId` (optional): Export specific book

**Example LLM prompt**: "Export all highlights as JSON"

---

## Readwise API Notes

- Highlights are linked to books and can have tags
- Book sources include Kindle, Pocket, Instapaper, etc.
- Cursor-based pagination for large result sets
- Export supports multiple formats for different use cases

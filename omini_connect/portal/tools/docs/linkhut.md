# LinkHut Tools

Provider: `linkhut` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the LinkHut API. They allow AI agents to manage bookmarks, tags, notes, and social connections. **Requires LinkHut OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with LinkHut
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.ln.ht`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `linkhut_list_bookmarks` | List bookmarks | GET | /v1/bookmarks |
| `linkhut_add_bookmark` | Add a bookmark | POST | /v1/bookmarks |
| `linkhut_delete_bookmark` | Delete a bookmark | DELETE | /v1/bookmarks/{bookmark_id} |
| `linkhut_list_tags` | List tags | GET | /v1/tags |
| `linkhut_list_notes` | List notes | GET | /v1/notes |
| `linkhut_add_note` | Add a note | POST | /v1/notes |
| `linkhut_delete_note` | Delete a note | DELETE | /v1/notes/{note_id} |
| `linkhut_get_user` | Get user info | GET | /v1/user |
| `linkhut_list_follows` | List follows | GET | /v1/follows |
| `linkhut_follow_user` | Follow a user | POST | /v1/follows |

---

## Tool Details

### linkhut_list_bookmarks

**What it does**: Lists all bookmarks.

**When to use**: Find saved links, search bookmarks.

**Arguments**:
- `tag` (optional): Filter by tag
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all bookmarks in LinkHut"

---

### linkhut_add_bookmark

**What it does**: Adds a new bookmark.

**When to use**: Save links, bookmark URLs.

**Arguments**:
- `url` (required): URL to bookmark
- `description` (optional): Bookmark description
- `tags` (optional): Tags for the bookmark

**Example LLM prompt**: "Bookmark https://example.com with tag 'reference'"

---

### linkhut_delete_bookmark

**What it does**: Deletes a bookmark.

**When to use**: Remove saved links.

**Arguments**:
- `bookmark_id` (required): Bookmark ID

**Example LLM prompt**: "Delete bookmark abc123"

---

### linkhut_list_tags

**What it does**: Lists all tags.

**When to use**: View tags, organize content.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all tags in LinkHut"

---

### linkhut_list_notes

**What it does**: Lists all notes.

**When to use**: View notes, find information.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all notes in LinkHut"

---

### linkhut_add_note

**What it does**: Adds a new note.

**When to use**: Create notes, save information.

**Arguments**:
- `title` (required): Note title
- `content` (required): Note content

**Example LLM prompt**: "Add a note titled 'Ideas' with content 'Some ideas'"

---

### linkhut_delete_note

**What it does**: Deletes a note.

**When to use**: Remove notes.

**Arguments**:
- `note_id` (required): Note ID

**Example LLM prompt**: "Delete note n1"

---

### linkhut_get_user

**What it does**: Gets current user information.

**When to use**: Verify authentication, get account info.

**Arguments**: None

**Example LLM prompt**: "Get my LinkHut user info"

---

### linkhut_list_follows

**What it does**: Lists all follows.

**When to use**: View followed users.

**Arguments**: None

**Example LLM prompt**: "List all users I follow in LinkHut"

---

### linkhut_follow_user

**What it does**: Follows a user.

**When to use**: Connect with other users.

**Arguments**:
- `username` (required): Username to follow

**Example LLM prompt**: "Follow user john in LinkHut"

---

## LinkHut API Notes

- **Social Bookmarking**: Save and share links
- **Bookmarks**: Saved URLs with tags
- **Tags**: Categorize bookmarks
- **Notes**: Personal notes
- **Follows**: Social connections

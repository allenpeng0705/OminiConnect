# OneNote Tools

Provider: `one-note` | Engine: `nango` | Auth: OAuth via Nango (Microsoft/OneNote)

## Overview

These tools wrap the Microsoft Graph API for OneNote. They allow AI agents to browse notebooks, sections, and pages, as well as create and update notes. **Requires Microsoft OAuth access for OneNote.**

## Authentication

**OAuth via Nango**:
- Alias for Microsoft integration
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Uses Microsoft Graph API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onenote_list_notebooks` | List notebooks | GET | /v1.0/me/onenote/notebooks |
| `onenote_get_notebook` | Get notebook details | GET | /v1.0/me/onenote/notebooks/{id} |
| `onenote_list_sections` | List sections | GET | /v1.0/me/onenote/notebooks/{notebookId}/sections |
| `onenote_get_section` | Get section details | GET | /v1.0/me/onenote/sections/{id} |
| `onenote_list_pages` | List pages | GET | /v1.0/me/onenote/sections/{sectionId}/pages |
| `onenote_get_page` | Get page details | GET | /v1.0/me/onenote/pages/{id} |
| `onenote_create_page` | Create page | POST | /v1.0/me/onenote/sections/{sectionId}/pages |
| `onenote_update_page` | Update page content | PATCH | /v1.0/me/onenote/pages/{id}/content |
| `onenote_search` | Search notes | GET | /v1.0/me/onenote/search |
| `onenote_get_recent_pages` | Get recent pages | GET | /v1.0/me/onenote/pages |

---

## Tool Details

### onenote_list_notebooks

**What it does**: Lists all OneNote notebooks.

**When to use**: Browse notebooks, find notebooks.

**Arguments**: None

**Example LLM prompt**: "List all my OneNote notebooks"

---

### onenote_get_notebook

**What it does**: Gets detailed information for a specific notebook.

**When to use**: View notebook details, section count.

**Arguments**:
- `id` (required): Notebook ID

**Example LLM prompt**: "Get details for notebook ABC123"

---

### onenote_list_sections

**What it does**: Lists all sections in a notebook.

**When to use**: Browse sections, find notes.

**Arguments**:
- `notebookId` (required): Notebook ID

**Example LLM prompt**: "List all sections in notebook ABC123"

---

### onenote_get_section

**What it does**: Gets detailed information for a specific section.

**When to use**: View section details, page count.

**Arguments**:
- `id` (required): Section ID

**Example LLM prompt**: "Get details for section XYZ456"

---

### onenote_list_pages

**What it does**: Lists all pages in a section.

**When to use**: Browse pages, find notes.

**Arguments**:
- `sectionId` (required): Section ID

**Example LLM prompt**: "List all pages in section XYZ456"

---

### onenote_get_page

**What it does**: Gets detailed information for a specific page.

**When to use**: View page metadata, title, created date.

**Arguments**:
- `id` (required): Page ID

**Example LLM prompt**: "Get details for page 123"

---

### onenote_create_page

**What it does**: Creates a new page in a section.

**When to use**: Create notes, add content.

**Arguments**:
- `sectionId` (required): Section ID
- `title` (required): Page title
- `content` (optional): Page HTML content

**Example LLM prompt**: "Create a new page called 'Meeting Notes' in section XYZ456"

---

### onenote_update_page

**What it does**: Updates page content.

**When to use**: Edit notes, append content.

**Arguments**:
- `id` (required): Page ID
- `content` (required): Updated HTML content

**Example LLM prompt**: "Update page 123 with new content"

---

### onenote_search

**What it does**: Searches OneNote pages and notes.

**When to use**: Find notes by keyword, locate content.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for pages containing 'project notes'"

---

### onenote_get_recent_pages

**What it does**: Gets recently modified pages.

**When to use**: Find recently edited notes.

**Arguments**:
- `top` (optional): Number of pages (default 20)

**Example LLM prompt**: "Get my 10 most recently modified pages"

---

## OneNote API Notes

- **Microsoft Graph**: Uses Microsoft Graph API endpoints
- **Content format**: Pages use HTML content
- **Hierarchical structure**: Notebooks > Sections > Pages
- **Search**: Searches page titles and content
- **Recent pages**: Sorted by last modified date

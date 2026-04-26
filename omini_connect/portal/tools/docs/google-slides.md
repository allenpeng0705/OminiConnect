# Google Slides Tools

Provider: `google-slides` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Slides API. They allow AI agents to manage presentations, pages, shapes, and perform batch updates. **Requires Google OAuth2 with Slides permissions.**

## Authentication

**Nango OAUTH2 (Google Slides)**:
- User authenticates via OAuth2 with Presentations scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://slides.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_slides_list_presentations` | List presentations | GET | /v1/presentations |
| `google_slides_get_presentation` | Get presentation details | GET | /v1/presentations/{presentation_id} |
| `google_slides_create_presentation` | Create presentation | POST | /v1/presentations |
| `google_slides_get_page` | Get page details | GET | /v1/presentations/{presentation_id}/pages/{page_id} |
| `google_slides_list_pages` | List pages | GET | /v1/presentations/{presentation_id} |
| `google_slides_batch_update` | Batch update | POST | /v1/presentations/{presentation_id}:batchUpdate |
| `google_slides_get_shape` | Get shape | GET | /v1/presentations/{presentation_id}/pages/{page_id} |
| `google_slides_list_templates` | List templates | GET | /v1/presentations/{presentation_id} |
| `google_slides_copy_slide` | Copy slide | POST | /v1/presentations/{presentation_id}:batchUpdate |
| `google_slides_delete_page` | Delete page | POST | /v1/presentations/{presentation_id}:batchUpdate |

---

## Tool Details

### google_slides_list_presentations

**What it does**: Lists presentations in Google Drive.

**When to use**: Browse available presentations.

**Arguments**: None

**Example LLM prompt**: "List my presentations"

---

### google_slides_get_presentation

**What it does**: Gets presentation details.

**When to use**: Get presentation metadata.

**Arguments**:
- `presentation_id` (required): Presentation ID

**Example LLM prompt**: "Get details for presentation abc123"

---

### google_slides_create_presentation

**What it does**: Creates a new presentation.

**When to use**: Create new slide decks.

**Arguments**:
- `title` (required): Presentation title

**Example LLM prompt**: "Create a presentation titled 'Q1 Report'"

---

### google_slides_get_page

**What it does**: Gets page details.

**When to use**: Get slide content.

**Arguments**:
- `presentation_id` (required): Presentation ID
- `page_id` (required): Page ID

**Example LLM prompt**: "Get page xyz789 in presentation abc123"

---

### google_slides_list_pages

**What it does**: Lists all pages in a presentation.

**When to use**: See slide structure.

**Arguments**:
- `presentation_id` (required): Presentation ID

**Example LLM prompt**: "List all pages in presentation abc123"

---

### google_slides_batch_update

**What it does**: Performs batch updates on a presentation.

**When to use**: Make multiple changes.

**Arguments**:
- `presentation_id` (required): Presentation ID
- `requests` (required): Array of update requests

**Example LLM prompt**: "Update presentation abc123 with new text"

---

### google_slides_get_shape

**What it does**: Gets shape details on a page.

**When to use**: Get shape properties.

**Arguments**:
- `presentation_id` (required): Presentation ID
- `page_id` (required): Page ID

**Example LLM prompt**: "Get shapes on page xyz789"

---

### google_slides_list_templates

**What it does**: Lists page templates in a presentation.

**When to use**: See available layouts.

**Arguments**:
- `presentation_id` (required): Presentation ID

**Example LLM prompt**: "List templates in presentation abc123"

---

### google_slides_copy_slide

**What it does**: Copies a slide within a presentation.

**When to use**: Duplicate slides.

**Arguments**:
- `presentation_id` (required): Presentation ID
- `slide_id` (required): Slide ID to copy
- `insertion_index` (optional): Insertion index

**Example LLM prompt**: "Copy slide xyz789 in presentation abc123"

---

### google_slides_delete_page

**What it does**: Deletes a page from a presentation.

**When to use**: Remove slides.

**Arguments**:
- `presentation_id` (required): Presentation ID
- `page_id` (required): Page ID to delete

**Example LLM prompt**: "Delete page xyz789 from presentation abc123"

---

## Google Slides API Notes

- **Presentation ID**: Found in the presentation URL
- **Page ID**: Unique identifier for each slide
- **Batch updates**: For text, shapes, positioning
- **Requests**: Specific update operations
- **Object IDs**: Unique IDs for each object on a slide

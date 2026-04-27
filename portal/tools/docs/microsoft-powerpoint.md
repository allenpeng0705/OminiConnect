# Microsoft PowerPoint Tools

Provider: `microsoft-powerpoint` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft PowerPoint API. They allow AI agents to manage presentations, slides, and shapes in OneDrive. **Requires PowerPoint OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft PowerPoint)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `powerpoint_list_presentations` | List presentations | GET | /v1.0/me/drive/root/children |
| `powerpoint_get_presentation` | Get presentation details | GET | /v1.0/me/drive/items/{itemId} |
| `powerpoint_list_slides` | List slides | GET | /v1.0/me/drive/items/{itemId}/presentation/slides |
| `powerpoint_get_slide` | Get slide details | GET | /v1.0/me/drive/items/{itemId}/presentation/slides/{slideId} |
| `powerpoint_list_shapes` | List shapes on a slide | GET | /v1.0/me/drive/items/{itemId}/presentation/slides/{slideId}/shapes |
| `powerpoint_get_shape` | Get shape details | GET | /v1.0/me/drive/items/{itemId}/presentation/slides/{slideId}/shapes/{shapeId} |
| `powerpoint_create_slide` | Create a new slide | POST | /v1.0/me/drive/items/{itemId}/presentation/slides |
| `powerpoint_delete_slide` | Delete a slide | DELETE | /v1.0/me/drive/items/{itemId}/presentation/slides/{slideId} |
| `powerpoint_update_slide` | Update slide content | PATCH | /v1.0/me/drive/items/{itemId}/presentation/slides/{slideId} |
| `powerpoint_list_themes` | List themes | GET | /v1.0/me/drive/root/children |

---

## Tool Details

### powerpoint_list_presentations

**What it does**: Lists all PowerPoint presentations accessible to the user.

**When to use**: Find presentations, browse files.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all PowerPoint files in my OneDrive"

---

### powerpoint_get_presentation

**What it does**: Gets details of a specific presentation.

**When to use**: Check presentation metadata.

**Arguments**:
- `itemId` (required): Presentation item ID

**Example LLM prompt**: "Get details for presentation ITEM-12345"

---

### powerpoint_list_slides

**What it does**: Lists all slides in a presentation.

**When to use**: Navigate slides, get slide count.

**Arguments**:
- `itemId` (required): Presentation item ID

**Example LLM prompt**: "List all slides in presentation ITEM-12345"

---

### powerpoint_get_slide

**What it does**: Gets details of a specific slide.

**When to use**: Check slide content, layout info.

**Arguments**:
- `itemId` (required): Presentation item ID
- `slideId` (required): Slide ID

**Example LLM prompt**: "Get details for slide 1 in presentation ITEM-12345"

---

### powerpoint_list_shapes

**What it does**: Lists all shapes on a specific slide.

**When to use**: Analyze slide content, find elements.

**Arguments**:
- `itemId` (required): Presentation item ID
- `slideId` (required): Slide ID

**Example LLM prompt**: "List all shapes on slide 1"

---

### powerpoint_get_shape

**What it does**: Gets details of a specific shape on a slide.

**When to use**: Check shape properties, text content.

**Arguments**:
- `itemId` (required): Presentation item ID
- `slideId` (required): Slide ID
- `shapeId` (required): Shape ID

**Example LLM prompt**: "Get details for shape SHAPE-12345 on slide 1"

---

### powerpoint_create_slide

**What it does**: Creates a new slide in a presentation.

**When to use**: Add new slides to presentations.

**Arguments**:
- `itemId` (required): Presentation item ID
- `layoutId` (optional): Slide layout ID to use
- `position` (optional): Position to insert (0-based index)

**Example LLM prompt**: "Create a new slide in presentation ITEM-12345"

---

### powerpoint_delete_slide

**What it does**: Deletes a slide from a presentation.

**When to use**: Remove unwanted slides.

**Arguments**:
- `itemId` (required): Presentation item ID
- `slideId` (required): Slide ID to delete

**Example LLM prompt**: "Delete slide 5 from presentation ITEM-12345"

---

### powerpoint_update_slide

**What it does**: Updates content on a slide.

**When to use**: Modify slide title, update content.

**Arguments**:
- `itemId` (required): Presentation item ID
- `slideId` (required): Slide ID
- `title` (optional): Slide title

**Example LLM prompt**: "Update the title of slide 5 in presentation ITEM-12345"

---

### powerpoint_list_themes

**What it does**: Lists available themes for presentations.

**When to use**: Find themes, apply consistent styling.

**Arguments**:
- `$filter` (optional): Filter for theme files

**Example LLM prompt**: "List available PowerPoint themes"

---

## PowerPoint API Notes

- **Presentations**: PowerPoint files (.pptx) in OneDrive
- **Slides**: Individual slides with content
- **Shapes**: Text boxes, images, etc.
- **Layouts**: Slide templates
- **Item IDs**: OneDrive item identifiers

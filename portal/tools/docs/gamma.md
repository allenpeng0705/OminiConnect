# Gamma Tools

Provider: `gamma` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Gamma API. They allow AI agents to manage folders, cards (presentations, documents, webpages), collaborators, and templates. **Requires Gamma API key.**

## Authentication

**Nango API_KEY**:
- User provides their Gamma API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://public-api.gamma.app`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gamma_list_folders` | List folders | GET | /v1.0/folders |
| `gamma_get_folder` | Get folder details | GET | /v1.0/folders/{folder_id} |
| `gamma_list_cards` | List cards | GET | /v1.0/cards |
| `gamma_get_card` | Get card details | GET | /v1.0/cards/{card_id} |
| `gamma_create_card` | Create a card | POST | /v1.0/cards |
| `gamma_update_card` | Update a card | PUT | /v1.0/cards/{card_id} |
| `gamma_delete_card` | Delete a card | DELETE | /v1.0/cards/{card_id} |
| `gamma_list_collaborators` | List collaborators | GET | /v1.0/cards/{card_id}/collaborators |
| `gamma_add_collaborator` | Add collaborator | POST | /v1.0/cards/{card_id}/collaborators |
| `gamma_list_templates` | List templates | GET | /v1.0/templates |

---

## Tool Details

### gamma_list_folders

**What it does**: Lists all folders in Gamma workspace.

**When to use**: Browse workspace structure.

**Arguments**: None

**Example LLM prompt**: "List all folders in my Gamma workspace"

---

### gamma_get_folder

**What it does**: Gets details of a specific folder.

**When to use**: Get folder contents and metadata.

**Arguments**:
- `folder_id` (required): Folder ID

**Example LLM prompt**: "Get details for folder abc123"

---

### gamma_list_cards

**What it does**: Lists all cards (presentations, docs, webpages) in Gamma.

**When to use**: Find cards, filter by folder.

**Arguments**:
- `folder_id` (optional): Filter by folder ID

**Example LLM prompt**: "List all cards in the marketing folder"

---

### gamma_get_card

**What it does**: Gets detailed information about a specific card.

**When to use**: Get card content, settings, and metadata.

**Arguments**:
- `card_id` (required): Card ID

**Example LLM prompt**: "Get details for card abc123"

---

### gamma_create_card

**What it does**: Creates a new card (presentation, doc, or webpage).

**When to use**: Generate new content in Gamma.

**Arguments**:
- `title` (required): Card title
- `type` (required): Card type (presentation, document, webpage)
- `folder_id` (optional): Folder ID

**Example LLM prompt**: "Create a new presentation titled Q4 Report"

---

### gamma_update_card

**What it does**: Updates an existing card's content or settings.

**When to use**: Modify card content or title.

**Arguments**:
- `card_id` (required): Card ID
- `title` (optional): New title
- `content` (optional): New content

**Example LLM prompt**: "Update card abc123 with new title"

---

### gamma_delete_card

**What it does**: Deletes a card from Gamma.

**When to use**: Remove unwanted cards.

**Arguments**:
- `card_id` (required): Card ID

**Example LLM prompt**: "Delete card abc123"

---

### gamma_list_collaborators

**What it does**: Lists all collaborators on a card.

**When to use**: Check who has access to a card.

**Arguments**:
- `card_id` (required): Card ID

**Example LLM prompt**: "List collaborators on card abc123"

---

### gamma_add_collaborator

**What it does**: Adds a collaborator to a card.

**When to use**: Share a card with team members.

**Arguments**:
- `card_id` (required): Card ID
- `email` (required): Collaborator email
- `role` (optional): Role (viewer, editor)

**Example LLM prompt**: "Add john@company.com as editor to card abc123"

---

### gamma_list_templates

**What it does**: Lists available templates for creating cards.

**When to use**: Browse template options.

**Arguments**: None

**Example LLM prompt**: "List available templates"

---

## Gamma API Notes

- **Card types**: presentation, document, webpage
- **Collaborator roles**: viewer (read-only), editor (can modify)
- **Templates**: Pre-built layouts for quick card creation

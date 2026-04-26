# Freepik Tools

Provider: `freepik` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Freepik API. They allow AI agents to browse creative resources, manage collections, and generate AI images. Freepik is a platform for creative assets including photos, vectors, and illustrations.

## Authentication

**Nango API_KEY**:
- User provides their Freepik API key via Nango Connect
- Key is passed in the x-freepik-api-key header
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `freepik_list_resources` | List resources | GET | /v1/resources |
| `freepik_get_resource` | Get resource details | GET | /v1/resources/{id} |
| `freepik_list_collections` | List collections | GET | /v1/collections |
| `freepik_get_collection` | Get collection details | GET | /v1/collections/{id} |
| `freepik_list_ai_styles` | List AI styles | GET | /v1/ai/styles |
| `freepik_generate_ai` | Generate AI image | POST | /v1/ai/generate |
| `freepik_list_categories` | List categories | GET | /v1/categories |
| `freepik_get_user` | Get user info | GET | /v1/user |
| `freepik_search` | Search resources | GET | /v1/search |
| `freepik_get_editorial` | Get editorial content | GET | /v1/editorial |

---

## Tool Details

### freepik_list_resources

**What it does**: Lists all available resources.

**When to use**: Browse creative assets.

**Arguments**:
- `type` (optional): Filter by type (photo, vector, video)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List photo resources"

---

### freepik_get_resource

**What it does**: Gets details of a specific resource.

**When to use**: View resource information, download.

**Arguments**:
- `id` (required): Resource ID

**Example LLM prompt**: "Get resource abc123"

---

### freepik_list_collections

**What it does**: Lists all user collections.

**When to use**: Browse saved collections.

**Arguments**: None

**Example LLM prompt**: "List my collections"

---

### freepik_get_collection

**What it does**: Gets details of a specific collection.

**When to use**: View collection contents.

**Arguments**:
- `id` (required): Collection ID

**Example LLM prompt**: "Get collection xyz789"

---

### freepik_list_ai_styles

**What it does**: Lists available AI generation styles.

**When to use**: View AI image generation options.

**Arguments**: None

**Example LLM prompt**: "List available AI styles"

---

### freepik_generate_ai

**What it does**: Generates an image using AI.

**When to use**: Create AI-generated artwork.

**Arguments**:
- `prompt` (required): Text prompt for generation
- `style` (optional): AI style to use

**Example LLM prompt**: "Generate an AI image of a sunset over mountains"

---

### freepik_list_categories

**What it does**: Lists all content categories.

**When to use**: Browse content categories.

**Arguments**: None

**Example LLM prompt**: "List all categories"

---

### freepik_get_user

**What it does**: Gets current user information.

**When to use**: Verify authentication, get account info.

**Arguments**: None

**Example LLM prompt**: "Get my Freepik profile"

---

### freepik_search

**What it does**: Searches for resources.

**When to use**: Find specific creative assets.

**Arguments**:
- `q` (required): Search query
- `type` (optional): Filter by type

**Example LLM prompt**: "Search for 'business meeting'"

---

### freepik_get_editorial

**What it does**: Gets editorial content.

**When to use**: View featured content.

**Arguments**: None

**Example LLM prompt**: "Get editorial content"

---

## Freepik API Notes

- **Resources**: Photos, vectors, videos, and illustrations
- **Collections**: Saved groups of resources
- **AI Generation**: Text-to-image AI generation
- **Categories**: Content organization topics
- **Editorial**: Featured and curated content
- **API Key Format**: Key starts with `FPSX` prefix

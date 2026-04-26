# Paligo Tools

Provider: `paligo` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Paligo documentation platform API. They allow AI agents to manage topics, publications, and assets for technical documentation. **Requires Paligo Basic authentication.**

## Authentication

**Basic Auth**:
- User provides Paligo username and password
- Credentials passed via Nango
- Base URL: `https://{subdomain}.paligoapp.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `paligo_list_topics` | List topics/documents | GET | /api/v1/topics |
| `paligo_get_topic` | Get topic details | GET | /api/v1/topics/{id} |
| `paligo_create_topic` | Create topic | POST | /api/v1/topics |
| `paligo_update_topic` | Update topic | PUT | /api/v1/topics/{id} |
| `paligo_list_publications` | List publications | GET | /api/v1/publications |
| `paligo_get_publication` | Get publication details | GET | /api/v1/publications/{id} |
| `paligo_list_assets` | List assets | GET | /api/v1/assets |
| `paligo_get_asset` | Get asset details | GET | /api/v1/assets/{id} |
| `paligo_search_content` | Search content | GET | /api/v1/search |
| `paligo_get_tree` | Get content tree | GET | /api/v1/treepaths |

---

## Tool Details

### paligo_list_topics

**What it does**: Lists all topics and documents in Paligo.

**When to use**: Browse documentation, find articles.

**Arguments**:
- `limit` (optional): Number of topics (default 20)

**Example LLM prompt**: "List all topics in the documentation"

---

### paligo_get_topic

**What it does**: Gets detailed information for a specific topic.

**When to use**: View topic content, check metadata.

**Arguments**:
- `id` (required): Topic ID

**Example LLM prompt**: "Get details for topic 12345"

---

### paligo_create_topic

**What it does**: Creates a new topic in Paligo.

**When to use**: Add new documentation, create articles.

**Arguments**:
- `title` (required): Topic title
- `content` (optional): Topic content (XML)
- `type` (optional): Topic type

**Example LLM prompt**: "Create a new topic called 'Getting Started Guide'"

---

### paligo_update_topic

**What it does**: Updates an existing topic.

**When to use**: Modify documentation, update content.

**Arguments**:
- `id` (required): Topic ID
- `title` (optional): Topic title
- `content` (optional): Topic content (XML)

**Example LLM prompt**: "Update topic 12345 with new content"

---

### paligo_list_publications

**What it does**: Lists all publications in Paligo.

**When to use**: Browse publications, find document sets.

**Arguments**: None

**Example LLM prompt**: "List all publications"

---

### paligo_get_publication

**What it does**: Gets detailed information for a specific publication.

**When to use**: View publication structure, included topics.

**Arguments**:
- `id` (required): Publication ID

**Example LLM prompt**: "Get details for publication 456"

---

### paligo_list_assets

**What it does**: Lists all assets in Paligo.

**When to use**: Browse images, media, attachments.

**Arguments**:
- `limit` (optional): Number of assets (default 20)

**Example LLM prompt**: "List all images in the asset library"

---

### paligo_get_asset

**What it does**: Gets detailed information for a specific asset.

**When to use**: View asset metadata, check usage.

**Arguments**:
- `id` (required): Asset ID

**Example LLM prompt**: "Get details for asset 789"

---

### paligo_search_content

**What it does**: Searches for content in Paligo.

**When to use**: Find specific documentation, locate topics.

**Arguments**:
- `query` (required): Search query

**Example LLM prompt**: "Search for 'installation' in the documentation"

---

### paligo_get_tree

**What it does**: Gets the content tree structure.

**When to use**: View documentation hierarchy, understand structure.

**Arguments**: None

**Example LLM prompt**: "Get the content tree structure"

---

## Paligo Notes

- **Documentation CMS**: Technical documentation platform
- **Topics**: Individual documentation articles
- **Publications**: Collections of topics
- **Assets**: Images, media files, attachments
- **Content format**: Topics use XML content
